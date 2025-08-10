import React, { useState, useEffect } from 'react';
import { PlaygroundEditor } from '../components/PlaygroundEditor';
import { Link } from 'react-router-dom';
import { 
  Save, 
  Share, 
  Download, 
  Upload, 
  BookOpen, 
  Code, 
  Zap,
  RefreshCw,
  Settings,
  FileText,
  Play
} from 'lucide-react';

const exampleProjects = [
  {
    id: 'hello-world',
    title: 'Hello World',
    description: 'Your first CURSED program',
    code: `yeet "vibez"

# Welcome to CURSED!
vibez.spill("Hello, World!")
vibez.spill("Ready to learn?")`,
    category: 'beginner'
  },
  {
    id: 'variables-types',
    title: 'Variables and Types',
    description: 'Learn about CURSED data types',
    code: `yeet "vibez"

# Different data types in CURSED
sus name tea = "Alice"
sus age drip = 25
sus height drip = 5.7
sus is_student lit = based
sus favorite_numbers []drip = [7, 42, 13]

vibez.spill("Name:", name)
vibez.spill("Age:", age)
vibez.spill("Height:", height, "feet")
vibez.spill("Student:", is_student)
vibez.spill("Lucky numbers:", favorite_numbers)`,
    category: 'beginner'
  },
  {
    id: 'functions',
    title: 'Functions',
    description: 'Define and call functions',
    code: `yeet "vibez"
yeet "mathz"

# Function to calculate circle area
slay circle_area(radius drip) drip {
    damn mathz.pi * radius * radius
}

# Function to greet someone
slay greet(name tea, age drip) tea {
    damn "Hello " + name + ", you are " + age + " years old!"
}

# Using the functions
sus radius drip = 5.0
sus area drip = circle_area(radius)
sus greeting tea = greet("Bob", 30)

vibez.spill("Circle area:", area)
vibez.spill(greeting)`,
    category: 'beginner'
  },
  {
    id: 'control-flow',
    title: 'Control Flow',
    description: 'Conditionals and loops',
    code: `yeet "vibez"

# Conditional statements
sus age drip = 20

ready (age >= 18) {
    vibez.spill("You can vote!")
} otherwise ready (age >= 16) {
    vibez.spill("You can drive!")
} otherwise {
    vibez.spill("Enjoy being young!")
}

# Loop examples
vibez.spill("Counting to 5:")
bestie (i drip: 1..6) {
    vibez.spill("Count:", i)
}

# Loop through array
sus fruits []tea = ["apple", "banana", "orange"]
vibez.spill("My favorite fruits:")
bestie (fruit tea: fruits) {
    vibez.spill("-", fruit)
}`,
    category: 'beginner'
  },
  {
    id: 'error-handling',
    title: 'Error Handling',
    description: 'Handle errors gracefully',
    code: `yeet "vibez"

# Function that can fail
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "division by zero"
    }
    damn a / b
}

# Safe division with error handling
slay safe_divide(a drip, b drip) drip {
    sus result drip = divide(a, b) fam {
        when "division by zero" -> {
            vibez.spill("Error: Cannot divide by zero!")
            damn 0
        }
        when _ -> {
            vibez.spill("Unexpected error occurred")
            damn 0
        }
    }
    damn result
}

# Test the functions
vibez.spill("10 / 2 =", safe_divide(10, 2))
vibez.spill("10 / 0 =", safe_divide(10, 0))`,
    category: 'intermediate'
  },
  {
    id: 'concurrency',
    title: 'Concurrency Basics',
    description: 'Goroutines and channels',
    code: `yeet "vibez"
yeet "concurrenz"
yeet "timez"

# Simple goroutine example
slay worker(id drip) {
    bestie (i drip: 1..4) {
        vibez.spill("Worker", id, "task", i)
        timez.sleep(100)  # Simulate work
    }
    vibez.spill("Worker", id, "finished")
}

# Channel communication
slay channel_example() {
    sus ch chan<tea> = concurrenz.make_channel()
    
    # Send data in a goroutine
    go {
        ch <- "Hello from goroutine!"
        ch <- "Channels are awesome!"
    }
    
    # Receive data
    sus msg1 tea = <-ch
    sus msg2 tea = <-ch
    
    vibez.spill("Received:", msg1)
    vibez.spill("Received:", msg2)
}

# Run the examples
vibez.spill("Starting workers...")
go { worker(1) }
go { worker(2) }

channel_example()`,
    category: 'intermediate'
  }
];

const categories = [
  { id: 'all', label: 'All Examples', icon: <FileText size={16} /> },
  { id: 'beginner', label: 'Beginner', icon: <BookOpen size={16} /> },
  { id: 'intermediate', label: 'Intermediate', icon: <Code size={16} /> },
  { id: 'advanced', label: 'Advanced', icon: <Zap size={16} /> },
];

export function PlaygroundPage() {
  const [code, setCode] = useState('');
  const [selectedExample, setSelectedExample] = useState<string | null>(null);
  const [selectedCategory, setSelectedCategory] = useState('all');
  const [isShareModalOpen, setIsShareModalOpen] = useState(false);

  // Load code from URL parameter on mount
  useEffect(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const encodedCode = urlParams.get('code');
    
    if (encodedCode) {
      try {
        const decodedCode = atob(encodedCode);
        setCode(decodedCode);
      } catch (error) {
        console.error('Failed to decode URL code:', error);
      }
    } else {
      // Load default hello world example
      const helloWorld = exampleProjects.find(p => p.id === 'hello-world');
      if (helloWorld) {
        setCode(helloWorld.code);
        setSelectedExample(helloWorld.id);
      }
    }
  }, []);

  const filteredExamples = selectedCategory === 'all' 
    ? exampleProjects 
    : exampleProjects.filter(p => p.category === selectedCategory);

  const handleExampleSelect = (example: typeof exampleProjects[0]) => {
    setCode(example.code);
    setSelectedExample(example.id);
  };

  const handleSave = () => {
    const blob = new Blob([code], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'cursed-playground.csd';
    a.click();
    URL.revokeObjectURL(url);
  };

  const handleShare = () => {
    const encodedCode = btoa(code);
    const shareUrl = `${window.location.origin}/playground?code=${encodedCode}`;
    navigator.clipboard.writeText(shareUrl);
    setIsShareModalOpen(true);
    setTimeout(() => setIsShareModalOpen(false), 3000);
  };

  const handleFileUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (e) => {
        const content = e.target?.result as string;
        setCode(content);
        setSelectedExample(null);
      };
      reader.readAsText(file);
    }
  };

  return (
    <div className="max-w-7xl mx-auto space-y-6">
      {/* Header */}
      <div className="text-center space-y-4">
        <h1 className="text-4xl font-bold text-slate-900 dark:text-white">
          CURSED Playground
        </h1>
        <p className="text-xl text-slate-600 dark:text-slate-300 max-w-3xl mx-auto">
          Write, run, and experiment with CURSED code directly in your browser. 
          No installation required!
        </p>
      </div>

      {/* Toolbar */}
      <div className="flex flex-wrap items-center justify-between gap-4 p-4 bg-white dark:bg-slate-800 rounded-lg shadow">
        <div className="flex items-center space-x-4">
          <button
            onClick={handleSave}
            className="flex items-center space-x-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
          >
            <Save size={16} />
            <span>Save</span>
          </button>
          
          <button
            onClick={handleShare}
            className="flex items-center space-x-2 px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors"
          >
            <Share size={16} />
            <span>Share</span>
          </button>
          
          <label className="flex items-center space-x-2 px-4 py-2 bg-slate-100 hover:bg-slate-200 dark:bg-slate-700 dark:hover:bg-slate-600 text-slate-900 dark:text-white rounded-lg transition-colors cursor-pointer">
            <Upload size={16} />
            <span>Upload</span>
            <input
              type="file"
              accept=".csd,.txt"
              onChange={handleFileUpload}
              className="hidden"
            />
          </label>
        </div>

        <div className="flex items-center space-x-4">
          <Link
            to="/tutorials"
            className="flex items-center space-x-2 text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
          >
            <BookOpen size={16} />
            <span>Learn CURSED</span>
          </Link>
          
          <Link
            to="/api"
            className="flex items-center space-x-2 text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
          >
            <Code size={16} />
            <span>API Docs</span>
          </Link>
        </div>
      </div>

      <div className="grid lg:grid-cols-4 gap-6">
        {/* Examples Sidebar */}
        <div className="lg:col-span-1 space-y-4">
          <div className="bg-white dark:bg-slate-800 rounded-lg shadow p-4">
            <h3 className="font-semibold text-slate-900 dark:text-white mb-4">
              Example Projects
            </h3>
            
            {/* Category Filter */}
            <div className="space-y-2 mb-4">
              {categories.map(category => (
                <button
                  key={category.id}
                  onClick={() => setSelectedCategory(category.id)}
                  className={`w-full flex items-center space-x-2 px-3 py-2 rounded-lg text-sm transition-colors ${
                    selectedCategory === category.id
                      ? 'bg-blue-100 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300'
                      : 'text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700'
                  }`}
                >
                  {category.icon}
                  <span>{category.label}</span>
                </button>
              ))}
            </div>

            {/* Examples List */}
            <div className="space-y-2">
              {filteredExamples.map(example => (
                <button
                  key={example.id}
                  onClick={() => handleExampleSelect(example)}
                  className={`w-full text-left p-3 rounded-lg transition-colors ${
                    selectedExample === example.id
                      ? 'bg-blue-100 dark:bg-blue-900/50 border-2 border-blue-200 dark:border-blue-700'
                      : 'bg-slate-50 dark:bg-slate-700 hover:bg-slate-100 dark:hover:bg-slate-600'
                  }`}
                >
                  <div className="font-medium text-slate-900 dark:text-white text-sm">
                    {example.title}
                  </div>
                  <div className="text-xs text-slate-600 dark:text-slate-400 mt-1">
                    {example.description}
                  </div>
                  <div className="flex items-center justify-between mt-2">
                    <span className={`text-xs px-2 py-1 rounded-full ${
                      example.category === 'beginner' 
                        ? 'bg-green-100 dark:bg-green-900/50 text-green-700 dark:text-green-300'
                        : example.category === 'intermediate'
                        ? 'bg-yellow-100 dark:bg-yellow-900/50 text-yellow-700 dark:text-yellow-300'
                        : 'bg-red-100 dark:bg-red-900/50 text-red-700 dark:text-red-300'
                    }`}>
                      {example.category}
                    </span>
                    <Play size={12} className="text-blue-600 dark:text-blue-400" />
                  </div>
                </button>
              ))}
            </div>
          </div>

          {/* Help Section */}
          <div className="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-4">
            <h4 className="font-medium text-blue-900 dark:text-blue-100 mb-2">
              Need Help?
            </h4>
            <div className="space-y-2 text-sm">
              <Link 
                to="/tutorials/beginner/01-hello-world"
                className="block text-blue-700 dark:text-blue-300 hover:underline"
              >
                → Start with Hello World tutorial
              </Link>
              <Link 
                to="/api/vibez"
                className="block text-blue-700 dark:text-blue-300 hover:underline"
              >
                → Check the API reference
              </Link>
              <Link 
                to="/community"
                className="block text-blue-700 dark:text-blue-300 hover:underline"
              >
                → Join our Discord community
              </Link>
            </div>
          </div>
        </div>

        {/* Main Editor */}
        <div className="lg:col-span-3">
          <div className="bg-white dark:bg-slate-800 rounded-lg shadow overflow-hidden h-[600px]">
            <PlaygroundEditor
              initialCode={code}
              language="cursed"
              onRun={(runCode) => {
                console.log('Running code:', runCode);
              }}
            />
          </div>
        </div>
      </div>

      {/* Share Modal */}
      {isShareModalOpen && (
        <div className="fixed top-4 right-4 bg-green-600 text-white px-6 py-3 rounded-lg shadow-lg z-50">
          <div className="flex items-center space-x-2">
            <Share size={16} />
            <span>Share link copied to clipboard!</span>
          </div>
        </div>
      )}

      {/* Quick Tips */}
      <div className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-6">
        <h3 className="font-semibold text-slate-900 dark:text-white mb-4">
          Playground Tips
        </h3>
        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4 text-sm">
          <div className="space-y-2">
            <div className="font-medium text-slate-900 dark:text-white">Keyboard Shortcuts</div>
            <div className="text-slate-600 dark:text-slate-400">
              <div>Ctrl+Enter: Run code</div>
              <div>Ctrl+S: Save file</div>
              <div>F11: Fullscreen</div>
            </div>
          </div>
          <div className="space-y-2">
            <div className="font-medium text-slate-900 dark:text-white">Code Features</div>
            <div className="text-slate-600 dark:text-slate-400">
              <div>Auto-completion</div>
              <div>Syntax highlighting</div>
              <div>Error checking</div>
            </div>
          </div>
          <div className="space-y-2">
            <div className="font-medium text-slate-900 dark:text-white">Sharing</div>
            <div className="text-slate-600 dark:text-slate-400">
              <div>Generate shareable links</div>
              <div>Export as .csd files</div>
              <div>Import from files</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
