// CURSED Blog JavaScript - Client-side functionality
// Handles form submissions, AJAX requests, and interactive features

class CursedBlogApp {
    constructor() {
        this.apiBase = '/api';
        this.init();
    }

    init() {
        this.bindEvents();
        this.setupCommentForm();
        this.setupAdminFunctions();
        this.loadPosts();
    }

    bindEvents() {
        // Navigation active states
        this.updateActiveNavigation();
        
        // Smooth scrolling for anchor links
        document.addEventListener('click', (e) => {
            if (e.target.matches('a[href^="#"]')) {
                e.preventDefault();
                const target = document.querySelector(e.target.getAttribute('href'));
                if (target) {
                    target.scrollIntoView({ behavior: 'smooth' });
                }
            }
        });

        // Handle form submissions
        document.addEventListener('submit', (e) => {
            if (e.target.matches('#commentForm')) {
                e.preventDefault();
                this.handleCommentSubmission(e.target);
            }
            
            if (e.target.matches('#loginForm')) {
                e.preventDefault();
                this.handleLogin(e.target);
            }
            
            if (e.target.matches('#postForm')) {
                e.preventDefault();
                this.handlePostSubmission(e.target);
            }
        });
    }

    updateActiveNavigation() {
        const currentPath = window.location.pathname;
        const navLinks = document.querySelectorAll('.nav-links a');
        
        navLinks.forEach(link => {
            link.classList.remove('active');
            if (link.getAttribute('href') === currentPath) {
                link.classList.add('active');
            }
        });
    }

    // Comment functionality
    setupCommentForm() {
        const commentForm = document.getElementById('commentForm');
        if (commentForm) {
            const submitButton = commentForm.querySelector('button[type="submit"]');
            const originalText = submitButton.textContent;

            commentForm.addEventListener('submit', (e) => {
                submitButton.textContent = 'Posting...';
                submitButton.disabled = true;
                
                // Re-enable after processing
                setTimeout(() => {
                    submitButton.textContent = originalText;
                    submitButton.disabled = false;
                }, 2000);
            });
        }
    }

    async handleCommentSubmission(form) {
        const formData = new FormData(form);
        const postId = this.getPostIdFromUrl();
        
        const commentData = {
            author: formData.get('author'),
            content: formData.get('content'),
            post_id: postId
        };

        try {
            const response = await fetch(`${this.apiBase}/posts/${postId}/comments`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(commentData)
            });

            if (response.ok) {
                const result = await response.json();
                this.showMessage('Comment posted successfully!', 'success');
                form.reset();
                this.refreshComments(postId);
            } else {
                this.showMessage('Failed to post comment. Please try again.', 'error');
            }
        } catch (error) {
            this.showMessage('Network error. Please try again.', 'error');
            console.error('Comment submission error:', error);
        }
    }

    async refreshComments(postId) {
        try {
            const response = await fetch(`${this.apiBase}/posts/${postId}/comments`);
            if (response.ok) {
                const comments = await response.json();
                this.renderComments(comments.data);
            }
        } catch (error) {
            console.error('Failed to refresh comments:', error);
        }
    }

    renderComments(comments) {
        const commentsList = document.querySelector('.comments-list');
        if (!commentsList) return;

        commentsList.innerHTML = '';
        
        comments.forEach(comment => {
            const commentElement = document.createElement('div');
            commentElement.className = 'comment';
            commentElement.innerHTML = `
                <div class="comment-header">
                    <span class="comment-author">${this.escapeHtml(comment.author)}</span>
                    <time class="comment-date">${this.formatDate(comment.created_at)}</time>
                </div>
                <div class="comment-content">${this.escapeHtml(comment.content)}</div>
            `;
            commentsList.appendChild(commentElement);
        });
    }

    // Admin functionality
    setupAdminFunctions() {
        // Make admin functions globally available
        window.showNewPostForm = this.showNewPostForm.bind(this);
        window.editPost = this.editPost.bind(this);
        window.deletePost = this.deletePost.bind(this);
        window.refreshPosts = this.refreshPosts.bind(this);
    }

    showNewPostForm() {
        const modal = this.createModal('New Post', this.getPostFormHTML());
        document.body.appendChild(modal);
        this.setupPostForm(modal);
    }

    async editPost(postId) {
        try {
            const response = await fetch(`${this.apiBase}/posts/${postId}`);
            if (response.ok) {
                const result = await response.json();
                const post = result.data;
                
                const modal = this.createModal('Edit Post', this.getPostFormHTML(post));
                document.body.appendChild(modal);
                this.setupPostForm(modal, post);
            }
        } catch (error) {
            this.showMessage('Failed to load post data', 'error');
        }
    }

    async deletePost(postId) {
        if (!confirm('Are you sure you want to delete this post?')) {
            return;
        }

        try {
            const response = await fetch(`${this.apiBase}/posts/${postId}`, {
                method: 'DELETE'
            });

            if (response.ok) {
                this.showMessage('Post deleted successfully!', 'success');
                this.refreshPosts();
            } else {
                this.showMessage('Failed to delete post', 'error');
            }
        } catch (error) {
            this.showMessage('Network error. Please try again.', 'error');
        }
    }

    async refreshPosts() {
        const postsTable = document.querySelector('.posts-table');
        if (postsTable) {
            postsTable.innerHTML = '<div class="loading">Loading posts...</div>';
            // In a real app, this would reload the admin posts data
            setTimeout(() => {
                window.location.reload();
            }, 1000);
        }
    }

    getPostFormHTML(post = null) {
        const isEdit = post !== null;
        return `
            <form id="postForm" class="post-form">
                <input type="hidden" name="id" value="${isEdit ? post.id : ''}">
                
                <div class="form-group">
                    <label for="title">Title:</label>
                    <input type="text" id="title" name="title" required 
                           value="${isEdit ? this.escapeHtml(post.title) : ''}">
                </div>
                
                <div class="form-group">
                    <label for="author">Author:</label>
                    <input type="text" id="author" name="author" required 
                           value="${isEdit ? this.escapeHtml(post.author) : ''}">
                </div>
                
                <div class="form-group">
                    <label for="content">Content:</label>
                    <textarea id="content" name="content" rows="10" required>${isEdit ? this.escapeHtml(post.content) : ''}</textarea>
                </div>
                
                <div class="form-group">
                    <label for="tags">Tags (comma separated):</label>
                    <input type="text" id="tags" name="tags" 
                           value="${isEdit && post.tags ? post.tags.join(', ') : ''}">
                </div>
                
                <div class="form-group">
                    <label>
                        <input type="checkbox" name="published" ${isEdit && post.published ? 'checked' : ''}>
                        Published
                    </label>
                </div>
                
                <div class="form-actions">
                    <button type="submit">${isEdit ? 'Update Post' : 'Create Post'}</button>
                    <button type="button" onclick="this.closest('.modal').remove()">Cancel</button>
                </div>
            </form>
        `;
    }

    setupPostForm(modal, post = null) {
        const form = modal.querySelector('#postForm');
        form.addEventListener('submit', async (e) => {
            e.preventDefault();
            await this.handlePostSubmission(form, post !== null);
            modal.remove();
        });
    }

    async handlePostSubmission(form, isEdit = false) {
        const formData = new FormData(form);
        const tags = formData.get('tags').split(',').map(tag => tag.trim()).filter(tag => tag);
        
        const postData = {
            title: formData.get('title'),
            author: formData.get('author'),
            content: formData.get('content'),
            tags: tags,
            published: formData.get('published') === 'on'
        };

        const url = isEdit ? 
            `${this.apiBase}/posts/${formData.get('id')}` : 
            `${this.apiBase}/posts`;
        
        const method = isEdit ? 'PUT' : 'POST';

        try {
            const response = await fetch(url, {
                method: method,
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(postData)
            });

            if (response.ok) {
                this.showMessage(isEdit ? 'Post updated successfully!' : 'Post created successfully!', 'success');
                this.refreshPosts();
            } else {
                this.showMessage('Failed to save post. Please try again.', 'error');
            }
        } catch (error) {
            this.showMessage('Network error. Please try again.', 'error');
            console.error('Post submission error:', error);
        }
    }

    // Authentication
    async handleLogin(form) {
        const formData = new FormData(form);
        
        const loginData = {
            username: formData.get('username'),
            password: formData.get('password')
        };

        try {
            const response = await fetch(`${this.apiBase}/login`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(loginData)
            });

            if (response.ok) {
                const result = await response.json();
                if (result.success) {
                    this.showMessage('Login successful!', 'success');
                    setTimeout(() => {
                        window.location.href = '/admin';
                    }, 1000);
                } else {
                    this.showMessage('Invalid credentials', 'error');
                }
            } else {
                this.showMessage('Login failed. Please try again.', 'error');
            }
        } catch (error) {
            this.showMessage('Network error. Please try again.', 'error');
            console.error('Login error:', error);
        }
    }

    // Data loading
    async loadPosts() {
        const postsContainer = document.querySelector('.posts-grid, .posts-list');
        if (!postsContainer || postsContainer.children.length > 0) return;

        try {
            const response = await fetch(`${this.apiBase}/posts`);
            if (response.ok) {
                const result = await response.json();
                this.renderPosts(result.data, postsContainer);
            }
        } catch (error) {
            console.error('Failed to load posts:', error);
            postsContainer.innerHTML = '<p>Failed to load posts. Please refresh the page.</p>';
        }
    }

    renderPosts(posts, container) {
        container.innerHTML = '';
        
        posts.forEach(post => {
            const postElement = document.createElement('article');
            postElement.className = 'post-card';
            postElement.innerHTML = `
                <header class="post-header">
                    <h2><a href="/posts/${post.id}">${this.escapeHtml(post.title)}</a></h2>
                    <div class="post-meta">
                        <span class="author">By ${this.escapeHtml(post.author)}</span>
                        <time class="date">${this.formatDate(post.created_at)}</time>
                        <div class="tags">
                            ${post.tags.map(tag => `<span class="tag">#${this.escapeHtml(tag)}</span>`).join('')}
                        </div>
                    </div>
                </header>
                <div class="post-excerpt">
                    ${this.truncateText(post.content, 200)}
                </div>
                <footer class="post-footer">
                    <a href="/posts/${post.id}" class="read-more">Read More</a>
                    <span class="comment-count">0 comments</span>
                </footer>
            `;
            container.appendChild(postElement);
        });
    }

    // UI helpers
    createModal(title, content) {
        const modal = document.createElement('div');
        modal.className = 'modal';
        modal.innerHTML = `
            <div class="modal-backdrop" onclick="this.parentElement.remove()"></div>
            <div class="modal-content">
                <div class="modal-header">
                    <h3>${title}</h3>
                    <button class="modal-close" onclick="this.closest('.modal').remove()">&times;</button>
                </div>
                <div class="modal-body">
                    ${content}
                </div>
            </div>
        `;
        
        // Add modal styles if not already present
        if (!document.querySelector('#modal-styles')) {
            const styles = document.createElement('style');
            styles.id = 'modal-styles';
            styles.textContent = `
                .modal {
                    position: fixed;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    z-index: 1000;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }
                .modal-backdrop {
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background: rgba(0, 0, 0, 0.5);
                }
                .modal-content {
                    background: white;
                    border-radius: 0.75rem;
                    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
                    position: relative;
                    max-width: 90%;
                    max-height: 90%;
                    overflow: auto;
                }
                .modal-header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 1rem 1.5rem;
                    border-bottom: 1px solid #e5e7eb;
                }
                .modal-close {
                    background: none;
                    border: none;
                    font-size: 1.5rem;
                    cursor: pointer;
                    color: #6b7280;
                }
                .modal-body {
                    padding: 1.5rem;
                }
                .form-group {
                    margin-bottom: 1rem;
                }
                .form-group label {
                    display: block;
                    margin-bottom: 0.5rem;
                    font-weight: 600;
                }
                .form-group input,
                .form-group textarea {
                    width: 100%;
                    padding: 0.75rem;
                    border: 1px solid #d1d5db;
                    border-radius: 0.5rem;
                }
                .form-actions {
                    display: flex;
                    gap: 1rem;
                    margin-top: 1.5rem;
                }
                .form-actions button {
                    padding: 0.75rem 1.5rem;
                    border: none;
                    border-radius: 0.5rem;
                    cursor: pointer;
                }
                .form-actions button[type="submit"] {
                    background: #6366f1;
                    color: white;
                }
                .form-actions button[type="button"] {
                    background: #f3f4f6;
                    color: #374151;
                }
            `;
            document.head.appendChild(styles);
        }
        
        return modal;
    }

    showMessage(message, type = 'info') {
        // Remove existing messages
        document.querySelectorAll('.message').forEach(msg => msg.remove());
        
        const messageEl = document.createElement('div');
        messageEl.className = `message message-${type}`;
        messageEl.textContent = message;
        
        // Add message styles if not already present
        if (!document.querySelector('#message-styles')) {
            const styles = document.createElement('style');
            styles.id = 'message-styles';
            styles.textContent = `
                .message {
                    position: fixed;
                    top: 2rem;
                    right: 2rem;
                    padding: 1rem 1.5rem;
                    border-radius: 0.5rem;
                    color: white;
                    font-weight: 500;
                    z-index: 1001;
                    animation: slideIn 0.3s ease;
                }
                .message-success { background: #10b981; }
                .message-error { background: #ef4444; }
                .message-info { background: #6366f1; }
                .message-warning { background: #f59e0b; }
                @keyframes slideIn {
                    from { transform: translateX(100%); opacity: 0; }
                    to { transform: translateX(0); opacity: 1; }
                }
            `;
            document.head.appendChild(styles);
        }
        
        document.body.appendChild(messageEl);
        
        // Auto-remove after 3 seconds
        setTimeout(() => {
            messageEl.style.animation = 'slideIn 0.3s ease reverse';
            setTimeout(() => messageEl.remove(), 300);
        }, 3000);
    }

    // Utility functions
    getPostIdFromUrl() {
        const pathParts = window.location.pathname.split('/');
        return pathParts[pathParts.length - 1];
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    truncateText(text, maxLength) {
        if (text.length <= maxLength) return text;
        return text.substring(0, maxLength) + '...';
    }

    formatDate(timestamp) {
        // Simple date formatting - in real app would use proper date library
        const date = new Date(timestamp * 1000);
        return date.toLocaleDateString('en-US', {
            year: 'numeric',
            month: 'long',
            day: 'numeric'
        });
    }
}

// Initialize the app when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.cursedBlog = new CursedBlogApp();
});

// Service Worker registration for PWA capabilities (optional)
if ('serviceWorker' in navigator) {
    window.addEventListener('load', () => {
        navigator.serviceWorker.register('/sw.js')
            .then((registration) => {
                console.log('SW registered: ', registration);
            })
            .catch((registrationError) => {
                console.log('SW registration failed: ', registrationError);
            });
    });
}
