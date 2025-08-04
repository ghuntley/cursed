// CURSED Documentation JavaScript
document.addEventListener('DOMContentLoaded', function() {
    console.log('CURSED Documentation loaded');
    
    // Add smooth scrolling to anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth'
                });
            }
        });
    });
    
    // Add copy button to code blocks
    document.querySelectorAll('pre code').forEach(block => {
        const button = document.createElement('button');
        button.textContent = 'Copy';
        button.className = 'copy-btn';
        button.onclick = () => {
            navigator.clipboard.writeText(block.textContent);
            button.textContent = 'Copied!';
            setTimeout(() => button.textContent = 'Copy', 2000);
        };
        block.parentNode.insertBefore(button, block);
    });
});
