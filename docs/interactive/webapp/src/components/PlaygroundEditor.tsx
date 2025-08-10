import React, { useRef, useEffect, useState } from 'react';
import * as monaco from 'monaco-editor';
import { Play, Copy, Share, Download, RotateCcw, Settings } from 'lucide-react';

interface PlaygroundEditorProps {
  initialCode?: string;
  onRun?: (code: string) => void;
  language?: string;
}

// CURSED language definition for Monaco Editor
const cursedLanguageDefinition = {
  id: 'cursed',
  extensions: ['.csd'],
  aliases: ['CURSED', 'cursed'],
  mimetypes: ['text/cursed'],
};

const cursedTokens = {
  tokenizer: {
    root: [
      // Keywords
      [/\b(sus|drip|tea|lit|based|cap|nocap|slay|yeet|bestie|ready|otherwise|sick|squad|collab|go|damn|yikes|fam|shook)\b/, 'keyword'],
      
      // Types
      [/\b(chan|struct|interface|enum)\b/, 'keyword.type'],
      
      // Built-in functions
      [/\b(spill|vibez|mathz|stringz|arrayz|testz|filez|networkz|timez|jsonz|cryptz|concurrenz|asyncz)\b/, 'keyword.builtin'],
      
      // Numbers
      [/\d*\.\d+([eE][\-+]?\d+)?/, 'number.float'],
      [/\d+/, 'number'],
      
      // Strings
      [/"([^"\\]|\\.)*$/, 'string.invalid'],
      [/"/, 'string', '@string'],
      
      // Comments
      [/\/\/.*$/, 'comment'],
      [/\/\*/, 'comment', '@comment'],
      
      // Operators
      [/[=><!~?:&|+\-*\/\^%]+/, 'operator'],
      
      // Delimiters
      [/[{}()\[\]]/, '@brackets'],
      [/[;,.]/, 'delimiter'],
    ],
    
    string: [
      [/[^\\"]+/, 'string'],
      [/\\./, 'string.escape'],
      [/"/, 'string', '@pop']
    ],
    
    comment: [
      [/[^\/*]+/, 'comment'],
      [/\*\//, 'comment', '@pop'],
      [/[\/*]/, 'comment']
    ],
  },
};

const cursedCompletionProvider = {
  provideCompletionItems: (model: monaco.editor.ITextModel, position: monaco.Position) => {
    const suggestions = [
      // Variables
      { label: 'sus', kind: monaco.languages.CompletionItemKind.Keyword, insertText: 'sus ${1:name} ${2:type} = ${3:value}', insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet },
      
      // Functions
      { label: 'slay', kind: monaco.languages.CompletionItemKind.Keyword, insertText: 'slay ${1:name}(${2:params}) ${3:return_type} {\n\t${4}\n}', insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet },
      
      // Control flow
      { label: 'ready', kind: monaco.languages.CompletionItemKind.Keyword, insertText: 'ready (${1:condition}) {\n\t${2}\n}', insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet },
      { label: 'bestie', kind: monaco.languages.CompletionItemKind.Keyword, insertText: 'bestie (${1:condition}) {\n\t${2}\n}', insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet },
      
      // Built-in modules
      { label: 'vibez.spill', kind: monaco.languages.CompletionItemKind.Function, insertText: 'vibez.spill(${1:value})' },
      { label: 'mathz.sqrt', kind: monaco.languages.CompletionItemKind.Function, insertText: 'mathz.sqrt(${1:number})' },
      { label: 'stringz.len', kind: monaco.languages.CompletionItemKind.Function, insertText: 'stringz.len(${1:string})' },
      
      // Concurrency
      { label: 'go', kind: monaco.languages.CompletionItemKind.Keyword, insertText: 'go {\n\t${1}\n}', insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet },
      { label: 'chan', kind: monaco.languages.CompletionItemKind.Keyword, insertText: 'chan<${1:type}>' },
    ];
    
    return { suggestions };
  }
};

export function PlaygroundEditor({ initialCode = '', onRun, language = 'cursed' }: PlaygroundEditorProps) {
  const editorRef = useRef<HTMLDivElement>(null);
  const [editor, setEditor] = useState<monaco.editor.IStandaloneCodeEditor | null>(null);
  const [code, setCode] = useState(initialCode);
  const [isRunning, setIsRunning] = useState(false);
  const [output, setOutput] = useState('');
  const [fontSize, setFontSize] = useState(14);

  useEffect(() => {
    if (editorRef.current) {
      // Register CURSED language
      monaco.languages.register(cursedLanguageDefinition);
      monaco.languages.setMonarchTokensProvider('cursed', cursedTokens);
      monaco.languages.registerCompletionItemProvider('cursed', cursedCompletionProvider);

      // Create editor
      const editorInstance = monaco.editor.create(editorRef.current, {
        value: code,
        language: language,
        theme: 'vs-dark',
        fontSize: fontSize,
        minimap: { enabled: false },
        scrollBeyondLastLine: false,
        automaticLayout: true,
        suggestOnTriggerCharacters: true,
        quickSuggestions: true,
        parameterHints: { enabled: true },
        formatOnPaste: true,
        formatOnType: true,
      });

      // Update code on change
      editorInstance.onDidChangeModelContent(() => {
        setCode(editorInstance.getValue());
      });

      setEditor(editorInstance);

      return () => {
        editorInstance.dispose();
      };
    }
  }, []);

  useEffect(() => {
    if (editor) {
      editor.updateOptions({ fontSize });
    }
  }, [fontSize, editor]);

  const handleRun = async () => {
    setIsRunning(true);
    try {
      // Simulate code execution
      setOutput('Running CURSED code...\n');
      
      // Mock execution result
      await new Promise(resolve => setTimeout(resolve, 1000));
      setOutput(prev => prev + 'Program executed successfully!\n');
      setOutput(prev => prev + '> Output: Hello, CURSED World!\n');
      
      if (onRun) {
        onRun(code);
      }
    } catch (error) {
      setOutput(prev => prev + `Error: ${error}\n`);
    } finally {
      setIsRunning(false);
    }
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(code);
  };

  const handleShare = () => {
    const encodedCode = btoa(code);
    const shareUrl = `${window.location.origin}/playground?code=${encodedCode}`;
    navigator.clipboard.writeText(shareUrl);
  };

  const handleDownload = () => {
    const blob = new Blob([code], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'example.csd';
    a.click();
    URL.revokeObjectURL(url);
  };

  const handleReset = () => {
    setCode(initialCode);
    setOutput('');
    if (editor) {
      editor.setValue(initialCode);
    }
  };

  return (
    <div className="h-full flex flex-col bg-slate-900 rounded-lg overflow-hidden">
      {/* Toolbar */}
      <div className="bg-slate-800 px-4 py-2 flex items-center justify-between border-b border-slate-700">
        <div className="flex items-center space-x-2">
          <button
            onClick={handleRun}
            disabled={isRunning}
            className="flex items-center space-x-2 px-4 py-2 bg-green-600 hover:bg-green-700 disabled:bg-green-800 text-white rounded-lg transition-colors"
          >
            <Play size={16} />
            <span>{isRunning ? 'Running...' : 'Run'}</span>
          </button>
          
          <button
            onClick={handleCopy}
            className="p-2 text-slate-300 hover:text-white hover:bg-slate-700 rounded-lg transition-colors"
            title="Copy Code"
          >
            <Copy size={16} />
          </button>
          
          <button
            onClick={handleShare}
            className="p-2 text-slate-300 hover:text-white hover:bg-slate-700 rounded-lg transition-colors"
            title="Share Code"
          >
            <Share size={16} />
          </button>
          
          <button
            onClick={handleDownload}
            className="p-2 text-slate-300 hover:text-white hover:bg-slate-700 rounded-lg transition-colors"
            title="Download"
          >
            <Download size={16} />
          </button>
          
          <button
            onClick={handleReset}
            className="p-2 text-slate-300 hover:text-white hover:bg-slate-700 rounded-lg transition-colors"
            title="Reset"
          >
            <RotateCcw size={16} />
          </button>
        </div>

        <div className="flex items-center space-x-2">
          <label className="text-slate-300 text-sm">Font Size:</label>
          <select
            value={fontSize}
            onChange={(e) => setFontSize(Number(e.target.value))}
            className="bg-slate-700 text-white px-2 py-1 rounded text-sm"
          >
            <option value={12}>12px</option>
            <option value={14}>14px</option>
            <option value={16}>16px</option>
            <option value={18}>18px</option>
          </select>
        </div>
      </div>

      {/* Editor and Output */}
      <div className="flex-1 flex">
        {/* Editor */}
        <div className="flex-1 relative">
          <div ref={editorRef} className="h-full" />
        </div>

        {/* Output Panel */}
        <div className="w-1/3 bg-slate-800 border-l border-slate-700 flex flex-col">
          <div className="px-4 py-2 bg-slate-700 text-slate-200 text-sm font-medium">
            Output
          </div>
          <div className="flex-1 p-4 font-mono text-sm text-slate-300 overflow-auto">
            <pre className="whitespace-pre-wrap">{output}</pre>
          </div>
        </div>
      </div>
    </div>
  );
}
