// Interactive Performance Charts for CURSED Build Reports

// Initialize charts when DOM is loaded
document.addEventListener('DOMContentLoaded', function() {
    initializePerformanceCharts();
});

function initializePerformanceCharts() {
    // Performance Overview Chart
    const performanceCtx = document.getElementById('performanceChart');
    if (performanceCtx) {
        new Chart(performanceCtx, {
            type: 'doughnut',
            data: {
                labels: ['Overall Score', 'Compilation Efficiency', 'Resource Efficiency'],
                datasets: [{
                    data: [
                        performanceData.score,
                        performanceData.efficiency,
                        performanceData.resources
                    ],
                    backgroundColor: [
                        '#667eea',
                        '#f093fb',
                        '#f5576c'
                    ],
                    borderWidth: 0
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    title: {
                        display: true,
                        text: 'Performance Overview',
                        font: {
                            size: 18,
                            weight: 'bold'
                        }
                    },
                    legend: {
                        position: 'bottom'
                    }
                }
            }
        });
    }

    // Resource Usage Chart
    const resourceCtx = document.getElementById('resourceChart');
    if (resourceCtx) {
        new Chart(resourceCtx, {
            type: 'line',
            data: {
                labels: generateTimeLabels(),
                datasets: [{
                    label: 'Memory Usage (MB)',
                    data: generateMemoryData(),
                    borderColor: '#667eea',
                    backgroundColor: 'rgba(102, 126, 234, 0.1)',
                    tension: 0.4,
                    yAxisID: 'y'
                }, {
                    label: 'CPU Usage (%)',
                    data: generateCpuData(),
                    borderColor: '#f093fb',
                    backgroundColor: 'rgba(240, 147, 251, 0.1)',
                    tension: 0.4,
                    yAxisID: 'y1'
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                interaction: {
                    mode: 'index',
                    intersect: false,
                },
                plugins: {
                    title: {
                        display: true,
                        text: 'Resource Usage Over Time',
                        font: {
                            size: 18,
                            weight: 'bold'
                        }
                    }
                },
                scales: {
                    x: {
                        display: true,
                        title: {
                            display: true,
                            text: 'Time'
                        }
                    },
                    y: {
                        type: 'linear',
                        display: true,
                        position: 'left',
                        title: {
                            display: true,
                            text: 'Memory (MB)'
                        }
                    },
                    y1: {
                        type: 'linear',
                        display: true,
                        position: 'right',
                        title: {
                            display: true,
                            text: 'CPU (%)'
                        },
                        grid: {
                            drawOnChartArea: false,
                        },
                    }
                }
            }
        });
    }
}

function generateTimeLabels() {
    const labels = [];
    for (let i = 0; i < 20; i++) {
        labels.push(`${i * 5}s`);
    }
    return labels;
}

function generateMemoryData() {
    const data = [];
    let base = 200;
    for (let i = 0; i < 20; i++) {
        base += (Math.random() - 0.5) * 50;
        base = Math.max(100, Math.min(800, base));
        data.push(Math.round(base));
    }
    return data;
}

function generateCpuData() {
    const data = [];
    let base = 40;
    for (let i = 0; i < 20; i++) {
        base += (Math.random() - 0.5) * 30;
        base = Math.max(10, Math.min(95, base));
        data.push(Math.round(base));
    }
    return data;
}

// Add interactivity
function updateCharts() {
    // Function to update charts with new data
    // This would be called when new performance data is available
    console.log('Updating performance charts...');
}

// Export functions for external use
window.performanceCharts = {
    initialize: initializePerformanceCharts,
    update: updateCharts
};
