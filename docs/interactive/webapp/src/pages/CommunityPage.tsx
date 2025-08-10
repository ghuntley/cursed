import React from 'react';
import { Link } from 'react-router-dom';
import { 
  Users, 
  MessageCircle, 
  GitBranch, 
  Heart, 
  ExternalLink, 
  Book,
  Code,
  HelpCircle,
  Star,
  Calendar
} from 'lucide-react';

export function CommunityPage() {
  const communityChannels = [
    {
      name: 'Discord Server',
      description: 'Real-time chat, help, and discussions',
      members: '10,000+',
      icon: <MessageCircle size={24} />,
      link: 'https://discord.gg/cursed-lang',
      color: 'bg-purple-600',
    },
    {
      name: 'GitHub Discussions',
      description: 'Long-form discussions and feature requests',
      members: '2,500+',
      icon: <GitBranch size={24} />,
      link: 'https://github.com/ghuntley/cursed/discussions',
      color: 'bg-gray-800',
    },
    {
      name: 'Reddit Community',
      description: 'News, tutorials, and community projects',
      members: '5,000+',
      icon: <Users size={24} />,
      link: 'https://reddit.com/r/cursedlang',
      color: 'bg-orange-600',
    },
    {
      name: 'Stack Overflow',
      description: 'Technical Q&A and troubleshooting',
      members: '1,200+',
      icon: <HelpCircle size={24} />,
      link: 'https://stackoverflow.com/questions/tagged/cursed-lang',
      color: 'bg-orange-500',
    },
  ];

  const contributionAreas = [
    {
      title: 'Documentation',
      description: 'Help improve tutorials, guides, and API docs',
      icon: <Book size={20} />,
      difficulty: 'Beginner',
      color: 'text-green-600 dark:text-green-400',
    },
    {
      title: 'Code Examples',
      description: 'Add runnable examples and sample projects',
      icon: <Code size={20} />,
      difficulty: 'Beginner',
      color: 'text-green-600 dark:text-green-400',
    },
    {
      title: 'Core Development',
      description: 'Contribute to the CURSED compiler and runtime',
      icon: <GitBranch size={20} />,
      difficulty: 'Advanced',
      color: 'text-red-600 dark:text-red-400',
    },
    {
      title: 'Community Support',
      description: 'Help other users in forums and chat',
      icon: <Heart size={20} />,
      difficulty: 'Beginner',
      color: 'text-green-600 dark:text-green-400',
    },
  ];

  const upcomingEvents = [
    {
      title: 'CURSED Office Hours',
      date: 'Every Wednesday',
      time: '3 PM UTC',
      description: 'Weekly Q&A with the core team',
      type: 'Recurring',
    },
    {
      title: 'Community Showcase',
      date: 'Every Friday',
      time: '8 PM UTC',
      description: 'Show off your CURSED projects',
      type: 'Recurring',
    },
    {
      title: 'CURSEDCon 2025',
      date: 'March 15-16, 2025',
      time: 'All Day',
      description: 'Annual CURSED conference',
      type: 'Conference',
    },
  ];

  return (
    <div className="max-w-7xl mx-auto space-y-8">
      {/* Header */}
      <div className="text-center space-y-4">
        <h1 className="text-4xl font-bold text-slate-900 dark:text-white">
          CURSED Community
        </h1>
        <p className="text-xl text-slate-600 dark:text-slate-300 max-w-3xl mx-auto">
          Join thousands of developers building amazing things with CURSED. 
          Get help, share projects, and contribute to the ecosystem.
        </p>
      </div>

      {/* Community Stats */}
      <div className="bg-gradient-to-r from-blue-600 to-purple-600 rounded-lg p-8 text-white">
        <div className="grid md:grid-cols-4 gap-6 text-center">
          <div>
            <div className="text-3xl font-bold">18,000+</div>
            <div className="text-blue-100">Community Members</div>
          </div>
          <div>
            <div className="text-3xl font-bold">2,500+</div>
            <div className="text-blue-100">GitHub Stars</div>
          </div>
          <div>
            <div className="text-3xl font-bold">450+</div>
            <div className="text-blue-100">Contributors</div>
          </div>
          <div>
            <div className="text-3xl font-bold">120+</div>
            <div className="text-blue-100">Countries</div>
          </div>
        </div>
      </div>

      {/* Community Channels */}
      <div className="space-y-6">
        <h2 className="text-2xl font-bold text-slate-900 dark:text-white">
          Join the Conversation
        </h2>
        <div className="grid md:grid-cols-2 gap-6">
          {communityChannels.map((channel, index) => (
            <a
              key={index}
              href={channel.link}
              target="_blank"
              rel="noopener noreferrer"
              className="block bg-white dark:bg-slate-800 rounded-lg shadow hover:shadow-lg transition-shadow p-6"
            >
              <div className="flex items-start space-x-4">
                <div className={`p-3 ${channel.color} text-white rounded-lg`}>
                  {channel.icon}
                </div>
                <div className="flex-1">
                  <div className="flex items-center justify-between mb-2">
                    <h3 className="text-xl font-semibold text-slate-900 dark:text-white">
                      {channel.name}
                    </h3>
                    <ExternalLink size={16} className="text-slate-400" />
                  </div>
                  <p className="text-slate-600 dark:text-slate-300 mb-2">
                    {channel.description}
                  </p>
                  <div className="text-sm text-slate-500">
                    {channel.members} members
                  </div>
                </div>
              </div>
            </a>
          ))}
        </div>
      </div>

      {/* Contributing */}
      <div className="space-y-6">
        <div className="text-center space-y-2">
          <h2 className="text-2xl font-bold text-slate-900 dark:text-white">
            Ways to Contribute
          </h2>
          <p className="text-slate-600 dark:text-slate-300">
            Help make CURSED better for everyone. Every contribution matters!
          </p>
        </div>

        <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
          {contributionAreas.map((area, index) => (
            <div
              key={index}
              className="bg-white dark:bg-slate-800 rounded-lg shadow p-6 text-center"
            >
              <div className="flex justify-center mb-4">
                <div className="p-3 bg-slate-100 dark:bg-slate-700 rounded-lg">
                  {area.icon}
                </div>
              </div>
              <h3 className="font-semibold text-slate-900 dark:text-white mb-2">
                {area.title}
              </h3>
              <p className="text-sm text-slate-600 dark:text-slate-400 mb-4">
                {area.description}
              </p>
              <span className={`text-xs px-3 py-1 rounded-full ${
                area.difficulty === 'Beginner' 
                  ? 'bg-green-100 dark:bg-green-900/50 text-green-700 dark:text-green-300'
                  : area.difficulty === 'Intermediate'
                  ? 'bg-yellow-100 dark:bg-yellow-900/50 text-yellow-700 dark:text-yellow-300'
                  : 'bg-red-100 dark:bg-red-900/50 text-red-700 dark:text-red-300'
              }`}>
                {area.difficulty}
              </span>
            </div>
          ))}
        </div>

        <div className="text-center">
          <Link
            to="/community/contributing"
            className="inline-flex items-center space-x-2 bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg"
          >
            <GitBranch size={16} />
            <span>Start Contributing</span>
          </Link>
        </div>
      </div>

      {/* Events */}
      <div className="space-y-6">
        <h2 className="text-2xl font-bold text-slate-900 dark:text-white">
          Upcoming Events
        </h2>
        <div className="grid md:grid-cols-3 gap-6">
          {upcomingEvents.map((event, index) => (
            <div
              key={index}
              className="bg-white dark:bg-slate-800 rounded-lg shadow p-6"
            >
              <div className="flex items-start space-x-3">
                <Calendar size={20} className="text-blue-600 dark:text-blue-400 mt-1" />
                <div className="flex-1">
                  <div className="flex items-center justify-between mb-2">
                    <h3 className="font-semibold text-slate-900 dark:text-white">
                      {event.title}
                    </h3>
                    <span className={`text-xs px-2 py-1 rounded-full ${
                      event.type === 'Recurring'
                        ? 'bg-green-100 dark:bg-green-900/50 text-green-700 dark:text-green-300'
                        : 'bg-purple-100 dark:bg-purple-900/50 text-purple-700 dark:text-purple-300'
                    }`}>
                      {event.type}
                    </span>
                  </div>
                  <div className="text-sm text-slate-600 dark:text-slate-400 mb-2">
                    {event.date} • {event.time}
                  </div>
                  <p className="text-sm text-slate-600 dark:text-slate-300">
                    {event.description}
                  </p>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Success Stories */}
      <div className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-8">
        <h2 className="text-2xl font-bold text-slate-900 dark:text-white mb-6 text-center">
          Community Success Stories
        </h2>
        <div className="grid md:grid-cols-3 gap-6">
          <div className="bg-white dark:bg-slate-800 rounded-lg p-6">
            <div className="flex items-center space-x-3 mb-4">
              <div className="w-10 h-10 bg-blue-600 rounded-full flex items-center justify-center text-white font-bold">
                A
              </div>
              <div>
                <div className="font-medium text-slate-900 dark:text-white">Alice Chen</div>
                <div className="text-sm text-slate-600 dark:text-slate-400">Senior Developer</div>
              </div>
            </div>
            <p className="text-slate-600 dark:text-slate-300 text-sm">
              "CURSED helped me build a high-performance microservice that handles 
              100k requests per second. The community was incredibly helpful!"
            </p>
          </div>

          <div className="bg-white dark:bg-slate-800 rounded-lg p-6">
            <div className="flex items-center space-x-3 mb-4">
              <div className="w-10 h-10 bg-green-600 rounded-full flex items-center justify-center text-white font-bold">
                M
              </div>
              <div>
                <div className="font-medium text-slate-900 dark:text-white">Mark Rodriguez</div>
                <div className="text-sm text-slate-600 dark:text-slate-400">Startup Founder</div>
              </div>
            </div>
            <p className="text-slate-600 dark:text-slate-300 text-sm">
              "Migrated our Python backend to CURSED and saw 10x performance 
              improvement. The migration guide made it seamless."
            </p>
          </div>

          <div className="bg-white dark:bg-slate-800 rounded-lg p-6">
            <div className="flex items-center space-x-3 mb-4">
              <div className="w-10 h-10 bg-purple-600 rounded-full flex items-center justify-center text-white font-bold">
                S
              </div>
              <div>
                <div className="font-medium text-slate-900 dark:text-white">Sarah Kim</div>
                <div className="text-sm text-slate-600 dark:text-slate-400">CS Student</div>
              </div>
            </div>
            <p className="text-slate-600 dark:text-slate-300 text-sm">
              "Started learning CURSED as my first systems language. The 
              tutorials and community made it easy to get started!"
            </p>
          </div>
        </div>
      </div>

      {/* Call to Action */}
      <div className="text-center space-y-6 bg-gradient-to-r from-green-50 to-blue-50 dark:from-green-900/20 dark:to-blue-900/20 rounded-lg p-8">
        <h2 className="text-2xl font-bold text-slate-900 dark:text-white">
          Ready to Join the Community?
        </h2>
        <p className="text-slate-600 dark:text-slate-300 max-w-2xl mx-auto">
          Whether you're a beginner or expert, there's a place for you in the 
          CURSED community. Join us and help shape the future of systems programming!
        </p>
        <div className="flex flex-wrap justify-center gap-4">
          <a
            href="https://discord.gg/cursed-lang"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center space-x-2 bg-purple-600 hover:bg-purple-700 text-white px-6 py-3 rounded-lg"
          >
            <MessageCircle size={16} />
            <span>Join Discord</span>
          </a>
          <a
            href="https://github.com/ghuntley/cursed"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center space-x-2 bg-gray-800 hover:bg-gray-900 text-white px-6 py-3 rounded-lg"
          >
            <Star size={16} />
            <span>Star on GitHub</span>
          </a>
          <Link
            to="/community/contributing"
            className="inline-flex items-center space-x-2 bg-slate-100 hover:bg-slate-200 dark:bg-slate-700 dark:hover:bg-slate-600 text-slate-900 dark:text-white px-6 py-3 rounded-lg"
          >
            <Code size={16} />
            <span>Start Contributing</span>
          </Link>
        </div>
      </div>
    </div>
  );
}
