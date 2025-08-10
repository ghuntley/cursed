import React, { useState } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { 
  Book, 
  Play, 
  Code, 
  Map, 
  ArrowRight, 
  Lightbulb, 
  Video, 
  Users,
  Search,
  Moon,
  Sun,
  Menu,
  X
} from 'lucide-react';
import { SearchBar } from './SearchBar';
import { useTheme } from '../contexts/ThemeContext';

const navigationItems = [
  { path: '/tutorials', label: 'Tutorials', icon: Book },
  { path: '/playground', label: 'Playground', icon: Play },
  { path: '/api', label: 'API Docs', icon: Code },
  { path: '/pathways', label: 'Learning Paths', icon: Map },
  { path: '/migration', label: 'Migration', icon: ArrowRight },
  { path: '/patterns', label: 'Best Practices', icon: Lightbulb },
  { path: '/videos', label: 'Videos', icon: Video },
  { path: '/community', label: 'Community', icon: Users },
];

export function Navigation() {
  const location = useLocation();
  const { theme, toggleTheme } = useTheme();
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  const isActive = (path: string) => location.pathname.startsWith(path);

  return (
    <nav className="bg-white/90 dark:bg-slate-900/90 backdrop-blur-sm border-b border-slate-200 dark:border-slate-700 sticky top-0 z-50">
      <div className="container mx-auto px-4">
        <div className="flex items-center justify-between h-16">
          {/* Logo */}
          <Link 
            to="/" 
            className="flex items-center space-x-2 text-xl font-bold text-slate-900 dark:text-white"
          >
            <div className="w-8 h-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center">
              <span className="text-white font-bold text-sm">C</span>
            </div>
            <span>CURSED Docs</span>
          </Link>

          {/* Desktop Navigation */}
          <div className="hidden lg:flex items-center space-x-1">
            {navigationItems.map(({ path, label, icon: Icon }) => (
              <Link
                key={path}
                to={path}
                className={`px-3 py-2 rounded-lg text-sm font-medium transition-colors duration-200 flex items-center space-x-1 ${
                  isActive(path)
                    ? 'bg-blue-100 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300'
                    : 'text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800'
                }`}
              >
                <Icon size={16} />
                <span>{label}</span>
              </Link>
            ))}
          </div>

          {/* Right Side */}
          <div className="flex items-center space-x-4">
            {/* Search */}
            <div className="hidden md:block">
              <SearchBar />
            </div>

            {/* Theme Toggle */}
            <button
              onClick={toggleTheme}
              className="p-2 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
            >
              {theme === 'dark' ? <Sun size={20} /> : <Moon size={20} />}
            </button>

            {/* Mobile Menu Toggle */}
            <button
              onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
              className="lg:hidden p-2 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
            >
              {isMobileMenuOpen ? <X size={20} /> : <Menu size={20} />}
            </button>
          </div>
        </div>

        {/* Mobile Menu */}
        {isMobileMenuOpen && (
          <div className="lg:hidden py-4 border-t border-slate-200 dark:border-slate-700">
            <div className="mb-4">
              <SearchBar />
            </div>
            <div className="space-y-2">
              {navigationItems.map(({ path, label, icon: Icon }) => (
                <Link
                  key={path}
                  to={path}
                  onClick={() => setIsMobileMenuOpen(false)}
                  className={`flex items-center space-x-2 px-3 py-2 rounded-lg text-sm font-medium transition-colors duration-200 ${
                    isActive(path)
                      ? 'bg-blue-100 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300'
                      : 'text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800'
                  }`}
                >
                  <Icon size={16} />
                  <span>{label}</span>
                </Link>
              ))}
            </div>
          </div>
        )}
      </div>
    </nav>
  );
}
