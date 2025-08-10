# CURSED Interactive Documentation Deployment

This directory contains deployment configurations and scripts for the CURSED interactive documentation platform.

## 🚀 Deployment Targets

### Production (docs.cursedlang.org)
- **Platform**: Vercel/Netlify with CDN
- **Build**: Optimized production builds
- **Features**: Full search indexing, analytics, performance monitoring
- **SSL**: Automatic HTTPS with custom domain

### Staging (staging-docs.cursedlang.org)
- **Platform**: Preview deployments
- **Build**: Production builds with staging flags
- **Features**: Testing new features before production
- **Access**: Development team only

### Development
- **Platform**: Local development servers
- **Build**: Hot reload, source maps, debug mode
- **Features**: Real-time development and testing

## 📦 Build System

### Frontend (React/TypeScript)
```bash
# Install dependencies
npm install

# Development build
npm run dev

# Production build
npm run build

# Preview production build
npm run preview

# Run tests
npm test
npm run test:e2e

# Lint and format
npm run lint
npm run format
```

### Content Pipeline
```bash
# Generate API documentation
./scripts/generate-api-docs.sh

# Build tutorial content
./scripts/build-tutorials.sh

# Process video content
./scripts/process-videos.sh

# Generate search index
./scripts/build-search-index.sh
```

## 🏗️ Infrastructure

### Static Site Generation
```yaml
# vercel.json
{
  "buildCommand": "npm run build",
  "outputDirectory": "dist",
  "functions": {
    "api/search.ts": {
      "runtime": "nodejs18.x"
    }
  },
  "routes": [
    {
      "src": "/api/(.*)",
      "dest": "/api/$1"
    },
    {
      "src": "/(.*)",
      "dest": "/index.html"
    }
  ]
}
```

### CDN Configuration
- **Static Assets**: Images, videos, fonts via CDN
- **Code Splitting**: Automatic chunk splitting for optimal loading
- **Caching**: Aggressive caching with cache invalidation
- **Compression**: Gzip/Brotli compression enabled

### Search Infrastructure
- **Index Generation**: Build-time search index creation
- **Search API**: Serverless search endpoint
- **Fuzzy Search**: Fuse.js for client-side search
- **Analytics**: Search query analytics and optimization

## 🔧 Environment Configuration

### Environment Variables
```bash
# .env.production
VITE_APP_TITLE="CURSED Interactive Documentation"
VITE_API_BASE_URL="https://api.cursedlang.org"
VITE_ANALYTICS_ID="GA_MEASUREMENT_ID"
VITE_SENTRY_DSN="SENTRY_DSN"
VITE_FEATURE_PLAYGROUND="true"
VITE_FEATURE_VIDEO_STREAMING="true"
VITE_SEARCH_API_URL="https://search.cursedlang.org"
```

### Build Configurations
```javascript
// Production
{
  "mode": "production",
  "optimization": {
    "minimize": true,
    "splitChunks": true
  },
  "analytics": true,
  "sourceMap": false
}

// Staging
{
  "mode": "production", 
  "optimization": {
    "minimize": false,
    "splitChunks": true
  },
  "analytics": false,
  "sourceMap": true
}

// Development
{
  "mode": "development",
  "optimization": {
    "minimize": false,
    "splitChunks": false
  },
  "analytics": false,
  "sourceMap": true,
  "hotReload": true
}
```

## 🚀 Deployment Scripts

### Automated Deployment
```bash
#!/bin/bash
# deploy.sh

set -e

echo "🚀 Starting deployment..."

# Build documentation content
echo "📚 Building content..."
./scripts/build-content.sh

# Build React application  
echo "⚛️  Building React app..."
npm run build

# Optimize assets
echo "🎯 Optimizing assets..."
./scripts/optimize-assets.sh

# Deploy to hosting platform
echo "🌐 Deploying to hosting..."
npm run deploy:production

# Update search index
echo "🔍 Updating search index..."
./scripts/update-search-index.sh

# Verify deployment
echo "✅ Verifying deployment..."
./scripts/verify-deployment.sh

echo "🎉 Deployment complete!"
```

### Content Update Pipeline
```bash
#!/bin/bash
# update-content.sh

# Pull latest content from repository
git pull origin main

# Regenerate API documentation
./scripts/generate-api-docs.sh

# Process new tutorial content
./scripts/process-tutorials.sh

# Update video metadata
./scripts/update-video-metadata.sh

# Rebuild search index
./scripts/rebuild-search-index.sh

# Deploy content updates
npm run deploy:content-only
```

## 📊 Monitoring and Analytics

### Performance Monitoring
```javascript
// Performance tracking
import { initPerformanceMonitoring } from './monitoring';

initPerformanceMonitoring({
  apiEndpoint: process.env.VITE_MONITORING_API,
  sampleRate: 0.1, // 10% sampling in production
  metrics: [
    'page-load-time',
    'interactive-editor-startup',
    'search-response-time',
    'video-loading-time'
  ]
});
```

### User Analytics
```javascript
// Google Analytics 4 integration
import { gtag } from './analytics';

// Track tutorial completion
gtag('event', 'tutorial_complete', {
  tutorial_name: 'hello-world',
  completion_time: 300, // seconds
  user_level: 'beginner'
});

// Track code playground usage
gtag('event', 'playground_run', {
  code_type: 'cursed',
  execution_time: 1200, // milliseconds
  lines_of_code: 15
});
```

### Error Tracking
```javascript
// Sentry integration
import * as Sentry from '@sentry/react';

Sentry.init({
  dsn: process.env.VITE_SENTRY_DSN,
  environment: process.env.NODE_ENV,
  release: process.env.VITE_APP_VERSION,
  beforeSend(event) {
    // Filter out non-critical errors
    if (event.exception) {
      const error = event.exception.values[0];
      if (error.type === 'ChunkLoadError') {
        return null; // Ignore chunk loading errors
      }
    }
    return event;
  }
});
```

## 🔄 CI/CD Pipeline

### GitHub Actions
```yaml
# .github/workflows/deploy.yml
name: Deploy Documentation

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
          cache: 'npm'
      
      - run: npm ci
      - run: npm run test
      - run: npm run test:e2e
      - run: npm run lint

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
          cache: 'npm'
      
      - run: npm ci
      - run: npm run build
      
      - uses: actions/upload-artifact@v3
        with:
          name: build-files
          path: dist/

  deploy-staging:
    if: github.event_name == 'pull_request'
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v3
        with:
          name: build-files
      
      - name: Deploy to staging
        run: |
          # Deploy to staging environment
          echo "Deploying to staging..."

  deploy-production:
    if: github.ref == 'refs/heads/main'
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v3
        with:
          name: build-files
      
      - name: Deploy to production
        run: |
          # Deploy to production environment
          echo "Deploying to production..."
          
      - name: Update search index
        run: |
          # Trigger search index update
          curl -X POST "${{ secrets.SEARCH_WEBHOOK_URL }}"
```

### Deployment Verification
```bash
#!/bin/bash
# verify-deployment.sh

BASE_URL="https://docs.cursedlang.org"

echo "🔍 Verifying deployment..."

# Check main page loads
if curl -f -s "$BASE_URL" > /dev/null; then
    echo "✅ Main page accessible"
else
    echo "❌ Main page failed"
    exit 1
fi

# Check playground functionality
if curl -f -s "$BASE_URL/playground" > /dev/null; then
    echo "✅ Playground accessible"
else
    echo "❌ Playground failed"
    exit 1
fi

# Check API endpoints
if curl -f -s "$BASE_URL/api/search?q=test" > /dev/null; then
    echo "✅ Search API working"
else
    echo "❌ Search API failed"
    exit 1
fi

# Check performance
LOAD_TIME=$(curl -o /dev/null -s -w '%{time_total}' "$BASE_URL")
if (( $(echo "$LOAD_TIME < 2.0" | bc -l) )); then
    echo "✅ Performance acceptable ($LOAD_TIME seconds)"
else
    echo "⚠️  Performance slow ($LOAD_TIME seconds)"
fi

echo "🎉 Verification complete!"
```

## 🛠️ Development Setup

### Local Development
```bash
# Clone repository
git clone https://github.com/ghuntley/cursed.git
cd cursed/docs/interactive

# Install dependencies
npm install

# Start development server
npm run dev

# In another terminal, start content server
npm run dev:content

# Open http://localhost:5173
```

### Content Development
```bash
# Watch for content changes
npm run dev:watch-content

# Preview content changes
npm run preview:content

# Validate content
npm run validate:content
```

## 📱 Mobile Optimization

### Progressive Web App
```javascript
// PWA configuration
const pwaConfig = {
  registerType: 'autoUpdate',
  workbox: {
    globPatterns: ['**/*.{js,css,html,ico,png,svg,woff2}'],
    runtimeCaching: [
      {
        urlPattern: /^https:\/\/api\.cursedlang\.org\//,
        handler: 'NetworkFirst',
        options: {
          cacheName: 'api-cache',
          expiration: {
            maxEntries: 100,
            maxAgeSeconds: 60 * 60 * 24 // 24 hours
          }
        }
      }
    ]
  }
};
```

### Responsive Design
- **Mobile-first**: Optimized for mobile devices
- **Touch-friendly**: Large touch targets, swipe gestures
- **Offline Support**: PWA capabilities for offline reading
- **Performance**: Lazy loading, image optimization

## 🔐 Security

### Content Security Policy
```nginx
# CSP headers
add_header Content-Security-Policy "
    default-src 'self';
    script-src 'self' 'unsafe-inline' https://www.googletagmanager.com;
    style-src 'self' 'unsafe-inline' https://fonts.googleapis.com;
    font-src 'self' https://fonts.gstatic.com;
    img-src 'self' data: https:;
    connect-src 'self' https://api.cursedlang.org;
    media-src 'self' https://videos.cursedlang.org;
" always;
```

### Input Validation
- **Code Execution**: Sandboxed environment for code execution
- **XSS Prevention**: Sanitized user input and content
- **CSRF Protection**: CSRF tokens for API endpoints
- **Rate Limiting**: API rate limiting to prevent abuse

## 📈 Performance Optimization

### Bundle Optimization
```javascript
// Webpack bundle analysis
const BundleAnalyzerPlugin = require('webpack-bundle-analyzer').BundleAnalyzerPlugin;

module.exports = {
  plugins: [
    new BundleAnalyzerPlugin({
      analyzerMode: 'static',
      openAnalyzer: false
    })
  ],
  optimization: {
    splitChunks: {
      chunks: 'all',
      cacheGroups: {
        vendor: {
          test: /[\\/]node_modules[\\/]/,
          name: 'vendors',
          chunks: 'all'
        }
      }
    }
  }
};
```

### Image Optimization
```bash
# Optimize images during build
npm run optimize:images

# Generate WebP versions
npm run generate:webp

# Compress videos
npm run compress:videos
```

## 📞 Support and Troubleshooting

### Common Issues
1. **Build Failures**: Check Node.js version and dependencies
2. **Slow Loading**: Review bundle size and optimization
3. **Search Issues**: Verify search index generation
4. **Mobile Issues**: Test responsive design and PWA features

### Debugging Tools
- **Source Maps**: Available in staging/development
- **Performance Profiler**: Built-in React profiling
- **Network Monitor**: Service worker debugging
- **Console Logging**: Conditional logging based on environment

### Contact
- **Technical Issues**: [GitHub Issues](https://github.com/ghuntley/cursed/issues)
- **Deployment Problems**: [DevOps Team](mailto:devops@cursedlang.org)
- **Content Issues**: [Documentation Team](mailto:docs@cursedlang.org)

---

**Deployment Status**: ✅ Production Ready  
**Last Updated**: January 2025  
**Version**: 1.0.0
