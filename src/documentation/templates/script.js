// CURSED Documentation JavaScript - Production Quality

// Global state and configuration
const CURSED_DOCS = {
    searchIndex: null,
    navigationData: null,
    currentTheme: 'auto',
    initialized: false
};

// Initialize documentation when DOM is ready
document.addEventListener('DOMContentLoaded', function() {
    initializeDocumentation();
});

// Main initialization function
function initializeDocumentation() {
    if (CURSED_DOCS.initialized) return;
    
    try {
        setupThemeSystem();
        setupSearchFunctionality();
        setupNavigationEnhancements();
        setupCodeBlockFeatures();
        setupKeyboardShortcuts();
        setupAccessibilityFeatures();
        setupPerformanceOptimizations();
        
        CURSED_DOCS.initialized = true;
        console.log('CURSED Documentation initialized successfully');
    } catch (error) {
        console.error('Failed to initialize documentation:', error);
    }
}

// Theme System
function setupThemeSystem() {
    const themeToggle = document.createElement('button');
    themeToggle.className = 'theme-toggle';
    themeToggle.innerHTML = '🌓';
    themeToggle.title = 'Toggle theme';
    themeToggle.addEventListener('click', toggleTheme);
    
    // Add theme toggle to sidebar header
    const sidebarHeader = document.querySelector('.sidebar-header');
    if (sidebarHeader) {
        sidebarHeader.appendChild(themeToggle);
    }
    
    // Apply saved theme
    const savedTheme = localStorage.getItem('cursed-docs-theme') || 'auto';
    applyTheme(savedTheme);
}

function toggleTheme() {
    const currentTheme = CURSED_DOCS.currentTheme;
    const newTheme = currentTheme === 'light' ? 'dark' : 
                    currentTheme === 'dark' ? 'auto' : 'light';
    
    applyTheme(newTheme);
    localStorage.setItem('cursed-docs-theme', newTheme);
}

function applyTheme(theme) {
    CURSED_DOCS.currentTheme = theme;
    document.body.className = document.body.className.replace(/theme-\w+/g, '');
    
    if (theme !== 'auto') {
        document.body.classList.add(`theme-${theme}`);
    }
}

// Enhanced Search Functionality
function setupSearchFunctionality() {
    const searchInput = document.getElementById('search-input');
    const searchButton = document.getElementById('search-button');
    const searchResults = document.getElementById('search-results');
    
    if (!searchInput) return;
    
    // Setup search input handlers
    searchInput.addEventListener('input', debounce(handleSearchInput, 300));
    searchInput.addEventListener('keydown', handleSearchKeydown);
    
    if (searchButton) {
        searchButton.addEventListener('click', () => performSearch(searchInput.value));
    }
    
    // Setup search suggestions
    setupSearchSuggestions(searchInput);
    
    // Load search index if available
    loadSearchIndex().then(index => {
        CURSED_DOCS.searchIndex = index;
        console.log('Search index loaded:', index ? 'success' : 'failed');
    });
}

function handleSearchInput(event) {
    const query = event.target.value.trim();
    
    if (query.length === 0) {
        clearSearchResults();
        return;
    }
    
    if (query.length < 2) {
        showSearchMessage('Type at least 2 characters to search');
        return;
    }
    
    performSearch(query);
}

function handleSearchKeydown(event) {
    if (event.key === 'Enter') {
        event.preventDefault();
        performSearch(event.target.value);
    } else if (event.key === 'Escape') {
        clearSearchResults();
        event.target.blur();
    }
}

function performSearch(query) {
    if (!query || query.length < 2) return;
    
    showSearchMessage('Searching...', 'loading');
    
    // Get active filters
    const filters = getActiveSearchFilters();
    
    // Perform search
    const results = searchDocumentation(query, filters);
    
    displaySearchResults(results, query);
    updateSearchStats(results.length, query);
}

function searchDocumentation(query, filters = {}) {
    if (!CURSED_DOCS.searchIndex) {
        return [];
    }
    
    const searchTerms = query.toLowerCase().split(/\s+/);
    const results = [];
    
    for (const item of CURSED_DOCS.searchIndex.items) {
        // Apply type filters
        if (filters.functions === false && item.type === 'Function') continue;
        if (filters.types === false && ['Squad', 'Collab'].includes(item.type)) continue;
        if (filters.packages === false && item.type === 'Package') continue;
        
        const score = calculateSearchScore(item, searchTerms);
        if (score > 0) {
            results.push({ ...item, score });
        }
    }
    
    // Sort by score (highest first)
    return results.sort((a, b) => b.score - a.score);
}

function calculateSearchScore(item, searchTerms) {
    let score = 0;
    const itemText = `${item.name} ${item.description} ${item.keywords.join(' ')}`.toLowerCase();
    
    for (const term of searchTerms) {
        // Exact name match gets highest score
        if (item.name.toLowerCase() === term) {
            score += 100;
        } else if (item.name.toLowerCase().includes(term)) {
            score += 50;
        }
        
        // Description matches
        if (item.description.toLowerCase().includes(term)) {
            score += 20;
        }
        
        // Keyword matches
        for (const keyword of item.keywords) {
            if (keyword.toLowerCase().includes(term)) {
                score += 10;
            }
        }
        
        // General text match
        if (itemText.includes(term)) {
            score += 5;
        }
    }
    
    return score;
}

function displaySearchResults(results, query) {
    const container = document.getElementById('search-results');
    if (!container) return;
    
    if (results.length === 0) {
        showSearchMessage(`No results found for "${query}"`);
        return;
    }
    
    const resultsHTML = results.map(item => createSearchResultHTML(item, query)).join('');
    container.innerHTML = `<div class="search-results">${resultsHTML}</div>`;
}

function createSearchResultHTML(item, query) {
    const highlightedName = highlightSearchTerms(item.name, query);
    const highlightedDescription = highlightSearchTerms(item.description, query);
    
    return `
        <div class="search-result">
            <h3>
                <a href="${item.url}">${highlightedName}</a>
                <span class="item-type item-type-${item.type.toLowerCase()}">${item.type}</span>
            </h3>
            <p>${highlightedDescription}</p>
            <div class="search-result-meta">
                <span class="result-category">${item.category}</span>
                <span class="result-score">Score: ${item.score}</span>
            </div>
        </div>
    `;
}

function highlightSearchTerms(text, query) {
    const terms = query.toLowerCase().split(/\s+/);
    let result = text;
    
    for (const term of terms) {
        const regex = new RegExp(`(${escapeRegex(term)})`, 'gi');
        result = result.replace(regex, '<span class="search-highlight">$1</span>');
    }
    
    return result;
}

function getActiveSearchFilters() {
    return {
        functions: document.getElementById('filter-functions')?.checked ?? true,
        types: document.getElementById('filter-types')?.checked ?? true,
        packages: document.getElementById('filter-packages')?.checked ?? true
    };
}

function updateSearchStats(count, query) {
    const statsElement = document.getElementById('search-stats');
    if (statsElement) {
        const timeStamp = new Date().toLocaleTimeString();
        statsElement.textContent = `Found ${count} results for "${query}" at ${timeStamp}`;
    }
}

function showSearchMessage(message, type = 'info') {
    const container = document.getElementById('search-results');
    if (!container) return;
    
    const iconMap = {
        loading: '⏳',
        info: 'ℹ️',
        error: '❌',
        warning: '⚠️'
    };
    
    container.innerHTML = `
        <div class="search-message search-message-${type}">
            <span class="message-icon">${iconMap[type] || '💬'}</span>
            <span class="message-text">${message}</span>
        </div>
    `;
}

function clearSearchResults() {
    const container = document.getElementById('search-results');
    if (container) {
        container.innerHTML = '<div class="search-placeholder"><div class="placeholder-icon">🔍</div><h3>Start typing to search</h3></div>';
    }
    
    const stats = document.getElementById('search-stats');
    if (stats) {
        stats.textContent = '';
    }
}

// Search Suggestions
function setupSearchSuggestions(searchInput) {
    const suggestionsContainer = document.getElementById('search-suggestions') || 
                                 createSearchSuggestions(searchInput);
    
    searchInput.addEventListener('focus', () => showPopularSearches(suggestionsContainer));
    searchInput.addEventListener('blur', () => 
        setTimeout(() => hideSuggestions(suggestionsContainer), 200));
}

function createSearchSuggestions(searchInput) {
    const container = document.createElement('div');
    container.id = 'search-suggestions';
    container.className = 'search-suggestions';
    searchInput.parentNode.appendChild(container);
    return container;
}

function showPopularSearches(container) {
    const popularSearches = [
        'functions', 'types', 'squad', 'collab', 'slay', 'yolo', 'normie', 'periodt'
    ];
    
    const suggestionsHTML = popularSearches.map(term => 
        `<div class="search-suggestion" onclick="selectSuggestion('${term}')">${term}</div>`
    ).join('');
    
    container.innerHTML = `
        <div class="suggestions-header">Popular searches:</div>
        ${suggestionsHTML}
    `;
    container.style.display = 'block';
}

function hideSuggestions(container) {
    container.style.display = 'none';
}

function selectSuggestion(term) {
    const searchInput = document.getElementById('search-input');
    if (searchInput) {
        searchInput.value = term;
        performSearch(term);
    }
}

// Navigation Enhancements
function setupNavigationEnhancements() {
    highlightActiveNavigation();
    setupSmoothScrolling();
    setupNavigationKeyboard();
    loadNavigationData();
}

function highlightActiveNavigation() {
    const currentPath = window.location.pathname;
    const navLinks = document.querySelectorAll('.sidebar a');
    
    navLinks.forEach(link => {
        if (link.getAttribute('href') === currentPath) {
            link.classList.add('active');
        }
    });
}

function setupSmoothScrolling() {
    document.querySelectorAll('a[href^="#"]').forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({ 
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
}

function setupNavigationKeyboard() {
    document.addEventListener('keydown', function(e) {
        // Alt + Left: Go back
        if (e.altKey && e.key === 'ArrowLeft') {
            e.preventDefault();
            history.back();
        }
        
        // Alt + Right: Go forward
        if (e.altKey && e.key === 'ArrowRight') {
            e.preventDefault();
            history.forward();
        }
    });
}

// Code Block Features
function setupCodeBlockFeatures() {
    addCopyButtons();
    addLineNumbers();
    setupSyntaxHighlighting();
    setupCodeFolding();
}

function addCopyButtons() {
    document.querySelectorAll('pre code').forEach(block => {
        const container = block.parentElement;
        if (container.querySelector('.copy-button')) return; // Already has button
        
        container.style.position = 'relative';
        container.classList.add('code-block');
        
        const copyButton = document.createElement('button');
        copyButton.className = 'copy-button';
        copyButton.textContent = 'Copy';
        copyButton.title = 'Copy code to clipboard';
        copyButton.onclick = () => copyCodeToClipboard(block, copyButton);
        
        container.appendChild(copyButton);
    });
}

function copyCodeToClipboard(codeBlock, button) {
    const text = codeBlock.textContent;
    
    navigator.clipboard.writeText(text).then(() => {
        const originalText = button.textContent;
        button.textContent = 'Copied!';
        button.classList.add('copied');
        
        setTimeout(() => {
            button.textContent = originalText;
            button.classList.remove('copied');
        }, 2000);
        
        showNotification('Code copied to clipboard!');
    }).catch(err => {
        console.error('Failed to copy code:', err);
        fallbackCopyTextToClipboard(text);
    });
}

function fallbackCopyTextToClipboard(text) {
    const textArea = document.createElement('textarea');
    textArea.value = text;
    textArea.style.position = 'fixed';
    textArea.style.top = '0';
    textArea.style.left = '0';
    textArea.style.width = '2em';
    textArea.style.height = '2em';
    textArea.style.padding = '0';
    textArea.style.border = 'none';
    textArea.style.outline = 'none';
    textArea.style.boxShadow = 'none';
    textArea.style.background = 'transparent';
    
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();
    
    try {
        document.execCommand('copy');
        showNotification('Code copied to clipboard!');
    } catch (err) {
        console.error('Fallback copy failed:', err);
        showNotification('Failed to copy code', 'error');
    }
    
    document.body.removeChild(textArea);
}

function addLineNumbers() {
    document.querySelectorAll('pre code').forEach(block => {
        if (block.parentElement.classList.contains('line-numbers')) return;
        
        const lines = block.textContent.split('\n');
        if (lines.length < 5) return; // Only add line numbers for longer code blocks
        
        const lineNumbers = lines.map((_, i) => i + 1).join('\n');
        
        const lineNumberElement = document.createElement('span');
        lineNumberElement.className = 'line-numbers-rows';
        lineNumberElement.textContent = lineNumbers;
        
        block.parentElement.classList.add('line-numbers');
        block.parentElement.insertBefore(lineNumberElement, block);
    });
}

function setupSyntaxHighlighting() {
    // Enhanced CURSED syntax highlighting
    document.querySelectorAll('code.language-cursed').forEach(block => {
        highlightCursedSyntax(block);
    });
}

function highlightCursedSyntax(block) {
    let content = block.innerHTML;
    
    // CURSED keywords with colors
    const keywordPatterns = [
        { pattern: /\b(slay|yolo)\b/g, class: 'keyword-function' },
        { pattern: /\b(sus|facts)\b/g, class: 'keyword-variable' },
        { pattern: /\b(lowkey|highkey)\b/g, class: 'keyword-conditional' },
        { pattern: /\b(periodt|bestie|flex)\b/g, class: 'keyword-control' },
        { pattern: /\b(squad|collab)\b/g, class: 'keyword-type' },
        { pattern: /\b(vibe_check|mood|basic)\b/g, class: 'keyword-switch' }
    ];
    
    // Types
    const typePatterns = [
        { pattern: /\b(normie|str|bool|void)\b/g, class: 'type' }
    ];
    
    // Apply highlighting
    keywordPatterns.forEach(({ pattern, class: className }) => {
        content = content.replace(pattern, `<span class="${className}">$1</span>`);
    });
    
    typePatterns.forEach(({ pattern, class: className }) => {
        content = content.replace(pattern, `<span class="${className}">$1</span>`);
    });
    
    // String literals
    content = content.replace(/"([^"\\]|\\.)*"/g, '<span class="string">"$1"</span>');
    
    // Comments
    content = content.replace(/\/\/.*$/gm, '<span class="comment">$&</span>');
    content = content.replace(/\/\*[\s\S]*?\*\//g, '<span class="comment">$&</span>');
    
    // Numbers
    content = content.replace(/\b\d+(\.\d+)?\b/g, '<span class="number">$&</span>');
    
    block.innerHTML = content;
}

function setupCodeFolding() {
    document.querySelectorAll('pre code').forEach(block => {
        const lines = block.textContent.split('\n');
        if (lines.length > 20) {
            addCodeFolding(block.parentElement);
        }
    });
}

function addCodeFolding(container) {
    const toggleButton = document.createElement('button');
    toggleButton.className = 'code-fold-toggle';
    toggleButton.textContent = '⊟';
    toggleButton.title = 'Fold/unfold code';
    toggleButton.onclick = () => toggleCodeFold(container);
    
    container.style.position = 'relative';
    container.appendChild(toggleButton);
}

function toggleCodeFold(container) {
    const button = container.querySelector('.code-fold-toggle');
    const codeBlock = container.querySelector('code');
    
    if (container.classList.contains('folded')) {
        container.classList.remove('folded');
        button.textContent = '⊟';
        button.title = 'Fold code';
    } else {
        container.classList.add('folded');
        button.textContent = '⊞';
        button.title = 'Unfold code';
    }
}

// Keyboard Shortcuts
function setupKeyboardShortcuts() {
    document.addEventListener('keydown', function(e) {
        // Ctrl/Cmd + K: Focus search
        if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
            e.preventDefault();
            const searchInput = document.getElementById('search-input');
            if (searchInput) {
                searchInput.focus();
                searchInput.select();
            }
        }
        
        // Escape: Clear search
        if (e.key === 'Escape') {
            const searchInput = document.getElementById('search-input');
            if (searchInput && document.activeElement === searchInput) {
                searchInput.blur();
                clearSearchResults();
            }
        }
        
        // Ctrl/Cmd + /: Show help
        if ((e.ctrlKey || e.metaKey) && e.key === '/') {
            e.preventDefault();
            showKeyboardShortcuts();
        }
    });
}

function showKeyboardShortcuts() {
    const shortcuts = [
        { key: 'Ctrl/Cmd + K', description: 'Focus search' },
        { key: 'Escape', description: 'Clear search / Close dialogs' },
        { key: 'Ctrl/Cmd + /', description: 'Show keyboard shortcuts' },
        { key: 'Alt + ←', description: 'Go back' },
        { key: 'Alt + →', description: 'Go forward' }
    ];
    
    const shortcutsHTML = shortcuts.map(({ key, description }) => 
        `<div class="shortcut"><kbd>${key}</kbd><span>${description}</span></div>`
    ).join('');
    
    showModal('Keyboard Shortcuts', `<div class="shortcuts-list">${shortcutsHTML}</div>`);
}

// Accessibility Features
function setupAccessibilityFeatures() {
    addSkipLinks();
    enhanceKeyboardNavigation();
    addAriaLabels();
    setupFocusManagement();
}

function addSkipLinks() {
    const skipLink = document.createElement('a');
    skipLink.href = '#main-content';
    skipLink.className = 'skip-link';
    skipLink.textContent = 'Skip to main content';
    skipLink.addEventListener('click', function(e) {
        e.preventDefault();
        const target = document.querySelector('#main-content, main, .main-content');
        if (target) {
            target.focus();
            target.scrollIntoView();
        }
    });
    
    document.body.insertBefore(skipLink, document.body.firstChild);
}

function enhanceKeyboardNavigation() {
    // Make all interactive elements keyboard accessible
    document.querySelectorAll('.item-card, .search-result').forEach(element => {
        if (!element.getAttribute('tabindex')) {
            element.setAttribute('tabindex', '0');
        }
        
        element.addEventListener('keydown', function(e) {
            if (e.key === 'Enter' || e.key === ' ') {
                const link = this.querySelector('a');
                if (link) {
                    link.click();
                }
            }
        });
    });
}

function addAriaLabels() {
    // Add ARIA labels where missing
    const searchInput = document.getElementById('search-input');
    if (searchInput && !searchInput.getAttribute('aria-label')) {
        searchInput.setAttribute('aria-label', 'Search documentation');
    }
    
    const searchButton = document.getElementById('search-button');
    if (searchButton && !searchButton.getAttribute('aria-label')) {
        searchButton.setAttribute('aria-label', 'Search');
    }
}

function setupFocusManagement() {
    // Manage focus for better accessibility
    document.addEventListener('click', function(e) {
        if (e.target.matches('.tab-button')) {
            const tabId = e.target.getAttribute('data-tab');
            const tabContent = document.getElementById(`${tabId}-tab`);
            if (tabContent) {
                setTimeout(() => tabContent.focus(), 100);
            }
        }
    });
}

// Performance Optimizations
function setupPerformanceOptimizations() {
    lazyLoadImages();
    optimizeScrolling();
    preloadImportantPages();
}

function lazyLoadImages() {
    if ('IntersectionObserver' in window) {
        const imageObserver = new IntersectionObserver((entries, observer) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    const img = entry.target;
                    img.src = img.dataset.src;
                    img.classList.remove('lazy');
                    observer.unobserve(img);
                }
            });
        });
        
        document.querySelectorAll('img[data-src]').forEach(img => {
            imageObserver.observe(img);
        });
    }
}

function optimizeScrolling() {
    let ticking = false;
    
    function updateScrollPosition() {
        // Update any scroll-dependent features
        ticking = false;
    }
    
    window.addEventListener('scroll', function() {
        if (!ticking) {
            requestAnimationFrame(updateScrollPosition);
            ticking = true;
        }
    });
}

function preloadImportantPages() {
    // Preload important pages on hover
    document.querySelectorAll('a[href]').forEach(link => {
        if (link.href.includes(window.location.origin)) {
            link.addEventListener('mouseenter', function() {
                const linkElement = document.createElement('link');
                linkElement.rel = 'prefetch';
                linkElement.href = this.href;
                document.head.appendChild(linkElement);
            }, { once: true });
        }
    });
}

// Utility Functions
function debounce(func, wait, immediate) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            timeout = null;
            if (!immediate) func(...args);
        };
        const callNow = immediate && !timeout;
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
        if (callNow) func(...args);
    };
}

function escapeRegex(string) {
    return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function generateTableOfContents(containerSelector, tocSelector) {
    const container = document.querySelector(`.${containerSelector}`);
    const tocList = document.getElementById(tocSelector);
    
    if (!container || !tocList) return;
    
    const headings = container.querySelectorAll('h2, h3');
    if (headings.length === 0) return;
    
    const tocHTML = Array.from(headings).map((heading, index) => {
        const id = `toc-${index}`;
        heading.id = id;
        
        const level = parseInt(heading.tagName.substring(1));
        const indent = level === 3 ? 'toc-indent' : '';
        
        return `<li class="${indent}"><a href="#${id}">${heading.textContent}</a></li>`;
    }).join('');
    
    tocList.innerHTML = tocHTML;
}

function setupTabSwitching() {
    document.querySelectorAll('.tab-button').forEach(button => {
        button.addEventListener('click', function() {
            const tabId = this.getAttribute('data-tab');
            
            // Remove active class from all tabs
            document.querySelectorAll('.tab-button').forEach(btn => 
                btn.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(content => 
                content.style.display = 'none');
            
            // Add active class to clicked tab
            this.classList.add('active');
            const tabContent = document.getElementById(`${tabId}-tab`);
            if (tabContent) {
                tabContent.style.display = 'block';
            }
        });
    });
}

function loadRelatedFunctions(functionName, containerId) {
    // This would typically load from an API or data file
    const container = document.getElementById(containerId);
    if (container) {
        container.innerHTML = '<p>Loading related functions...</p>';
        
        // Simulate loading
        setTimeout(() => {
            container.innerHTML = '<p>No related functions found.</p>';
        }, 1000);
    }
}

function generateTypeHierarchy(typeName, containerId) {
    // This would typically generate a visual hierarchy
    const container = document.getElementById(containerId);
    if (container) {
        container.innerHTML = `<p>Type hierarchy for ${typeName} would be displayed here.</p>`;
    }
}

function showNotification(message, type = 'success') {
    const notification = document.createElement('div');
    notification.className = `notification notification-${type}`;
    notification.textContent = message;
    
    document.body.appendChild(notification);
    
    setTimeout(() => {
        notification.remove();
    }, 3000);
}

function showModal(title, content) {
    const modal = document.createElement('div');
    modal.className = 'modal';
    modal.innerHTML = `
        <div class="modal-content">
            <div class="modal-header">
                <h3>${title}</h3>
                <button class="modal-close">&times;</button>
            </div>
            <div class="modal-body">${content}</div>
        </div>
    `;
    
    const closeModal = () => {
        modal.remove();
        document.removeEventListener('keydown', escapeHandler);
    };
    
    const escapeHandler = (e) => {
        if (e.key === 'Escape') closeModal();
    };
    
    modal.querySelector('.modal-close').addEventListener('click', closeModal);
    modal.addEventListener('click', (e) => {
        if (e.target === modal) closeModal();
    });
    
    document.addEventListener('keydown', escapeHandler);
    document.body.appendChild(modal);
    
    // Focus management
    modal.querySelector('.modal-close').focus();
}

// Advanced Search Functions
function initializeAdvancedSearch() {
    setupSearchKeyboardShortcuts();
    setupSearchFilters();
    setupSearchHistory();
}

function setupSearchKeyboardShortcuts() {
    const searchInput = document.getElementById('search-input');
    if (!searchInput) return;
    
    searchInput.addEventListener('keydown', function(e) {
        if (e.key === 'ArrowDown') {
            e.preventDefault();
            focusFirstSearchResult();
        }
    });
}

function setupSearchFilters() {
    document.querySelectorAll('#search-filters input[type="checkbox"]').forEach(checkbox => {
        checkbox.addEventListener('change', function() {
            const searchInput = document.getElementById('search-input');
            if (searchInput && searchInput.value.trim()) {
                performSearch(searchInput.value);
            }
        });
    });
}

function setupSearchHistory() {
    const searchInput = document.getElementById('search-input');
    if (!searchInput) return;
    
    // Load search history
    const history = JSON.parse(localStorage.getItem('cursed-search-history') || '[]');
    
    searchInput.addEventListener('keydown', function(e) {
        if (e.key === 'Enter' && this.value.trim()) {
            addToSearchHistory(this.value.trim());
        }
    });
}

function addToSearchHistory(query) {
    let history = JSON.parse(localStorage.getItem('cursed-search-history') || '[]');
    history = history.filter(item => item !== query); // Remove duplicates
    history.unshift(query);
    history = history.slice(0, 10); // Keep only last 10
    localStorage.setItem('cursed-search-history', JSON.stringify(history));
}

function focusFirstSearchResult() {
    const firstResult = document.querySelector('.search-result a');
    if (firstResult) {
        firstResult.focus();
    }
}

// Data Loading Functions
async function loadSearchIndex() {
    try {
        // Try to load from search-index.js
        if (window.CURSED_SEARCH_INDEX) {
            return window.CURSED_SEARCH_INDEX;
        }
        
        // Fallback: try to fetch if available
        const response = await fetch('assets/search-index.js');
        if (response.ok) {
            const script = await response.text();
            eval(script); // Execute the script to set window.CURSED_SEARCH_INDEX
            return window.CURSED_SEARCH_INDEX;
        }
        
        return null;
    } catch (error) {
        console.warn('Failed to load search index:', error);
        return null;
    }
}

async function loadNavigationData() {
    try {
        if (window.CURSED_NAVIGATION) {
            CURSED_DOCS.navigationData = window.CURSED_NAVIGATION;
            return;
        }
        
        const response = await fetch('assets/navigation.js');
        if (response.ok) {
            const script = await response.text();
            eval(script);
            CURSED_DOCS.navigationData = window.CURSED_NAVIGATION;
        }
    } catch (error) {
        console.warn('Failed to load navigation data:', error);
    }
}

// Export for external use
window.CURSED_DOCS = CURSED_DOCS;
window.performSearch = performSearch;
window.generateTableOfContents = generateTableOfContents;
window.setupTabSwitching = setupTabSwitching;
window.loadRelatedFunctions = loadRelatedFunctions;
window.generateTypeHierarchy = generateTypeHierarchy;
window.showNotification = showNotification;
