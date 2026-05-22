#!/bin/bash
# ============================================================
# SBMUMC Universal Build Script
# Builds multi-platform Docker images
# ============================================================

set -euo pipefail

# Configuration
REGISTRY="${REGISTRY:-ghcr.io/benwellonedge28}"
IMAGE_NAME="${IMAGE_NAME:-sbmumc}"
VERSION="${VERSION:-0.1.0}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Parse arguments
BUILD_PLATFORM="${BUILD_PLATFORM:-linux/amd64,linux/arm64}"
PUSH_IMAGE="${PUSH_IMAGE:-true}"
BUILD_CACHE="${BUILD_CACHE:-true}"

while [[ $# -gt 0 ]]; do
    case $1 in
        --registry)
            REGISTRY="$2"
            shift 2
            ;;
        --version)
            VERSION="$2"
            shift 2
            ;;
        --platform)
            BUILD_PLATFORM="$2"
            shift 2
            ;;
        --no-push)
            PUSH_IMAGE="false"
            shift
            ;;
        --no-cache)
            BUILD_CACHE="false"
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --registry REGISTRY    Container registry (default: ghcr.io/benwellonedge28)"
            echo "  --version VERSION      Version tag (default: 0.1.0)"
            echo "  --platform PLATFORM    Build platforms comma-separated (default: linux/amd64,linux/arm64)"
            echo "  --no-push              Don't push images after build"
            echo "  --no-cache             Don't use Docker build cache"
            echo "  --help                 Show this help"
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Full image reference
FULL_IMAGE="${REGISTRY}/${IMAGE_NAME}:${VERSION}"
LATEST_IMAGE="${REGISTRY}/${IMAGE_NAME}:latest"

log_info "Building SBMUMC Docker images"
log_info "Registry: ${REGISTRY}"
log_info "Version: ${VERSION}"
log_info "Platforms: ${BUILD_PLATFORM}"

# Determine build type
DOCKER_BUILDKIT="${BUILD_CACHE:+--build-arg BUILDKIT_INLINE_CACHE=1}"
DOCKER_PLATFORM_ARG="--platform ${BUILD_PLATFORM}"

# Build function
build_image() {
    local target="$1"
    local tag="$2"

    log_info "Building ${tag} (target: ${target})..."

    local build_args=(
        --target "$target"
        --tag "${REGISTRY}/${IMAGE_NAME}:${tag}"
        --build-arg VERSION="${VERSION}"
        --build-arg BUILD_DATE="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
        --build-arg VCS_REF="$(git rev-parse HEAD 2>/dev/null || echo 'unknown')"
    )

    if [ "${BUILD_CACHE}" = "false" ]; then
        build_args+=(--no-cache)
    fi

    docker build ${build_args[@]} -f Dockerfile.multiplatform .

    return 0
}

# Multi-platform build using buildx
build_multiplatform() {
    log_info "Building multi-platform image..."

    # Create buildx builder if needed
    docker buildx create --use --name sbmumc-builder 2>/dev/null || \
    docker buildx use sbmumc-builder 2>/dev/null || true

    # Inspect builder
    docker buildx inspect --bootstrap

    # Build with buildx
    local buildx_args=(
        buildx build
        --platform "${BUILD_PLATFORM}"
        --tag "${FULL_IMAGE}"
        --tag "${LATEST_IMAGE}"
        --build-arg VERSION="${VERSION}"
        --build-arg BUILD_DATE="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
        --build-arg VCS_REF="$(git rev-parse HEAD 2>/dev/null || echo 'unknown')"
        --push
    )

    if [ "${BUILD_CACHE}" = "false" ]; then
        buildx_args+=(--no-cache)
    else
        buildx_args+=(--cache-from type=gha --cache-to type=gha,mode=max)
    fi

    docker "${buildx_args[@]}" -f Dockerfile.multiplatform .
}

# Alternative: Build individual platforms
build_individual() {
    local platforms=($(echo "${BUILD_PLATFORM}" | tr ',' ' '))

    for platform in "${platforms[@]}"; do
        local arch=$(basename "$platform")
        local target="final-${arch}"

        if [ "$arch" = "amd64" ]; then
            target="final-amd64"
        elif [ "$arch" = "arm64" ]; then
            target="final-arm64"
        fi

        build_image "$target" "${VERSION}-${arch}"

        if [ "${PUSH_IMAGE}" = "true" ]; then
            docker push "${REGISTRY}/${IMAGE_NAME}:${VERSION}-${arch}"
        fi
    done

    # Create manifest
    docker manifest create "${FULL_IMAGE}" \
        "${REGISTRY}/${IMAGE_NAME}:${VERSION}-amd64" \
        "${REGISTRY}/${IMAGE_NAME}:${VERSION}-arm64" 2>/dev/null || true

    docker manifest push "${FULL_IMAGE}"
}

# Push images
push_images() {
    log_info "Pushing images to registry..."

    docker push "${FULL_IMAGE}"
    docker push "${LATEST_IMAGE}"

    log_info "Images pushed successfully"
}

# Main execution
main() {
    # Check Docker
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed"
        exit 1
    fi

    # Check buildx
    if docker buildx version &> /dev/null; then
        log_info "Using Docker BuildKit for multi-platform build"
        build_multiplatform
    else
        log_warn "BuildKit not available, building individual platforms"
        build_individual
    fi

    # Push if requested
    if [ "${PUSH_IMAGE}" = "true" ]; then
        push_images
    fi

    log_info "Build completed successfully"
    log_info "Image: ${FULL_IMAGE}"
}

# Run
main "$@"