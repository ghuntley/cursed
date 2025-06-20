// CURSED Brat Website JavaScript

document.addEventListener('DOMContentLoaded', function() {
    // Tab functionality for examples section
    const tabBtns = document.querySelectorAll('.tab-btn');
    const tabContents = document.querySelectorAll('.tab-content');

    tabBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            const targetTab = btn.getAttribute('data-tab');
            
            // Remove active class from all tabs and contents
            tabBtns.forEach(b => b.classList.remove('active'));
            tabContents.forEach(c => c.classList.remove('active'));
            
            // Add active class to clicked tab and corresponding content
            btn.classList.add('active');
            document.getElementById(targetTab).classList.add('active');
        });
    });

    // Smooth scrolling for navigation links
    const navLinks = document.querySelectorAll('.nav-link[href^="#"]');
    navLinks.forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            const targetId = link.getAttribute('href').substring(1);
            const targetElement = document.getElementById(targetId);
            
            if (targetElement) {
                const navHeight = document.querySelector('.brat-nav').offsetHeight;
                const targetPosition = targetElement.offsetTop - navHeight - 20;
                
                window.scrollTo({
                    top: targetPosition,
                    behavior: 'smooth'
                });
            }
        });
    });

    // Active navigation highlighting based on scroll position
    const sections = document.querySelectorAll('section[id]');
    const navLinksAll = document.querySelectorAll('.nav-link');

    function updateActiveNav() {
        const scrollPos = window.scrollY + 100;
        
        sections.forEach(section => {
            const sectionTop = section.offsetTop;
            const sectionHeight = section.offsetHeight;
            const sectionId = section.getAttribute('id');
            
            if (scrollPos >= sectionTop && scrollPos < sectionTop + sectionHeight) {
                navLinksAll.forEach(link => {
                    link.classList.remove('active');
                    if (link.getAttribute('href') === `#${sectionId}`) {
                        link.classList.add('active');
                    }
                });
            }
        });
    }

    window.addEventListener('scroll', updateActiveNav);

    // Add scroll effect to navigation
    const nav = document.querySelector('.brat-nav');
    let lastScrollY = window.scrollY;

    window.addEventListener('scroll', () => {
        const currentScrollY = window.scrollY;
        
        if (currentScrollY > 100) {
            nav.style.background = 'rgba(0, 0, 0, 0.98)';
        } else {
            nav.style.background = 'rgba(0, 0, 0, 0.95)';
        }
        
        lastScrollY = currentScrollY;
    });

    // Add typing animation to hero title
    const heroTitle = document.querySelector('.title-main');
    if (heroTitle) {
        const text = heroTitle.textContent;
        heroTitle.textContent = '';
        
        let i = 0;
        const typeWriter = () => {
            if (i < text.length) {
                heroTitle.textContent += text.charAt(i);
                i++;
                setTimeout(typeWriter, 100);
            }
        };
        
        // Start typing animation after a short delay
        setTimeout(typeWriter, 500);
    }

    // Add floating animation to feature cards
    const featureCards = document.querySelectorAll('.feature-card');
    featureCards.forEach((card, index) => {
        card.style.animationDelay = `${index * 0.1}s`;
        card.addEventListener('mouseenter', () => {
            card.style.transform = 'translateY(-10px) rotate(0.5deg)';
        });
        card.addEventListener('mouseleave', () => {
            card.style.transform = 'translateY(0) rotate(0deg)';
        });
    });

    // Add glitch effect to logo on hover
    const logo = document.querySelector('.logo-text');
    if (logo) {
        const originalText = logo.textContent;
        const glitchChars = ['¢', '∪', '®', '$', '€', 'Ð'];
        
        logo.addEventListener('mouseenter', () => {
            let iterations = 0;
            const glitchInterval = setInterval(() => {
                logo.textContent = originalText
                    .split('')
                    .map((char, index) => {
                        if (index < iterations) {
                            return originalText[index];
                        }
                        return glitchChars[Math.floor(Math.random() * glitchChars.length)];
                    })
                    .join('');

                if (iterations >= originalText.length) {
                    clearInterval(glitchInterval);
                    logo.textContent = originalText;
                }

                iterations += 1 / 3;
            }, 30);
        });
    }

    // Add rainbow effect to certain elements on scroll
    const rainbowElements = document.querySelectorAll('.section-title');
    
    window.addEventListener('scroll', () => {
        const scrollPercent = window.scrollY / (document.documentElement.scrollHeight - window.innerHeight);
        const hue = scrollPercent * 360;
        
        rainbowElements.forEach(element => {
            if (isElementInViewport(element)) {
                element.style.filter = `hue-rotate(${hue}deg)`;
            }
        });
    });

    // Utility function to check if element is in viewport
    function isElementInViewport(el) {
        const rect = el.getBoundingClientRect();
        return (
            rect.top >= 0 &&
            rect.left >= 0 &&
            rect.bottom <= (window.innerHeight || document.documentElement.clientHeight) &&
            rect.right <= (window.innerWidth || document.documentElement.clientWidth)
        );
    }

    // Add particle effect to hero background
    createParticleEffect();
    
    // Konami code easter egg (up, up, down, down, left, right, left, right, b, a)
    let konamiSequence = [38, 38, 40, 40, 37, 39, 37, 39, 66, 65];
    let konamiIndex = 0;
    
    document.addEventListener('keydown', (e) => {
        if (e.keyCode === konamiSequence[konamiIndex]) {
            konamiIndex++;
            if (konamiIndex === konamiSequence.length) {
                activateBratMode();
                konamiIndex = 0;
            }
        } else {
            konamiIndex = 0;
        }
    });
});

// Copy to clipboard functionality
function copyToClipboard(button) {
    const codeText = button.parentElement.querySelector('code').textContent;
    
    navigator.clipboard.writeText(codeText).then(() => {
        const originalText = button.textContent;
        button.textContent = 'copied!';
        button.style.background = 'var(--brat-green-light)';
        
        setTimeout(() => {
            button.textContent = originalText;
            button.style.background = 'var(--brat-green)';
        }, 2000);
    }).catch(err => {
        console.error('Failed to copy text: ', err);
        button.textContent = 'failed';
        setTimeout(() => {
            button.textContent = 'copy';
        }, 2000);
    });
}

// Particle effect for hero background
function createParticleEffect() {
    const hero = document.querySelector('.hero');
    if (!hero) return;

    const canvas = document.createElement('canvas');
    const ctx = canvas.getContext('2d');
    canvas.style.position = 'absolute';
    canvas.style.top = '0';
    canvas.style.left = '0';
    canvas.style.width = '100%';
    canvas.style.height = '100%';
    canvas.style.pointerEvents = 'none';
    canvas.style.zIndex = '-1';
    hero.appendChild(canvas);

    function resizeCanvas() {
        canvas.width = hero.offsetWidth;
        canvas.height = hero.offsetHeight;
    }

    resizeCanvas();
    window.addEventListener('resize', resizeCanvas);

    const particles = [];
    const particleCount = 50;

    class Particle {
        constructor() {
            this.x = Math.random() * canvas.width;
            this.y = Math.random() * canvas.height;
            this.vx = (Math.random() - 0.5) * 0.5;
            this.vy = (Math.random() - 0.5) * 0.5;
            this.size = Math.random() * 2 + 1;
            this.opacity = Math.random() * 0.3 + 0.1;
        }

        update() {
            this.x += this.vx;
            this.y += this.vy;

            if (this.x < 0 || this.x > canvas.width) this.vx *= -1;
            if (this.y < 0 || this.y > canvas.height) this.vy *= -1;
        }

        draw() {
            ctx.save();
            ctx.globalAlpha = this.opacity;
            ctx.fillStyle = '#8ACE00';
            ctx.beginPath();
            ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
            ctx.fill();
            ctx.restore();
        }
    }

    for (let i = 0; i < particleCount; i++) {
        particles.push(new Particle());
    }

    function animate() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        particles.forEach(particle => {
            particle.update();
            particle.draw();
        });

        // Draw connections between nearby particles
        particles.forEach((particle, i) => {
            particles.slice(i + 1).forEach(otherParticle => {
                const dx = particle.x - otherParticle.x;
                const dy = particle.y - otherParticle.y;
                const distance = Math.sqrt(dx * dx + dy * dy);

                if (distance < 100) {
                    ctx.save();
                    ctx.globalAlpha = 0.1 * (1 - distance / 100);
                    ctx.strokeStyle = '#8ACE00';
                    ctx.lineWidth = 1;
                    ctx.beginPath();
                    ctx.moveTo(particle.x, particle.y);
                    ctx.lineTo(otherParticle.x, otherParticle.y);
                    ctx.stroke();
                    ctx.restore();
                }
            });
        });

        requestAnimationFrame(animate);
    }

    animate();
}

// Easter egg: Brat mode activation
function activateBratMode() {
    document.body.style.animation = 'rainbow-bg 2s ease-in-out infinite';
    
    // Add CSS for rainbow background animation
    const style = document.createElement('style');
    style.textContent = `
        @keyframes rainbow-bg {
            0% { filter: hue-rotate(0deg); }
            25% { filter: hue-rotate(90deg); }
            50% { filter: hue-rotate(180deg); }
            75% { filter: hue-rotate(270deg); }
            100% { filter: hue-rotate(360deg); }
        }
    `;
    document.head.appendChild(style);
    
    // Show brat mode message
    const bratMessage = document.createElement('div');
    bratMessage.innerHTML = '✨ BRAT MODE ACTIVATED ✨<br>you found the secret! absolute icon behavior 💅';
    bratMessage.style.cssText = `
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background: rgba(139, 206, 0, 0.95);
        color: black;
        padding: 2rem;
        border-radius: 1rem;
        font-weight: bold;
        text-align: center;
        z-index: 10000;
        animation: pulse 1s ease-in-out infinite;
        font-size: 1.2rem;
        backdrop-filter: blur(10px);
        border: 2px solid #9BFF00;
        box-shadow: 0 0 30px rgba(155, 255, 0, 0.5);
    `;
    
    document.body.appendChild(bratMessage);
    
    setTimeout(() => {
        document.body.style.animation = '';
        bratMessage.remove();
        style.remove();
    }, 10000);
}

// Performance optimization: Debounced scroll handler
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

// Apply debouncing to scroll handlers
const debouncedScrollHandler = debounce(() => {
    // Any expensive scroll operations go here
}, 16); // ~60fps

window.addEventListener('scroll', debouncedScrollHandler);
