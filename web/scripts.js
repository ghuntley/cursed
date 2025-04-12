// CURSED Programming Language Website Scripts

document.addEventListener('DOMContentLoaded', () => {
  // Smooth scrolling for anchor links
  document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function(e) {
      e.preventDefault();
      const targetId = this.getAttribute('href');
      const targetElement = document.querySelector(targetId);
      
      if (targetElement) {
        window.scrollTo({
          top: targetElement.offsetTop - 80, // Account for fixed header
          behavior: 'smooth'
        });
      }
    });
  });

  // Dark mode toggle
  const themeToggle = document.getElementById('theme-toggle');
  const moonIcon = '🌙';
  const sunIcon = '☀️';
  
  // Check for saved theme preference or use system preference
  const savedTheme = localStorage.getItem('theme');
  const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  
  if (savedTheme === 'dark' || (!savedTheme && systemPrefersDark)) {
    document.body.classList.add('dark-mode');
    themeToggle.textContent = sunIcon;
  } else {
    themeToggle.textContent = moonIcon;
  }
  
  // Theme toggle click handler
  themeToggle.addEventListener('click', () => {
    document.body.classList.toggle('dark-mode');
    
    if (document.body.classList.contains('dark-mode')) {
      localStorage.setItem('theme', 'dark');
      themeToggle.textContent = sunIcon;
    } else {
      localStorage.setItem('theme', 'light');
      themeToggle.textContent = moonIcon;
    }
  });
  
  // Add animation classes as elements come into view
  const animateElements = document.querySelectorAll('.animate-on-scroll');
  
  const animateOnScroll = () => {
    animateElements.forEach(element => {
      const elementTop = element.getBoundingClientRect().top;
      const elementVisible = 150;
      
      if (elementTop < window.innerHeight - elementVisible) {
        element.classList.add('fade-in');
      }
    });
  };
  
  // Initial check for elements in view
  animateOnScroll();
  
  // Check for elements on scroll
  window.addEventListener('scroll', animateOnScroll);
  
  // Code copy functionality
  document.querySelectorAll('.code-block').forEach(block => {
    const codeElement = block.querySelector('code');
    const copyButton = document.createElement('button');
    copyButton.classList.add('copy-button');
    copyButton.textContent = 'Copy';
    
    block.appendChild(copyButton);
    
    copyButton.addEventListener('click', () => {
      navigator.clipboard.writeText(codeElement.textContent);
      copyButton.textContent = 'Copied!';
      
      setTimeout(() => {
        copyButton.textContent = 'Copy';
      }, 2000);
    });
  });
  
  // Mobile menu toggle
  const menuToggle = document.getElementById('menu-toggle');
  const navMenu = document.querySelector('nav ul');
  
  if (menuToggle && navMenu) {
    menuToggle.addEventListener('click', () => {
      navMenu.classList.toggle('show');
      menuToggle.classList.toggle('active');
    });
  }
});