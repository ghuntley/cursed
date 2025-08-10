import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { Search, Book, Package, Function, ArrowRight, ExternalLink } from 'lucide-react';

// Mock API data
const modules = [
  {
    id: 'vibez',
    name: 'vibez',
    description: 'Input/output operations and printing functions',
    category: 'core',
    functions: [
      { name: 'spill', signature: 'spill(...values)', description: 'Print values to console' },
      { name: 'input', signature: 'input(prompt tea) tea', description: 'Read user input' },
      { name: 'spill_line', signature: 'spill_line(value)', description: 'Print with newline' },
    ]
  },
  {
    id: 'mathz',
    name: 'mathz',
    description: 'Mathematical functions and constants',
    category: 'core',
    functions: [
      { name: 'sqrt', signature: 'sqrt(x drip) drip', description: 'Square root' },
      { name: 'pow', signature: 'pow(base drip, exp drip) drip', description: 'Power function' },
      { name: 'sin', signature: 'sin(x drip) drip', description: 'Sine function' },
      { name: 'cos', signature: 'cos(x drip) drip', description: 'Cosine function' },
    ]
  },
  {
    id: 'stringz',
    name: 'stringz',
    description: 'String manipulation and processing functions',
    category: 'core',
    functions: [
      { name: 'len', signature: 'len(s tea) drip', description: 'String length' },
      { name: 'slice', signature: 'slice(s tea, start drip, end drip) tea', description: 'String slice' },
      { name: 'contains', signature: 'contains(s tea, substr tea) lit', description: 'Check if string contains substring' },
    ]
  },
  {
    id: 'arrayz',
    name: 'arrayz',
    description: 'Array and slice operations',
    category: 'collections',
    functions: [
      { name: 'len', signature: 'len<T>(arr []T) drip', description: 'Array length' },
      { name: 'push', signature: 'push<T>(arr []T, item T)', description: 'Add item to array' },
      { name: 'pop', signature: 'pop<T>(arr []T) T', description: 'Remove and return last item' },
    ]
  },
  {
    id: 'concurrenz',
    name: 'concurrenz',
    description: 'Concurrency primitives and goroutines',
    category: 'concurrency',
    functions: [
      { name: 'make_channel', signature: 'make_channel<T>() chan<T>', description: 'Create new channel' },
      { name: 'spawn', signature: 'spawn(func slay())', description: 'Start new goroutine' },
      { name: 'sleep', signature: 'sleep(ms drip)', description: 'Sleep for milliseconds' },
    ]
  },
  {
    id: 'filez',
    name: 'filez',
    description: 'File system operations',
    category: 'io',
    functions: [
      { name: 'read_string', signature: 'read_string(path tea) yikes<tea>', description: 'Read file as string' },
      { name: 'write_string', signature: 'write_string(path tea, content tea) yikes<tea>', description: 'Write string to file' },
      { name: 'exists', signature: 'exists(path tea) lit', description: 'Check if file exists' },
    ]
  },
];

const categories = [
  { id: 'all', name: 'All Modules', count: modules.length },
  { id: 'core', name: 'Core', count: modules.filter(m => m.category === 'core').length },
  { id: 'collections', name: 'Collections', count: modules.filter(m => m.category === 'collections').length },
  { id: 'concurrency', name: 'Concurrency', count: modules.filter(m => m.category === 'concurrency').length },
  { id: 'io', name: 'I/O', count: modules.filter(m => m.category === 'io').length },
];

export function APIDocsPage() {
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedCategory, setSelectedCategory] = useState('all');
  const [selectedModule, setSelectedModule] = useState<string | null>(null);

  const filteredModules = modules.filter(module => {
    const matchesSearch = module.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                         module.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
                         module.functions.some(f => 
                           f.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                           f.description.toLowerCase().includes(searchQuery.toLowerCase())
                         );
    const matchesCategory = selectedCategory === 'all' || module.category === selectedCategory;
    return matchesSearch && matchesCategory;
  });

  const selectedModuleData = selectedModule ? modules.find(m => m.id === selectedModule) : null;

  return (
    <div className="max-w-7xl mx-auto">
      <div className="flex h-screen">
        {/* Sidebar */}
        <div className="w-80 bg-white dark:bg-slate-800 border-r border-slate-200 dark:border-slate-700 flex flex-col">
          {/* Header */}
          <div className="p-6 border-b border-slate-200 dark:border-slate-700">
            <h1 className="text-2xl font-bold text-slate-900 dark:text-white mb-4">
              API Reference
            </h1>
            
            {/* Search */}
            <div className="relative">
              <Search size={18} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-slate-400" />
              <input
                type="text"
                placeholder="Search modules and functions..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full pl-10 pr-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
          </div>

          {/* Categories */}
          <div className="p-4 border-b border-slate-200 dark:border-slate-700">
            <h3 className="font-medium text-slate-900 dark:text-white mb-3">Categories</h3>
            <div className="space-y-1">
              {categories.map(category => (
                <button
                  key={category.id}
                  onClick={() => setSelectedCategory(category.id)}
                  className={`w-full text-left px-3 py-2 rounded-lg text-sm transition-colors ${
                    selectedCategory === category.id
                      ? 'bg-blue-100 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300'
                      : 'text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700'
                  }`}
                >
                  <div className="flex items-center justify-between">
                    <span>{category.name}</span>
                    <span className="text-xs bg-slate-200 dark:bg-slate-600 px-2 py-1 rounded-full">
                      {category.count}
                    </span>
                  </div>
                </button>
              ))}
            </div>
          </div>

          {/* Modules List */}
          <div className="flex-1 overflow-y-auto p-4">
            <div className="space-y-2">
              {filteredModules.map(module => (
                <button
                  key={module.id}
                  onClick={() => setSelectedModule(module.id)}
                  className={`w-full text-left p-3 rounded-lg transition-colors ${
                    selectedModule === module.id
                      ? 'bg-blue-100 dark:bg-blue-900/50 border-2 border-blue-200 dark:border-blue-700'
                      : 'bg-slate-50 dark:bg-slate-700 hover:bg-slate-100 dark:hover:bg-slate-600'
                  }`}
                >
                  <div className="flex items-center space-x-3">
                    <Package size={16} className="text-blue-600 dark:text-blue-400" />
                    <div className="flex-1">
                      <div className="font-medium text-slate-900 dark:text-white">
                        {module.name}
                      </div>
                      <div className="text-xs text-slate-600 dark:text-slate-400 mt-1">
                        {module.description}
                      </div>
                      <div className="text-xs text-blue-600 dark:text-blue-400 mt-1">
                        {module.functions.length} functions
                      </div>
                    </div>
                  </div>
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Main Content */}
        <div className="flex-1 overflow-y-auto">
          {selectedModuleData ? (
            <div className="p-8">
              {/* Module Header */}
              <div className="mb-8">
                <div className="flex items-center space-x-3 mb-4">
                  <Package size={24} className="text-blue-600 dark:text-blue-400" />
                  <h1 className="text-3xl font-bold text-slate-900 dark:text-white">
                    {selectedModuleData.name}
                  </h1>
                  <span className="px-3 py-1 bg-blue-100 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300 rounded-full text-sm">
                    {selectedModuleData.category}
                  </span>
                </div>
                <p className="text-lg text-slate-600 dark:text-slate-300 mb-6">
                  {selectedModuleData.description}
                </p>

                {/* Import Example */}
                <div className="bg-slate-900 rounded-lg p-4 mb-6">
                  <div className="text-sm text-slate-400 mb-2">Import this module:</div>
                  <code className="text-green-400">yeet "{selectedModuleData.name}"</code>
                </div>

                {/* Quick Actions */}
                <div className="flex space-x-4">
                  <Link
                    to={`/playground?code=${btoa(`yeet "${selectedModuleData.name}"\nyeet "vibez"\n\n# Try ${selectedModuleData.name} functions here\nvibez.spill("Using ${selectedModuleData.name} module!")`)}`}
                    className="inline-flex items-center space-x-2 bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg"
                  >
                    <ExternalLink size={16} />
                    <span>Try in Playground</span>
                  </Link>
                  <Link
                    to={`/tutorials?search=${selectedModuleData.name}`}
                    className="inline-flex items-center space-x-2 bg-slate-100 hover:bg-slate-200 dark:bg-slate-700 dark:hover:bg-slate-600 text-slate-900 dark:text-white px-4 py-2 rounded-lg"
                  >
                    <Book size={16} />
                    <span>View Tutorials</span>
                  </Link>
                </div>
              </div>

              {/* Functions */}
              <div className="space-y-6">
                <h2 className="text-2xl font-bold text-slate-900 dark:text-white">Functions</h2>
                
                {selectedModuleData.functions.map((func, index) => (
                  <div key={index} className="bg-white dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-700 p-6">
                    <div className="flex items-start space-x-4">
                      <Function size={20} className="text-blue-600 dark:text-blue-400 mt-1" />
                      <div className="flex-1">
                        <h3 className="text-xl font-semibold text-slate-900 dark:text-white mb-2">
                          {func.name}
                        </h3>
                        
                        {/* Signature */}
                        <div className="bg-slate-100 dark:bg-slate-700 rounded-lg p-3 mb-4">
                          <code className="text-sm text-slate-800 dark:text-slate-200">
                            {func.signature}
                          </code>
                        </div>
                        
                        {/* Description */}
                        <p className="text-slate-600 dark:text-slate-300 mb-4">
                          {func.description}
                        </p>
                        
                        {/* Example */}
                        <div className="space-y-3">
                          <h4 className="font-medium text-slate-900 dark:text-white">Example:</h4>
                          <div className="bg-slate-900 rounded-lg p-4">
                            <code className="text-green-400 text-sm">
                              {getExampleCode(selectedModuleData.name, func.name)}
                            </code>
                          </div>
                        </div>
                        
                        {/* Try Button */}
                        <div className="mt-4">
                          <Link
                            to={`/playground?code=${btoa(getExampleCode(selectedModuleData.name, func.name))}`}
                            className="inline-flex items-center space-x-2 text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300 text-sm"
                          >
                            <span>Try this example</span>
                            <ArrowRight size={14} />
                          </Link>
                        </div>
                      </div>
                    </div>
                  </div>
                ))}
              </div>

              {/* Related Modules */}
              <div className="mt-12 bg-slate-50 dark:bg-slate-800/50 rounded-lg p-6">
                <h3 className="font-semibold text-slate-900 dark:text-white mb-4">
                  Related Modules
                </h3>
                <div className="grid md:grid-cols-2 gap-4">
                  {modules
                    .filter(m => m.category === selectedModuleData.category && m.id !== selectedModuleData.id)
                    .slice(0, 4)
                    .map(module => (
                      <button
                        key={module.id}
                        onClick={() => setSelectedModule(module.id)}
                        className="text-left p-3 bg-white dark:bg-slate-800 rounded-lg hover:shadow-md transition-shadow"
                      >
                        <div className="font-medium text-slate-900 dark:text-white">{module.name}</div>
                        <div className="text-sm text-slate-600 dark:text-slate-400">{module.description}</div>
                      </button>
                    ))}
                </div>
              </div>
            </div>
          ) : (
            /* Welcome View */
            <div className="flex items-center justify-center h-full">
              <div className="text-center space-y-6 max-w-md">
                <Package size={64} className="mx-auto text-slate-400" />
                <div className="space-y-2">
                  <h2 className="text-2xl font-bold text-slate-900 dark:text-white">
                    CURSED API Reference
                  </h2>
                  <p className="text-slate-600 dark:text-slate-300">
                    Select a module from the sidebar to view its functions and documentation.
                  </p>
                </div>
                <div className="space-y-3">
                  <button
                    onClick={() => setSelectedModule('vibez')}
                    className="block w-full bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg"
                  >
                    Start with vibez module
                  </button>
                  <Link
                    to="/tutorials"
                    className="block w-full bg-slate-100 hover:bg-slate-200 dark:bg-slate-700 dark:hover:bg-slate-600 text-slate-900 dark:text-white px-6 py-3 rounded-lg"
                  >
                    View Tutorials
                  </Link>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

function getExampleCode(moduleName: string, functionName: string): string {
  const examples: Record<string, Record<string, string>> = {
    vibez: {
      spill: `yeet "vibez"\n\nvibez.spill("Hello, World!")\nvibez.spill("Number:", 42)\nvibez.spill("Multiple", "values", "at", "once")`,
      input: `yeet "vibez"\n\nsus name tea = vibez.input("Enter your name: ")\nvibez.spill("Hello,", name, "!")`,
      spill_line: `yeet "vibez"\n\nvibez.spill_line("Line 1")\nvibez.spill_line("Line 2")\nvibez.spill_line("Line 3")`,
    },
    mathz: {
      sqrt: `yeet "mathz"\nyeet "vibez"\n\nsus number drip = 16\nsus result drip = mathz.sqrt(number)\nvibez.spill("Square root of", number, "is", result)`,
      pow: `yeet "mathz"\nyeet "vibez"\n\nsus base drip = 2\nsus exponent drip = 8\nsus result drip = mathz.pow(base, exponent)\nvibez.spill(base, "^", exponent, "=", result)`,
      sin: `yeet "mathz"\nyeet "vibez"\n\nsus angle drip = mathz.pi / 2\nsus result drip = mathz.sin(angle)\nvibez.spill("sin(π/2) =", result)`,
      cos: `yeet "mathz"\nyeet "vibez"\n\nsus angle drip = 0\nsus result drip = mathz.cos(angle)\nvibez.spill("cos(0) =", result)`,
    },
    stringz: {
      len: `yeet "stringz"\nyeet "vibez"\n\nsus text tea = "Hello, CURSED!"\nsus length drip = stringz.len(text)\nvibez.spill("Length of '", text, "' is", length)`,
      slice: `yeet "stringz"\nyeet "vibez"\n\nsus text tea = "Hello, World!"\nsus part tea = stringz.slice(text, 0, 5)\nvibez.spill("First 5 characters:", part)`,
      contains: `yeet "stringz"\nyeet "vibez"\n\nsus text tea = "Hello, CURSED!"\nsus found lit = stringz.contains(text, "CURSED")\nvibez.spill("Contains 'CURSED':", found)`,
    },
    arrayz: {
      len: `yeet "arrayz"\nyeet "vibez"\n\nsus numbers []drip = [1, 2, 3, 4, 5]\nsus length drip = arrayz.len(numbers)\nvibez.spill("Array length:", length)`,
      push: `yeet "arrayz"\nyeet "vibez"\n\nsus numbers []drip = [1, 2, 3]\narrayz.push(numbers, 4)\nvibez.spill("After push:", numbers)`,
      pop: `yeet "arrayz"\nyeet "vibez"\n\nsus numbers []drip = [1, 2, 3, 4]\nsus last drip = arrayz.pop(numbers)\nvibez.spill("Popped:", last)\nvibez.spill("Remaining:", numbers)`,
    },
    concurrenz: {
      make_channel: `yeet "concurrenz"\nyeet "vibez"\n\nsus ch chan<tea> = concurrenz.make_channel()\n\ngo {\n    ch <- "Hello from goroutine!"\n}\n\nsus message tea = <-ch\nvibez.spill("Received:", message)`,
      spawn: `yeet "concurrenz"\nyeet "vibez"\n\nslay worker() {\n    vibez.spill("Worker running!")\n}\n\nconcurrenz.spawn(worker)\nvibez.spill("Started worker")`,
      sleep: `yeet "concurrenz"\nyeet "vibez"\n\nvibez.spill("Before sleep")\nconcurrenz.sleep(1000)  # Sleep 1 second\nvibez.spill("After sleep")`,
    },
    filez: {
      read_string: `yeet "filez"\nyeet "vibez"\n\nsus content tea = filez.read_string("example.txt") fam {\n    when _ -> {\n        vibez.spill("Failed to read file")\n        damn ""\n    }\n}\nvibez.spill("File content:", content)`,
      write_string: `yeet "filez"\nyeet "vibez"\n\nsus content tea = "Hello, file system!"\nfilez.write_string("output.txt", content) fam {\n    when _ -> vibez.spill("Failed to write file")\n} shook {\n    vibez.spill("File written successfully")\n}`,
      exists: `yeet "filez"\nyeet "vibez"\n\nsus file_exists lit = filez.exists("example.txt")\nready (file_exists) {\n    vibez.spill("File exists!")\n} otherwise {\n    vibez.spill("File not found")\n}`,
    },
  };

  return examples[moduleName]?.[functionName] || `yeet "${moduleName}"\nyeet "vibez"\n\n# Example for ${moduleName}.${functionName}\nvibez.spill("Try this function!")`;
}
