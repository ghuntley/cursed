const fs = require('fs');
const path = require('path');

function generateSearchIndex() {
    const documents = [];
    
    // Process markdown files
    function processDirectory(dir, prefix = '') {
        const files = fs.readdirSync(dir);
        
        for (const file of files) {
            const fullPath = path.join(dir, file);
            const stat = fs.statSync(fullPath);
            
            if (stat.isDirectory()) {
                processDirectory(fullPath, prefix + file + '/');
            } else if (file.endsWith('.md')) {
                const content = fs.readFileSync(fullPath, 'utf8');
                const title = extractTitle(content);
                
                documents.push({
                    id: prefix + file,
                    title: title,
                    content: content,
                    url: '/' + prefix + file.replace('.md', ''),
                    type: getDocumentType(prefix)
                });
            }
        }
    }
    
    function extractTitle(content) {
        const match = content.match(/^#\s+(.+)$/m);
        return match ? match[1] : 'Untitled';
    }
    
    function getDocumentType(prefix) {
        if (prefix.includes('tutorials/')) return 'tutorial';
        if (prefix.includes('api-docs/')) return 'api';
        if (prefix.includes('migration/')) return 'migration';
        if (prefix.includes('patterns/')) return 'pattern';
        return 'documentation';
    }
    
    // Process all documentation directories
    const docsDir = path.join(__dirname, '..');
    processDirectory(path.join(docsDir, 'tutorials'), 'tutorials/');
    processDirectory(path.join(docsDir, 'api-docs'), 'api/');
    processDirectory(path.join(docsDir, 'migration'), 'migration/');
    processDirectory(path.join(docsDir, 'patterns'), 'patterns/');
    processDirectory(path.join(docsDir, 'pathways'), 'pathways/');
    
    // Write search index
    const searchIndex = {
        documents: documents,
        version: '1.0.0',
        generated: new Date().toISOString()
    };
    
    fs.writeFileSync(
        path.join(__dirname, '../webapp/public/search-index.json'),
        JSON.stringify(searchIndex, null, 2)
    );
    
    console.log(`Generated search index with ${documents.length} documents`);
}

generateSearchIndex();
