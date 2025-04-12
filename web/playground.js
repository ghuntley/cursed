// CURSED Programming Language Playground Scripts

// Wait for the entire window to load, not just the DOM
window.addEventListener('load', () => {
  const codeEditor = document.getElementById('code-editor');
  const output = document.getElementById('output');
  const runButton = document.getElementById('run-code');
  const shareButton = document.getElementById('share-code');
  const formatButton = document.getElementById('format-code');
  const clearCodeButton = document.getElementById('clear-code');
  const copyCodeButton = document.getElementById('copy-code');
  const clearOutputButton = document.getElementById('clear-output');
  const examplesDropdown = document.querySelector('.examples-dropdown-toggle');
  const examplesMenu = document.querySelector('.examples-dropdown-menu');
  const exampleOptions = document.querySelectorAll('.example-option');

  // Code examples
  const examples = {
    hello: `vibe main

slay main() {
    vibez.spill("Hello bestie!");
}`,
    fibonacci: `vibe main

// Calculate Fibonacci numbers
slay fibonacci(n normie) normie {
    lowkey n <= 1 {
        yolo n
    }
    yolo fibonacci(n-1) + fibonacci(n-2)
}

slay main() {
    bestie i := 0; i < 10; i++ {
        vibez.spill(tea(i) + ": " + tea(fibonacci(i)))
    }
}`,
    concurrency: `vibe main

yeet "time"

slay worker(id normie, ch dm<tea>) {
    time.Sleep(time.Millisecond * 500)
    ch <- "Task " + tea(id) + " completed"
}

slay main() {
    ch := make(dm<tea>, 5)
    
    bestie i := 0; i < 5; i++ {
        stan worker(i, ch)
    }
    
    bestie i := 0; i < 5; i++ {
        msg := <-ch
        vibez.spill(msg)
    }
}`,
    structs: `vibe main

be_like User squad {
    username tea
    followers normie
    verified lit
}

slay (u User) isInfluencer() lit {
    lowkey u.followers > 10000 || u.verified {
        yolo based
    }
    yolo sus
}

slay main() {
    user := User{
        username: "TheMainCharacter",
        followers: 15000,
        verified: based
    }
    
    vibez.spill(user.username + " is an influencer: " + 
               tea(user.isInfluencer()))
}`,
    switch: `vibe main

slay rateVibe(vibe tea) {
    vibe_check vibe {
        mood "immaculate":
            vibez.spill("100/10, no notes")
        mood "fire":
            vibez.spill("Absolutely slaying")
        mood "mid": 
            vibez.spill("Could be better, bestie")
        default:
            vibez.spill("Not it, tbh")
    }
}

slay main() {
    rateVibe("fire")
    rateVibe("mid")
    rateVibe("cursed")
}`
  };

  // Toggle examples dropdown
  examplesDropdown.addEventListener('click', function(e) {
    e.preventDefault();
    e.stopPropagation(); // Prevent event from bubbling up
    examplesMenu.classList.toggle('show');
    console.log('Dropdown toggled');
  });

  // Close dropdown when clicking outside
  document.addEventListener('click', (e) => {
    if (!e.target.closest('.examples-dropdown') && examplesMenu.classList.contains('show')) {
      examplesMenu.classList.remove('show');
    }
  });

  // Load examples
  exampleOptions.forEach(option => {
    option.addEventListener('click', () => {
      const exampleName = option.getAttribute('data-example');
      codeEditor.textContent = examples[exampleName];
      examplesMenu.classList.remove('show');
      highlightCode();
    });
  });

  // Run code button
  runButton.addEventListener('click', () => {
    const code = codeEditor.textContent;
    runCode(code);
  });

  // Share code button
  shareButton.addEventListener('click', () => {
    shareCode(codeEditor.textContent);
  });

  // Format code button
  formatButton.addEventListener('click', () => {
    formatCode();
  });

  // Clear code button
  clearCodeButton.addEventListener('click', () => {
    if (confirm('Are you sure you want to clear your code?')) {
      codeEditor.textContent = '';
    }
  });

  // Copy code button
  copyCodeButton.addEventListener('click', () => {
    navigator.clipboard.writeText(codeEditor.textContent);
    copyCodeButton.textContent = 'Copied!';
    setTimeout(() => {
      copyCodeButton.textContent = 'Copy';
    }, 2000);
  });

  // Clear output button
  clearOutputButton.addEventListener('click', () => {
    output.textContent = '// Run your code to see the output here';
  });

  // Simulate code execution (this would connect to a server in a real implementation)
  function runCode(code) {
    // Show loading state
    output.textContent = 'Running... \n';
    
    // Simulate network delay
    setTimeout(() => {
      try {
        // This is a simulation - in a real implementation,
        // this would send the code to a server for execution
        simulateExecution(code);
      } catch (error) {
        output.textContent = `Error: ${error.message}`;
      }
    }, 1000);
  }

  // Simulate code execution with mocked responses
  function simulateExecution(code) {
    // This is a simplified simulation for demo purposes
    if (code.includes('vibez.spill("Hello bestie!")')) {
      output.textContent = 'Hello bestie!\n';
    } 
    else if (code.includes('fibonacci')) {
      output.textContent = '0: 0\n1: 1\n2: 1\n3: 2\n4: 3\n5: 5\n6: 8\n7: 13\n8: 21\n9: 34\n';
    }
    else if (code.includes('ch := make(dm<tea>')) {
      output.textContent = 'Task 0 completed\nTask 2 completed\nTask 1 completed\nTask 3 completed\nTask 4 completed\n';
    }
    else if (code.includes('be_like User')) {
      output.textContent = 'TheMainCharacter is an influencer: true\n';
    }
    else if (code.includes('vibe_check')) {
      output.textContent = 'Absolutely slaying\nCould be better, bestie\nNot it, tbh\n';
    }
    else if (code.trim() === '') {
      output.textContent = 'Error: Empty code\n';
    }
    else {
      // Generate a random output for unknown code
      const randomOutputs = [
        'Program ran successfully with no output.',
        'Error: Undefined symbol "vibeCheck" at line 7',
        'Warning: You're not using enough GenZ slang, bestie.',
        'Compilation successful, but the vibe check failed.',
        'Runtime error: Not enough clout to execute this function.',
        'Success! Your code is absolutely slaying!',
      ];
      
      output.textContent = randomOutputs[Math.floor(Math.random() * randomOutputs.length)];
    }
  }

  // Function to share code (simplified for demo)
  function shareCode(code) {
    // Encode the code for URL sharing
    const encodedCode = encodeURIComponent(code);
    
    // In a real implementation, you might generate a shorter URL or save to a database
    const shareableUrl = `${window.location.href}?code=${encodedCode}`;
    
    // Show sharing dialog or copy to clipboard
    if (navigator.clipboard) {
      navigator.clipboard.writeText(shareableUrl)
        .then(() => {
          alert('Shareable link copied to clipboard!');
        })
        .catch(err => {
          console.error('Failed to copy: ', err);
          promptManualCopy(shareableUrl);
        });
    } else {
      promptManualCopy(shareableUrl);
    }
  }
  
  function promptManualCopy(text) {
    // Fallback for browsers that don't support clipboard API
    alert('Copy this URL to share your code:\n\n' + text);
  }

  // Simulate code formatting (in a real implementation, this would call a server)
  function formatCode() {
    // Get current code
    const code = codeEditor.textContent;
    
    // Add some simple formatting (this is very simplified)
    let formatted = code
      // Ensure consistent spacing around brackets
      .replace(/\{\s*/g, '{ ')
      .replace(/\s*\}/g, ' }')
      // Clean up extra spaces
      .replace(/\s+/g, ' ')
      .replace(/\{ /g, '{\n    ')
      .replace(/ \}/g, '\n}')
      .replace(/; /g, ';\n    ')
      // Ensure consistent spacing around operators
      .replace(/([\+\-\*\/\=\>\<])/g, ' $1 ')
      // Clean up multiple spaces
      .replace(/\s+/g, ' ')
      // Fix new lines
      .replace(/ \{\n/g, ' {\n')
      .replace(/\n \}/g, '\n}')
      .replace(/\n /g, '\n');
    
    // Update editor with formatted code  
    codeEditor.textContent = formatted;
    highlightCode();
  }

  // Simple syntax highlighting function
  function highlightCode() {
    // This is a simplified implementation
    // In a real app, use a proper syntax highlighting library
    const code = codeEditor.textContent;
    
    // Add syntax highlighting by wrapping keywords, strings, etc.
    const highlighted = code
      // Keywords
      .replace(/\b(vibe|slay|yolo|yeet|fr|lit|highkey|lowkey|bestie|periodt|stan|be_like|squad|mood|default|vibe_check|sus|based|dm|facts|tea|normie)\b/g, '<span class="keyword">$1</span>')
      // Strings
      .replace(/"([^"]*)"/g, '<span class="string">"$1"</span>')
      // Comments
      .replace(/(fr fr .*$)/gm, '<span class="comment">$1</span>');
    
    // Update the editor with highlighted code
    codeEditor.innerHTML = highlighted;
    
    // Preserve cursor position and editability
    codeEditor.focus();
  }

  // Highlight initial code
  highlightCode();

  // Add event listeners for editing
  codeEditor.addEventListener('input', () => {
    // Remove any styling when editing
    const selection = window.getSelection();
    const range = selection.getRangeAt(0);
    const startOffset = range.startOffset;
    
    // Debounce the highlighting to avoid performance issues during typing
    clearTimeout(codeEditor.highlightTimeout);
    codeEditor.highlightTimeout = setTimeout(() => {
      highlightCode();
      
      // Try to restore cursor position
      try {
        const newRange = document.createRange();
        newRange.setStart(codeEditor.childNodes[0], startOffset);
        newRange.collapse(true);
        selection.removeAllRanges();
        selection.addRange(newRange);
      } catch (e) {
        // If cursor position can't be restored, just focus the editor
        codeEditor.focus();
      }
    }, 300);
  });

  // Load code from URL if present
  function loadCodeFromURL() {
    const urlParams = new URLSearchParams(window.location.search);
    const codeParam = urlParams.get('code');
    
    if (codeParam) {
      try {
        const decodedCode = decodeURIComponent(codeParam);
        codeEditor.textContent = decodedCode;
        highlightCode();
      } catch (e) {
        console.error('Failed to load code from URL:', e);
      }
    }
  }
  
  // Load code on initial page load
  loadCodeFromURL();
});