// CURSED Documentation JavaScript

(function() {
    'use strict';

    // Search functionality
    let searchIndex = window.searchIndex || [];
    const searchInput = document.getElementById('search-input');
    const searchResults = document.getElementById('search-results');
    let searchTimeout = null;

    if (searchInput && searchResults) {
        searchInput.addEventListener('input', function(e) {
            clearTimeout(searchTimeout);
            const query = e.target.value.trim();
            
            if (query.length < 2) {
                hideSearchResults();
                return;
            }

            searchTimeout = setTimeout(() => {
                performSearch(query);
            }, 150);
        });

        searchInput.addEventListener('keydown', function(e) {
            if (e.key === 'Escape') {
                hideSearchResults();
                searchInput.blur();
            } else if (e.key === 'ArrowDown') {
                e.preventDefault();
                selectNextResult();
            } else if (e.key === 'ArrowUp') {
                e.preventDefault();
                selectPreviousResult();
            } else if (e.key === 'Enter') {
                e.preventDefault();
                openSelectedResult();
            }
        });

        // Hide search results when clicking outside
        document.addEventListener('click', function(e) {
            if (!searchInput.contains(e.target) && !searchResults.contains(e.target)) {
                hideSearchResults();
            }
        });
    }

    function performSearch(query) {
        const results = searchInIndex(query);
        displaySearchResults(results.slice(0, 10)); // Limit to 10 results
    }

    function searchInIndex(query) {
        const queryLower = query.toLowerCase();
        const results = [];

        for (const item of searchIndex) {
            let score = 0;
            let matchedTerms = [];

            // Exact name match gets highest score
            if (item.name.toLowerCase() === queryLower) {
                score += 100;
                matchedTerms.push('name');
            } else if (item.name.toLowerCase().includes(queryLower)) {
                score += 50;
                matchedTerms.push('name');
            }

            // Type/kind match
            if (item.kind.toLowerCase().includes(queryLower)) {
                score += 30;
                matchedTerms.push('kind');
            }

            // Description match
            if (item.description.toLowerCase().includes(queryLower)) {
                score += 20;
                matchedTerms.push('description');
            }

            // Module match
            if (item.module.toLowerCase().includes(queryLower)) {
                score += 15;
                matchedTerms.push('module');
            }

            // Keywords match
            for (const keyword of item.keywords) {
                if (keyword.toLowerCase().includes(queryLower)) {
                    score += 10;
                    matchedTerms.push('keyword');
                    break;
                }
            }

            // Fuzzy matching for typos
            if (score === 0) {
                const fuzzyScore = calculateFuzzyScore(item.name.toLowerCase(), queryLower);
                if (fuzzyScore > 0.6) {
                    score += fuzzyScore * 25;
                    matchedTerms.push('fuzzy');
                }
            }

            if (score > 0) {
                results.push({
                    ...item,
                    score,
                    matchedTerms
                });
            }
        }

        // Sort by score (highest first)
        results.sort((a, b) => b.score - a.score);
        return results;
    }

    function calculateFuzzyScore(target, query) {
        if (query.length === 0) return 1;
        if (target.length === 0) return 0;

        const matrix = [];
        for (let i = 0; i <= target.length; i++) {
            matrix[i] = [i];
        }
        for (let j = 0; j <= query.length; j++) {
            matrix[0][j] = j;
        }

        for (let i = 1; i <= target.length; i++) {
            for (let j = 1; j <= query.length; j++) {
                if (target[i - 1] === query[j - 1]) {
                    matrix[i][j] = matrix[i - 1][j - 1];
                } else {
                    matrix[i][j] = Math.min(
                        matrix[i - 1][j - 1] + 1,
                        matrix[i][j - 1] + 1,
                        matrix[i - 1][j] + 1
                    );
                }
            }
        }

        const distance = matrix[target.length][query.length];
        const maxLength = Math.max(target.length, query.length);
        return 1 - (distance / maxLength);
    }

    function displaySearchResults(results) {
        if (results.length === 0) {
            searchResults.innerHTML = '<div class="search-result"><div class="search-result-title">No results found</div></div>';
        } else {
            searchResults.innerHTML = results.map((result, index) => `
                <div class="search-result" data-index="${index}" data-url="${result.url}">
                    <div class="search-result-title">${highlightMatch(result.name, searchInput.value)}</div>
                    <div class="search-result-description">${highlightMatch(result.description, searchInput.value)}</div>
                    <div class="search-result-module">${result.module} • ${result.kind}</div>
                </div>
            `).join('');

            // Add click handlers
            searchResults.querySelectorAll('.search-result').forEach(result => {
                result.addEventListener('click', function() {
                    const url = this.getAttribute('data-url');
                    if (url) {
                        window.location.href = url;
                    }
                });
            });
        }

        showSearchResults();
    }

    function highlightMatch(text, query) {
        if (!query) return text;
        
        const regex = new RegExp(`(${escapeRegExp(query)})`, 'gi');
        return text.replace(regex, '<strong>$1</strong>');
    }

    function escapeRegExp(string) {
        return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    }

    function showSearchResults() {
        searchResults.style.display = 'block';
    }

    function hideSearchResults() {
        searchResults.style.display = 'none';
        clearSelection();
    }

    let selectedIndex = -1;

    function selectNextResult() {
        const results = searchResults.querySelectorAll('.search-result');
        if (results.length === 0) return;

        clearSelection();
        selectedIndex = (selectedIndex + 1) % results.length;
        results[selectedIndex].classList.add('selected');
    }

    function selectPreviousResult() {
        const results = searchResults.querySelectorAll('.search-result');
        if (results.length === 0) return;

        clearSelection();
        selectedIndex = selectedIndex <= 0 ? results.length - 1 : selectedIndex - 1;
        results[selectedIndex].classList.add('selected');
    }

    function clearSelection() {
        const selected = searchResults.querySelector('.search-result.selected');
        if (selected) {
            selected.classList.remove('selected');
        }
    }

    function openSelectedResult() {
        const selected = searchResults.querySelector('.search-result.selected');
        if (selected) {
            const url = selected.getAttribute('data-url');
            if (url) {
                window.location.href = url;
            }
        } else {
            // Open first result if none selected
            const firstResult = searchResults.querySelector('.search-result');
            if (firstResult) {
                const url = firstResult.getAttribute('data-url');
                if (url) {
                    window.location.href = url;
                }
            }
        }
    }

    // Dark mode toggle
    function initThemeToggle() {
        const savedTheme = localStorage.getItem('theme') || 'light';
        document.documentElement.setAttribute('data-theme', savedTheme);

        // Create theme toggle button if it doesn't exist
        if (!document.querySelector('.theme-toggle')) {
            const themeToggle = document.createElement('button');
            themeToggle.className = 'theme-toggle';
            themeToggle.innerHTML = savedTheme === 'dark' ? '☀️' : '🌙';
            themeToggle.title = `Switch to ${savedTheme === 'dark' ? 'light' : 'dark'} mode`;
            
            themeToggle.addEventListener('click', toggleTheme);
            
            const navbar = document.querySelector('.navbar-nav');
            if (navbar) {
                navbar.appendChild(themeToggle);
            }
        }
    }

    function toggleTheme() {
        const currentTheme = document.documentElement.getAttribute('data-theme');
        const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
        
        document.documentElement.setAttribute('data-theme', newTheme);
        localStorage.setItem('theme', newTheme);
        
        const themeToggle = document.querySelector('.theme-toggle');
        if (themeToggle) {
            themeToggle.innerHTML = newTheme === 'dark' ? '☀️' : '🌙';
            themeToggle.title = `Switch to ${newTheme === 'dark' ? 'light' : 'dark'} mode`;
        }
    }

    // Smooth scrolling for anchor links
    function initSmoothScrolling() {
        document.querySelectorAll('a[href^="#"]').forEach(anchor => {
            anchor.addEventListener('click', function (e) {
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

    // Copy code blocks
    function initCodeCopyButtons() {
        document.querySelectorAll('pre code').forEach(code => {
            const button = document.createElement('button');
            button.className = 'copy-button';
            button.textContent = 'Copy';
            button.addEventListener('click', () => {
                navigator.clipboard.writeText(code.textContent).then(() => {
                    button.textContent = 'Copied!';
                    setTimeout(() => {
                        button.textContent = 'Copy';
                    }, 2000);
                });
            });
            
            const pre = code.parentElement;
            pre.style.position = 'relative';
            pre.appendChild(button);
        });
    }

    // Initialize everything when DOM is loaded
    document.addEventListener('DOMContentLoaded', function() {
        initThemeToggle();
        initSmoothScrolling();
        initCodeCopyButtons();
        
        // Add some CSS for new elements
        const style = document.createElement('style');
        style.textContent = `
            .theme-toggle {
                background: none;
                border: none;
                font-size: 1.2rem;
                cursor: pointer;
                padding: 0.5rem;
                border-radius: 0.375rem;
                transition: background-color 0.2s;
            }
            
            .theme-toggle:hover {
                background-color: var(--border-color);
            }
            
            .search-result.selected {
                background-color: var(--primary-color);
                color: white;
            }
            
            .search-result.selected .search-result-title,
            .search-result.selected .search-result-description,
            .search-result.selected .search-result-module {
                color: white;
            }
            
            .copy-button {
                position: absolute;
                top: 0.5rem;
                right: 0.5rem;
                background-color: var(--primary-color);
                color: white;
                border: none;
                padding: 0.25rem 0.5rem;
                font-size: 0.75rem;
                border-radius: 0.25rem;
                cursor: pointer;
                opacity: 0;
                transition: opacity 0.2s;
            }
            
            pre:hover .copy-button {
                opacity: 1;
            }
            
            .copy-button:hover {
                background-color: var(--secondary-color);
            }
        `;
        document.head.appendChild(style);
    });

    // Export for testing
    window.CursedDocs = {
        performSearch,
        calculateFuzzyScore,
        highlightMatch
    };
})();
