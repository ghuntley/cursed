// CURSED Documentation Generator JavaScript
// Provides interactive functionality for the documentation

(function() {
    'use strict';

    // Initialize when DOM is ready
    document.addEventListener('DOMContentLoaded', function() {
        initializeSearch();
        initializeNavigation();
        initializeSyntaxHighlighting();
        initializeCodeExamples();
        initializeTooltips();
        initializeThemeToggle();
    });

    // Search functionality
    function initializeSearch() {
        const searchContainer = document.querySelector('.search-container');
        if (!searchContainer) return;

        const searchInput = document.querySelector('.search-input');
        const searchResults = document.querySelector('.search-results');
        
        if (!searchInput || !searchResults) return;

        let searchData = [];
        
        // Load search index
        fetch('search.json')
            .then(response => response.json())
            .then(data => {
                searchData = data;
            })
            .catch(error => {
                console.error('Failed to load search index:', error);
            });

        // Handle search input
        searchInput.addEventListener('input', function() {
            const query = this.value.trim().toLowerCase();
            
            if (query.length < 2) {
                searchResults.style.display = 'none';
                return;
            }

            const results = searchData.filter(item => 
                item.title.toLowerCase().includes(query) ||
                item.content.toLowerCase().includes(query)
            ).slice(0, 10);

            displaySearchResults(results, searchResults);
        });

        // Handle search result clicks
        searchResults.addEventListener('click', function(e) {
            const resultElement = e.target.closest('.search-result');
            if (resultElement) {
                const url = resultElement.dataset.url;
                if (url) {
                    window.location.href = url;
                }
            }
        });

        // Hide search results when clicking outside
        document.addEventListener('click', function(e) {
            if (!searchContainer.contains(e.target)) {
                searchResults.style.display = 'none';
            }
        });
    }

    function displaySearchResults(results, container) {
        if (results.length === 0) {
            container.innerHTML = '<div class="search-result">No results found</div>';
        } else {
            container.innerHTML = results.map(result => `
                <div class="search-result" data-url="${result.url}">
                    <div class="search-result-title">${escapeHtml(result.title)}</div>
                    <div class="search-result-type">${escapeHtml(result.type)}</div>
                </div>
            `).join('');
        }
        
        container.style.display = 'block';
    }

    // Navigation functionality
    function initializeNavigation() {
        // Smooth scrolling for anchor links
        document.querySelectorAll('a[href^="#"]').forEach(anchor => {
            anchor.addEventListener('click', function(e) {
                e.preventDefault();
                const targetId = this.getAttribute('href').substring(1);
                const targetElement = document.getElementById(targetId);
                
                if (targetElement) {
                    targetElement.scrollIntoView({
                        behavior: 'smooth',
                        block: 'start'
                    });
                    
                    // Update URL
                    history.pushState(null, null, `#${targetId}`);
                }
            });
        });

        // Highlight current navigation item
        highlightCurrentNav();
        
        // Update navigation on scroll
        window.addEventListener('scroll', highlightCurrentNav);
    }

    function highlightCurrentNav() {
        const sections = document.querySelectorAll('section[id]');
        const navLinks = document.querySelectorAll('.module-nav a');
        
        let current = '';
        const scrollPos = window.pageYOffset + 150;
        
        sections.forEach(section => {
            const top = section.offsetTop;
            const height = section.offsetHeight;
            
            if (scrollPos >= top && scrollPos < top + height) {
                current = section.id;
            }
        });
        
        navLinks.forEach(link => {
            link.classList.remove('current');
            if (link.getAttribute('href') === `#${current}`) {
                link.classList.add('current');
            }
        });
    }

    // Syntax highlighting
    function initializeSyntaxHighlighting() {
        // Simple syntax highlighting for CURSED code
        document.querySelectorAll('pre code').forEach(block => {
            if (block.classList.contains('cursed')) {
                highlightCursedSyntax(block);
            }
        });
    }

    function highlightCursedSyntax(block) {
        let code = block.textContent;
        
        // CURSED keywords
        const keywords = [
            'slay', 'damn', 'sus', 'lowkey', 'highkey', 'yeet', 'vibes',
            'based', 'cap', 'cringe', 'fr', 'bestie', 'yolo', 'ready',
            'ghosted', 'simp', 'yikes', 'shook', 'fam', 'defer'
        ];
        
        // CURSED types
        const types = [
            'normie', 'smol', 'mid', 'thicc', 'drip', 'snack', 'meal',
            'byte', 'rune', 'extra', 'tea', 'lit', 'sip'
        ];
        
        // Apply syntax highlighting
        keywords.forEach(keyword => {
            const regex = new RegExp(`\\b${keyword}\\b`, 'g');
            code = code.replace(regex, `<span class="hljs-keyword">${keyword}</span>`);
        });
        
        types.forEach(type => {
            const regex = new RegExp(`\\b${type}\\b`, 'g');
            code = code.replace(regex, `<span class="hljs-type">${type}</span>`);
        });
        
        // Strings
        code = code.replace(/"([^"\\]*(\\.[^"\\]*)*)"/g, '<span class="hljs-string">"$1"</span>');
        code = code.replace(/'([^'\\]*(\\.[^'\\]*)*)'/g, '<span class="hljs-string">\'$1\'</span>');
        
        // Comments
        code = code.replace(/fr fr (.*?)$/gm, '<span class="hljs-comment">fr fr $1</span>');
        code = code.replace(/\/\*(.*?)\*\//gs, '<span class="hljs-comment">/*$1*/</span>');
        
        // Numbers
        code = code.replace(/\b\d+\.?\d*\b/g, '<span class="hljs-number">$&</span>');
        
        block.innerHTML = code;
    }

    // Code examples functionality
    function initializeCodeExamples() {
        document.querySelectorAll('.example').forEach(example => {
            // Add copy button
            const copyButton = document.createElement('button');
            copyButton.textContent = 'Copy';
            copyButton.className = 'copy-button';
            copyButton.addEventListener('click', function() {
                const code = example.querySelector('code').textContent;
                navigator.clipboard.writeText(code).then(function() {
                    copyButton.textContent = 'Copied!';
                    setTimeout(() => {
                        copyButton.textContent = 'Copy';
                    }, 2000);
                }).catch(function(err) {
                    console.error('Failed to copy code:', err);
                });
            });
            
            example.style.position = 'relative';
            example.appendChild(copyButton);
            
            // Add run button for examples (if supported)
            if (example.dataset.runnable === 'true') {
                const runButton = document.createElement('button');
                runButton.textContent = 'Run';
                runButton.className = 'run-button';
                runButton.addEventListener('click', function() {
                    runCodeExample(example);
                });
                example.appendChild(runButton);
            }
        });
    }

    function runCodeExample(example) {
        // Placeholder for code execution
        // In a real implementation, this would send the code to a server
        // or use a client-side CURSED interpreter
        console.log('Running code example:', example.querySelector('code').textContent);
        
        // Show output placeholder
        let output = example.querySelector('.example-output');
        if (!output) {
            output = document.createElement('div');
            output.className = 'example-output';
            example.appendChild(output);
        }
        
        output.innerHTML = '<div class="output-placeholder">Code execution not implemented</div>';
    }

    // Tooltips
    function initializeTooltips() {
        document.querySelectorAll('[data-tooltip]').forEach(element => {
            element.addEventListener('mouseenter', function() {
                showTooltip(this, this.dataset.tooltip);
            });
            
            element.addEventListener('mouseleave', function() {
                hideTooltip();
            });
        });
    }

    function showTooltip(element, text) {
        const tooltip = document.createElement('div');
        tooltip.className = 'tooltip';
        tooltip.textContent = text;
        document.body.appendChild(tooltip);
        
        const rect = element.getBoundingClientRect();
        tooltip.style.left = rect.left + (rect.width / 2) - (tooltip.offsetWidth / 2) + 'px';
        tooltip.style.top = rect.top - tooltip.offsetHeight - 10 + 'px';
        
        setTimeout(() => {
            tooltip.classList.add('visible');
        }, 10);
    }

    function hideTooltip() {
        const tooltip = document.querySelector('.tooltip');
        if (tooltip) {
            tooltip.remove();
        }
    }

    // Theme toggle
    function initializeThemeToggle() {
        const themeToggle = document.querySelector('.theme-toggle');
        if (!themeToggle) return;
        
        // Check for saved theme or default to light
        const savedTheme = localStorage.getItem('cursed-docs-theme');
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        const theme = savedTheme || (prefersDark ? 'dark' : 'light');
        
        document.documentElement.setAttribute('data-theme', theme);
        updateThemeToggle(themeToggle, theme);
        
        themeToggle.addEventListener('click', function() {
            const currentTheme = document.documentElement.getAttribute('data-theme');
            const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
            
            document.documentElement.setAttribute('data-theme', newTheme);
            localStorage.setItem('cursed-docs-theme', newTheme);
            updateThemeToggle(themeToggle, newTheme);
        });
    }

    function updateThemeToggle(toggle, theme) {
        toggle.textContent = theme === 'dark' ? '☀️' : '🌙';
        toggle.setAttribute('aria-label', `Switch to ${theme === 'dark' ? 'light' : 'dark'} theme`);
    }

    // Utility functions
    function escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    function debounce(func, wait) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    }

    // Keyboard shortcuts
    document.addEventListener('keydown', function(e) {
        // Ctrl/Cmd + K to focus search
        if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
            e.preventDefault();
            const searchInput = document.querySelector('.search-input');
            if (searchInput) {
                searchInput.focus();
            }
        }
        
        // Escape to close search results
        if (e.key === 'Escape') {
            const searchResults = document.querySelector('.search-results');
            if (searchResults) {
                searchResults.style.display = 'none';
            }
        }
    });

    // Performance monitoring
    if (window.performance && window.performance.mark) {
        window.performance.mark('cursed-docs-js-loaded');
    }

    // Analytics placeholder
    function trackPageView() {
        // Placeholder for analytics tracking
        console.log('Page view tracked:', window.location.pathname);
    }

    function trackEvent(category, action, label) {
        // Placeholder for event tracking
        console.log('Event tracked:', { category, action, label });
    }

    // Export for testing
    window.CursedDocs = {
        trackPageView,
        trackEvent,
        escapeHtml,
        debounce
    };

})();
