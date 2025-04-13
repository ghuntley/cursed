#!/bin/bash

# Get a list of all HTML files
HTML_FILES=$(find web/docs -name "*.html" -type f)

# Process each file
for file in $HTML_FILES; do
  echo "Processing $file..."
  
  # Skip already updated files (our newly created ones)
  if grep -q "docs\.css" "$file"; then
    echo "  Already updated, skipping."
    continue
  fi

  # Check if the file is a fragment or a complete HTML document
  if grep -q "<!DOCTYPE html>" "$file"; then
    # Complete HTML document - update stylesheet reference
    echo "  Updating stylesheet reference"
    sed -i 's|<link rel="stylesheet" href="../styles/main.css">|<link rel="stylesheet" href="../styles/docs.css">|' "$file"
    sed -i 's|<link rel="stylesheet" href="../styles/responsive.css">||' "$file"

    # Add standard header/nav if it doesn't exist
    if ! grep -q '<a href="index.html"' "$file"; then
      echo "  Adding standard header/nav links"
      sed -i 's|<header>|<header>\n        <div class="header-container container">\n            <a href="../index.html" class="logo">\n                <img src="../images/cursed-logo.svg" alt="CURSED Logo">\n                <span>CURSED</span>\n            </a>\n            \n            <nav>\n                <button id="menu-toggle" class="menu-toggle">\n                    <span></span>\n                    <span></span>\n                    <span></span>\n                </button>\n                \n                <ul>\n                    <li><a href="../index.html#features">Features</a></li>\n                    <li><a href="../index.html#code">Examples</a></li>\n                    <li><a href="index.html" class="active">Docs</a></li>\n                    <li><a href="../index.html#community">Community</a></li>\n                    <li><a href="https://github.com/ghuntley/cursed" target="_blank">GitHub</a></li>\n                    <li><button id="theme-toggle" class="theme-toggle">🌙</button></li>\n                </ul>\n            </nav>\n        </div>|' "$file"
    fi

    # Add sidebar navigation if it doesn't exist
    if ! grep -q '<aside class="sidebar">' "$file"; then
      # Extract the current page title
      title=$(grep -o -P '(?<=<title>).*?(?= - CURSED)' "$file" || grep -o -P '(?<=<title>CURSED - ).*?(?= \|)' "$file" || echo "Documentation")
      echo "  Adding sidebar with current page: $title"
      
      # Insert sidebar before main content
      sed -i 's|<article class="content">|<div class="doc-grid">\n                <aside class="sidebar">\n                    <nav>\n                        <ul>\n                            <li><a href="index.html">Documentation Home</a></li>\n                            <li><a href="overview.html">Language Overview</a></li>\n                            <li><a href="installation.html">Installation</a></li>\n                            <li><a href="hello-world.html">Hello World</a></li>\n                            <li><a href="#" class="active">'