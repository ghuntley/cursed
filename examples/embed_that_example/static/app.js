// EmbedThat Demo JavaScript
document.addEventListener('DOMContentLoaded', function() {
    console.log('🔥 EmbedThat Demo loaded!');
    
    // Load statistics from the API
    loadStatistics();
    
    // Set up periodic updates
    setInterval(loadStatistics, 30000); // Update every 30 seconds
});

async function loadStatistics() {
    try {
        const response = await fetch('/api/stats');
        const data = await response.json();
        
        updateStatDisplay(data);
        console.log('📊 Statistics updated:', data);
    } catch (error) {
        console.error('Failed to load statistics:', error);
        showErrorState();
    }
}

function updateStatDisplay(data) {
    const fileCountEl = document.getElementById('file-count');
    const memoryUsageEl = document.getElementById('memory-usage');
    const cacheCountEl = document.getElementById('cache-count');
    
    if (fileCountEl) {
        fileCountEl.textContent = data.stats?.total_files || 'N/A';
        fileCountEl.classList.remove('loading');
    }
    
    if (memoryUsageEl) {
        const memUsage = data.memory_usage;
        if (memUsage) {
            memoryUsageEl.textContent = memUsage.total_size_formatted || 'N/A';
        } else {
            memoryUsageEl.textContent = 'N/A';
        }
        memoryUsageEl.classList.remove('loading');
    }
    
    if (cacheCountEl) {
        cacheCountEl.textContent = data.module_info?.cache_entries || 'N/A';
        cacheCountEl.classList.remove('loading');
    }
}

function showErrorState() {
    const elements = ['file-count', 'memory-usage', 'cache-count'];
    elements.forEach(id => {
        const el = document.getElementById(id);
        if (el) {
            el.textContent = 'Error';
            el.classList.remove('loading');
        }
    });
}

// Demo features
function demonstrateFeatures() {
    console.log('🎯 Demonstrating EmbedThat features...');
    
    // Simulate file loading
    console.log('📁 Loading embedded files...');
    setTimeout(() => {
        console.log('✅ Files loaded from embedded storage');
    }, 1000);
    
    // Simulate template rendering
    console.log('🎨 Rendering templates...');
    setTimeout(() => {
        console.log('✅ Templates rendered successfully');
    }, 1500);
    
    // Simulate compression analysis
    console.log('🗜️ Analyzing compression...');
    setTimeout(() => {
        console.log('✅ Compression analysis complete');
    }, 2000);
}

// Call demo features on load
setTimeout(demonstrateFeatures, 2000);

// Add some interactive elements
document.addEventListener('click', function(e) {
    if (e.target.classList.contains('stat-card')) {
        e.target.style.transform = 'scale(1.05)';
        setTimeout(() => {
            e.target.style.transform = '';
        }, 200);
    }
});

// Console art
console.log(`
🔥 CURSED EmbedThat Demo 🔥

   ___________  ___   ____________  
  / ____/ __ \\/ _ | / __/ __/ __ \\ 
 / /   / /_/ / __ |_\\ \\/ _// / / / 
/_/    \\____/_/ |_/___/___/_/ /_/  

Embedding files like a boss! 💪
`);
