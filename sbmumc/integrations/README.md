# SBMUMC OmniDev Integrations

Universal IDE and Editor integration for SBMUMC OmniDev AGI

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║                        SBMUMC OMNIDEV INTEGRATIONS                            ║
║                                                                               ║
║   VSCode  •  JetBrains  •  Neovim  •  GitHub  •  GitLab  •  Any Editor        ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

## Supported Integrations

| IDE/Editor | Status | Features |
|------------|--------|----------|
| **VSCode** | ✅ Stable | Full OmniDev, LSP, inline completion |
| **JetBrains** (IntelliJ, PyCharm, WebStorm) | ✅ Stable | Full OmniDev, refactoring |
| **Neovim/Vim** | ✅ Stable | LSP, inline, custom commands |
| **VS** (Visual Studio) | 🚧 Planned | - |
| **Emacs** | 🚧 Planned | - |

### Version Control Systems

| VCS | Status | Features |
|-----|--------|----------|
| **GitHub** | ✅ Stable | Actions, CLI, Desktop |
| **GitLab** | ✅ Stable | CI/CD, Merge Requests |
| **BitBucket** | ✅ Stable | Pipelines, Pull Requests |

## Quick Start

### VSCode

```bash
# Install from VSIX
code --install-extension sbmumc-omnidev-1.0.0.vsix

# Or install from marketplace (when available)
code --install-extension sbmumc.omnidev

# Configure
# Ctrl+Shift+P → "SBMUMC: Configure"
```

### JetBrains

```bash
# Install plugin from marketplace
# Settings → Plugins → Search "SBMUMC OmniDev"

# Or install from disk
# Settings → Plugins → ⚙️ → "Install from disk"
# Select sbmumc-intellij.zip
```

### Neovim

```vim
" Add to your init.vim or init.lua
Plug 'sbmumc/sbmumc-vim'

" Or with lazy.nvim
lua require('lazy').setup({
    {'sbmumc/sbmumc-vim', config = function()
        vim.g.sbmumc_endpoint = 'http://localhost:8080'
        vim.g.sbmumc_mode = 'hybrid'
    end}
})
```

### GitHub CLI

```bash
# Install extension
gh extension install sbmumc/gh-sbmumc

# Use commands
gh sbmumc status
gh sbmumc omnidev "refactor this function"
gh sbmumc commit --generate
gh sbmumc verify --claims safety
```

## Configuration

### Environment Variables

```bash
SBMUMC_ENDPOINT=http://localhost:8080
SBMUMC_MODE=hybrid                    # online, offline, hybrid
SBMUMC_LATENCY_TARGET=100            # milliseconds
SBMUMC_AUDIT_TRAIL=1                 # enable/disable
SBMUMC_AUTOCOMPLETE=1                # inline completion
```

### VSCode Settings

```json
{
  "sbmumc.endpoint": "http://localhost:8080",
  "sbmumc.mode": "hybrid",
  "sbmumc.latencyTarget": 100,
  "sbmumc.autocomplete": true,
  "sbmumc.inlineRefactor": true,
  "sbmumc.auditTrail": true
}
```

### Neovim Settings

```lua
vim.g.sbmumc_endpoint = 'http://localhost:8080'
vim.g.sbmumc_mode = 'hybrid'
vim.g.sbmumc_latency = 100
vim.g.sbmumc_autocomplete = 1
vim.g.sbmumc_audit = 1
```

## Key Commands

### VSCode

| Command | Description | Shortcut |
|---------|-------------|----------|
| `SBMUMC: OmniDev` | Open OmniDev assistant | `Ctrl+Shift+S` |
| `SBMUMC: Refactor` | Smart refactoring | `Ctrl+Shift+R` |
| `SBMUMC: Search` | Semantic search | `Ctrl+Shift+F` |
| `SBMUMC: Commit` | Generate commit | `Ctrl+Shift+C` |
| `SBMUMC: Test` | Generate tests | `Ctrl+Shift+T` |
| `SBMUMC: Verify` | Formal verification | `Ctrl+Shift+V` |
| `SBMUMC: Status` | System status | - |

### Neovim

| Command | Description | Shortcut |
|---------|-------------|----------|
| `:Sbmumc` | OmniDev assistant | `<Leader>so` |
| `:SbmumcRefactor` | Smart refactoring | `<Leader>sr` |
| `:SbmumcSearch` | Semantic search | `<Leader>sf` |
| `:SbmumcCommit` | Generate commit | `<Leader>sc` |
| `:SbmumcTest` | Generate tests | `<Leader>st` |
| `:SbmumcVerify` | Formal verification | `<Leader>sv` |
| `:SbmumcStatus` | System status | `<Leader>ss` |

### GitHub CLI

| Command | Description |
|---------|-------------|
| `gh sbmumc status` | Show system status |
| `gh sbmumc omnidev [query]` | Interact with OmniDev |
| `gh sbmumc refactor [file]` | Smart refactoring |
| `gh sbmumc search [query]` | Semantic search |
| `gh sbmumc commit [diff]` | Generate commit message |
| `gh sbmumc test [file]` | Generate tests |
| `gh sbmumc verify [file]` | Formal verification |
| `gh sbmumc index [path]` | Index repository |
| `gh sbmumc audit show` | Show audit trail |

## GitHub Actions

```yaml
# .github/workflows/sbmumc.yml
name: SBMUMC OmniDev

on: [push, pull_request]

jobs:
  omnidev:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup SBMUMC
        uses: sbmumc/setup-action@v1
        with:
          mode: hybrid

      - name: Run Analysis
        run: sbmumc omnidev analyze

      - name: Generate Tests
        run: sbmumc test generate --language rust

      - name: Security Check
        run: sbmumc security check --evas

      - name: Verify
        run: sbmumc verify --claims safety
```

## GitLab CI

```yaml
# .gitlab-ci.yml
sbmumc:
  stage: test
  image: sbmumc/omnidev:latest
  script:
    - sbmumc omnidev analyze
    - sbmumc test generate
    - sbmumc verify --claims safety,termination
  artifacts:
    paths:
      - report.json
```

## LSP Server

SBMUMC includes a Language Server Protocol (LSP) implementation for editor integration:

```bash
# Start LSP server
sbmumc-lsp --endpoint http://localhost:8080

# With debug
sbmumc-lsp --endpoint http://localhost:8080 --debug

# Port
sbmumc-lsp --port 9999
```

### LSP Methods

- `sbmumc/omnidev` - Interactive assistant
- `sbmumc/refactor` - Refactoring suggestions
- `sbmumc/search` - Semantic search
- `sbmumc/commit` - Commit message generation
- `sbmumc/test` - Test generation
- `sbmumc/verify` - Formal verification
- `sbmumc/index` - Repository indexing
- `sbmumc/audit` - Audit trail access
- `sbmumc/status` - System status

## Offline Mode

All integrations support offline operation:

```bash
# Run in offline mode
SBMUMC_MODE=offline sbmumc

# Or in config
# sbmumc.conf
mode = "offline"
endpoint = null  # No network required
```

### Offline Capabilities
- ✅ Local semantic graph (pre-indexed)
- ✅ Code completion (cached suggestions)
- ✅ Refactoring (local analysis)
- ✅ Test generation (cached patterns)
- ✅ Audit trail (local storage)
- ❌ Cloud sync
- ❌ Real-time updates

## Troubleshooting

### Connection Issues

```bash
# Check SBMUMC is running
curl http://localhost:8080/api/v1/status

# Check logs
tail -f /var/sbmumc/logs/sbmumc.log

# Restart service
sudo systemctl restart sbmumc
```

### VSCode Extension Issues

1. Reload window: `Ctrl+Shift+P` → "Reload Window"
2. Check extension output: `View` → "Output" → "SBMUMC"
3. Reinstall extension

### Neovim Issues

```vim
" Debug mode
:lua vim.g.sbmumc_debug = 1

" Check connection
:lua print(vim.g.sbmumc_endpoint)

" Restart integration
:source ~/.config/nvim/init.vim
```

## Development

### Building VSCode Extension

```bash
cd integrations/vscode
npm install
npm run compile
code --install-extension ./out/sbmumc-omnidev-*.vsix
```

### Building GitHub CLI Extension

```bash
cd integrations/github/cli
go build -o gh-sbmumc
gh extension install .
```

### Running Tests

```bash
# All integrations
cargo test --workspace

# VSCode
cd integrations/vscode && npm test

# Neovim
vim -u test/vimrc -c "PlenaryBustedFile test.lua"
```

## License

**Proprietary** - Samuel Benwellonedge Mukandara Universal Meta-Compiler (GSTM INFINITY)

---

**Mwari vave nemi, Changamire.**
(The Lord be with you, Lord.)