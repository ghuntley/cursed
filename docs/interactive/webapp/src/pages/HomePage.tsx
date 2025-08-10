import React from 'react';
import { Link } from 'react-router-dom';
import { 
  Book, 
  Play, 
  Code, 
  Map, 
  ArrowRight, 
  Lightbulb, 
  Video, 
  Users,
  Star,
  Zap,
  Target,
  GitBranch,
  Rocket,
  CheckCircle,
  Clock,
  TrendingUp
} from 'lucide-react';
import { PlaygroundEditor } from '../components/PlaygroundEditor';

const quickStartCode = `yeet "vibez"
yeet "mathz"

# Welcome to CURSED!
sus name tea = "Developer"
sus numbers []drip = [1, 2, 3, 4, 5]

vibez.spill("Hello,", name, "!")
vibez.spill("Sum of numbers:", mathz.sum(numbers))

# Try modifying this code and click Run!`;

const featuredTutorials = [
  {
    title: "Hello World",
    description: "Your first CURSED program",
    level: "Beginner",
    duration: "5 min",
    path: "/tutorials/beginner/01-hello-world",
    icon: <Play size={20} />
  },
  {
    title: "Concurrency Basics", 
    description: "Learn goroutines and channels",
    level: "Intermediate",
    duration: "20 min",
    path: "/tutorials/intermediate/concurrency",
    icon: <GitBranch size={20} />
  },
  {
    title: "Building REST APIs",
    description: "Create web services in CURSED",
    level: "Advanced",
    duration: "45 min", 
    path: "/tutorials/advanced/rest-api",
    icon: <Code size={20} />
  }
];

const learningPaths = [
  {
    title: "Complete Beginner",
    description: "New to programming? Start here!",
    lessons: 24,
    duration: "4-6 weeks",
    path: "/pathways/complete-beginner",
    color: "bg-green-500"
  },
  {
    title: "Python Developer",
    description: "Transition from Python to CURSED",
    lessons: 18,
    duration: "2-3 weeks",
    path: "/pathways/python-developer", 
    color: "bg-blue-500"
  },
  {
    title: "Systems Programmer",
    description: "High-performance systems development",
    lessons: 32,
    duration: "6-8 weeks",
    path: "/pathways/systems-developer",
    color: "bg-purple-500"
  },
  {
    title: "Web Developer",
    description: "Build web applications with CURSED",
    lessons: 28,
    duration: "5-7 weeks", 
    path: "/pathways/web-developer",
    color: "bg-orange-500"
  }
];

const features = [
  {
    icon: <Play size={24} />,
    title: "Interactive Playground",
    description: "Run CURSED code directly in your browser with real-time feedback and syntax highlighting."
  },
  {
    icon: <Book size={24} />,
    title: "Guided Tutorials",
    description: "Step-by-step learning with interactive examples and hands-on exercises."
  },
  {
    icon: <Map size={24} />,
    title: "Learning Pathways",
    description: "Personalized learning tracks based on your background and goals."
  },
  {
    icon: <Video size={24} />,
    title: "Video Content",
    description: "High-quality video tutorials and screencasts from CURSED experts."
  },
  {
    icon: <Code size={24} />,
    title: "API Reference",
    description: "Comprehensive, searchable documentation with live examples."
  },
  {
    icon: <Users size={24} />,
    title: "Community Support",
    description: "Active community forums, Discord, and mentorship programs."
  }
];

const stats = [
  { label: "Interactive Tutorials", value: "50+", icon: <Book size={20} /> },
  { label: "Code Examples", value: "500+", icon: <Code size={20} /> },
  { label: "Video Hours", value: "40+", icon: <Video size={20} /> },
  { label: "Community Members", value: "10K+", icon: <Users size={20} /> }
];

export function HomePage() {
  return (
    <div className="space-y-16">
      {/* Hero Section */}
      <section className="text-center space-y-8 py-12">
        <div className="space-y-4">
          <h1 className="text-5xl font-bold text-slate-900 dark:text-white">
            Learn <span className="text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-purple-600">CURSED</span>
          </h1>
          <p className="text-xl text-slate-600 dark:text-slate-300 max-w-3xl mx-auto">
            Master the CURSED programming language with interactive tutorials, comprehensive documentation, 
            and a supportive community. From beginner to expert, we'll guide your journey.
          </p>
        </div>

        <div className="flex flex-wrap justify-center gap-4">
          <Link 
            to="/tutorials"
            className="inline-flex items-center space-x-2 bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg font-medium transition-colors"
          >
            <Book size={20} />
            <span>Start Learning</span>
          </Link>
          <Link
            to="/playground"
            className="inline-flex items-center space-x-2 bg-slate-100 hover:bg-slate-200 dark:bg-slate-800 dark:hover:bg-slate-700 text-slate-900 dark:text-white px-6 py-3 rounded-lg font-medium transition-colors"
          >
            <Play size={20} />
            <span>Try Playground</span>
          </Link>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-6 max-w-4xl mx-auto pt-8">
          {stats.map((stat, index) => (
            <div key={index} className="text-center">
              <div className="flex justify-center mb-2 text-blue-600 dark:text-blue-400">
                {stat.icon}
              </div>
              <div className="text-2xl font-bold text-slate-900 dark:text-white">{stat.value}</div>
              <div className="text-sm text-slate-600 dark:text-slate-400">{stat.label}</div>
            </div>
          ))}
        </div>
      </section>

      {/* Interactive Playground Preview */}
      <section className="space-y-6">
        <div className="text-center space-y-2">
          <h2 className="text-3xl font-bold text-slate-900 dark:text-white">Try CURSED Right Now</h2>
          <p className="text-slate-600 dark:text-slate-300">
            No installation required. Start coding immediately in your browser.
          </p>
        </div>
        
        <div className="bg-white dark:bg-slate-800 rounded-lg shadow-lg overflow-hidden">
          <div className="h-96">
            <PlaygroundEditor 
              initialCode={quickStartCode}
              language="cursed"
            />
          </div>
        </div>
        
        <div className="text-center">
          <Link
            to="/playground"
            className="inline-flex items-center space-x-2 text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300 font-medium"
          >
            <span>Open Full Playground</span>
            <ArrowRight size={16} />
          </Link>
        </div>
      </section>

      {/* Featured Tutorials */}
      <section className="space-y-6">
        <div className="text-center space-y-2">
          <h2 className="text-3xl font-bold text-slate-900 dark:text-white">Featured Tutorials</h2>
          <p className="text-slate-600 dark:text-slate-300">
            Popular tutorials to get you started quickly.
          </p>
        </div>

        <div className="grid md:grid-cols-3 gap-6">
          {featuredTutorials.map((tutorial, index) => (
            <Link
              key={index}
              to={tutorial.path}
              className="block bg-white dark:bg-slate-800 rounded-lg shadow-lg hover:shadow-xl transition-shadow p-6 space-y-4"
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center space-x-2 text-blue-600 dark:text-blue-400">
                  {tutorial.icon}
                  <span className="font-medium">{tutorial.level}</span>
                </div>
                <div className="flex items-center space-x-1 text-slate-500 text-sm">
                  <Clock size={14} />
                  <span>{tutorial.duration}</span>
                </div>
              </div>

              <div className="space-y-2">
                <h3 className="text-xl font-semibold text-slate-900 dark:text-white">
                  {tutorial.title}
                </h3>
                <p className="text-slate-600 dark:text-slate-300">
                  {tutorial.description}
                </p>
              </div>

              <div className="flex items-center text-blue-600 dark:text-blue-400 font-medium">
                <span>Start Tutorial</span>
                <ArrowRight size={16} className="ml-1" />
              </div>
            </Link>
          ))}
        </div>

        <div className="text-center">
          <Link
            to="/tutorials"
            className="inline-flex items-center space-x-2 bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg font-medium transition-colors"
          >
            <span>View All Tutorials</span>
            <ArrowRight size={16} />
          </Link>
        </div>
      </section>

      {/* Learning Pathways */}
      <section className="space-y-6">
        <div className="text-center space-y-2">
          <h2 className="text-3xl font-bold text-slate-900 dark:text-white">Choose Your Learning Path</h2>
          <p className="text-slate-600 dark:text-slate-300">
            Structured learning tracks tailored to your background and goals.
          </p>
        </div>

        <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
          {learningPaths.map((path, index) => (
            <Link
              key={index}
              to={path.path}
              className="block bg-white dark:bg-slate-800 rounded-lg shadow-lg hover:shadow-xl transition-shadow overflow-hidden"
            >
              <div className={`h-2 ${path.color}`} />
              <div className="p-6 space-y-4">
                <div className="space-y-2">
                  <h3 className="text-xl font-semibold text-slate-900 dark:text-white">
                    {path.title}
                  </h3>
                  <p className="text-slate-600 dark:text-slate-300 text-sm">
                    {path.description}
                  </p>
                </div>

                <div className="space-y-2 text-sm text-slate-500">
                  <div className="flex items-center justify-between">
                    <span>{path.lessons} lessons</span>
                    <span>{path.duration}</span>
                  </div>
                </div>

                <div className="flex items-center text-blue-600 dark:text-blue-400 font-medium">
                  <span>Start Path</span>
                  <ArrowRight size={16} className="ml-1" />
                </div>
              </div>
            </Link>
          ))}
        </div>

        <div className="text-center">
          <Link
            to="/pathways"
            className="inline-flex items-center space-x-2 bg-slate-100 hover:bg-slate-200 dark:bg-slate-800 dark:hover:bg-slate-700 text-slate-900 dark:text-white px-6 py-3 rounded-lg font-medium transition-colors"
          >
            <span>Browse All Paths</span>
            <ArrowRight size={16} />
          </Link>
        </div>
      </section>

      {/* Features */}
      <section className="space-y-6">
        <div className="text-center space-y-2">
          <h2 className="text-3xl font-bold text-slate-900 dark:text-white">Why Learn with Us?</h2>
          <p className="text-slate-600 dark:text-slate-300">
            The most comprehensive and interactive CURSED learning experience.
          </p>
        </div>

        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
          {features.map((feature, index) => (
            <div key={index} className="text-center space-y-4">
              <div className="flex justify-center">
                <div className="p-3 bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 rounded-lg">
                  {feature.icon}
                </div>
              </div>
              <div className="space-y-2">
                <h3 className="text-xl font-semibold text-slate-900 dark:text-white">
                  {feature.title}
                </h3>
                <p className="text-slate-600 dark:text-slate-300">
                  {feature.description}
                </p>
              </div>
            </div>
          ))}
        </div>
      </section>

      {/* Quick Navigation */}
      <section className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-8">
        <div className="text-center space-y-6">
          <h2 className="text-2xl font-bold text-slate-900 dark:text-white">What are you looking for?</h2>
          
          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-4">
            <Link
              to="/migration/python-to-cursed"
              className="bg-white dark:bg-slate-800 p-4 rounded-lg shadow hover:shadow-md transition-shadow text-center"
            >
              <div className="text-blue-600 dark:text-blue-400 mb-2">
                <ArrowRight size={24} className="mx-auto" />
              </div>
              <div className="font-medium text-slate-900 dark:text-white">Python Migration</div>
              <div className="text-sm text-slate-600 dark:text-slate-400">Coming from Python?</div>
            </Link>

            <Link
              to="/patterns/common-patterns"
              className="bg-white dark:bg-slate-800 p-4 rounded-lg shadow hover:shadow-md transition-shadow text-center"
            >
              <div className="text-blue-600 dark:text-blue-400 mb-2">
                <Lightbulb size={24} className="mx-auto" />
              </div>
              <div className="font-medium text-slate-900 dark:text-white">Best Practices</div>
              <div className="text-sm text-slate-600 dark:text-slate-400">Design patterns</div>
            </Link>

            <Link
              to="/api"
              className="bg-white dark:bg-slate-800 p-4 rounded-lg shadow hover:shadow-md transition-shadow text-center"
            >
              <div className="text-blue-600 dark:text-blue-400 mb-2">
                <Code size={24} className="mx-auto" />
              </div>
              <div className="font-medium text-slate-900 dark:text-white">API Reference</div>
              <div className="text-sm text-slate-600 dark:text-slate-400">Function docs</div>
            </Link>

            <Link
              to="/videos"
              className="bg-white dark:bg-slate-800 p-4 rounded-lg shadow hover:shadow-md transition-shadow text-center"
            >
              <div className="text-blue-600 dark:text-blue-400 mb-2">
                <Video size={24} className="mx-auto" />
              </div>
              <div className="font-medium text-slate-900 dark:text-white">Video Tutorials</div>
              <div className="text-sm text-slate-600 dark:text-slate-400">Watch and learn</div>
            </Link>
          </div>
        </div>
      </section>

      {/* Community */}
      <section className="text-center space-y-6">
        <div className="space-y-2">
          <h2 className="text-3xl font-bold text-slate-900 dark:text-white">Join the Community</h2>
          <p className="text-slate-600 dark:text-slate-300">
            Connect with other CURSED developers, get help, and share your projects.
          </p>
        </div>

        <div className="flex flex-wrap justify-center gap-4">
          <a
            href="https://discord.gg/cursed-lang"
            className="inline-flex items-center space-x-2 bg-purple-600 hover:bg-purple-700 text-white px-6 py-3 rounded-lg font-medium transition-colors"
          >
            <Users size={20} />
            <span>Join Discord</span>
          </a>
          <Link
            to="/community"
            className="inline-flex items-center space-x-2 bg-slate-100 hover:bg-slate-200 dark:bg-slate-800 dark:hover:bg-slate-700 text-slate-900 dark:text-white px-6 py-3 rounded-lg font-medium transition-colors"
          >
            <Lightbulb size={20} />
            <span>Contribute</span>
          </Link>
        </div>
      </section>
    </div>
  );
}
