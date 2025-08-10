import React from 'react';
import { Link } from 'react-router-dom';
import { Lightbulb, ArrowRight, Code, Book } from 'lucide-react';

export function PatternsPage() {
  const patterns = [
    {
      id: 'common-patterns',
      title: 'Common Design Patterns',
      description: 'Essential patterns every CURSED developer should know',
      category: 'fundamentals',
      difficulty: 'Beginner',
      topics: ['Error Handling', 'Resource Management', 'Data Processing'],
    },
    {
      id: 'concurrency-patterns',
      title: 'Concurrency Patterns',
      description: 'Patterns for concurrent and parallel programming',
      category: 'concurrency',
      difficulty: 'Intermediate',
      topics: ['Worker Pools', 'Pipeline', 'Fan-out/Fan-in'],
    },
    {
      id: 'performance-patterns',
      title: 'Performance Optimization',
      description: 'Patterns for writing high-performance CURSED code',
      category: 'performance',
      difficulty: 'Advanced',
      topics: ['Memory Pools', 'Lazy Loading', 'Batch Processing'],
    },
    {
      id: 'testing-patterns',
      title: 'Testing Patterns',
      description: 'Best practices for testing CURSED applications',
      category: 'testing',
      difficulty: 'Intermediate',
      topics: ['Table-Driven Tests', 'Mocking', 'Integration Tests'],
    },
  ];

  return (
    <div className="max-w-6xl mx-auto space-y-8">
      <div className="text-center space-y-4">
        <h1 className="text-4xl font-bold text-slate-900 dark:text-white">
          Design Patterns & Best Practices
        </h1>
        <p className="text-xl text-slate-600 dark:text-slate-300 max-w-3xl mx-auto">
          Learn proven patterns and best practices for writing maintainable, 
          efficient CURSED code.
        </p>
      </div>

      <div className="grid md:grid-cols-2 gap-6">
        {patterns.map((pattern) => (
          <Link
            key={pattern.id}
            to={`/patterns/${pattern.id}`}
            className="block bg-white dark:bg-slate-800 rounded-lg shadow hover:shadow-lg transition-shadow p-6"
          >
            <div className="flex items-start space-x-4">
              <Lightbulb size={24} className="text-yellow-600 dark:text-yellow-400 mt-1" />
              <div className="flex-1">
                <div className="flex items-center justify-between mb-2">
                  <h3 className="text-xl font-semibold text-slate-900 dark:text-white">
                    {pattern.title}
                  </h3>
                  <span className={`text-xs px-2 py-1 rounded-full ${
                    pattern.difficulty === 'Beginner' 
                      ? 'bg-green-100 dark:bg-green-900/50 text-green-700 dark:text-green-300'
                      : pattern.difficulty === 'Intermediate'
                      ? 'bg-yellow-100 dark:bg-yellow-900/50 text-yellow-700 dark:text-yellow-300'
                      : 'bg-red-100 dark:bg-red-900/50 text-red-700 dark:text-red-300'
                  }`}>
                    {pattern.difficulty}
                  </span>
                </div>
                
                <p className="text-slate-600 dark:text-slate-300 mb-4">
                  {pattern.description}
                </p>
                
                <div className="space-y-2">
                  <div className="text-sm font-medium text-slate-900 dark:text-white">
                    Topics covered:
                  </div>
                  <div className="flex flex-wrap gap-2">
                    {pattern.topics.map((topic, index) => (
                      <span
                        key={index}
                        className="text-xs bg-slate-100 dark:bg-slate-700 text-slate-700 dark:text-slate-300 px-2 py-1 rounded"
                      >
                        {topic}
                      </span>
                    ))}
                  </div>
                </div>
                
                <div className="flex items-center justify-between mt-4">
                  <span className="text-sm text-slate-500 capitalize">
                    {pattern.category}
                  </span>
                  <ArrowRight size={16} className="text-blue-600 dark:text-blue-400" />
                </div>
              </div>
            </div>
          </Link>
        ))}
      </div>

      <div className="bg-gradient-to-r from-yellow-50 to-orange-50 dark:from-yellow-900/20 dark:to-orange-900/20 rounded-lg p-8 text-center">
        <h2 className="text-2xl font-bold text-slate-900 dark:text-white mb-4">
          Want to contribute a pattern?
        </h2>
        <p className="text-slate-600 dark:text-slate-300 mb-6">
          Share your expertise with the community by contributing new patterns 
          and best practices.
        </p>
        <div className="flex flex-wrap justify-center gap-4">
          <Link
            to="/community/contributing"
            className="inline-flex items-center space-x-2 bg-yellow-600 hover:bg-yellow-700 text-white px-6 py-3 rounded-lg"
          >
            <Code size={16} />
            <span>Contribute Pattern</span>
          </Link>
          <Link
            to="/community"
            className="inline-flex items-center space-x-2 bg-slate-100 hover:bg-slate-200 dark:bg-slate-700 dark:hover:bg-slate-600 text-slate-900 dark:text-white px-6 py-3 rounded-lg"
          >
            <Book size={16} />
            <span>Join Community</span>
          </Link>
        </div>
      </div>
    </div>
  );
}
