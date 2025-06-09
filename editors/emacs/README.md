# CURSED Language Support for Emacs

This directory contains Emacs configuration for CURSED language support.

## Installation

### Method 1: Manual Installation

1. Copy the mode file to your Emacs configuration directory:
   ```bash
   cp cursed-mode.el ~/.emacs.d/lisp/
   ```

2. Add to your `init.el` or `.emacs`:
   ```elisp
   (add-to-list 'load-path "~/.emacs.d/lisp")
   (require 'cursed-mode)
   ```

### Method 2: Package Installation (if published)

Add to your `init.el`:
```elisp
(use-package cursed-mode
  :ensure t
  :mode "\\.csd\\'")
```

## Configuration

### Basic Configuration

Add to your `init.el`:

```elisp
(use-package cursed-mode
  :ensure t
  :mode "\\.csd\\'"
  :custom
  (cursed-indent-offset 4)
  (cursed-format-on-save t)
  (cursed-lsp-command '("cursed-lsp" "--debug"))
  :hook
  (cursed-mode . lsp-deferred))
```

### LSP Integration

#### Using lsp-mode

```elisp
(use-package lsp-mode
  :ensure t
  :commands (lsp lsp-deferred)
  :config
  ;; CURSED LSP is automatically configured by cursed-mode
  (setq lsp-cursed-server-command '("cursed-lsp" "--debug")))

(use-package cursed-mode
  :ensure t
  :mode "\\.csd\\'"
  :hook (cursed-mode . lsp-deferred))
```

#### Using eglot

```elisp
(use-package eglot
  :ensure t
  :config
  (add-to-list 'eglot-server-programs
               '(cursed-mode . ("cursed-lsp" "--debug"))))

(use-package cursed-mode
  :ensure t
  :mode "\\.csd\\'"
  :hook (cursed-mode . eglot-ensure))
```

### Advanced Configuration

```elisp
(use-package cursed-mode
  :ensure t
  :mode "\\.csd\\'"
  :custom
  ;; Indentation
  (cursed-indent-offset 4)
  
  ;; LSP
  (cursed-lsp-command '("cursed-lsp" "--debug" "--log-file" "/tmp/cursed-lsp.log"))
  
  ;; Formatting
  (cursed-format-on-save t)
  
  :bind
  (:map cursed-mode-map
   ("C-c C-f" . cursed-format-buffer)
   ("C-c C-l" . cursed-run-linter)
   ("C-c C-t" . cursed-show-type-info)
   ("C-c C-a" . cursed-show-ast-node))
  
  :hook
  (cursed-mode . lsp-deferred)
  (cursed-mode . company-mode)      ; Auto-completion
  (cursed-mode . flycheck-mode)     ; Syntax checking
  (cursed-mode . hl-line-mode)      ; Highlight current line
  
  :config
  ;; Additional CURSED-specific settings
  (add-hook 'cursed-mode-hook
            (lambda ()
              (setq-local tab-width 4)
              (setq-local indent-tabs-mode nil)
              (setq-local comment-auto-fill-only-comments t))))
```

## Features

### Syntax Highlighting

- Complete syntax highlighting for CURSED keywords, types, and constructs
- Support for Gen Z slang keywords (`slay`, `facts`, `sus`, `lowkey`, etc.)
- Proper highlighting of strings, comments, numbers, and operators

### LSP Integration

- Auto-completion with context-aware suggestions
- Go to definition and find references
- Hover information for types and functions
- Real-time diagnostics (errors, warnings, hints)
- Document formatting
- Symbol search and workspace navigation

### Key Bindings

| Key Binding | Function |
|-------------|----------|
| `C-c C-f` | Format buffer |
| `C-c C-l` | Run linter |
| `C-c C-t` | Show type information at point |
| `C-c C-a` | Show AST node at point |

### Auto-completion

When using with `company-mode`:

```elisp
(use-package company
  :ensure t
  :hook (cursed-mode . company-mode)
  :config
  (setq company-tooltip-align-annotations t)
  (setq company-minimum-prefix-length 1))
```

### Syntax Checking

When using with `flycheck`:

```elisp
(use-package flycheck
  :ensure t
  :hook (cursed-mode . flycheck-mode))
```

### Project Management

Integration with `projectile`:

```elisp
(use-package projectile
  :ensure t
  :config
  (add-to-list 'projectile-project-root-files "CursedPackage.toml")
  (add-to-list 'projectile-project-root-files "CursedBuild.toml"))
```

## Customization

### Variables

- `cursed-indent-offset`: Number of spaces for indentation (default: 4)
- `cursed-lsp-command`: Command to start the language server (default: `("cursed-lsp")`)
- `cursed-format-on-save`: Format buffer on save (default: nil)

### Faces

Customize syntax highlighting colors:

```elisp
(custom-set-faces
 '(font-lock-keyword-face ((t (:foreground "#ff6b6b" :weight bold))))
 '(font-lock-function-name-face ((t (:foreground "#4ecdc4"))))
 '(font-lock-type-face ((t (:foreground "#45b7d1"))))
 '(font-lock-constant-face ((t (:foreground "#feca57")))))
```

## Troubleshooting

### LSP Server Not Starting

1. Ensure `cursed-lsp` is installed and in your PATH:
   ```bash
   which cursed-lsp
   ```

2. Check the server command configuration:
   ```elisp
   (setq cursed-lsp-command '("cursed-lsp" "--debug"))
   ```

3. Enable LSP debug logging:
   ```elisp
   (setq lsp-log-io t)
   ```

### Syntax Highlighting Not Working

1. Check that the file extension is `.csd`
2. Manually enable the mode: `M-x cursed-mode`
3. Reload the mode definition: `M-x eval-buffer` in `cursed-mode.el`

### Performance Issues

For large files, consider:

```elisp
(setq lsp-file-watch-threshold 2000)
(setq lsp-enable-file-watchers nil)  ; Disable file watchers if needed
```

## Integration with Other Packages

### Tree-sitter (Emacs 29+)

```elisp
(use-package treesit
  :when (treesit-available-p)
  :config
  ;; Tree-sitter grammar for CURSED would need to be implemented
  (add-to-list 'treesit-language-source-alist
               '(cursed "https://github.com/ghuntley/tree-sitter-cursed")))
```

### DAP (Debug Adapter Protocol)

```elisp
(use-package dap-mode
  :ensure t
  :after lsp-mode
  :config
  ;; CURSED debug adapter configuration would go here
  (dap-register-debug-template "CURSED Debug"
                               (list :type "cursed"
                                     :request "launch"
                                     :name "CURSED Debug"
                                     :program nil)))
```

This provides a complete Emacs integration for the CURSED programming language with modern IDE features through LSP.
