const Parser = require('tree-sitter');
const path = require('path');
const fs = require('fs');

// This test assumes you've already built the grammar using `tree-sitter generate`
// You would run this test with `node test/parser_test.js`

describe('CURSED grammar', () => {
  let parser;

  beforeAll(() => {
    // Setup the parser
    parser = new Parser();
    try {
      const CursedLanguage = require('../');
      parser.setLanguage(CursedLanguage);
    } catch (e) {
      console.error('Failed to load CURSED grammar:', e);
      throw e;
    }
  });

  test('Can parse a simple program', () => {
    const source = fs.readFileSync(path.join(__dirname, '../examples/hello_world.csd'), 'utf8');
    const tree = parser.parse(source);
    
    // Just checking that it parses without errors
    expect(tree).toBeDefined();
    expect(tree.rootNode.type).toBe('source_file');
    
    // Verify the structure
    const packageClause = tree.rootNode.child(0);
    expect(packageClause.type).toBe('package_clause');
    
    const importDecl = tree.rootNode.child(1);
    expect(importDecl.type).toBe('import_declaration');
    
    const funcDecl = tree.rootNode.child(2);
    expect(funcDecl.type).toBe('function_declaration');
  });

  test('Can parse different statements', () => {
    const source = `
    vibe test
    
    slay main() {
      sus x normie = 10
      lowkey x > 5 {
        yolo "Greater"
      }
      bestie i := 0; i < 10; i++ {
        print(i)
      }
    }
    `;
    
    const tree = parser.parse(source);
    expect(tree).toBeDefined();
    expect(tree.rootNode.type).toBe('source_file');
  });
});