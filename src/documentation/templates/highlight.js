// CURSED Syntax Highlighting JavaScript

// Initialize syntax highlighting when DOM is ready
document.addEventListener('DOMContentLoaded', function() {
    initializeSyntaxHighlighting();
});

function initializeSyntaxHighlighting() {
    // Highlight all CURSED code blocks
    document.querySelectorAll('code.language-cursed, code[class*="cursed"]').forEach(block => {
        highlightCursedCode(block);
    });
    
    // Add enhancement features
    enhanceCodeBlocks();
    
    console.log('CURSED syntax highlighting initialized');
}

function highlightCursedCode(codeBlock) {
    const code = codeBlock.textContent;
    const highlightedCode = applyCursedSyntaxHighlighting(code);
    codeBlock.innerHTML = highlightedCode;
    
    // Add language class if missing
    if (!codeBlock.classList.contains('language-cursed')) {
        codeBlock.classList.add('language-cursed');
    }
}

function applyCursedSyntaxHighlighting(code) {
    // Escape HTML first
    let highlightedCode = escapeHtml(code);
    
    // Apply highlighting patterns in order of precedence
    highlightedCode = highlightComments(highlightedCode);
    highlightedCode = highlightStrings(highlightedCode);
    highlightedCode = highlightNumbers(highlightedCode);
    highlightedCode = highlightCursedKeywords(highlightedCode);
    highlightedCode = highlightTypes(highlightedCode);
    highlightedCode = highlightOperators(highlightedCode);
    highlightedCode = highlightFunctions(highlightedCode);
    highlightedCode = highlightVariables(highlightedCode);
    
    return highlightedCode;
}

function highlightComments(code) {
    // Single-line comments
    code = code.replace(/(\/\/.*$)/gm, '<span class="comment">$1</span>');
    
    // Multi-line comments
    code = code.replace(/(\/\*[\s\S]*?\*\/)/g, '<span class="comment">$1</span>');
    
    // Documentation comments
    code = code.replace(/(\/\*\*[\s\S]*?\*\/)/g, '<span class="comment doc-comment">$1</span>');
    
    return code;
}

function highlightStrings(code) {
    // Double-quoted strings
    code = code.replace(/"([^"\\]|\\.)*"/g, '<span class="string">"$1"</span>');
    
    // Single-quoted strings  
    code = code.replace(/'([^'\\]|\\.)*'/g, '<span class="string">\'$1\'</span>');
    
    // Template literals (if supported)
    code = code.replace(/`([^`\\]|\\.)*`/g, '<span class="string template-literal">`$1`</span>');
    
    return code;
}

function highlightNumbers(code) {
    // Integer and floating-point numbers
    code = code.replace(/\b(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?\b/g, '<span class="number">$&</span>');
    
    // Hexadecimal numbers
    code = code.replace(/\b0[xX][0-9a-fA-F]+\b/g, '<span class="number hex">$&</span>');
    
    // Binary numbers
    code = code.replace(/\b0[bB][01]+\b/g, '<span class="number binary">$&</span>');
    
    // Octal numbers
    code = code.replace(/\b0[oO][0-7]+\b/g, '<span class="number octal">$&</span>');
    
    return code;
}

function highlightCursedKeywords(code) {
    const keywordPatterns = [
        // Function declaration keywords
        { 
            pattern: /\b(slay|yolo)\b/g, 
            class: 'keyword-function',
            description: 'Function declaration keywords'
        },
        
        // Variable declaration keywords
        { 
            pattern: /\b(sus|facts)\b/g, 
            class: 'keyword-variable',
            description: 'Variable declaration keywords'
        },
        
        // Conditional keywords
        { 
            pattern: /\b(lowkey|highkey)\b/g, 
            class: 'keyword-conditional',
            description: 'Conditional keywords (if/else)'
        },
        
        // Control flow keywords
        { 
            pattern: /\b(periodt|bestie|flex)\b/g, 
            class: 'keyword-control',
            description: 'Control flow keywords'
        },
        
        // Type definition keywords
        { 
            pattern: /\b(squad|collab)\b/g, 
            class: 'keyword-type',
            description: 'Type definition keywords (struct/interface)'
        },
        
        // Switch statement keywords
        { 
            pattern: /\b(vibe_check|mood|basic)\b/g, 
            class: 'keyword-switch',
            description: 'Switch statement keywords'
        },
        
        // Loop keywords
        { 
            pattern: /\b(stan|unstannable)\b/g, 
            class: 'keyword-loop',
            description: 'Loop keywords'
        },
        
        // Error handling keywords
        { 
            pattern: /\b(catch_these_hands|no_cap)\b/g, 
            class: 'keyword-error',
            description: 'Error handling keywords'
        },
        
        // Async keywords
        { 
            pattern: /\b(send_it|wait_up)\b/g, 
            class: 'keyword-async',
            description: 'Async/await keywords'
        },
        
        // Memory keywords
        { 
            pattern: /\b(snatch|release)\b/g, 
            class: 'keyword-memory',
            description: 'Memory management keywords'
        }
    ];
    
    keywordPatterns.forEach(({ pattern, class: className }) => {
        code = code.replace(pattern, `<span class="${className}">$&</span>`);
    });
    
    return code;
}

function highlightTypes(code) {
    const typePatterns = [
        // Primitive types
        { pattern: /\b(normie|str|bool|void)\b/g, class: 'type primitive' },
        
        // Numeric types
        { pattern: /\b(i8|i16|i32|i64|u8|u16|u32|u64|f32|f64)\b/g, class: 'type numeric' },
        
        // Collection types
        { pattern: /\b(Array|Map|Set|Vec)\b/g, class: 'type collection' },
        
        // Special types
        { pattern: /\b(Option|Result|Box|Rc|Arc)\b/g, class: 'type special' },
        
        // Generic type parameters
        { pattern: /\b[A-Z][A-Za-z0-9]*\b(?=\s*[<>])/g, class: 'type generic' }
    ];
    
    typePatterns.forEach(({ pattern, class: className }) => {
        code = code.replace(pattern, `<span class="${className}">$&</span>`);
    });
    
    return code;
}

function highlightOperators(code) {
    const operatorPatterns = [
        // Arithmetic operators
        { pattern: /(\+|-|\*|\/|%)/g, class: 'operator arithmetic' },
        
        // Comparison operators
        { pattern: /(==|!=|<=|>=|<|>)/g, class: 'operator comparison' },
        
        // Logical operators
        { pattern: /(&&|\|\||!)/g, class: 'operator logical' },
        
        // Assignment operators
        { pattern: /(=|\+=|-=|\*=|\/=|%=)/g, class: 'operator assignment' },
        
        // Bitwise operators
        { pattern: /(&|\||\^|<<|>>|~)/g, class: 'operator bitwise' },
        
        // Special operators
        { pattern: /(\?|:|\.|->|=>)/g, class: 'operator special' }
    ];
    
    operatorPatterns.forEach(({ pattern, class: className }) => {
        code = code.replace(pattern, `<span class="${className}">$1</span>`);
    });
    
    return code;
}

function highlightFunctions(code) {
    // Function calls
    code = code.replace(/\b([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/g, '<span class="function">$1</span>(');
    
    // Method calls
    code = code.replace(/\.([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/g, '.<span class="function method">$1</span>(');
    
    // Constructor calls
    code = code.replace(/\bnew\s+([A-Z][a-zA-Z0-9_]*)/g, 'new <span class="function constructor">$1</span>');
    
    return code;
}

function highlightVariables(code) {
    // Constants (ALL_CAPS)
    code = code.replace(/\b[A-Z][A-Z0-9_]*\b/g, '<span class="constant">$&</span>');
    
    // Variables (after keywords)
    const variableContext = /(sus|facts)\s+([a-zA-Z_][a-zA-Z0-9_]*)/g;
    code = code.replace(variableContext, '$1 <span class="variable">$2</span>');
    
    return code;
}

function enhanceCodeBlocks() {
    document.querySelectorAll('pre code').forEach(block => {
        const container = block.parentElement;
        
        // Add line numbers for longer code blocks
        if (shouldAddLineNumbers(block)) {
            addLineNumbers(container, block);
        }
        
        // Add copy functionality
        if (!container.querySelector('.copy-button')) {
            addCopyButton(container, block);
        }
        
        // Add folding for very long code blocks
        if (shouldAddFolding(block)) {
            addCodeFolding(container, block);
        }
        
        // Add language indicator
        addLanguageIndicator(container, block);
        
        // Add accessibility attributes
        enhanceAccessibility(container, block);
    });
}

function shouldAddLineNumbers(codeBlock) {
    const lines = codeBlock.textContent.split('\n');
    return lines.length > 5; // Add line numbers for code with more than 5 lines
}

function addLineNumbers(container, codeBlock) {
    if (container.classList.contains('line-numbers')) return;
    
    const lines = codeBlock.textContent.split('\n');
    const lineCount = lines.length;
    
    const lineNumbers = document.createElement('span');
    lineNumbers.className = 'line-numbers-rows';
    lineNumbers.setAttribute('aria-hidden', 'true');
    
    // Generate line numbers
    const lineNumbersText = Array.from({length: lineCount}, (_, i) => i + 1).join('\n');
    lineNumbers.textContent = lineNumbersText;
    
    container.classList.add('line-numbers');
    container.insertBefore(lineNumbers, codeBlock);
}

function addCopyButton(container, codeBlock) {
    const copyButton = document.createElement('button');
    copyButton.className = 'copy-button';
    copyButton.textContent = 'Copy';
    copyButton.setAttribute('aria-label', 'Copy code to clipboard');
    copyButton.title = 'Copy code to clipboard';
    
    copyButton.addEventListener('click', () => copyCodeToClipboard(codeBlock, copyButton));
    
    container.style.position = 'relative';
    container.appendChild(copyButton);
}

function shouldAddFolding(codeBlock) {
    const lines = codeBlock.textContent.split('\n');
    return lines.length > 25; // Add folding for code with more than 25 lines
}

function addCodeFolding(container, codeBlock) {
    const foldButton = document.createElement('button');
    foldButton.className = 'code-fold-toggle';
    foldButton.textContent = '⊟';
    foldButton.setAttribute('aria-label', 'Fold code block');
    foldButton.title = 'Click to fold/unfold code';
    
    foldButton.addEventListener('click', () => toggleCodeFold(container, foldButton));
    
    container.appendChild(foldButton);
}

function addLanguageIndicator(container, codeBlock) {
    const classes = Array.from(codeBlock.classList);
    const languageClass = classes.find(cls => cls.startsWith('language-'));
    
    if (languageClass) {
        const language = languageClass.replace('language-', '').toUpperCase();
        const indicator = document.createElement('span');
        indicator.className = 'language-indicator';
        indicator.textContent = language;
        indicator.setAttribute('aria-label', `Code language: ${language}`);
        
        container.style.position = 'relative';
        container.appendChild(indicator);
    }
}

function enhanceAccessibility(container, codeBlock) {
    // Add ARIA attributes
    codeBlock.setAttribute('role', 'code');
    codeBlock.setAttribute('tabindex', '0');
    
    // Add keyboard navigation
    codeBlock.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' && e.ctrlKey) {
            // Ctrl+Enter to copy code
            const copyButton = container.querySelector('.copy-button');
            if (copyButton) {
                copyButton.click();
            }
        }
    });
}

function copyCodeToClipboard(codeBlock, button) {
    const text = codeBlock.textContent;
    
    navigator.clipboard.writeText(text).then(() => {
        showCopySuccess(button);
        
        // Announce to screen readers
        announceToScreenReader('Code copied to clipboard');
    }).catch(err => {
        console.error('Failed to copy code:', err);
        fallbackCopyTextToClipboard(text, button);
    });
}

function showCopySuccess(button) {
    const originalText = button.textContent;
    const originalClass = button.className;
    
    button.textContent = 'Copied!';
    button.classList.add('copied');
    
    setTimeout(() => {
        button.textContent = originalText;
        button.className = originalClass;
    }, 2000);
}

function fallbackCopyTextToClipboard(text, button) {
    const textArea = document.createElement('textarea');
    textArea.value = text;
    textArea.style.position = 'fixed';
    textArea.style.left = '-999999px';
    textArea.style.top = '-999999px';
    
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();
    
    try {
        const successful = document.execCommand('copy');
        if (successful) {
            showCopySuccess(button);
            announceToScreenReader('Code copied to clipboard');
        } else {
            announceToScreenReader('Failed to copy code');
        }
    } catch (err) {
        console.error('Fallback copy failed:', err);
        announceToScreenReader('Failed to copy code');
    }
    
    document.body.removeChild(textArea);
}

function toggleCodeFold(container, button) {
    const isFolded = container.classList.contains('folded');
    
    if (isFolded) {
        container.classList.remove('folded');
        button.textContent = '⊟';
        button.setAttribute('aria-label', 'Fold code block');
        button.title = 'Click to fold code';
    } else {
        container.classList.add('folded');
        button.textContent = '⊞';
        button.setAttribute('aria-label', 'Unfold code block');
        button.title = 'Click to unfold code';
    }
    
    // Announce state change to screen readers
    const state = isFolded ? 'unfolded' : 'folded';
    announceToScreenReader(`Code block ${state}`);
}

function announceToScreenReader(message) {
    const announcement = document.createElement('div');
    announcement.setAttribute('aria-live', 'polite');
    announcement.setAttribute('aria-atomic', 'true');
    announcement.style.position = 'absolute';
    announcement.style.left = '-10000px';
    announcement.style.width = '1px';
    announcement.style.height = '1px';
    announcement.style.overflow = 'hidden';
    
    announcement.textContent = message;
    document.body.appendChild(announcement);
    
    setTimeout(() => {
        document.body.removeChild(announcement);
    }, 1000);
}

function escapeHtml(text) {
    const map = {
        '&': '&amp;',
        '<': '&lt;',
        '>': '&gt;',
        '"': '&quot;',
        "'": '&#039;'
    };
    
    return text.replace(/[&<>"']/g, (m) => map[m]);
}

// Additional syntax highlighting features

function addSyntaxTooltips() {
    document.querySelectorAll('.keyword-function, .keyword-variable, .keyword-type').forEach(element => {
        element.addEventListener('mouseenter', showSyntaxTooltip);
        element.addEventListener('mouseleave', hideSyntaxTooltip);
    });
}

function showSyntaxTooltip(event) {
    const element = event.target;
    const keyword = element.textContent;
    const tooltip = getSyntaxTooltipContent(keyword);
    
    if (tooltip) {
        const tooltipElement = document.createElement('div');
        tooltipElement.className = 'syntax-tooltip';
        tooltipElement.innerHTML = tooltip;
        
        document.body.appendChild(tooltipElement);
        
        // Position tooltip
        const rect = element.getBoundingClientRect();
        tooltipElement.style.left = rect.left + 'px';
        tooltipElement.style.top = (rect.bottom + 5) + 'px';
        
        element._tooltip = tooltipElement;
    }
}

function hideSyntaxTooltip(event) {
    const element = event.target;
    if (element._tooltip) {
        document.body.removeChild(element._tooltip);
        element._tooltip = null;
    }
}

function getSyntaxTooltipContent(keyword) {
    const tooltips = {
        'slay': 'Function declaration keyword - defines a new function',
        'yolo': 'Async function declaration - defines an asynchronous function',
        'sus': 'Variable declaration - creates a mutable variable',
        'facts': 'Constant declaration - creates an immutable value',
        'lowkey': 'Conditional keyword - equivalent to "if"',
        'highkey': 'Conditional keyword - equivalent to "else"',
        'periodt': 'Control flow - equivalent to "return"',
        'bestie': 'Loop keyword - equivalent to "for"',
        'flex': 'Loop keyword - equivalent to "while"',
        'squad': 'Type definition - defines a struct',
        'collab': 'Type definition - defines an interface',
        'vibe_check': 'Switch statement keyword',
        'mood': 'Switch case keyword',
        'basic': 'Switch default keyword',
        'normie': 'Integer type',
        'str': 'String type',
        'bool': 'Boolean type',
        'void': 'Void/empty type'
    };
    
    return tooltips[keyword] || null;
}

// Initialize tooltips
document.addEventListener('DOMContentLoaded', function() {
    setTimeout(addSyntaxTooltips, 100); // Small delay to ensure highlighting is complete
});

// Export functions for external use
window.CURSED_HIGHLIGHT = {
    highlightCursedCode,
    applyCursedSyntaxHighlighting,
    enhanceCodeBlocks,
    copyCodeToClipboard,
    toggleCodeFold
};

console.log('CURSED syntax highlighting loaded');
