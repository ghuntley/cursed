import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { Navigation } from './components/Navigation';
import { HomePage } from './pages/HomePage';
import { TutorialPage } from './pages/TutorialPage';
import { PlaygroundPage } from './pages/PlaygroundPage';
import { APIDocsPage } from './pages/APIDocsPage';
import { PathwaysPage } from './pages/PathwaysPage';
import { MigrationPage } from './pages/MigrationPage';
import { PatternsPage } from './pages/PatternsPage';
import { VideosPage } from './pages/VideosPage';
import { CommunityPage } from './pages/CommunityPage';
import { SearchProvider } from './contexts/SearchContext';
import { ProgressProvider } from './contexts/ProgressContext';
import { ThemeProvider } from './contexts/ThemeContext';
import './App.css';

function App() {
  return (
    <ThemeProvider>
      <SearchProvider>
        <ProgressProvider>
          <Router>
            <div className="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50 dark:from-slate-900 dark:to-slate-800">
              <Navigation />
              <main className="container mx-auto px-4 py-8">
                <Routes>
                  <Route path="/" element={<HomePage />} />
                  <Route path="/tutorials/*" element={<TutorialPage />} />
                  <Route path="/playground" element={<PlaygroundPage />} />
                  <Route path="/api/*" element={<APIDocsPage />} />
                  <Route path="/pathways/*" element={<PathwaysPage />} />
                  <Route path="/migration/*" element={<MigrationPage />} />
                  <Route path="/patterns/*" element={<PatternsPage />} />
                  <Route path="/videos/*" element={<VideosPage />} />
                  <Route path="/community/*" element={<CommunityPage />} />
                </Routes>
              </main>
            </div>
          </Router>
        </ProgressProvider>
      </SearchProvider>
    </ThemeProvider>
  );
}

export default App;
