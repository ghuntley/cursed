import React, { createContext, useContext, useState, useCallback, useEffect } from 'react';

interface ProgressData {
  completedTutorials: string[];
  currentPathway?: string;
  bookmarks: string[];
  preferences: {
    theme: 'light' | 'dark' | 'system';
    codeTheme: 'vs-dark' | 'vs-light';
    fontSize: number;
    notifications: boolean;
  };
  achievements: string[];
  timeSpent: Record<string, number>; // URL -> minutes
}

interface ProgressContextType {
  progress: ProgressData;
  markTutorialComplete: (tutorialId: string) => void;
  setCurrentPathway: (pathwayId: string) => void;
  addBookmark: (url: string) => void;
  removeBookmark: (url: string) => void;
  updatePreferences: (preferences: Partial<ProgressData['preferences']>) => void;
  addTimeSpent: (url: string, minutes: number) => void;
  getCompletionPercentage: (pathwayId: string) => number;
  exportProgress: () => string;
  importProgress: (data: string) => boolean;
}

const ProgressContext = createContext<ProgressContextType | undefined>(undefined);

const defaultProgress: ProgressData = {
  completedTutorials: [],
  bookmarks: [],
  preferences: {
    theme: 'system',
    codeTheme: 'vs-dark',
    fontSize: 14,
    notifications: true,
  },
  achievements: [],
  timeSpent: {},
};

// Pathway definitions with tutorial lists
const pathwayTutorials: Record<string, string[]> = {
  'complete-beginner': [
    'beginner/01-hello-world',
    'beginner/02-variables',
    'beginner/03-functions',
    'beginner/04-control-flow',
    'beginner/05-data-structures',
    'intermediate/01-error-handling',
    'intermediate/02-modules',
    'intermediate/03-testing',
  ],
  'python-developer': [
    'migration/python-syntax',
    'beginner/01-hello-world',
    'intermediate/01-error-handling',
    'intermediate/02-concurrency',
    'advanced/01-performance',
  ],
  'systems-developer': [
    'beginner/01-hello-world',
    'intermediate/02-concurrency',
    'advanced/01-performance',
    'advanced/02-memory-management',
    'advanced/03-ffi',
  ],
  'web-developer': [
    'beginner/01-hello-world',
    'intermediate/01-error-handling',
    'web/01-http-server',
    'web/02-rest-api',
    'web/03-database',
  ],
};

export function ProgressProvider({ children }: { children: React.ReactNode }) {
  const [progress, setProgress] = useState<ProgressData>(defaultProgress);

  // Load progress from localStorage on mount
  useEffect(() => {
    const savedProgress = localStorage.getItem('cursed-docs-progress');
    if (savedProgress) {
      try {
        const parsedProgress = JSON.parse(savedProgress);
        setProgress({ ...defaultProgress, ...parsedProgress });
      } catch (error) {
        console.error('Failed to parse saved progress:', error);
      }
    }
  }, []);

  // Save progress to localStorage whenever it changes
  useEffect(() => {
    localStorage.setItem('cursed-docs-progress', JSON.stringify(progress));
  }, [progress]);

  const markTutorialComplete = useCallback((tutorialId: string) => {
    setProgress(prev => {
      const newCompleted = [...prev.completedTutorials];
      if (!newCompleted.includes(tutorialId)) {
        newCompleted.push(tutorialId);
        
        // Check for achievements
        const newAchievements = [...prev.achievements];
        
        // First tutorial achievement
        if (newCompleted.length === 1 && !newAchievements.includes('first-tutorial')) {
          newAchievements.push('first-tutorial');
        }
        
        // Milestone achievements
        if (newCompleted.length === 5 && !newAchievements.includes('tutorial-explorer')) {
          newAchievements.push('tutorial-explorer');
        }
        
        if (newCompleted.length === 10 && !newAchievements.includes('tutorial-master')) {
          newAchievements.push('tutorial-master');
        }
        
        // Pathway completion achievements
        Object.entries(pathwayTutorials).forEach(([pathwayId, tutorials]) => {
          const pathwayCompleted = tutorials.every(t => newCompleted.includes(t));
          const achievementId = `pathway-${pathwayId}`;
          
          if (pathwayCompleted && !newAchievements.includes(achievementId)) {
            newAchievements.push(achievementId);
          }
        });
        
        return {
          ...prev,
          completedTutorials: newCompleted,
          achievements: newAchievements,
        };
      }
      return prev;
    });
  }, []);

  const setCurrentPathway = useCallback((pathwayId: string) => {
    setProgress(prev => ({
      ...prev,
      currentPathway: pathwayId,
    }));
  }, []);

  const addBookmark = useCallback((url: string) => {
    setProgress(prev => {
      const newBookmarks = [...prev.bookmarks];
      if (!newBookmarks.includes(url)) {
        newBookmarks.push(url);
      }
      return {
        ...prev,
        bookmarks: newBookmarks,
      };
    });
  }, []);

  const removeBookmark = useCallback((url: string) => {
    setProgress(prev => ({
      ...prev,
      bookmarks: prev.bookmarks.filter(b => b !== url),
    }));
  }, []);

  const updatePreferences = useCallback((newPreferences: Partial<ProgressData['preferences']>) => {
    setProgress(prev => ({
      ...prev,
      preferences: {
        ...prev.preferences,
        ...newPreferences,
      },
    }));
  }, []);

  const addTimeSpent = useCallback((url: string, minutes: number) => {
    setProgress(prev => ({
      ...prev,
      timeSpent: {
        ...prev.timeSpent,
        [url]: (prev.timeSpent[url] || 0) + minutes,
      },
    }));
  }, []);

  const getCompletionPercentage = useCallback((pathwayId: string) => {
    const tutorials = pathwayTutorials[pathwayId] || [];
    if (tutorials.length === 0) return 0;
    
    const completedCount = tutorials.filter(t => 
      progress.completedTutorials.includes(t)
    ).length;
    
    return Math.round((completedCount / tutorials.length) * 100);
  }, [progress.completedTutorials]);

  const exportProgress = useCallback(() => {
    return JSON.stringify(progress, null, 2);
  }, [progress]);

  const importProgress = useCallback((data: string) => {
    try {
      const importedProgress = JSON.parse(data);
      
      // Validate the imported data structure
      if (typeof importedProgress === 'object' && importedProgress !== null) {
        setProgress({
          ...defaultProgress,
          ...importedProgress,
          preferences: {
            ...defaultProgress.preferences,
            ...importedProgress.preferences,
          },
        });
        return true;
      }
    } catch (error) {
      console.error('Failed to import progress:', error);
    }
    return false;
  }, []);

  const value: ProgressContextType = {
    progress,
    markTutorialComplete,
    setCurrentPathway,
    addBookmark,
    removeBookmark,
    updatePreferences,
    addTimeSpent,
    getCompletionPercentage,
    exportProgress,
    importProgress,
  };

  return (
    <ProgressContext.Provider value={value}>
      {children}
    </ProgressContext.Provider>
  );
}

export function useProgress() {
  const context = useContext(ProgressContext);
  if (context === undefined) {
    throw new Error('useProgress must be used within a ProgressProvider');
  }
  return context;
}

// Achievement definitions
export const achievements = {
  'first-tutorial': {
    title: 'First Steps',
    description: 'Complete your first tutorial',
    icon: '🎯',
  },
  'tutorial-explorer': {
    title: 'Tutorial Explorer',
    description: 'Complete 5 tutorials',
    icon: '🧭',
  },
  'tutorial-master': {
    title: 'Tutorial Master',
    description: 'Complete 10 tutorials',
    icon: '🏆',
  },
  'pathway-complete-beginner': {
    title: 'Beginner Graduate',
    description: 'Complete the Complete Beginner pathway',
    icon: '🎓',
  },
  'pathway-python-developer': {
    title: 'Python Migrant',
    description: 'Complete the Python Developer pathway',
    icon: '🐍',
  },
  'pathway-systems-developer': {
    title: 'Systems Expert',
    description: 'Complete the Systems Developer pathway',
    icon: '⚙️',
  },
  'pathway-web-developer': {
    title: 'Web Wizard',
    description: 'Complete the Web Developer pathway',
    icon: '🌐',
  },
};
