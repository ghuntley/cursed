;;; cursed-mode.el --- Major mode for CURSED programming language -*- lexical-binding: t; -*-

;; Copyright (C) 2024 Geoffrey Huntley
;; Author: Geoffrey Huntley
;; Version: 0.1.0
;; Package-Requires: ((emacs "26.1"))
;; Keywords: languages
;; URL: https://github.com/ghuntley/cursed

;;; Commentary:

;; This package provides a major mode for editing CURSED programming language files.
;; CURSED is a programming language with Gen Z slang syntax and Go-like grammar.

;;; Code:

(require 'font-lock)
(require 'rx)

(defgroup cursed nil
  "Support for the CURSED programming language."
  :group 'languages
  :prefix "cursed-")

(defcustom cursed-indent-offset 4
  "Indentation offset for CURSED code."
  :type 'integer
  :group 'cursed)

(defcustom cursed-lsp-command '("cursed-lsp")
  "Command to start the CURSED language server."
  :type '(repeat string)
  :group 'cursed)

(defcustom cursed-format-on-save nil
  "Format CURSED code on save."
  :type 'boolean
  :group 'cursed)

;; Syntax highlighting
(defconst cursed-keywords
  '("slay" "yolo" "facts" "sus" "squad" "collab" "vibes" "use" "from"
    "lowkey" "highkey" "periodt" "bestie" "flex" "vibe_check" "mood" "basic"
    "bounce" "yeet" "catch" "finally" "spawn" "await" "async"
    "public" "private" "chan" "send" "recv" "select" "defer" "go")
  "CURSED language keywords.")

(defconst cursed-constants
  '("true" "false" "nil")
  "CURSED language constants.")

(defconst cursed-types
  '("string" "int" "float" "bool" "char" "byte" "array" "slice" "map" "set" "chan"
    "Vec" "HashMap" "Option" "Result" "Any" "interface")
  "CURSED language types.")

(defconst cursed-builtins
  '("print" "println" "len" "str" "int" "float" "type" "panic" "spawn" "make"
    "append" "copy" "delete" "close")
  "CURSED built-in functions.")

(defconst cursed-font-lock-keywords
  `(
    ;; Keywords
    (,(regexp-opt cursed-keywords 'words) . font-lock-keyword-face)
    
    ;; Constants
    (,(regexp-opt cursed-constants 'words) . font-lock-constant-face)
    
    ;; Types
    (,(regexp-opt cursed-types 'words) . font-lock-type-face)
    
    ;; Built-in functions
    (,(regexp-opt cursed-builtins 'words) . font-lock-builtin-face)
    
    ;; Function definitions
    ("\\<\\(slay\\|yolo\\)\\s-+\\(\\w+\\)\\s-*(" 
     (1 font-lock-keyword-face) (2 font-lock-function-name-face))
    
    ;; Type definitions
    ("\\<\\(squad\\|collab\\|vibes\\)\\s-+\\(\\w+\\)" 
     (1 font-lock-keyword-face) (2 font-lock-type-face))
    
    ;; Variable declarations
    ("\\<\\(facts\\|sus\\)\\s-+\\(\\w+\\)" 
     (1 font-lock-keyword-face) (2 font-lock-variable-name-face))
    
    ;; Numbers
    ("\\<[0-9]+\\(\\.[0-9]+\\)?\\([eE][+-]?[0-9]+\\)?\\>" . font-lock-constant-face)
    ("\\<0[xX][0-9a-fA-F]+\\>" . font-lock-constant-face)
    ("\\<0[bB][01]+\\>" . font-lock-constant-face)
    ("\\<0[oO][0-7]+\\>" . font-lock-constant-face)
    
    ;; Strings
    ("\"[^\"\\\\]*\\(?:\\\\.[^\"\\\\]*\\)*\"" . font-lock-string-face)
    ("'[^'\\\\]*\\(?:\\\\.[^'\\\\]*\\)*'" . font-lock-string-face)
    ("`[^`]*`" . font-lock-string-face)
    
    ;; Comments
    ("//.*$" . font-lock-comment-face)
    ("/\\*[^*]*\\*+\\(?:[^/*][^*]*\\*+\\)*/" . font-lock-comment-face))
  "Font lock keywords for CURSED mode.")

;; Syntax table
(defvar cursed-mode-syntax-table
  (let ((table (make-syntax-table)))
    ;; Comments
    (modify-syntax-entry ?/ ". 124b" table)
    (modify-syntax-entry ?* ". 23" table)
    (modify-syntax-entry ?\n "> b" table)
    
    ;; Strings
    (modify-syntax-entry ?\" "\"" table)
    (modify-syntax-entry ?' "\"" table)
    (modify-syntax-entry ?` "\"" table)
    
    ;; Operators
    (modify-syntax-entry ?+ "." table)
    (modify-syntax-entry ?- "." table)
    (modify-syntax-entry ?* "." table)
    (modify-syntax-entry ?/ "." table)
    (modify-syntax-entry ?% "." table)
    (modify-syntax-entry ?< "." table)
    (modify-syntax-entry ?> "." table)
    (modify-syntax-entry ?& "." table)
    (modify-syntax-entry ?| "." table)
    (modify-syntax-entry ?^ "." table)
    (modify-syntax-entry ?! "." table)
    (modify-syntax-entry ?= "." table)
    
    ;; Brackets
    (modify-syntax-entry ?\( "()" table)
    (modify-syntax-entry ?\) ")(" table)
    (modify-syntax-entry ?\[ "(]" table)
    (modify-syntax-entry ?\] ")[" table)
    (modify-syntax-entry ?\{ "(}" table)
    (modify-syntax-entry ?\} "){" table)
    
    table)
  "Syntax table for CURSED mode.")

;; Indentation
(defun cursed-indent-line ()
  "Indent current line as CURSED code."
  (interactive)
  (let ((indent-level 0)
        (case-fold-search nil))
    (save-excursion
      (beginning-of-line)
      (if (bobp)
          (setq indent-level 0)
        (let ((not-indented t) cur-indent)
          (if (looking-at "^[ \t]*\\(}\\|]\\|)\\)")
              (progn
                (save-excursion
                  (forward-line -1)
                  (setq cur-indent (- (current-indentation) cursed-indent-offset)))
                (if (< cur-indent 0)
                    (setq indent-level 0)
                  (setq indent-level cur-indent)))
            (save-excursion
              (while not-indented
                (forward-line -1)
                (if (looking-at "^[ \t]*\\(}\\|]\\|)\\)")
                    (progn
                      (setq indent-level (current-indentation))
                      (setq not-indented nil))
                  (if (looking-at "^.*\\({\\|\\[\\|(\\)[ \t]*$")
                      (progn
                        (setq indent-level (+ (current-indentation) cursed-indent-offset))
                        (setq not-indented nil))
                    (if (bobp)
                        (setq not-indented nil))))))))))
    (if (looking-at "^[ \t]*")
        (replace-match ""))
    (indent-to indent-level)))

;; LSP support
(defun cursed-lsp-setup ()
  "Set up LSP for CURSED mode."
  (when (featurep 'lsp-mode)
    (add-to-list 'lsp-language-id-configuration '(cursed-mode . "cursed"))
    (lsp-register-client
     (make-lsp-client :new-connection (lsp-stdio-connection cursed-lsp-command)
                      :major-modes '(cursed-mode)
                      :server-id 'cursed-lsp
                      :initialization-options
                      `((settings . ((cursed . ((format . ((enable . t)
                                                          (indentSize . ,cursed-indent-offset)
                                                          (lineWidth . 120)))
                                               (lint . ((enable . t)
                                                      (checkStyle . t)
                                                      (checkPerformance . t)))))))))))

;; Interactive functions
(defun cursed-format-buffer ()
  "Format the current CURSED buffer."
  (interactive)
  (if (featurep 'lsp-mode)
      (lsp-format-buffer)
    (message "LSP not available, manual formatting not implemented")))

(defun cursed-run-linter ()
  "Run linter on the current CURSED buffer."
  (interactive)
  (if (featurep 'lsp-mode)
      (lsp-execute-command "cursed/runLinter" 
                          `((textDocument . ((uri . ,(lsp--buffer-uri))))
                            (options . ((checkStyle . t)
                                      (checkPerformance . t)
                                      (checkSecurity . t)))))
    (message "LSP not available, cannot run linter")))

(defun cursed-show-type-info ()
  "Show type information at point."
  (interactive)
  (if (featurep 'lsp-mode)
      (lsp-execute-command "cursed/getTypeInfo"
                          `((textDocument . ((uri . ,(lsp--buffer-uri))))
                            (position . ,(lsp--cur-position))
                            (includeHierarchy . t)))
    (message "LSP not available, cannot show type info")))

(defun cursed-show-ast-node ()
  "Show AST node at point."
  (interactive)
  (if (featurep 'lsp-mode)
      (lsp-execute-command "cursed/getAstNode"
                          `((textDocument . ((uri . ,(lsp--buffer-uri))))
                            (position . ,(lsp--cur-position))
                            (includeChildren . t)
                            (maxDepth . 3)))
    (message "LSP not available, cannot show AST node")))

;; Keymap
(defvar cursed-mode-map
  (let ((map (make-sparse-keymap)))
    (define-key map (kbd "C-c C-f") 'cursed-format-buffer)
    (define-key map (kbd "C-c C-l") 'cursed-run-linter)
    (define-key map (kbd "C-c C-t") 'cursed-show-type-info)
    (define-key map (kbd "C-c C-a") 'cursed-show-ast-node)
    map)
  "Keymap for CURSED mode.")

;; Menu
(easy-menu-define cursed-mode-menu cursed-mode-map
  "Menu for CURSED mode."
  '("CURSED"
    ["Format Buffer" cursed-format-buffer t]
    ["Run Linter" cursed-run-linter t]
    "---"
    ["Show Type Info" cursed-show-type-info t]
    ["Show AST Node" cursed-show-ast-node t]
    "---"
    ["Customize CURSED" (customize-group 'cursed) t]))

;; Auto-format on save
(defun cursed-format-on-save ()
  "Format buffer on save if enabled."
  (when cursed-format-on-save
    (cursed-format-buffer)))

;; Define the major mode
;;;###autoload
(define-derived-mode cursed-mode prog-mode "CURSED"
  "Major mode for editing CURSED programming language files."
  :syntax-table cursed-mode-syntax-table
  
  ;; Font lock
  (setq font-lock-defaults '(cursed-font-lock-keywords))
  
  ;; Comments
  (setq-local comment-start "// ")
  (setq-local comment-end "")
  (setq-local comment-start-skip "//+\\s-*")
  
  ;; Indentation
  (setq-local indent-line-function 'cursed-indent-line)
  (setq-local tab-width cursed-indent-offset)
  
  ;; Electric behavior
  (setq-local electric-indent-chars '(?\{ ?\} ?\; ?\n))
  
  ;; LSP setup
  (cursed-lsp-setup)
  
  ;; Format on save
  (add-hook 'before-save-hook 'cursed-format-on-save nil t))

;; Auto-mode setup
;;;###autoload
(add-to-list 'auto-mode-alist '("\\.csd\\'" . cursed-mode))

;; Provide the feature
(provide 'cursed-mode)

;;; cursed-mode.el ends here
