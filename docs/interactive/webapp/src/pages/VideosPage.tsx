import React from 'react';
import { Link } from 'react-router-dom';
import { Play, Clock, User, ExternalLink } from 'lucide-react';

export function VideosPage() {
  const videoSeries = [
    {
      id: 'getting-started',
      title: 'Getting Started Series',
      description: 'Perfect for absolute beginners to CURSED programming',
      videos: [
        {
          id: '01-welcome',
          title: 'Welcome to CURSED',
          duration: '5:30',
          thumbnail: '/thumbnails/01-welcome.jpg',
          description: 'Introduction to CURSED and installation guide',
        },
        {
          id: '02-syntax',
          title: 'CURSED Syntax Fundamentals',
          duration: '12:15',
          thumbnail: '/thumbnails/02-syntax.jpg',
          description: 'Learn the basic syntax and language features',
        },
        {
          id: '03-data-types',
          title: 'Working with Data Types',
          duration: '15:45',
          thumbnail: '/thumbnails/03-data-types.jpg',
          description: 'Understand variables, types, and data structures',
        },
      ],
    },
    {
      id: 'building-apps',
      title: 'Building Applications',
      description: 'Intermediate content for building real applications',
      videos: [
        {
          id: '04-error-handling',
          title: 'Error Handling Like a Pro',
          duration: '18:20',
          thumbnail: '/thumbnails/04-error-handling.jpg',
          description: 'Master the yikes/fam/shook error system',
        },
        {
          id: '05-concurrency',
          title: 'Concurrency with Goroutines',
          duration: '22:10',
          thumbnail: '/thumbnails/05-concurrency.jpg',
          description: 'Learn goroutines, channels, and select statements',
        },
        {
          id: '06-rest-api',
          title: 'Building a REST API',
          duration: '28:30',
          thumbnail: '/thumbnails/06-rest-api.jpg',
          description: 'Create a complete web service from scratch',
        },
      ],
    },
  ];

  const quickTips = [
    {
      id: 'vscode-setup',
      title: 'Setting up VS Code for CURSED',
      duration: '3:15',
      category: 'Development Environment',
    },
    {
      id: 'debugging-demo',
      title: 'Debugging CURSED Programs',
      duration: '4:20',
      category: 'Development Environment',
    },
    {
      id: 'builder-pattern',
      title: 'Implementing Builder Pattern',
      duration: '5:10',
      category: 'Code Patterns',
    },
    {
      id: 'worker-pool',
      title: 'Worker Pool Concurrency',
      duration: '4:45',
      category: 'Code Patterns',
    },
  ];

  return (
    <div className="max-w-7xl mx-auto space-y-8">
      <div className="text-center space-y-4">
        <h1 className="text-4xl font-bold text-slate-900 dark:text-white">
          Video Tutorials
        </h1>
        <p className="text-xl text-slate-600 dark:text-slate-300 max-w-3xl mx-auto">
          Learn CURSED through high-quality video tutorials, screencasts, and 
          educational content from experts.
        </p>
      </div>

      {/* Featured Video */}
      <div className="bg-gradient-to-r from-blue-600 to-purple-600 rounded-lg overflow-hidden">
        <div className="p-8 text-white">
          <div className="flex items-center space-x-2 mb-4">
            <Play size={20} />
            <span className="text-sm font-medium">FEATURED</span>
          </div>
          <h2 className="text-3xl font-bold mb-2">
            Building Your First CURSED Application
          </h2>
          <p className="text-blue-100 mb-6">
            A comprehensive 45-minute workshop covering everything from setup to deployment.
          </p>
          <div className="flex items-center space-x-6">
            <button className="bg-white text-blue-600 px-6 py-3 rounded-lg font-medium hover:bg-blue-50 transition-colors">
              Watch Now
            </button>
            <div className="flex items-center space-x-4 text-blue-100">
              <div className="flex items-center space-x-1">
                <Clock size={16} />
                <span>45:30</span>
              </div>
              <div className="flex items-center space-x-1">
                <User size={16} />
                <span>CURSED Team</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Video Series */}
      {videoSeries.map((series) => (
        <div key={series.id} className="space-y-6">
          <div className="space-y-2">
            <h2 className="text-2xl font-bold text-slate-900 dark:text-white">
              {series.title}
            </h2>
            <p className="text-slate-600 dark:text-slate-300">
              {series.description}
            </p>
          </div>

          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
            {series.videos.map((video) => (
              <div
                key={video.id}
                className="bg-white dark:bg-slate-800 rounded-lg shadow hover:shadow-lg transition-shadow overflow-hidden"
              >
                {/* Video Thumbnail */}
                <div className="relative aspect-video bg-slate-200 dark:bg-slate-700">
                  <div className="absolute inset-0 flex items-center justify-center">
                    <Play size={48} className="text-slate-400" />
                  </div>
                  <div className="absolute bottom-2 right-2 bg-black/75 text-white text-xs px-2 py-1 rounded">
                    {video.duration}
                  </div>
                </div>

                {/* Video Info */}
                <div className="p-4 space-y-3">
                  <h3 className="font-semibold text-slate-900 dark:text-white">
                    {video.title}
                  </h3>
                  <p className="text-sm text-slate-600 dark:text-slate-400">
                    {video.description}
                  </p>

                  <div className="flex items-center justify-between">
                    <div className="flex items-center space-x-1 text-xs text-slate-500">
                      <Clock size={12} />
                      <span>{video.duration}</span>
                    </div>
                    <button className="flex items-center space-x-1 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 text-sm">
                      <Play size={14} />
                      <span>Watch</span>
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      ))}

      {/* Quick Tips */}
      <div className="space-y-6">
        <div className="space-y-2">
          <h2 className="text-2xl font-bold text-slate-900 dark:text-white">
            Quick Tips & Screencasts
          </h2>
          <p className="text-slate-600 dark:text-slate-300">
            Short, focused videos on specific topics (2-5 minutes each).
          </p>
        </div>

        <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-4">
          {quickTips.map((tip) => (
            <div
              key={tip.id}
              className="bg-white dark:bg-slate-800 rounded-lg shadow hover:shadow-md transition-shadow p-4"
            >
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <div className="w-8 h-8 bg-blue-100 dark:bg-blue-900/50 rounded-lg flex items-center justify-center">
                    <Play size={16} className="text-blue-600 dark:text-blue-400" />
                  </div>
                  <span className="text-xs text-slate-500">{tip.duration}</span>
                </div>

                <div className="space-y-2">
                  <h3 className="font-medium text-slate-900 dark:text-white text-sm">
                    {tip.title}
                  </h3>
                  <span className="text-xs text-blue-600 dark:text-blue-400">
                    {tip.category}
                  </span>
                </div>

                <button className="w-full text-left text-xs text-slate-600 dark:text-slate-400 hover:text-blue-600 dark:hover:text-blue-400">
                  Watch now →
                </button>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Community Content */}
      <div className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-8">
        <div className="text-center space-y-6">
          <h2 className="text-2xl font-bold text-slate-900 dark:text-white">
            Community Content
          </h2>
          <p className="text-slate-600 dark:text-slate-300 max-w-2xl mx-auto">
            Discover content created by the CURSED community, including conference talks, 
            tutorials, and live coding sessions.
          </p>

          <div className="grid md:grid-cols-3 gap-6">
            <div className="bg-white dark:bg-slate-800 rounded-lg p-6 text-center">
              <h3 className="font-semibold text-slate-900 dark:text-white mb-2">
                Conference Talks
              </h3>
              <p className="text-sm text-slate-600 dark:text-slate-400 mb-4">
                Presentations from CURSEDCon and other events
              </p>
              <Link
                to="/videos/conference-talks"
                className="inline-flex items-center space-x-1 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 text-sm"
              >
                <span>View talks</span>
                <ExternalLink size={14} />
              </Link>
            </div>

            <div className="bg-white dark:bg-slate-800 rounded-lg p-6 text-center">
              <h3 className="font-semibold text-slate-900 dark:text-white mb-2">
                Live Streams
              </h3>
              <p className="text-sm text-slate-600 dark:text-slate-400 mb-4">
                Weekly live coding and Q&A sessions
              </p>
              <Link
                to="/videos/live-streams"
                className="inline-flex items-center space-x-1 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 text-sm"
              >
                <span>Join stream</span>
                <ExternalLink size={14} />
              </Link>
            </div>

            <div className="bg-white dark:bg-slate-800 rounded-lg p-6 text-center">
              <h3 className="font-semibold text-slate-900 dark:text-white mb-2">
                Community Tutorials
              </h3>
              <p className="text-sm text-slate-600 dark:text-slate-400 mb-4">
                Tutorials created by community members
              </p>
              <Link
                to="/videos/community"
                className="inline-flex items-center space-x-1 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 text-sm"
              >
                <span>Browse videos</span>
                <ExternalLink size={14} />
              </Link>
            </div>
          </div>
        </div>
      </div>

      {/* Subscribe Section */}
      <div className="bg-gradient-to-r from-green-50 to-blue-50 dark:from-green-900/20 dark:to-blue-900/20 rounded-lg p-8 text-center">
        <h2 className="text-2xl font-bold text-slate-900 dark:text-white mb-4">
          Stay Updated
        </h2>
        <p className="text-slate-600 dark:text-slate-300 mb-6">
          New videos published every Tuesday and Thursday. 
          Subscribe to get notified of new content!
        </p>
        <div className="flex flex-wrap justify-center gap-4">
          <button className="bg-red-600 hover:bg-red-700 text-white px-6 py-3 rounded-lg">
            Subscribe on YouTube
          </button>
          <Link
            to="/newsletter"
            className="bg-slate-100 hover:bg-slate-200 dark:bg-slate-700 dark:hover:bg-slate-600 text-slate-900 dark:text-white px-6 py-3 rounded-lg"
          >
            Email Newsletter
          </Link>
        </div>
      </div>
    </div>
  );
}
