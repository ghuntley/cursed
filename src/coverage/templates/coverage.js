// CURSED Coverage Report JavaScript

document.addEventListener('DOMContentLoaded', function() {
    // Add interactive features to coverage reports
    initializeCoverageReport();
});

function initializeCoverageReport() {
    // Initialize sorting for file table
    initializeTableSorting();
    
    // Initialize filtering
    initializeFiltering();
    
    // Initialize tooltips
    initializeTooltips();
    
    // Initialize line highlighting
    initializeLineHighlighting();
    
    // Initialize search functionality
    initializeSearch();
}

function initializeTableSorting() {
    const table = document.querySelector('.file-list table');
    if (!table) return;
    
    const headers = table.querySelectorAll('th');
    headers.forEach((header, index) => {
        if (index === 0) return; // Skip file name column
        
        header.style.cursor = 'pointer';
        header.addEventListener('click', () => sortTable(table, index));
    });
}

function sortTable(table, columnIndex) {
    const tbody = table.querySelector('tbody');
    const rows = Array.from(tbody.querySelectorAll('tr'));
    
    // Determine sort direction
    const isAscending = table.dataset.sortDirection !== 'asc';
    table.dataset.sortDirection = isAscending ? 'asc' : 'desc';
    
    // Sort rows
    rows.sort((a, b) => {
        const aValue = parseFloat(a.cells[columnIndex].textContent.replace('%', ''));
        const bValue = parseFloat(b.cells[columnIndex].textContent.replace('%', ''));
        
        if (isAscending) {
            return aValue - bValue;
        } else {
            return bValue - aValue;
        }
    });
    
    // Reorder rows in DOM
    rows.forEach(row => tbody.appendChild(row));
    
    // Update header indicators
    const headers = table.querySelectorAll('th');
    headers.forEach((header, index) => {
        const indicator = header.querySelector('.sort-indicator');
        if (indicator) indicator.remove();
        
        if (index === columnIndex) {
            const arrow = document.createElement('span');
            arrow.className = 'sort-indicator';
            arrow.textContent = isAscending ? ' ↑' : ' ↓';
            header.appendChild(arrow);
        }
    });
}

function initializeFiltering() {
    // Add filter controls if on index page
    const fileList = document.querySelector('.file-list');
    if (!fileList) return;
    
    const filterContainer = document.createElement('div');
    filterContainer.className = 'filter-container';
    filterContainer.innerHTML = `
        <div class="filter-controls">
            <label>
                <input type="checkbox" id="show-high-coverage" checked>
                Show High Coverage (≥90%)
            </label>
            <label>
                <input type="checkbox" id="show-medium-coverage" checked>
                Show Medium Coverage (70-90%)
            </label>
            <label>
                <input type="checkbox" id="show-low-coverage" checked>
                Show Low Coverage (<70%)
            </label>
        </div>
    `;
    
    fileList.insertBefore(filterContainer, fileList.querySelector('table'));
    
    // Add filter event listeners
    document.getElementById('show-high-coverage').addEventListener('change', filterTable);
    document.getElementById('show-medium-coverage').addEventListener('change', filterTable);
    document.getElementById('show-low-coverage').addEventListener('change', filterTable);
}

function filterTable() {
    const showHigh = document.getElementById('show-high-coverage').checked;
    const showMedium = document.getElementById('show-medium-coverage').checked;
    const showLow = document.getElementById('show-low-coverage').checked;
    
    const rows = document.querySelectorAll('.file-list tbody tr');
    rows.forEach(row => {
        const coverageCell = row.cells[2]; // Line coverage column
        const coverage = parseFloat(coverageCell.textContent.replace('%', ''));
        
        let shouldShow = false;
        if (coverage >= 90 && showHigh) shouldShow = true;
        if (coverage >= 70 && coverage < 90 && showMedium) shouldShow = true;
        if (coverage < 70 && showLow) shouldShow = true;
        
        row.style.display = shouldShow ? '' : 'none';
    });
}

function initializeTooltips() {
    // Add tooltips to coverage metrics
    const metrics = document.querySelectorAll('.metric');
    metrics.forEach(metric => {
        const value = metric.querySelector('.metric-value');
        const label = metric.querySelector('.metric-label');
        
        if (value && label) {
            const tooltip = document.createElement('div');
            tooltip.className = 'tooltip';
            tooltip.textContent = getTooltipText(label.textContent);
            metric.appendChild(tooltip);
            
            metric.addEventListener('mouseenter', () => {
                tooltip.style.visibility = 'visible';
                tooltip.style.opacity = '1';
            });
            
            metric.addEventListener('mouseleave', () => {
                tooltip.style.visibility = 'hidden';
                tooltip.style.opacity = '0';
            });
        }
    });
}

function getTooltipText(metricType) {
    switch (metricType) {
        case 'Line Coverage':
            return 'Percentage of executable lines that were executed during testing';
        case 'Function Coverage':
            return 'Percentage of functions that were called during testing';
        case 'Branch Coverage':
            return 'Percentage of decision branches that were taken during testing';
        default:
            return '';
    }
}

function initializeLineHighlighting() {
    // Add line highlighting for source code view
    const sourceLines = document.querySelectorAll('.source-table tbody tr');
    sourceLines.forEach(line => {
        line.addEventListener('click', () => {
            // Remove previous highlights
            sourceLines.forEach(l => l.classList.remove('highlighted'));
            
            // Add highlight to clicked line
            line.classList.add('highlighted');
            
            // Update URL hash
            const lineNumber = line.querySelector('.line-number').textContent;
            window.location.hash = `L${lineNumber}`;
        });
    });
    
    // Highlight line from URL hash on page load
    if (window.location.hash.startsWith('#L')) {
        const lineNumber = window.location.hash.substring(2);
        const targetLine = document.querySelector(`.line-number[data-line="${lineNumber}"]`);
        if (targetLine) {
            targetLine.closest('tr').classList.add('highlighted');
            targetLine.scrollIntoView({ behavior: 'smooth', block: 'center' });
        }
    }
}

function initializeSearch() {
    // Add search functionality
    const fileList = document.querySelector('.file-list');
    if (!fileList) return;
    
    const searchContainer = document.createElement('div');
    searchContainer.className = 'search-container';
    searchContainer.innerHTML = `
        <input type="text" id="file-search" placeholder="Search files...">
        <button id="clear-search">Clear</button>
    `;
    
    fileList.insertBefore(searchContainer, fileList.querySelector('table'));
    
    const searchInput = document.getElementById('file-search');
    const clearButton = document.getElementById('clear-search');
    
    searchInput.addEventListener('input', handleSearch);
    clearButton.addEventListener('click', () => {
        searchInput.value = '';
        handleSearch();
    });
}

function handleSearch() {
    const searchTerm = document.getElementById('file-search').value.toLowerCase();
    const rows = document.querySelectorAll('.file-list tbody tr');
    
    rows.forEach(row => {
        const fileName = row.cells[0].textContent.toLowerCase();
        const matches = fileName.includes(searchTerm);
        row.style.display = matches ? '' : 'none';
    });
}

// Add CSS for interactive elements
const style = document.createElement('style');
style.textContent = `
    .filter-container, .search-container {
        margin-bottom: 1rem;
        padding: 1rem;
        background: #21262d;
        border-radius: 6px;
        border: 1px solid #30363d;
    }
    
    .filter-controls {
        display: flex;
        gap: 2rem;
        flex-wrap: wrap;
    }
    
    .filter-controls label {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: #c9d1d9;
        cursor: pointer;
    }
    
    .search-container {
        display: flex;
        gap: 1rem;
        align-items: center;
    }
    
    #file-search {
        flex: 1;
        padding: 0.5rem;
        background: #0d1117;
        border: 1px solid #30363d;
        border-radius: 4px;
        color: #c9d1d9;
    }
    
    #clear-search {
        padding: 0.5rem 1rem;
        background: #21262d;
        border: 1px solid #30363d;
        border-radius: 4px;
        color: #c9d1d9;
        cursor: pointer;
    }
    
    #clear-search:hover {
        background: #30363d;
    }
    
    .tooltip {
        position: absolute;
        background: #1c2128;
        color: #c9d1d9;
        padding: 0.5rem;
        border-radius: 4px;
        font-size: 0.8rem;
        border: 1px solid #30363d;
        z-index: 1000;
        max-width: 200px;
        visibility: hidden;
        opacity: 0;
        transition: opacity 0.3s;
        bottom: 100%;
        left: 50%;
        transform: translateX(-50%);
    }
    
    .highlighted {
        background: rgba(88, 166, 255, 0.1) !important;
        border-left: 3px solid #58a6ff !important;
    }
    
    .sort-indicator {
        color: #58a6ff;
        font-weight: bold;
    }
    
    th:hover {
        background: #30363d;
    }
`;

document.head.appendChild(style);
