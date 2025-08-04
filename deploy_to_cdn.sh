#!/bin/bash
# Deploy documentation to CDN

echo "Deploying to CDN..."

# Example deployment commands (adjust for your CDN)
# aws s3 sync docs_production/html/ s3://cursed-docs-production/ --delete
# aws cloudfront create-invalidation --distribution-id YOUR_DISTRIBUTION_ID --paths "/*"

echo "CDN deployment completed"
