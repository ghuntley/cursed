/// Interactive console utilities for CURSED
// use crate::stdlib::io::{
    error::{IoError, IoResult},
    console::{print, println, read_line}
};

/// Show a prompt and read user input
pub fn prompt(message: &str) -> IoResult<String> {
    print(message)?;
    read_line()
}

/// Show a yes/no confirmation prompt
pub fn confirm(message: &str) -> IoResult<bool> {
    loop {
        print(&format!("{} (y/n): ", message))?;
        let input = read_line()?;
        let trimmed = input.trim().to_lowercase();
        
        match trimmed.as_str() {
            "y" | "yes" | "true" | "1" => return Ok(true),
            "n" | "no" | "false" | "0" => return Ok(false),
            "" => return Ok(false), // Default to no on empty input
            _ => {
                println("Please enter 'y' for yes or 'n' for no.")?;
                continue;
            }
        }
    }
}

/// Show a menu and get user selection
pub fn select(prompt_msg: &str, options: &[String]) -> IoResult<usize> {
    if options.is_empty() {
        return Err(IoError::InvalidInput("No options provided".to_string()));
    }

    loop {
        println(prompt_msg)?;
        for (i, option) in options.iter().enumerate() {
            println(&format!("  {}: {}", i + 1, option))?;
        }
        
        print("Select an option (number): ")?;
        let input = read_line()?;
        
        match input.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= options.len() => {
                return Ok(num - 1); // Convert to 0-based index
            }
            _ => {
                println(&format!("Please enter a number between 1 and {}", options.len()))?;
                continue;
            }
        }
    }
}

/// Multi-choice selection with checkbox-style interface
pub fn multi_select(prompt_msg: &str, options: &[String]) -> IoResult<Vec<usize>> {
    if options.is_empty() {
        return Err(IoError::InvalidInput("No options provided".to_string()));
    }

    let mut selected = vec![false; options.len()];
    
    loop {
        // Clear screen (simplified - just print newlines)
        for _ in 0..3 {
            println("")?;
        }
        
        println(prompt_msg)?;
        println("Use numbers to toggle selection, 'done' to finish:")?;
        
        for (i, option) in options.iter().enumerate() {
            let marker = if selected[i] { "[x]" } else { "[ ]" };
            println(&format!("  {} {}: {}", marker, i + 1, option))?;
        }
        
        print("\nEnter command (number to toggle, 'done' to finish): ")?;
        let input = read_line()?.trim().to_lowercase();
        
        if input == "done" || input == "d" {
            let result: Vec<usize> = selected.iter()
                .enumerate()
                .filter_map(|(i, &is_selected)| if is_selected { Some(i) } else { None })
                .collect();
            return Ok(result);
        }
        
        match input.parse::<usize>() {
            Ok(num) if num >= 1 && num <= options.len() => {
                selected[num - 1] = !selected[num - 1];
            }
            _ => {
                println(&format!("Invalid input. Enter a number between 1 and {} or 'done'", options.len()))?;
                continue;
            }
        }
    }
}

/// Password input with hidden characters (basic implementation)
pub fn read_password(prompt_msg: &str) -> IoResult<String> {
    print(prompt_msg)?;
    
    // Note: This is a simplified implementation
    // In a real implementation, you would disable echo
    let password = read_line()?;
    Ok(password)
}

/// Paginated display of content
pub fn paginate(content: &[String], page_size: usize) -> IoResult<()> {
    if content.is_empty() {
        println("No content to display.")?;
        return Ok(());
    }

    let page_size = if page_size == 0 { 10 } else { page_size };
    let total_pages = (content.len() + page_size - 1) / page_size;
    let mut current_page = 0;

    loop {
        // Display current page
        let start = current_page * page_size;
        let end = std::cmp::min(start + page_size, content.len());
        
        for line in &content[start..end] {
            println(line)?;
        }
        
        // Show navigation info
        println(&format!("\nPage {} of {} (showing {} - {} of {} items)", 
            current_page + 1, total_pages, start + 1, end, content.len()))?;
        println("Commands: (n)ext, (p)revious, (q)uit, (g)oto page")?;
        
        print("Enter command: ")?;
        let input = read_line()?.trim().to_lowercase();
        
        match input.as_str() {
            "n" | "next" => {
                if current_page < total_pages - 1 {
                    current_page += 1;
                } else {
                    println("Already at last page.")?;
                }
            }
            "p" | "prev" | "previous" => {
                if current_page > 0 {
                    current_page -= 1;
                } else {
                    println("Already at first page.")?;
                }
            }
            "q" | "quit" | "exit" => {
                break;
            }
            input if input.starts_with("g") => {
                // Extract page number
                let page_str = input.trim_start_matches('g').trim();
                match page_str.parse::<usize>() {
                    Ok(page) if page >= 1 && page <= total_pages => {
                        current_page = page - 1;
                    }
                    _ => {
                        println(&format!("Invalid page number. Enter a number between 1 and {}", total_pages))?;
                    }
                }
            }
            "" => {
                // Empty input - next page
                if current_page < total_pages - 1 {
                    current_page += 1;
                }
            }
            _ => {
                println("Invalid command. Use 'n' (next), 'p' (previous), 'q' (quit), or 'g<number>' (goto page).")?;
            }
        }
    }
    
    Ok(())
}

/// Progress bar display
pub struct ProgressBar {
    total: usize,
    current: usize,
    width: usize,
    message: String,
}

impl ProgressBar {
    /// Create a new progress bar
    pub fn new(total: usize, width: usize) -> Self {
        Self {
            total,
            current: 0,
            width: if width == 0 { 50 } else { width },
            message: String::new(),
        }
    }

    /// Set the progress bar message
    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    /// Update progress
    pub fn update(&mut self, progress: usize) -> IoResult<()> {
        self.current = std::cmp::min(progress, self.total);
        self.display()
    }

    /// Increment progress by 1
    pub fn increment(&mut self) -> IoResult<()> {
        self.update(self.current + 1)
    }

    /// Display the progress bar
    fn display(&self) -> IoResult<()> {
        let percentage = if self.total == 0 { 100.0 } else { (self.current as f64 / self.total as f64) * 100.0 };
        let filled = ((percentage / 100.0) * self.width as f64) as usize;
        let empty = self.width - filled;

        let bar = format!(
            "\r{} [{}{}] {:.1}% ({}/{})",
            self.message,
            "=".repeat(filled),
            " ".repeat(empty),
            percentage,
            self.current,
            self.total
        );

        print(&bar)?;
        Ok(())
    }

    /// Finish the progress bar
    pub fn finish(&self) -> IoResult<()> {
        println("")?; // Move to next line
        Ok(())
    }
}

