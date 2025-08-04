// Search functionality for CURSED documentation
class DocumentationSearch {
    constructor() {
        this.searchIndex = [];
        this.loadSearchIndex();
    }
    
    async loadSearchIndex() {
        try {
            const response = await fetch('/json/search-index.json');
            this.searchIndex = await response.json();
        } catch (error) {
            console.warn('Search index not available:', error);
        }
    }
    
    search(query) {
        if (!query || query.length < 2) return [];
        
        const terms = query.toLowerCase().split(' ');
        return this.searchIndex.filter(doc => {
            const content = (doc.title + ' ' + doc.content).toLowerCase();
            return terms.every(term => content.includes(term));
        }).slice(0, 10);
    }
}

// Initialize search when DOM is loaded
document.addEventListener('DOMContentLoaded', function() {
    const search = new DocumentationSearch();
    
    const searchInput = document.getElementById('search');
    const searchResults = document.getElementById('search-results');
    
    if (searchInput && searchResults) {
        searchInput.addEventListener('input', function(e) {
            const query = e.target.value;
            const results = search.search(query);
            
            if (results.length > 0) {
                searchResults.innerHTML = results.map(result => 
                    `<div class="search-result">
                        <a href="${result.url}">${result.title}</a>
                        <p>${result.content.substring(0, 100)}...</p>
                    </div>`
                ).join('');
                searchResults.style.display = 'block';
            } else {
                searchResults.style.display = 'none';
            }
        });
    }
});
