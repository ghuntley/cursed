import React from 'react';
import { Link } from 'react-router-dom';
import { ArrowRight, Code, Book } from 'lucide-react';

export function MigrationPage() {
  const migrationGuides = [
    {
      id: 'python-to-cursed',
      title: 'Python to CURSED',
      description: 'Transition from Python with syntax comparisons and examples',
      difficulty: 'Medium',
      estimatedTime: '2-3 weeks',
      icon: '🐍',
    },
    {
      id: 'go-to-cursed',
      title: 'Go to CURSED',
      description: 'Learn CURSED coming from Go background',
      difficulty: 'Easy',
      estimatedTime: '1-2 weeks',
      icon: '🐹',
    },
    {
      id: 'rust-to-cursed',
      title: 'Rust to CURSED',
      description: 'Migration guide for Rust developers',
      difficulty: 'Easy',
      estimatedTime: '1-2 weeks',
      icon: '🦀',
    },
    {
      id: 'javascript-to-cursed',
      title: 'JavaScript to CURSED',
      description: 'From dynamic to static typing with CURSED',
      difficulty: 'Medium',
      estimatedTime: '2-3 weeks',
      icon: '🟨',
    },
  ];

  return (
    <div className="max-w-6xl mx-auto space-y-8">
      <div className="text-center space-y-4">
        <h1 className="text-4xl font-bold text-slate-900 dark:text-white">
          Migration Guides
        </h1>
        <p className="text-xl text-slate-600 dark:text-slate-300 max-w-3xl mx-auto">
          Transition to CURSED from your current programming language with our 
          comprehensive migration guides.
        </p>
      </div>

      <div className="grid md:grid-cols-2 gap-6">
        {migrationGuides.map((guide) => (
          <Link
            key={guide.id}
            to={`/migration/${guide.id}`}
            className="block bg-white dark:bg-slate-800 rounded-lg shadow hover:shadow-lg transition-shadow p-6"
          >
            <div className="flex items-start space-x-4">
              <div className="text-3xl">{guide.icon}</div>
              <div className="flex-1">
                <h3 className="text-xl font-semibold text-slate-900 dark:text-white mb-2">
                  {guide.title}
                </h3>
                <p className="text-slate-600 dark:text-slate-300 mb-4">
                  {guide.description}
                </p>
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-4 text-sm text-slate-500">
                    <span>{guide.difficulty}</span>
                    <span>•</span>
                    <span>{guide.estimatedTime}</span>
                  </div>
                  <ArrowRight size={16} className="text-blue-600 dark:text-blue-400" />
                </div>
              </div>
            </div>
          </Link>
        ))}
      </div>

      <div className="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-8 text-center">
        <h2 className="text-2xl font-bold text-slate-900 dark:text-white mb-4">
          Don't see your language?
        </h2>
        <p className="text-slate-600 dark:text-slate-300 mb-6">
          We're constantly adding new migration guides. Check out our general 
          getting started guide or request a specific language guide.
        </p>
        <div className="flex flex-wrap justify-center gap-4">
          <Link
            to="/tutorials/beginner/01-hello-world"
            className="inline-flex items-center space-x-2 bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg"
          >
            <Book size={16} />
            <span>General Tutorial</span>
          </Link>
          <Link
            to="/community"
            className="inline-flex items-center space-x-2 bg-slate-100 hover:bg-slate-200 dark:bg-slate-700 dark:hover:bg-slate-600 text-slate-900 dark:text-white px-6 py-3 rounded-lg"
          >
            <Code size={16} />
            <span>Request Guide</span>
          </Link>
        </div>
      </div>
    </div>
  );
}
