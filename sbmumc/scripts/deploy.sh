#!/bin/bash
# ============================================================
# SBMUMC Universal Deployment Script
# Deploys to AWS, GCP, Azure, or Kubernetes
# ============================================================

set -euo pipefail

# Configuration
REGISTRY="${REGISTRY:-ghcr.io/benwellonedge28}"
IMAGE="${REGISTRY}/sbmumc:0.1.0"
NAMESPACE="${NAMESPACE:-sbmumc}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_step() { echo -e "${BLUE}[STEP]${NC} $1"; }

# Detect cloud provider
detect_cloud() {
    if [ -n "${AWS_REGION:-}" ]; then
        echo "aws"
    elif [ -n "${GCP_PROJECT:-}" ]; then
        echo "gcp"
    elif [ -n "${AZURE_SUBSCRIPTION_ID:-}" ]; then
        echo "azure"
    elif command -v kubectl &> /dev/null; then
        echo "kubernetes"
    else
        echo "docker"
    fi
}

# Deploy to Docker
deploy_docker() {
    log_step "Deploying to Docker..."

    docker build -t "${IMAGE}" -f Dockerfile.multiplatform .
    docker run -d \
        --name sbmumc \
        -p 8080:8080 \
        -p 8081:8081 \
        -p 8082:8082 \
        -p 8083:8083 \
        -v sbmumc-data:/app/data \
        -e RUST_LOG=info \
        "${IMAGE}"

    log_info "Docker deployment complete"
}

# Deploy to Kubernetes
deploy_kubernetes() {
    log_step "Deploying to Kubernetes..."

    # Apply configurations
    kubectl apply -f k8s/deployment.yaml

    # Wait for deployment
    kubectl rollout status deployment/sbmumc -n "${NAMESPACE}" --timeout=300s

    log_info "Kubernetes deployment complete"
}

# Deploy to AWS
deploy_aws() {
    log_step "Deploying to AWS..."

    # ECR login
    aws ecr get-login-password | docker login --username AWS --password-stdin "${REGISTRY}"

    # Build and push
    docker build -t "${IMAGE}" -f Dockerfile.multiplatform .
    docker push "${IMAGE}"

    # Update Kubernetes deployment
    kubectl set image deployment/sbmumc sbmumc="${IMAGE}"

    log_info "AWS deployment complete"
}

# Deploy to GCP
deploy_gcp() {
    log_step "Deploying to GCP..."

    # Configure Docker
    gcloud auth configure-docker

    # Build and push
    docker build -t "${IMAGE}" -f Dockerfile.multiplatform .
    docker push "${IMAGE}"

    # Update Kubernetes deployment
    kubectl set image deployment/sbmumc sbmumc="${IMAGE}"

    log_info "GCP deployment complete"
}

# Deploy to Azure
deploy_azure() {
    log_step "Deploying to Azure..."

    # Azure Container Registry login
    az acr login --name sbmumc

    # Build and push
    docker build -t "${IMAGE}" -f Dockerfile.multiplatform .
    docker push "${IMAGE}"

    # Update Kubernetes deployment
    kubectl set image deployment/sbmumc sbmumc="${IMAGE}"

    log_info "Azure deployment complete"
}

# Deploy Helm chart
deploy_helm() {
    log_step "Deploying Helm chart..."

    # Add repo if needed
    helm repo add sbmumc https://charts.sbmumc.com || true
    helm repo update

    # Install/upgrade
    helm upgrade --install sbmumc k8s/helm \
        --namespace "${NAMESPACE}" \
        --create-namespace \
        --set image.repository="${REGISTRY}/sbmumc" \
        --set image.tag="0.1.0"

    log_info "Helm deployment complete"
}

# Verify deployment
verify() {
    log_step "Verifying deployment..."

    case "${CLOUD}" in
        kubernetes|k8s)
            kubectl get pods -n "${NAMESPACE}"
            kubectl get services -n "${NAMESPACE}"
            ;;
        docker)
            docker ps | grep sbmumc
            ;;
    esac

    log_info "Verification complete"
}

# Show help
show_help() {
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  docker      Deploy to local Docker"
    echo "  k8s         Deploy to Kubernetes"
    echo "  helm        Deploy using Helm chart"
    echo "  aws         Deploy to AWS EKS"
    echo "  gcp         Deploy to GCP GKE"
    echo "  azure       Deploy to Azure AKS"
    echo "  all         Deploy to all platforms"
    echo "  verify      Verify current deployment"
    echo "  clean       Clean up resources"
    echo ""
    echo "Options:"
    echo "  --registry URL    Container registry"
    echo "  --namespace NS    Kubernetes namespace"
    echo "  --image IMG       Docker image name"
    echo "  --help            Show this help"
}

# Clean up
clean() {
    log_step "Cleaning up..."

    case "${CLOUD}" in
        kubernetes|k8s)
            kubectl delete -f k8s/deployment.yaml || true
            ;;
        docker)
            docker stop sbmumc 2>/dev/null || true
            docker rm sbmumc 2>/dev/null || true
            ;;
    esac

    log_info "Cleanup complete"
}

# Main
main() {
    local command="${1:-docker}"
    shift || true

    CLOUD=$(detect_cloud)
    log_info "Detected platform: ${CLOUD}"

    case "${command}" in
        docker)
            deploy_docker
            ;;
        k8s|kubernetes)
            deploy_kubernetes
            ;;
        helm)
            deploy_helm
            ;;
        aws)
            deploy_aws
            ;;
        gcp)
            deploy_gcp
            ;;
        azure)
            deploy_azure
            ;;
        all)
            deploy_docker
            deploy_kubernetes
            ;;
        verify)
            verify
            ;;
        clean)
            clean
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            log_error "Unknown command: ${command}"
            show_help
            exit 1
            ;;
    esac
}

main "$@"