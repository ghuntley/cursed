import React, { createContext, useContext, useState, useCallback, useEffect } from 'react';
import Fuse from 'fuse.js';

interface SearchDocument {
  id: string;
  title: string;
  content: string;
  url: string;
  type: 'tutorial' | 'api' | 'migration' | 'pattern' | 'documentation';
}

interface SearchResult {
  item: SearchDocument;
  score?: number;
  matches?: any[];
}

interface SearchContextType {
  query: string;
  results: SearchResult[];
  isLoading: boolean;
  search: (query: string) => void;
  clearSearch: () => void;
}

const SearchContext = createContext<SearchContextType | undefined>(undefined);

const fuseOptions = {
  keys: [
    { name: 'title', weight: 2 },
    { name: 'content', weight: 1 },
  ],
  threshold: 0.3,
  includeScore: true,
  includeMatches: true,
  minMatchCharLength: 2,
};

export function SearchProvider({ children }: { children: React.ReactNode }) {
  const [documents, setDocuments] = useState<SearchDocument[]>([]);
  const [fuse, setFuse] = useState<Fuse<SearchDocument> | null>(null);
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResult[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  // Load search index on mount
  useEffect(() => {
    const loadSearchIndex = async () => {
      try {
        const response = await fetch('/search-index.json');
        const data = await response.json();
        setDocuments(data.documents);
        setFuse(new Fuse(data.documents, fuseOptions));
      } catch (error) {
        console.error('Failed to load search index:', error);
        // Fallback to empty documents
        setDocuments([]);
        setFuse(new Fuse([], fuseOptions));
      }
    };

    loadSearchIndex();
  }, []);

  const search = useCallback((searchQuery: string) => {
    setQuery(searchQuery);
    setIsLoading(true);

    if (!fuse || !searchQuery.trim()) {
      setResults([]);
      setIsLoading(false);
      return;
    }

    // Simulate slight delay for better UX
    setTimeout(() => {
      const searchResults = fuse.search(searchQuery, { limit: 20 });
      setResults(searchResults);
      setIsLoading(false);
    }, 100);
  }, [fuse]);

  const clearSearch = useCallback(() => {
    setQuery('');
    setResults([]);
  }, []);

  const value: SearchContextType = {
    query,
    results,
    isLoading,
    search,
    clearSearch,
  };

  return (
    <SearchContext.Provider value={value}>
      {children}
    </SearchContext.Provider>
  );
}

export function useSearch() {
  const context = useContext(SearchContext);
  if (context === undefined) {
    throw new Error('useSearch must be used within a SearchProvider');
  }
  return context;
}
