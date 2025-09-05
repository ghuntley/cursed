import React, { useState, useRef, useEffect } from 'react';
import dynamic from 'next/dynamic';
import { Play, Download, Share, BookOpen, Package, Users, Star } from 'lucide-react';
import { toast } from 'react-hot-toast';
import Link from 'next/link';

// Dynamically import Monaco Editor to avoid SSR issues
const MonacoEditor = dynamic(() => import('@monaco-editor/react'), { ssr: false });

interface PlaygroundState {
  code: string;
  output: string;
  isRunning: boolean;
  shareUrl?: string;
}

const defaultCode = `// Welcome to the CURSED Language Playground!
// Try writing some CURSED code below and click Run to see it in action

yeet "vibez"

slay greet(name tea) tea {
    damn "Hello, " + name + "! Welcome to CURSED!"
}

slay main() drip {
    vibez.spill(greet("World"))
    
    // Try some basic math
    sus result drip = 42 + 8
    vibez.spill("The answer is:", result)
    
    // Array operations
    sus numbers []drip = [1, 2, 3, 4, 5]
    vibez.spill("Array length:", len(numbers))
    
    damn 0
}
`;

const examples = [
  {
    name: "Hello World",
    code: `yeet "vibez"

slay main() drip {
    vibez.spill("Hello, CURSED World!")
    damn 0
}`
  },
  {
    name: "Functions & Variables",
    code: `yeet "vibez"

slay add(a drip, b drip) drip {
    damn a + b
}

slay main() drip {
    sus x drip = 10
    sus y drip = 32
    sus result drip = add(x, y)
    vibez.spill("Result:", result)
    damn 0
}`
  },
  {
    name: "Pattern Matching",
    code: `yeet "vibez"

slay check_number(n drip) tea {
    sick (n) {
        when 0 -> "zero"
        when 1 -> "one"
        when x ready (x > 0 && x < 10) -> "small positive"
        when x ready (x >= 10) -> "large positive"
        when _ -> "negative"
    }
}

slay main() drip {
    vibez.spill(check_number(0))
    vibez.spill(check_number(5))
    vibez.spill(check_number(15))
    vibez.spill(check_number(-3))
    damn 0
}`
  },
  {
    name: "Control Flow",
    code: `yeet "vibez"

slay main() drip {
    sus count drip = 5
    
    // If-else
    ready (count > 0) {
        vibez.spill("Count is positive")
    } otherwise {
        vibez.spill("Count is not positive")
    }
    
    // While loop
    bestie (count > 0) {
        vibez.spill("Countdown:", count)
        count = count - 1
    }
    
    vibez.spill("Done!")
    damn 0
}`
  },
  {
    name: "Data Structures",
    code: `yeet "vibez"

squad Person {
    spill name tea
    spill age drip
}

slay greet_person(p Person) {
    vibez.spill("Hello,", p.name, "- you are", p.age, "years old")
}

slay main() drip {
    sus person Person = Person{
        name: "Alice",
        age: 30
    }
    
    greet_person(person)
    damn 0
}`
  },
  {
    name: "Error Handling",
    code: `yeet "vibez"

slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "Cannot divide by zero"
    }
    damn a / b
}

slay main() drip {
    sus result drip = divide(10, 2) fam {
        when "Cannot divide by zero" -> {
            vibez.spill("Error: Division by zero!")
            damn 0
        }
        when other -> {
            vibez.spill("Unexpected error:", other)
            damn 1
        }
    }
    
    vibez.spill("Result:", result)
    damn 0
}`
  }
];

export default function PlaygroundPage() {
  const [playground, setPlayground] = useState<PlaygroundState>({
    code: defaultCode,
    output: '',
    isRunning: false
  });
  
  const editorRef = useRef<any>(null);

  const handleEditorDidMount = (editor: any, monaco: any) => {
    editorRef.current = editor;
    
    // Register CURSED language
    monaco.languages.register({ id: 'cursed' });
    
    // Define CURSED language tokens
    monaco.languages.setMonarchTokensProvider('cursed', {
      tokenizer: {
        root: [
          // Keywords
          [/\b(sus|slay|damn|vibez|yeet|ready|otherwise|bestie|squad|collab|sick|when|stan|defer|yikes|fam|shook|based|cringe|facts|lit|tea|drip|normie)\b/, 'keyword'],
          
          // Types
          [/\b(normie|smol|mid|thicc|drip|snack|meal|byte|rune|extra|tea|lit|sip)\b/, 'type'],
          
          // Strings
          [/"([^"\\]|\\.)*$/, 'string.invalid'],
          [/"/, 'string', '@string'],
          [/'([^'\\]|\\.)*$/, 'string.invalid'],
          [/'/, 'string', '@string_single'],
          [/`/, 'string', '@string_backtick'],
          
          // Comments
          [/\/\/.*$/, 'comment'],
          [/\/\*/, 'comment', '@comment'],
          
          // Numbers
          [/\d*\.\d+([eE][\-+]?\d+)?/, 'number.float'],
          [/0[xX][0-9a-fA-F]+/, 'number.hex'],
          [/0[oO][0-7]+/, 'number.octal'],
          [/0[bB][01]+/, 'number.binary'],
          [/\d+/, 'number'],
          
          // Operators
          [/[{}()\[\]]/, '@brackets'],
          [/[<>]=?/, 'operator'],
          [/[+\-*/%&|^~!=]/, 'operator'],
          [/:=|[+\-*/%&|^]=/, 'operator'],
          [/<-/, 'operator'],
          
          // Identifiers
          [/[a-zA-Z_]\w*/, 'identifier'],
        ],
        
        string: [
          [/[^\\"]+/, 'string'],
          [/\\./, 'string.escape'],
          [/"/, 'string', '@pop']
        ],
        
        string_single: [
          [/[^\\']+/, 'string'],
          [/\\./, 'string.escape'],
          [/'/, 'string', '@pop']
        ],
        
        string_backtick: [
          [/[^`]+/, 'string'],
          [/`/, 'string', '@pop']
        ],
        
        comment: [
          [/[^\/*]+/, 'comment'],
          [/\*\//, 'comment', '@pop'],
          [/[\/*]/, 'comment']
        ]
      }
    });
    
    // Define theme
    monaco.editor.defineTheme('cursed-dark', {
      base: 'vs-dark',
      inherit: true,
      rules: [
        { token: 'keyword', foreground: '#569CD6', fontStyle: 'bold' },
        { token: 'type', foreground: '#4EC9B0' },
        { token: 'string', foreground: '#CE9178' },
        { token: 'comment', foreground: '#6A9955', fontStyle: 'italic' },
        { token: 'number', foreground: '#B5CEA8' },
        { token: 'operator', foreground: '#D4D4D4' },
      ],
      colors: {
        'editor.background': '#0F1419',
        'editor.foreground': '#BFBDB6',
      }
    });
    
    monaco.editor.setTheme('cursed-dark');
  };

  const runCode = async () => {
    setPlayground(prev => ({ ...prev, isRunning: true, output: 'Running...' }));
    
    try {
      const response = await fetch('/api/compile', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ code: playground.code }),
      });
      
      const result = await response.json();
      
      if (result.success) {
        setPlayground(prev => ({ ...prev, output: result.output }));
        toast.success('Code executed successfully!');
      } else {
        setPlayground(prev => ({ ...prev, output: `Error: ${result.error}` }));
        toast.error('Compilation failed');
      }
    } catch (error) {
      setPlayground(prev => ({ ...prev, output: `Network error: ${error}` }));
      toast.error('Failed to run code');
    } finally {
      setPlayground(prev => ({ ...prev, isRunning: false }));
    }
  };

  const shareCode = async () => {
    try {
      const response = await fetch('/api/share', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ code: playground.code }),
      });
      
      const result = await response.json();
      
      if (result.success) {
        const shareUrl = `${window.location.origin}/share/${result.id}`;
        await navigator.clipboard.writeText(shareUrl);
        setPlayground(prev => ({ ...prev, shareUrl }));
        toast.success('Share link copied to clipboard!');
      }
    } catch (error) {
      toast.error('Failed to create share link');
    }
  };

  const loadExample = (example: typeof examples[0]) => {
    setPlayground(prev => ({ ...prev, code: example.code, output: '' }));
    toast.success(`Loaded example: ${example.name}`);
  };

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      {/* Header */}
      <header className="border-b border-gray-800 bg-gray-900/95 backdrop-blur supports-[backdrop-filter]:bg-gray-900/60">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center space-x-4">
              <h1 className="text-2xl font-bold text-yellow-400">CURSED</h1>
              <span className="text-gray-400">Playground</span>
            </div>
            
            <nav className="flex items-center space-x-6">
              <Link href="/docs" className="text-gray-300 hover:text-white flex items-center space-x-1">
                <BookOpen size={16} />
                <span>Docs</span>
              </Link>
              <Link href="/packages" className="text-gray-300 hover:text-white flex items-center space-x-1">
                <Package size={16} />
                <span>Packages</span>
              </Link>
              <Link href="/community" className="text-gray-300 hover:text-white flex items-center space-x-1">
                <Users size={16} />
                <span>Community</span>
              </Link>
              <a href="https://github.com/ghuntley/cursed" target="_blank" rel="noopener noreferrer" className="text-gray-300 hover:text-white flex items-center space-x-1">
                <Star size={16} />
                <span>GitHub</span>
              </a>
            </nav>
          </div>
        </div>
      </header>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
        {/* Controls */}
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center space-x-4">
            <button
              onClick={runCode}
              disabled={playground.isRunning}
              className="bg-green-600 hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed px-4 py-2 rounded-lg flex items-center space-x-2 font-medium transition-colors"
            >
              <Play size={16} />
              <span>{playground.isRunning ? 'Running...' : 'Run'}</span>
            </button>
            
            <button
              onClick={shareCode}
              className="bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded-lg flex items-center space-x-2 font-medium transition-colors"
            >
              <Share size={16} />
              <span>Share</span>
            </button>
            
            <button
              onClick={() => {
                const blob = new Blob([playground.code], { type: 'text/plain' });
                const url = URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = 'program.💀';
                a.click();
                URL.revokeObjectURL(url);
              }}
              className="bg-gray-600 hover:bg-gray-700 px-4 py-2 rounded-lg flex items-center space-x-2 font-medium transition-colors"
            >
              <Download size={16} />
              <span>Download</span>
            </button>
          </div>
          
          <div className="flex items-center space-x-2">
            <span className="text-gray-400 text-sm">Examples:</span>
            <select
              onChange={(e) => {
                const example = examples.find(ex => ex.name === e.target.value);
                if (example) loadExample(example);
              }}
              className="bg-gray-800 border border-gray-700 rounded px-3 py-1 text-sm"
            >
              <option value="">Select an example...</option>
              {examples.map((example) => (
                <option key={example.name} value={example.name}>
                  {example.name}
                </option>
              ))}
            </select>
          </div>
        </div>

        {/* Editor and Output */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Code Editor */}
          <div className="bg-gray-800 rounded-lg border border-gray-700 overflow-hidden">
            <div className="bg-gray-750 px-4 py-2 border-b border-gray-700">
              <h3 className="text-sm font-medium text-gray-300">Code Editor</h3>
            </div>
            <div style={{ height: '500px' }}>
              <MonacoEditor
                language="cursed"
                theme="cursed-dark"
                value={playground.code}
                onChange={(value) => setPlayground(prev => ({ ...prev, code: value || '' }))}
                onMount={handleEditorDidMount}
                options={{
                  minimap: { enabled: false },
                  fontSize: 14,
                  wordWrap: 'on',
                  automaticLayout: true,
                  scrollBeyondLastLine: false,
                  padding: { top: 16, bottom: 16 },
                }}
              />
            </div>
          </div>

          {/* Output */}
          <div className="bg-gray-800 rounded-lg border border-gray-700 overflow-hidden">
            <div className="bg-gray-750 px-4 py-2 border-b border-gray-700">
              <h3 className="text-sm font-medium text-gray-300">Output</h3>
            </div>
            <div className="p-4 h-[500px] overflow-auto">
              <pre className="text-sm text-gray-300 whitespace-pre-wrap font-mono">
                {playground.output || 'Click "Run" to see the output...'}
              </pre>
            </div>
          </div>
        </div>

        {/* Quick Start Guide */}
        <div className="mt-12 bg-gray-800 rounded-lg border border-gray-700 p-6">
          <h2 className="text-xl font-bold mb-4 text-yellow-400">Quick Start Guide</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <div>
              <h3 className="font-semibold mb-2 text-green-400">Variables</h3>
              <pre className="text-sm bg-gray-900 p-3 rounded text-gray-300 font-mono">
{`sus name tea = "CURSED"
sus count drip = 42
sus flag lit = based`}
              </pre>
            </div>
            
            <div>
              <h3 className="font-semibold mb-2 text-blue-400">Functions</h3>
              <pre className="text-sm bg-gray-900 p-3 rounded text-gray-300 font-mono">
{`slay greet(name tea) tea {
    damn "Hello, " + name
}`}
              </pre>
            </div>
            
            <div>
              <h3 className="font-semibold mb-2 text-purple-400">Control Flow</h3>
              <pre className="text-sm bg-gray-900 p-3 rounded text-gray-300 font-mono">
{`ready (condition) {
    // if body
} otherwise {
    // else body
}`}
              </pre>
            </div>
          </div>
          
          <div className="mt-6 flex items-center space-x-4">
            <Link href="/docs" className="text-blue-400 hover:text-blue-300 underline">
              Read the full documentation →
            </Link>
            <Link href="/packages" className="text-green-400 hover:text-green-300 underline">
              Browse packages →
            </Link>
          </div>
        </div>
      </div>
    </div>
  );
}
