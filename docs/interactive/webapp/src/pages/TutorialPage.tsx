import React from 'react';
import { Routes, Route, Link, useParams, useLocation } from 'react-router-dom';
import { 
  Book, 
  CheckCircle, 
  Clock, 
  Star, 
  ArrowLeft, 
  ArrowRight,
  Play,
  Code,
  Trophy,
  Target
} from 'lucide-react';
import { useProgress } from '../contexts/ProgressContext';

// Mock tutorial data - in real app this would come from API/CMS
const tutorials = {
  beginner: [
    {
      id: '01-hello-world',
      title: 'Hello World',
      description: 'Your first CURSED program',
      duration: '5 min',
      difficulty: 'Beginner',
      content: `# Hello World - Your First CURSED Program

Welcome to CURSED! Let's start with the classic "Hello, World!" program.

## Learning Objectives
- Write your first CURSED program
- Understand basic syntax
- Use the \`vibez\` module for output

## The Program

\`\`\`cursed
yeet "vibez"
vibez.spill("Hello, World!")
\`\`\`

Click "Try in Playground" to run this code!`,
      nextTutorial: '02-variables',
      prevTutorial: null,
    },
    {
      id: '02-variables',
      title: 'Variables and Types',
      description: 'Learn about CURSED data types',
      duration: '10 min',
      difficulty: 'Beginner',
      content: `# Variables and Types

Learn how to store and work with data in CURSED.

## Variable Declaration

\`\`\`cursed
sus name tea = "Alice"
sus age drip = 25
sus height drip = 5.7
sus is_student lit = based
\`\`\`

## Data Types
- \`tea\` - strings
- \`drip\` - numbers  
- \`lit\` - booleans (based/cap)`,
      nextTutorial: '03-functions',
      prevTutorial: '01-hello-world',
    },
  ],
  intermediate: [
    {
      id: 'concurrency',
      title: 'Concurrency with Goroutines',
      description: 'Learn concurrent programming',
      duration: '25 min',
      difficulty: 'Intermediate',
      content: `# Concurrency with Goroutines

Learn how to write concurrent programs in CURSED.

## Goroutines

\`\`\`cursed
yeet "concurrenz"

go {
    vibez.spill("Running in goroutine!")
}
\`\`\``,
      nextTutorial: null,
      prevTutorial: null,
    },
  ],
};

function TutorialNavigation() {
  const { progress } = useProgress();
  
  const allTutorials = [
    ...tutorials.beginner.map(t => ({ ...t, category: 'beginner' })),
    ...tutorials.intermediate.map(t => ({ ...t, category: 'intermediate' })),
  ];

  return (
    <div className="w-64 bg-white dark:bg-slate-800 border-r border-slate-200 dark:border-slate-700 p-4 space-y-6">
      <div>
        <h3 className="font-semibold text-slate-900 dark:text-white mb-3">Beginner</h3>
        <div className="space-y-2">
          {tutorials.beginner.map((tutorial) => {
            const isCompleted = progress.completedTutorials.includes(`beginner/${tutorial.id}`);
            return (
              <Link
                key={tutorial.id}
                to={`/tutorials/beginner/${tutorial.id}`}
                className="block p-3 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors"
              >
                <div className="flex items-center space-x-3">
                  {isCompleted ? (
                    <CheckCircle size={16} className="text-green-600 dark:text-green-400" />
                  ) : (
                    <div className="w-4 h-4 border-2 border-slate-300 dark:border-slate-600 rounded-full" />
                  )}
                  <div className="flex-1">
                    <div className="font-medium text-slate-900 dark:text-white text-sm">
                      {tutorial.title}
                    </div>
                    <div className="text-xs text-slate-600 dark:text-slate-400">
                      {tutorial.duration}
                    </div>
                  </div>
                </div>
              </Link>
            );
          })}
        </div>
      </div>

      <div>
        <h3 className="font-semibold text-slate-900 dark:text-white mb-3">Intermediate</h3>
        <div className="space-y-2">
          {tutorials.intermediate.map((tutorial) => {
            const isCompleted = progress.completedTutorials.includes(`intermediate/${tutorial.id}`);
            return (
              <Link
                key={tutorial.id}
                to={`/tutorials/intermediate/${tutorial.id}`}
                className="block p-3 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors"
              >
                <div className="flex items-center space-x-3">
                  {isCompleted ? (
                    <CheckCircle size={16} className="text-green-600 dark:text-green-400" />
                  ) : (
                    <div className="w-4 h-4 border-2 border-slate-300 dark:border-slate-600 rounded-full" />
                  )}
                  <div className="flex-1">
                    <div className="font-medium text-slate-900 dark:text-white text-sm">
                      {tutorial.title}
                    </div>
                    <div className="text-xs text-slate-600 dark:text-slate-400">
                      {tutorial.duration}
                    </div>
                  </div>
                </div>
              </Link>
            );
          })}
        </div>
      </div>
    </div>
  );
}

function TutorialContent({ category, id }: { category: string; id: string }) {
  const { markTutorialComplete, progress } = useProgress();
  const tutorial = (tutorials as any)[category]?.find((t: any) => t.id === id);
  
  if (!tutorial) {
    return (
      <div className="text-center py-12">
        <h2 className="text-2xl font-bold text-slate-900 dark:text-white mb-4">
          Tutorial Not Found
        </h2>
        <p className="text-slate-600 dark:text-slate-400 mb-6">
          The tutorial you're looking for doesn't exist.
        </p>
        <Link
          to="/tutorials"
          className="inline-flex items-center space-x-2 bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg"
        >
          <ArrowLeft size={16} />
          <span>Back to Tutorials</span>
        </Link>
      </div>
    );
  }

  const tutorialPath = `${category}/${id}`;
  const isCompleted = progress.completedTutorials.includes(tutorialPath);

  const handleComplete = () => {
    markTutorialComplete(tutorialPath);
  };

  const playgroundCode = tutorial.content.match(/```cursed\n([\s\S]*?)\n```/)?.[1] || '';

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      {/* Tutorial Header */}
      <div className="bg-white dark:bg-slate-800 rounded-lg shadow p-6">
        <div className="flex items-start justify-between mb-4">
          <div className="space-y-2">
            <div className="flex items-center space-x-2 text-sm text-slate-600 dark:text-slate-400">
              <span className="capitalize">{category}</span>
              <span>•</span>
              <div className="flex items-center space-x-1">
                <Clock size={14} />
                <span>{tutorial.duration}</span>
              </div>
              <span>•</span>
              <span>{tutorial.difficulty}</span>
            </div>
            <h1 className="text-3xl font-bold text-slate-900 dark:text-white">
              {tutorial.title}
            </h1>
            <p className="text-lg text-slate-600 dark:text-slate-300">
              {tutorial.description}
            </p>
          </div>
          
          <div className="flex items-center space-x-2">
            {isCompleted && (
              <div className="flex items-center space-x-1 text-green-600 dark:text-green-400">
                <CheckCircle size={16} />
                <span className="text-sm">Completed</span>
              </div>
            )}
          </div>
        </div>

        {/* Action Buttons */}
        <div className="flex flex-wrap gap-3">
          <Link
            to={`/playground?code=${btoa(playgroundCode)}`}
            className="inline-flex items-center space-x-2 bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg"
          >
            <Play size={16} />
            <span>Try in Playground</span>
          </Link>
          
          {!isCompleted && (
            <button
              onClick={handleComplete}
              className="inline-flex items-center space-x-2 bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg"
            >
              <CheckCircle size={16} />
              <span>Mark Complete</span>
            </button>
          )}
          
          <Link
            to={`/api`}
            className="inline-flex items-center space-x-2 bg-slate-100 hover:bg-slate-200 dark:bg-slate-700 dark:hover:bg-slate-600 text-slate-900 dark:text-white px-4 py-2 rounded-lg"
          >
            <Code size={16} />
            <span>API Reference</span>
          </Link>
        </div>
      </div>

      {/* Tutorial Content */}
      <div className="bg-white dark:bg-slate-800 rounded-lg shadow p-6">
        <div className="prose dark:prose-invert max-w-none">
          <div dangerouslySetInnerHTML={{ __html: tutorial.content.replace(/```cursed\n([\s\S]*?)\n```/g, 
            '<div class="bg-slate-900 rounded-lg p-4 my-4"><pre><code class="text-green-400">$1</code></pre></div>'
          ) }} />
        </div>
      </div>

      {/* Navigation */}
      <div className="flex justify-between">
        {tutorial.prevTutorial ? (
          <Link
            to={`/tutorials/${category}/${tutorial.prevTutorial}`}
            className="inline-flex items-center space-x-2 bg-slate-100 hover:bg-slate-200 dark:bg-slate-700 dark:hover:bg-slate-600 text-slate-900 dark:text-white px-4 py-2 rounded-lg"
          >
            <ArrowLeft size={16} />
            <span>Previous</span>
          </Link>
        ) : (
          <div />
        )}
        
        {tutorial.nextTutorial ? (
          <Link
            to={`/tutorials/${category}/${tutorial.nextTutorial}`}
            className="inline-flex items-center space-x-2 bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg"
          >
            <span>Next</span>
            <ArrowRight size={16} />
          </Link>
        ) : (
          <div />
        )}
      </div>

      {/* Related Resources */}
      <div className="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-6">
        <h3 className="font-semibold text-blue-900 dark:text-blue-100 mb-4">
          Related Resources
        </h3>
        <div className="grid md:grid-cols-2 gap-4">
          <Link
            to="/patterns/common-patterns"
            className="block p-3 bg-white dark:bg-slate-800 rounded-lg hover:shadow-md transition-shadow"
          >
            <div className="font-medium text-slate-900 dark:text-white">Best Practices</div>
            <div className="text-sm text-slate-600 dark:text-slate-400">Common patterns and idioms</div>
          </Link>
          <Link
            to="/community"
            className="block p-3 bg-white dark:bg-slate-800 rounded-lg hover:shadow-md transition-shadow"
          >
            <div className="font-medium text-slate-900 dark:text-white">Get Help</div>
            <div className="text-sm text-slate-600 dark:text-slate-400">Join our community</div>
          </Link>
        </div>
      </div>
    </div>
  );
}

function TutorialsList() {
  const { progress, getCompletionPercentage } = useProgress();
  
  const categories = [
    {
      id: 'beginner',
      title: 'Beginner Tutorials',
      description: 'Start your CURSED journey with the fundamentals',
      tutorials: tutorials.beginner,
      color: 'bg-green-500',
    },
    {
      id: 'intermediate',
      title: 'Intermediate Tutorials',
      description: 'Advance your skills with more complex topics',
      tutorials: tutorials.intermediate,
      color: 'bg-blue-500',
    },
  ];

  return (
    <div className="max-w-6xl mx-auto space-y-8">
      {/* Header */}
      <div className="text-center space-y-4">
        <h1 className="text-4xl font-bold text-slate-900 dark:text-white">
          CURSED Tutorials
        </h1>
        <p className="text-xl text-slate-600 dark:text-slate-300 max-w-3xl mx-auto">
          Learn CURSED step by step with interactive tutorials and hands-on examples.
        </p>
      </div>

      {/* Progress Overview */}
      <div className="bg-white dark:bg-slate-800 rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-slate-900 dark:text-white mb-4">
          Your Progress
        </h2>
        <div className="grid md:grid-cols-3 gap-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
              {progress.completedTutorials.length}
            </div>
            <div className="text-sm text-slate-600 dark:text-slate-400">Tutorials Completed</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-green-600 dark:text-green-400">
              {progress.achievements.length}
            </div>
            <div className="text-sm text-slate-600 dark:text-slate-400">Achievements Earned</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-purple-600 dark:text-purple-400">
              {Object.values(progress.timeSpent).reduce((total, time) => total + time, 0)}
            </div>
            <div className="text-sm text-slate-600 dark:text-slate-400">Minutes Learned</div>
          </div>
        </div>
      </div>

      {/* Tutorial Categories */}
      {categories.map((category) => (
        <div key={category.id} className="space-y-4">
          <div className="flex items-center space-x-3">
            <div className={`w-4 h-4 ${category.color} rounded-full`} />
            <h2 className="text-2xl font-bold text-slate-900 dark:text-white">
              {category.title}
            </h2>
          </div>
          <p className="text-slate-600 dark:text-slate-300">
            {category.description}
          </p>
          
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
            {category.tutorials.map((tutorial) => {
              const tutorialPath = `${category.id}/${tutorial.id}`;
              const isCompleted = progress.completedTutorials.includes(tutorialPath);
              
              return (
                <Link
                  key={tutorial.id}
                  to={`/tutorials/${category.id}/${tutorial.id}`}
                  className="block bg-white dark:bg-slate-800 rounded-lg shadow hover:shadow-lg transition-shadow p-6"
                >
                  <div className="flex items-start justify-between mb-3">
                    <h3 className="font-semibold text-slate-900 dark:text-white">
                      {tutorial.title}
                    </h3>
                    {isCompleted && (
                      <CheckCircle size={20} className="text-green-600 dark:text-green-400" />
                    )}
                  </div>
                  
                  <p className="text-slate-600 dark:text-slate-300 text-sm mb-4">
                    {tutorial.description}
                  </p>
                  
                  <div className="flex items-center justify-between text-sm">
                    <div className="flex items-center space-x-4 text-slate-500">
                      <div className="flex items-center space-x-1">
                        <Clock size={14} />
                        <span>{tutorial.duration}</span>
                      </div>
                      <span>{tutorial.difficulty}</span>
                    </div>
                    
                    <div className="flex items-center space-x-1 text-blue-600 dark:text-blue-400">
                      <span>Start</span>
                      <ArrowRight size={14} />
                    </div>
                  </div>
                </Link>
              );
            })}
          </div>
        </div>
      ))}

      {/* Call to Action */}
      <div className="text-center space-y-4 bg-gradient-to-r from-blue-50 to-purple-50 dark:from-blue-900/20 dark:to-purple-900/20 rounded-lg p-8">
        <h2 className="text-2xl font-bold text-slate-900 dark:text-white">
          Ready to Get Started?
        </h2>
        <p className="text-slate-600 dark:text-slate-300">
          Begin your CURSED learning journey with our beginner-friendly tutorials.
        </p>
        <div className="flex flex-wrap justify-center gap-4">
          <Link
            to="/tutorials/beginner/01-hello-world"
            className="inline-flex items-center space-x-2 bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg"
          >
            <Play size={16} />
            <span>Start with Hello World</span>
          </Link>
          <Link
            to="/pathways"
            className="inline-flex items-center space-x-2 bg-slate-100 hover:bg-slate-200 dark:bg-slate-700 dark:hover:bg-slate-600 text-slate-900 dark:text-white px-6 py-3 rounded-lg"
          >
            <Target size={16} />
            <span>Choose Learning Path</span>
          </Link>
        </div>
      </div>
    </div>
  );
}

export function TutorialPage() {
  const location = useLocation();
  const isListView = location.pathname === '/tutorials';

  return (
    <div className="min-h-screen">
      {isListView ? (
        <TutorialsList />
      ) : (
        <div className="flex">
          <TutorialNavigation />
          <div className="flex-1 p-6">
            <Routes>
              <Route path=":category/:id" element={<TutorialContentWrapper />} />
            </Routes>
          </div>
        </div>
      )}
    </div>
  );
}

function TutorialContentWrapper() {
  const { category, id } = useParams<{ category: string; id: string }>();
  
  if (!category || !id) {
    return <div>Invalid tutorial URL</div>;
  }
  
  return <TutorialContent category={category} id={id} />;
}
