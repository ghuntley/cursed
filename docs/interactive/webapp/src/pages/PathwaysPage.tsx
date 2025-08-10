import React from 'react';
import { Link } from 'react-router-dom';
import { useProgress } from '../contexts/ProgressContext';

// Placeholder component - you would implement the full pathways page
export function PathwaysPage() {
  const { progress, getCompletionPercentage } = useProgress();

  return (
    <div className="max-w-4xl mx-auto space-y-8">
      <div className="text-center space-y-4">
        <h1 className="text-4xl font-bold text-slate-900 dark:text-white">
          Learning Pathways
        </h1>
        <p className="text-xl text-slate-600 dark:text-slate-300">
          Structured learning tracks tailored to your background and goals.
        </p>
      </div>

      <div className="grid md:grid-cols-2 gap-6">
        <div className="bg-white dark:bg-slate-800 rounded-lg shadow p-6">
          <h2 className="text-2xl font-bold text-slate-900 dark:text-white mb-4">
            Complete Beginner
          </h2>
          <p className="text-slate-600 dark:text-slate-300 mb-4">
            New to programming? Start here with fundamentals.
          </p>
          <div className="space-y-2">
            <div className="flex justify-between text-sm">
              <span>Progress</span>
              <span>{getCompletionPercentage('complete-beginner')}%</span>
            </div>
            <div className="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-2">
              <div 
                className="bg-green-500 h-2 rounded-full"
                style={{ width: `${getCompletionPercentage('complete-beginner')}%` }}
              />
            </div>
          </div>
          <Link
            to="/pathways/complete-beginner"
            className="mt-4 inline-block bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg"
          >
            Continue Learning
          </Link>
        </div>

        <div className="bg-white dark:bg-slate-800 rounded-lg shadow p-6">
          <h2 className="text-2xl font-bold text-slate-900 dark:text-white mb-4">
            Python Developer
          </h2>
          <p className="text-slate-600 dark:text-slate-300 mb-4">
            Transition from Python to CURSED efficiently.
          </p>
          <div className="space-y-2">
            <div className="flex justify-between text-sm">
              <span>Progress</span>
              <span>{getCompletionPercentage('python-developer')}%</span>
            </div>
            <div className="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-2">
              <div 
                className="bg-blue-500 h-2 rounded-full"
                style={{ width: `${getCompletionPercentage('python-developer')}%` }}
              />
            </div>
          </div>
          <Link
            to="/pathways/python-developer"
            className="mt-4 inline-block bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg"
          >
            Start Migration
          </Link>
        </div>
      </div>
    </div>
  );
}
