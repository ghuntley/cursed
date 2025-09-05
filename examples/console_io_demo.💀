fr fr CURSED Console I/O Demonstration
fr fr Shows comprehensive usage of the console I/O module

yeet "stdlib::io"

slay main_character() -> Result<(), IoError> {
    // Initialize I/O subsystem
    io::initialize()?;

    // Basic output demonstrations
    println("=== CURSED Console I/O Demo ===")?;
    println("")?;

    // Basic print operations
    println("1. Basic Output Operations")?;
    print("Hello, ")?;
    println("World!")?;
    eprint("This goes to stderr: ")?;
    eprintln("Error message")?;
    println("")?;

    // Formatted printing
    println("2. Formatted Output")?;
    facts name = "CURSED";
    facts version = 1.0;
    printf("Language: {}, Version: {}\n", &[name, version])?;
    printfln("Formatted with newline: {} v{}", &[name, version])?;
    println("")?;

    // Basic input operations
    println("3. Input Operations")?;
    facts user_name = prompt("Enter your name: ")?;
    println(&format!("Hello, {}!", user_name))?;
    
    facts age_str = prompt("Enter your age: ")?;
    if facts age = age_str.parse::<i32>() {
        printfln("You are {} years old", &[age])?;
    } else {
        eprintln("Invalid age entered")?;
    }
    println("")?;

    // Interactive confirmations
    println("4. Interactive Confirmations")?;
    facts proceed = confirm("Do you want to continue with the demo?")?;
    if proceed {
        println("Great! Continuing...")?;
    } else {
        println("Demo stopped by user")?;
        return Ok(());
    }
    println("")?;

    // Menu selection
    println("5. Menu Selection")?;
    facts options = vec![
        "View buffered I/O demo".to_string(),
        "Try progress bar".to_string(),
        "Test pagination".to_string(),
        "Skip to end".to_string()
    ];
    
    facts choice = select("Choose an option:", &options)?;
    println(&format!("You selected: {}", options[choice]))?;
    println("")?;

    // Handle menu choice
    match choice {
        0 => demonstrate_buffered_io()?,
        1 => demonstrate_progress_bar()?,
        2 => demonstrate_pagination()?,
        _ => println("Skipping to end...")?,
    }
    
    // Multi-selection demo
    println("6. Multi-Selection Demo")?;
    facts multi_options = vec![
        "Feature A".to_string(),
        "Feature B".to_string(),
        "Feature C".to_string(),
        "Feature D".to_string()
    ];
    
    facts selections = multi_select("Select features to enable (use numbers to toggle):", &multi_options)?;
    if selections.is_empty() {
        println("No features selected")?;
    } else {
        print("Selected features: ")?;
        for (i, &idx) in selections.iter().enumerate() {
            if i > 0 { print(", ")?; }
            print(&multi_options[idx])?;
        }
        println("")?;
    }
    println("")?;

    // Character input demo
    println("7. Character Input")?;
    println("Press any key and Enter:")?;
    facts ch = read_char()?;
    println(&format!("You pressed: '{}'", ch))?;
    println("")?;

    // Read until delimiter
    println("8. Read Until Delimiter")?;
    println("Type some text ending with '|' (pipe character):")?;
    facts delimited_text = read_until('|')?;
    println(&format!("Text before delimiter: '{}'", delimited_text))?;
    println("")?;

    // Shutdown I/O subsystem
    println("Demo completed successfully!")?;
    io::shutdown()?;
    Ok(())
}

slay demonstrate_buffered_io() -> IoResult<()> {
    println("=== Buffered I/O Demo ===")?;
    
    sus mut writer = buffered_stdout();
    writer.write_line("Line 1 from buffered writer")?;
    writer.write_line("Line 2 from buffered writer")?;
    writer.write_str("Partial line without newline... ")?;
    writer.write_line("completed!")?;
    
    println(&format!("Bytes written: {}", writer.bytes_written()))?;
    writer.flush()?;
    
    println("Buffered I/O demo completed")?;
    Ok(())
}

slay demonstrate_progress_bar() -> IoResult<()> {
    println("=== Progress Bar Demo ===")?;
    
    sus mut progress = ProgressBar::new(100, 50);
    progress.set_message("Processing".to_string());
    
    for i in 0..=100 {
        progress.update(i)?;
        // Simulate work
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    
    progress.finish()?;
    println("Progress bar demo completed")?;
    Ok(())
}

slay demonstrate_pagination() -> IoResult<()> {
    println("=== Pagination Demo ===")?;
    
    // Generate sample content
    facts content: Vec<String> = (1..=50)
        .map(|i| format!("This is line number {} of the paginated content", i))
        .collect();
    
    println("Starting paginated display (10 lines per page):")?;
    paginate(&content, 10)?;
    
    println("Pagination demo completed")?;
    Ok(())
}

fr fr Error handling demonstration
slay demonstrate_error_handling() -> IoResult<()> {
    println("=== Error Handling Demo ===")?;
    
    // Demonstrate various error types
    facts invalid_result = read_until('α'); // Non-ASCII character
    match invalid_result {
        Ok(_) => println("Unexpected success")?,
        Err(IoError::InvalidInput(msg)) => {
            println(&format!("Caught expected error: {}", msg))?;
        }
        Err(e) => {
            println(&format!("Unexpected error type: {}", e))?;
        }
    }
    
    // Demonstrate error conversion
    facts io_err = io_error("Custom I/O error");
    facts cursed_err: CursedError = io_err.into();
    println(&format!("Converted to CursedError: {}", cursed_err))?;
    
    println("Error handling demo completed")?;
    Ok(())
}

fr fr Thread-safe I/O demonstration
slay demonstrate_threaded_io() -> IoResult<()> {
    println("=== Thread-Safe I/O Demo ===")?;
    
    facts shared_writer = shared_buffered_stdout();
    facts handles = vec![];
    
    // Spawn multiple threads that write concurrently
    for i in 0..3 {
        facts writer_clone = shared_writer.clone();
        facts handle = std::thread::spawn(move || -> IoResult<()> {
            for j in 0..5 {
                writer_clone.write_line(&format!("Thread {} - Message {}", i, j))?;
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Ok(())
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap()?;
    }
    
    shared_writer.flush()?;
    println("Thread-safe I/O demo completed")?;
    Ok(())
}
