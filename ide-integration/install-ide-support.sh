#!/bin/bash
# CURSED IDE Integration Installation Script
# Installs advanced IDE support for VS Code, IntelliJ, and Vim/Neovim

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CURSED_ROOT="$(dirname "$SCRIPT_DIR")"

# Installation paths
HOME_DIR="$HOME"
VSCODE_EXTENSIONS_DIR="$HOME_DIR/.vscode/extensions"
INTELLIJ_PLUGINS_DIR="$HOME_DIR/.local/share/JetBrains"
VIM_DIR="$HOME_DIR/.vim"
NVIM_DIR="$HOME_DIR/.config/nvim"

# Utility functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_header() {
    echo -e "\n${PURPLE}=== $1 ===${NC}\n"
}

check_command() {
    if command -v "$1" &> /dev/null; then
        return 0
    else
        return 1
    fi
}

check_cursed_installation() {
    log_header "Checking CURSED Installation"
    
    if check_command "cursed-zig"; then
        local version=$(cursed-zig --version 2>/dev/null || echo "unknown")
        log_success "CURSED compiler found: $version"
    else
        log_error "CURSED compiler not found. Please install CURSED first."
        log_info "Visit: https://cursed-lang.org/install"
        exit 1
    fi
    
    if check_command "cursed-lsp"; then
        log_success "CURSED LSP server found"
    else
        log_warning "CURSED LSP server not found. Building from source..."
        cd "$CURSED_ROOT"
        if [ -f "build.zig" ]; then
            zig build cursed-lsp
            if [ -f "zig-out/bin/cursed-lsp" ]; then
                log_success "CURSED LSP server built successfully"
                # Add to PATH if not already there
                if ! check_command "cursed-lsp"; then
                    log_info "Consider adding $CURSED_ROOT/zig-out/bin to your PATH"
                fi
            else
                log_error "Failed to build CURSED LSP server"
                exit 1
            fi
        else
            log_error "Cannot build LSP server: build.zig not found"
            exit 1
        fi
    fi
}

install_vscode_extension() {
    log_header "Installing VS Code Extension"
    
    if ! check_command "code"; then
        log_warning "VS Code not found. Skipping VS Code extension installation."
        return
    fi
    
    local extension_dir="$SCRIPT_DIR/../cursed-vscode-extension-advanced"
    
    if [ ! -d "$extension_dir" ]; then
        log_error "VS Code extension directory not found: $extension_dir"
        return
    fi
    
    log_info "Building VS Code extension..."
    cd "$extension_dir"
    
    # Install dependencies
    if [ -f "package.json" ]; then
        if check_command "npm"; then
            npm install
            npm run compile
        elif check_command "yarn"; then
            yarn install
            yarn compile
        else
            log_error "Neither npm nor yarn found. Cannot build VS Code extension."
            return
        fi
    fi
    
    # Package extension
    if check_command "vsce"; then
        vsce package --no-dependencies
        local vsix_file=$(find . -name "*.vsix" | head -1)
        if [ -n "$vsix_file" ]; then
            code --install-extension "$vsix_file"
            log_success "VS Code extension installed successfully"
        else
            log_error "Failed to package VS Code extension"
        fi
    else
        log_warning "vsce not found. Installing extension manually..."
        
        # Manual installation
        local target_dir="$VSCODE_EXTENSIONS_DIR/cursed-language-advanced"
        mkdir -p "$target_dir"
        cp -r . "$target_dir/"
        log_success "VS Code extension installed manually"
    fi
    
    # Install recommended extensions
    log_info "Installing recommended VS Code extensions..."
    code --install-extension ms-vscode.vscode-json
    code --install-extension ms-vscode.vscode-typescript-next
    code --install-extension ms-vscode.debugger-extension-host
    
    log_info "VS Code extension installation complete!"
    log_info "Restart VS Code to activate the CURSED language support."
}

install_intellij_plugin() {
    log_header "Installing IntelliJ Plugin"
    
    local plugin_dir="$SCRIPT_DIR/../cursed-intellij-plugin"
    
    if [ ! -d "$plugin_dir" ]; then
        log_error "IntelliJ plugin directory not found: $plugin_dir"
        return
    fi
    
    log_info "Building IntelliJ plugin..."
    cd "$plugin_dir"
    
    if [ -f "build.gradle.kts" ]; then
        if check_command "gradle"; then
            gradle clean buildPlugin
            local plugin_zip=$(find build/distributions -name "*.zip" | head -1)
            if [ -n "$plugin_zip" ]; then
                log_success "IntelliJ plugin built: $plugin_zip"
                log_info "To install:"
                log_info "1. Open IntelliJ IDEA"
                log_info "2. Go to File > Settings > Plugins"
                log_info "3. Click 'Install Plugin from Disk'"
                log_info "4. Select: $plugin_zip"
            else
                log_error "Failed to build IntelliJ plugin"
            fi
        else
            log_warning "Gradle not found. IntelliJ plugin build skipped."
            log_info "To build manually:"
            log_info "1. Install Gradle"
            log_info "2. Run: gradle clean buildPlugin"
            log_info "3. Install the generated zip file in IntelliJ"
        fi
    else
        log_error "build.gradle.kts not found in IntelliJ plugin directory"
    fi
}

install_vim_plugin() {
    log_header "Installing Vim/Neovim Plugin"
    
    local vim_plugin_dir="$SCRIPT_DIR/../cursed-vim-advanced"
    
    if [ ! -d "$vim_plugin_dir" ]; then
        log_error "Vim plugin directory not found: $vim_plugin_dir"
        return
    fi
    
    # Install for Vim
    if check_command "vim"; then
        log_info "Installing for Vim..."
        mkdir -p "$VIM_DIR/pack/cursed/start"
        cp -r "$vim_plugin_dir" "$VIM_DIR/pack/cursed/start/cursed-vim"
        
        # Create help tags
        if [ -d "$VIM_DIR/pack/cursed/start/cursed-vim/doc" ]; then
            vim -c "helptags $VIM_DIR/pack/cursed/start/cursed-vim/doc" -c "quit"
        fi
        
        log_success "Vim plugin installed"
    else
        log_info "Vim not found. Skipping Vim installation."
    fi
    
    # Install for Neovim
    if check_command "nvim"; then
        log_info "Installing for Neovim..."
        mkdir -p "$NVIM_DIR/pack/cursed/start"
        cp -r "$vim_plugin_dir" "$NVIM_DIR/pack/cursed/start/cursed-nvim"
        
        # Create help tags
        if [ -d "$NVIM_DIR/pack/cursed/start/cursed-nvim/doc" ]; then
            nvim -c "helptags $NVIM_DIR/pack/cursed/start/cursed-nvim/doc" -c "quit"
        fi
        
        log_success "Neovim plugin installed"
        
        # Check for LSP support
        if nvim --version | grep -q "NVIM v0.[89]" || nvim --version | grep -q "NVIM v[1-9]"; then
            log_info "Neovim LSP support detected"
        else
            log_warning "Neovim version may not support built-in LSP. Consider upgrading to 0.8+"
        fi
    else
        log_info "Neovim not found. Skipping Neovim installation."
    fi
    
    # Install syntax highlighting for other editors
    install_syntax_highlighting
}

install_syntax_highlighting() {
    log_info "Installing syntax highlighting for additional editors..."
    
    # Tree-sitter grammar
    local tree_sitter_dir="$SCRIPT_DIR/../tree-sitter"
    if [ -d "$tree_sitter_dir" ]; then
        log_info "Tree-sitter grammar available for advanced editors"
    fi
    
    # Sublime Text
    local sublime_dir="$HOME_DIR/.config/sublime-text-3/Packages/User"
    if [ -d "$sublime_dir" ]; then
        log_info "Sublime Text detected. Installing syntax highlighting..."
        cp "$SCRIPT_DIR/syntaxes/CURSED.sublime-syntax" "$sublime_dir/" 2>/dev/null || true
    fi
    
    # Kate/KWrite
    local kate_dir="$HOME_DIR/.local/share/katepart5/syntax"
    if [ -d "$(dirname "$kate_dir")" ]; then
        log_info "Kate/KWrite detected. Installing syntax highlighting..."
        mkdir -p "$kate_dir"
        cp "$SCRIPT_DIR/syntaxes/cursed.xml" "$kate_dir/" 2>/dev/null || true
    fi
}

install_project_templates() {
    log_header "Installing Project Templates"
    
    local templates_dir="$SCRIPT_DIR/project-templates"
    local target_dir="$HOME_DIR/.cursed/templates"
    
    if [ -d "$templates_dir" ]; then
        log_info "Installing project templates..."
        mkdir -p "$target_dir"
        cp -r "$templates_dir/"* "$target_dir/"
        
        # Make scaffolding script executable
        local scaffold_script="$SCRIPT_DIR/scaffolding/cursed-scaffold.py"
        if [ -f "$scaffold_script" ]; then
            chmod +x "$scaffold_script"
            
            # Create symlink in PATH if possible
            local bin_dir="$HOME_DIR/.local/bin"
            if [ -d "$bin_dir" ]; then
                ln -sf "$scaffold_script" "$bin_dir/cursed-scaffold"
                log_success "Project scaffolding tool installed: cursed-scaffold"
            else
                log_info "To use the scaffolding tool, add $scaffold_script to your PATH"
            fi
        fi
        
        log_success "Project templates installed to $target_dir"
    else
        log_warning "Project templates directory not found"
    fi
}

install_debug_adapter() {
    log_header "Installing Debug Adapter"
    
    local debug_adapter="$SCRIPT_DIR/debug-adapter/cursed-debug-adapter.py"
    
    if [ -f "$debug_adapter" ]; then
        chmod +x "$debug_adapter"
        
        # Install to user bin directory
        local bin_dir="$HOME_DIR/.local/bin"
        mkdir -p "$bin_dir"
        cp "$debug_adapter" "$bin_dir/cursed-debug-adapter"
        
        log_success "Debug adapter installed: cursed-debug-adapter"
        log_info "Debug adapter supports VS Code, Vim DAP, and other DAP-compatible editors"
    else
        log_warning "Debug adapter not found"
    fi
}

setup_shell_integration() {
    log_header "Setting Up Shell Integration"
    
    local shell_config=""
    
    # Detect shell
    if [ -n "${BASH_VERSION:-}" ]; then
        shell_config="$HOME_DIR/.bashrc"
    elif [ -n "${ZSH_VERSION:-}" ]; then
        shell_config="$HOME_DIR/.zshrc"
    elif [ -f "$HOME_DIR/.profile" ]; then
        shell_config="$HOME_DIR/.profile"
    fi
    
    if [ -n "$shell_config" ]; then
        log_info "Setting up shell integration in $shell_config"
        
        # Add PATH entries if not already present
        local cursed_bin="$CURSED_ROOT/zig-out/bin"
        local user_bin="$HOME_DIR/.local/bin"
        
        if [ -d "$cursed_bin" ] && ! grep -q "$cursed_bin" "$shell_config" 2>/dev/null; then
            echo "# CURSED compiler path" >> "$shell_config"
            echo "export PATH=\"$cursed_bin:\$PATH\"" >> "$shell_config"
        fi
        
        if [ -d "$user_bin" ] && ! grep -q "$user_bin" "$shell_config" 2>/dev/null; then
            echo "# User local bin path" >> "$shell_config"
            echo "export PATH=\"$user_bin:\$PATH\"" >> "$shell_config"
        fi
        
        # Add CURSED environment variables
        if ! grep -q "CURSED_HOME" "$shell_config" 2>/dev/null; then
            echo "" >> "$shell_config"
            echo "# CURSED environment" >> "$shell_config"
            echo "export CURSED_HOME=\"$CURSED_ROOT\"" >> "$shell_config"
            echo "export CURSED_TEMPLATES=\"$HOME_DIR/.cursed/templates\"" >> "$shell_config"
        fi
        
        log_success "Shell integration configured"
        log_info "Restart your shell or run: source $shell_config"
    else
        log_warning "Could not detect shell configuration file"
    fi
}

print_usage() {
    echo "CURSED IDE Integration Installation Script"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --all                 Install all IDE integrations (default)"
    echo "  --vscode             Install VS Code extension only"
    echo "  --intellij           Install IntelliJ plugin only"
    echo "  --vim                Install Vim/Neovim plugin only"
    echo "  --templates          Install project templates only"
    echo "  --debug              Install debug adapter only"
    echo "  --shell              Setup shell integration only"
    echo "  --check              Check system requirements only"
    echo "  --help               Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                   # Install everything"
    echo "  $0 --vscode         # Install VS Code extension only"
    echo "  $0 --vim --templates # Install Vim plugin and templates"
}

print_summary() {
    log_header "Installation Summary"
    
    echo "✅ IDE Integration Installation Complete!"
    echo ""
    echo "🔧 What was installed:"
    
    if check_command "code"; then
        echo "   • VS Code extension with advanced features"
    fi
    
    if check_command "vim" || check_command "nvim"; then
        echo "   • Vim/Neovim plugin with LSP support"
    fi
    
    echo "   • Project templates and scaffolding tools"
    echo "   • Debug adapter for debugging support"
    echo "   • Shell integration and PATH configuration"
    echo ""
    
    echo "🚀 Next steps:"
    echo "   1. Restart your editors to load the CURSED support"
    echo "   2. Create a new project: cursed-scaffold new web-app my-project"
    echo "   3. Open the project in your favorite editor"
    echo "   4. Start coding with full IDE support!"
    echo ""
    
    echo "📚 Resources:"
    echo "   • Documentation: https://docs.cursed-lang.org"
    echo "   • VS Code Extension: Search 'CURSED' in VS Code marketplace"
    echo "   • Vim Plugin Help: :help cursed"
    echo "   • Templates: ls ~/.cursed/templates"
    echo ""
    
    echo "🐛 Issues? Report at: https://github.com/ghuntley/cursed/issues"
}

# Main installation logic
main() {
    local install_all=true
    local install_vscode=false
    local install_intellij=false
    local install_vim=false
    local install_templates=false
    local install_debug=false
    local install_shell=false
    local check_only=false
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --all)
                install_all=true
                shift
                ;;
            --vscode)
                install_all=false
                install_vscode=true
                shift
                ;;
            --intellij)
                install_all=false
                install_intellij=true
                shift
                ;;
            --vim)
                install_all=false
                install_vim=true
                shift
                ;;
            --templates)
                install_all=false
                install_templates=true
                shift
                ;;
            --debug)
                install_all=false
                install_debug=true
                shift
                ;;
            --shell)
                install_all=false
                install_shell=true
                shift
                ;;
            --check)
                check_only=true
                shift
                ;;
            --help)
                print_usage
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                print_usage
                exit 1
                ;;
        esac
    done
    
    echo -e "${CYAN}"
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║                   CURSED IDE Integration                     ║"
    echo "║              Professional Development Environment            ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    
    # Check CURSED installation
    check_cursed_installation
    
    if [ "$check_only" = true ]; then
        log_success "System requirements check passed!"
        exit 0
    fi
    
    # Install components based on flags
    if [ "$install_all" = true ]; then
        install_vscode_extension
        install_intellij_plugin
        install_vim_plugin
        install_project_templates
        install_debug_adapter
        setup_shell_integration
    else
        [ "$install_vscode" = true ] && install_vscode_extension
        [ "$install_intellij" = true ] && install_intellij_plugin
        [ "$install_vim" = true ] && install_vim_plugin
        [ "$install_templates" = true ] && install_project_templates
        [ "$install_debug" = true ] && install_debug_adapter
        [ "$install_shell" = true ] && setup_shell_integration
    fi
    
    print_summary
}

# Run main function with all arguments
main "$@"
