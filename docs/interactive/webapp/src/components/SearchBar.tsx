import React, { useState, useRef, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { Search, X, Book, Code, Map, Lightbulb, FileText } from 'lucide-react';
import { useSearch } from '../contexts/SearchContext';

const getTypeIcon = (type: string) => {
  switch (type) {
    case 'tutorial':
      return <Book size={16} />;
    case 'api':
      return <Code size={16} />;
    case 'migration':
      return <Map size={16} />;
    case 'pattern':
      return <Lightbulb size={16} />;
    default:
      return <FileText size={16} />;
  }
};

const getTypeColor = (type: string) => {
  switch (type) {
    case 'tutorial':
      return 'text-green-600 dark:text-green-400';
    case 'api':
      return 'text-blue-600 dark:text-blue-400';
    case 'migration':
      return 'text-purple-600 dark:text-purple-400';
    case 'pattern':
      return 'text-yellow-600 dark:text-yellow-400';
    default:
      return 'text-slate-600 dark:text-slate-400';
  }
};

export function SearchBar() {
  const { query, results, isLoading, search, clearSearch } = useSearch();
  const [isOpen, setIsOpen] = useState(false);
  const [localQuery, setLocalQuery] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);
  const resultsRef = useRef<HTMLDivElement>(null);

  // Handle input changes with debouncing
  useEffect(() => {
    const timer = setTimeout(() => {
      if (localQuery.trim()) {
        search(localQuery);
        setIsOpen(true);
      } else {
        clearSearch();
        setIsOpen(false);
      }
    }, 300);

    return () => clearTimeout(timer);
  }, [localQuery, search, clearSearch]);

  // Close results when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        resultsRef.current &&
        !resultsRef.current.contains(event.target as Node) &&
        !inputRef.current?.contains(event.target as Node)
      ) {
        setIsOpen(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  // Handle keyboard navigation
  const handleKeyDown = (event: React.KeyboardEvent) => {
    if (event.key === 'Escape') {
      setIsOpen(false);
      inputRef.current?.blur();
    }
  };

  const handleClear = () => {
    setLocalQuery('');
    clearSearch();
    setIsOpen(false);
    inputRef.current?.focus();
  };

  const handleResultClick = () => {
    setIsOpen(false);
    setLocalQuery('');
    clearSearch();
  };

  return (
    <div className="relative w-full max-w-md">
      {/* Search Input */}
      <div className="relative">
        <div className="absolute left-3 top-1/2 transform -translate-y-1/2 text-slate-400">
          <Search size={18} />
        </div>
        <input
          ref={inputRef}
          type="text"
          placeholder="Search documentation..."
          value={localQuery}
          onChange={(e) => setLocalQuery(e.target.value)}
          onKeyDown={handleKeyDown}
          onFocus={() => localQuery.trim() && setIsOpen(true)}
          className="w-full pl-10 pr-10 py-2 bg-slate-100 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-slate-900 dark:text-white placeholder-slate-500 dark:placeholder-slate-400"
        />
        {localQuery && (
          <button
            onClick={handleClear}
            className="absolute right-3 top-1/2 transform -translate-y-1/2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-300"
          >
            <X size={18} />
          </button>
        )}
      </div>

      {/* Search Results */}
      {isOpen && (localQuery.trim() || results.length > 0) && (
        <div
          ref={resultsRef}
          className="absolute top-full left-0 right-0 mt-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg shadow-lg max-h-96 overflow-y-auto z-50"
        >
          {isLoading ? (
            <div className="p-4 text-center text-slate-500 dark:text-slate-400">
              <div className="animate-spin inline-block w-4 h-4 border-2 border-slate-300 border-t-blue-500 rounded-full mr-2"></div>
              Searching...
            </div>
          ) : results.length > 0 ? (
            <div className="py-2">
              {results.map((result, index) => (
                <Link
                  key={result.item.id}
                  to={result.item.url}
                  onClick={handleResultClick}
                  className="block px-4 py-3 hover:bg-slate-50 dark:hover:bg-slate-700 border-b border-slate-100 dark:border-slate-700 last:border-b-0"
                >
                  <div className="flex items-start space-x-3">
                    <div className={`mt-1 ${getTypeColor(result.item.type)}`}>
                      {getTypeIcon(result.item.type)}
                    </div>
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center space-x-2">
                        <h4 className="text-sm font-medium text-slate-900 dark:text-white truncate">
                          {result.item.title}
                        </h4>
                        <span className={`text-xs px-2 py-1 rounded-full bg-slate-100 dark:bg-slate-700 ${getTypeColor(result.item.type)} capitalize`}>
                          {result.item.type}
                        </span>
                      </div>
                      <p className="text-xs text-slate-600 dark:text-slate-400 mt-1 line-clamp-2">
                        {result.item.content.length > 100
                          ? `${result.item.content.substring(0, 100)}...`
                          : result.item.content}
                      </p>
                    </div>
                  </div>
                </Link>
              ))}
            </div>
          ) : localQuery.trim() ? (
            <div className="p-4 text-center text-slate-500 dark:text-slate-400">
              <div className="mb-2">No results found for "{localQuery}"</div>
              <div className="text-xs">
                Try different keywords or browse the{' '}
                <Link to="/tutorials" className="text-blue-600 dark:text-blue-400 hover:underline">
                  tutorials
                </Link>{' '}
                or{' '}
                <Link to="/api" className="text-blue-600 dark:text-blue-400 hover:underline">
                  API documentation
                </Link>
              </div>
            </div>
          ) : null}

          {/* Search suggestions when no query */}
          {!localQuery.trim() && (
            <div className="p-4">
              <h4 className="text-sm font-medium text-slate-900 dark:text-white mb-3">
                Popular searches
              </h4>
              <div className="space-y-2">
                {[
                  { query: 'hello world', type: 'tutorial' },
                  { query: 'concurrency', type: 'tutorial' },
                  { query: 'error handling', type: 'pattern' },
                  { query: 'vibez.spill', type: 'api' },
                  { query: 'python migration', type: 'migration' },
                ].map((suggestion) => (
                  <button
                    key={suggestion.query}
                    onClick={() => {
                      setLocalQuery(suggestion.query);
                      search(suggestion.query);
                    }}
                    className="flex items-center space-x-2 text-sm text-slate-600 dark:text-slate-400 hover:text-blue-600 dark:hover:text-blue-400 w-full text-left"
                  >
                    <Search size={14} />
                    <span>{suggestion.query}</span>
                    <span className={`text-xs ${getTypeColor(suggestion.type)}`}>
                      {suggestion.type}
                    </span>
                  </button>
                ))}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
