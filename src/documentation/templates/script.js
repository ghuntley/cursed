// CURSED Documentation Generator JavaScript
// Provides search functionality, syntax highlighting, and interactive features

class CursedDocumentation {
    constructor() {
        this.searchIndex = [];
        this.searchResults = [];
        this.currentSearchQuery = '';
        this.searchTimeout = null;
        
        this.init();
    }
    
    init() {
        this.setupSearch();
        this.setupSyntaxHighlighting();
        this.setupNavigation();
        this.setupTooltips();
        this.setupKeyboardShortcuts();
        this.loadSearchIndex();
    }
    
    // Search functionality
    setupSearch() {
        const searchInput = document.querySelector('.search-input');
        const searchResults = document.querySelector('.search-results');
        
        if (searchInput) {
            searchInput.addEventListener('input', (e) => {
                this.handleSearch(e.target.value);
            });
            
            searchInput.addEventListener('keydown', (e) => {
                if (e.key === 'Escape') {
                    this.clearSearch();
                }
            });
        }
        
        // Close search results when clicking outside
        document.addEventListener('click', (e) => {
            if (!e.target.closest('.search-container')) {
                this.clearSearch();
            }
        });
    }
    
    handleSearch(query) {
        clearTimeout(this.searchTimeout);
        
        this.searchTimeout = setTimeout(() => {
            this.performSearch(query);
        }, 300);
    }
    
    performSearch(query) {
        if (query.length < 2) {
            this.clearSearch();
            return;
        }
        
        this.currentSearchQuery = query.toLowerCase();
        const results = this.searchIndex.filter(item => 
            item.title.toLowerCase().includes(this.currentSearchQuery) ||
            item.content.toLowerCase().includes(this.currentSearchQuery)
        );
        
        this.displaySearchResults(results);
    }
    
    displaySearchResults(results) {
        const searchResultsContainer = document.querySelector('.search-results');
        if (!searchResultsContainer) return;
        
        if (results.length === 0) {
            searchResultsContainer.innerHTML = '<div class="search-result">No results found</div>';
            searchResultsContainer.style.display = 'block';
            return;
        }
        
        const html = results.slice(0, 10).map(result => `
            <div class="search-result" onclick="window.location.href='${result.url}'">
                <div class="search-result-title">${this.highlightMatch(result.title)}</div>
                <div class="search-result-type">${result.type}</div>
                <div class="search-result-description">${this.highlightMatch(result.content.substring(0, 100))}...</div>
            </div>
        `).join('');
        
        searchResultsContainer.innerHTML = html;
        searchResultsContainer.style.display = 'block';
    }
    
    highlightMatch(text) {
        if (!this.currentSearchQuery) return text;
        
        const regex = new RegExp(`(${this.currentSearchQuery})`, 'gi');
        return text.replace(regex, '<mark>$1</mark>');
    }
    
    clearSearch() {
        const searchResults = document.querySelector('.search-results');
        if (searchResults) {
            searchResults.style.display = 'none';
        }
    }
    
    // Load search index from JSON
    async loadSearchIndex() {
        try {
            const response = await fetch('search.json');
            if (response.ok) {
                this.searchIndex = await response.json();
            }
        } catch (error) {
            console.warn('Search index not available:', error);
        }
    }
    
    // Syntax highlighting
    setupSyntaxHighlighting() {
        const codeBlocks = document.querySelectorAll('pre code');
        codeBlocks.forEach(block => {
            this.highlightCode(block);
        });
    }
    
    highlightCode(element) {
        let code = element.textContent;
        
        // CURSED language syntax highlighting
        const patterns = [
            { pattern: /\b(slay|sus|damn|yeet|facts|bestie|yolo|ready|lowkey|based|cap|cringe|fr|vibe|lit|tea|normie|drip|thicc|smol|mid|meal|snack|byte|rune|extra|sip)\b/g, className: 'keyword' },
            { pattern: /"([^"\\]|\\.)*"/g, className: 'string' },
            { pattern: /'([^'\\]|\\.)*'/g, className: 'string' },
            { pattern: /\/\/.*$/gm, className: 'comment' },
            { pattern: /\/\*[\s\S]*?\*\//g, className: 'comment' },
            { pattern: /\b\d+(\.\d+)?\b/g, className: 'number' },
            { pattern: /\b[a-zA-Z_][a-zA-Z0-9_]*\s*\(/g, className: 'function' },
            { pattern: /\b[A-Z_][A-Z0-9_]*\b/g, className: 'constant' },
        ];
        
        patterns.forEach(({ pattern, className }) => {
            code = code.replace(pattern, (match) => `<span class="${className}">${match}</span>`);
        });
        
        element.innerHTML = code;
    }
    
    // Navigation
    setupNavigation() {
        // Smooth scrolling for anchor links
        document.querySelectorAll('a[href^="#"]').forEach(anchor => {
            anchor.addEventListener('click', (e) => {
                e.preventDefault();
                const target = document.querySelector(anchor.getAttribute('href'));
                if (target) {
                    target.scrollIntoView({ behavior: 'smooth' });
                }
            });
        });
        
        // Highlight current section in navigation
        this.setupScrollSpy();
    }
    
    setupScrollSpy() {
        const navLinks = document.querySelectorAll('.module-nav a');
        const sections = document.querySelectorAll('section[id]');
        
        if (navLinks.length === 0 || sections.length === 0) return;
        
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    const id = entry.target.getAttribute('id');
                    navLinks.forEach(link => {
                        link.classList.remove('active');
                        if (link.getAttribute('href') === `#${id}`) {
                            link.classList.add('active');
                        }
                    });
                }
            });
        }, {
            threshold: 0.1,
            rootMargin: '-20% 0px -80% 0px'
        });
        
        sections.forEach(section => observer.observe(section));
    }
    
    // Tooltips
    setupTooltips() {
        const tooltips = document.querySelectorAll('[data-tooltip]');
        
        tooltips.forEach(element => {
            element.addEventListener('mouseenter', (e) => {
                this.showTooltip(e.target, e.target.getAttribute('data-tooltip'));
            });
            
            element.addEventListener('mouseleave', () => {
                this.hideTooltip();
            });
        });
    }
    
    showTooltip(element, text) {
        const tooltip = document.createElement('div');
        tooltip.className = 'tooltip';
        tooltip.textContent = text;
        
        document.body.appendChild(tooltip);
        
        const rect = element.getBoundingClientRect();
        tooltip.style.left = `${rect.left + (rect.width / 2) - (tooltip.offsetWidth / 2)}px`;
        tooltip.style.top = `${rect.top - tooltip.offsetHeight - 10}px`;
        
        setTimeout(() => tooltip.classList.add('show'), 10);
    }
    
    hideTooltip() {
        const tooltip = document.querySelector('.tooltip');
        if (tooltip) {
            tooltip.remove();
        }
    }
    
    // Keyboard shortcuts
    setupKeyboardShortcuts() {
        document.addEventListener('keydown', (e) => {
            // Ctrl/Cmd + K to focus search
            if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
                e.preventDefault();
                const searchInput = document.querySelector('.search-input');
                if (searchInput) {
                    searchInput.focus();
                }
            }
            
            // Escape to clear search
            if (e.key === 'Escape') {
                this.clearSearch();
            }
        });
    }
    
    // Copy code functionality
    setupCodeCopy() {
        const codeBlocks = document.querySelectorAll('pre code');
        
        codeBlocks.forEach(block => {
            const button = document.createElement('button');
            button.className = 'copy-button';
            button.textContent = 'Copy';
            button.onclick = () => this.copyCode(block, button);
            
            block.parentNode.style.position = 'relative';
            block.parentNode.appendChild(button);
        });
    }
    
    copyCode(codeElement, button) {
        const text = codeElement.textContent;
        navigator.clipboard.writeText(text).then(() => {
            button.textContent = 'Copied!';
            setTimeout(() => {
                button.textContent = 'Copy';
            }, 2000);
        });
    }
    
    // Theme toggle
    setupThemeToggle() {
        const themeToggle = document.querySelector('.theme-toggle');
        
        if (themeToggle) {
            themeToggle.addEventListener('click', () => {
                document.body.classList.toggle('dark-theme');
                localStorage.setItem('theme', document.body.classList.contains('dark-theme') ? 'dark' : 'light');
            });
        }
        
        // Load saved theme
        const savedTheme = localStorage.getItem('theme');
        if (savedTheme === 'dark') {
            document.body.classList.add('dark-theme');
        }
    }
    
    // Collapsible sections
    setupCollapsibleSections() {
        const toggles = document.querySelectorAll('.section-toggle');
        
        toggles.forEach(toggle => {
            toggle.addEventListener('click', () => {
                const section = toggle.closest('.collapsible-section');
                const content = section.querySelector('.section-content');
                
                section.classList.toggle('collapsed');
                
                if (section.classList.contains('collapsed')) {
                    content.style.maxHeight = '0';
                    toggle.textContent = '▶';
                } else {
                    content.style.maxHeight = content.scrollHeight + 'px';
                    toggle.textContent = '▼';
                }
            });
        });
    }
    
    // Progress indicator
    setupProgressIndicator() {
        const progressBar = document.querySelector('.progress-bar');
        
        if (progressBar) {
            window.addEventListener('scroll', () => {
                const totalHeight = document.documentElement.scrollHeight - window.innerHeight;
                const progress = (window.scrollY / totalHeight) * 100;
                progressBar.style.width = `${progress}%`;
            });
        }
    }
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    new CursedDocumentation();
});

// Add CSS for JavaScript-generated elements
const style = document.createElement('style');
style.textContent = `
    .tooltip {
        position: absolute;
        background: #333;
        color: white;
        padding: 8px 12px;
        border-radius: 4px;
        font-size: 12px;
        opacity: 0;
        transition: opacity 0.3s;
        z-index: 1000;
        pointer-events: none;
    }
    
    .tooltip.show {
        opacity: 1;
    }
    
    .tooltip::after {
        content: '';
        position: absolute;
        top: 100%;
        left: 50%;
        transform: translateX(-50%);
        border: 5px solid transparent;
        border-top-color: #333;
    }
    
    .copy-button {
        position: absolute;
        top: 10px;
        right: 10px;
        background: #007bff;
        color: white;
        border: none;
        padding: 5px 10px;
        border-radius: 4px;
        cursor: pointer;
        font-size: 12px;
        opacity: 0;
        transition: opacity 0.3s;
    }
    
    pre:hover .copy-button {
        opacity: 1;
    }
    
    .copy-button:hover {
        background: #0056b3;
    }
    
    .theme-toggle {
        background: none;
        border: none;
        color: inherit;
        cursor: pointer;
        font-size: 18px;
        padding: 5px;
        border-radius: 4px;
        transition: background-color 0.3s;
    }
    
    .theme-toggle:hover {
        background-color: rgba(255, 255, 255, 0.1);
    }
    
    .progress-bar {
        position: fixed;
        top: 0;
        left: 0;
        height: 3px;
        background: #007bff;
        z-index: 1000;
        transition: width 0.3s;
    }
    
    .section-toggle {
        background: none;
        border: none;
        font-size: 16px;
        cursor: pointer;
        padding: 0;
        margin-right: 10px;
        transition: transform 0.3s;
    }
    
    .collapsible-section.collapsed .section-toggle {
        transform: rotate(-90deg);
    }
    
    .section-content {
        overflow: hidden;
        transition: max-height 0.3s ease;
    }
    
    .collapsible-section.collapsed .section-content {
        max-height: 0;
    }
    
    .module-nav a.active {
        background-color: var(--secondary-color);
        color: white;
    }
    
    mark {
        background-color: #ffeb3b;
        padding: 0 2px;
        border-radius: 2px;
    }
`;
document.head.appendChild(style);
