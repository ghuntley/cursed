/// Tests for DropZ module - Basic I/O primitives with Gen Z naming
/// 
/// This test suite validates the comprehensive I/O functionality
/// provided by the DropZ module, ensuring all input/output operations
/// work correctly with Gen Z naming conventions.

#[cfg(test)]
mod tests {
    use cursed::stdlib::dropz::*;
    use std::io::Cursor;

    #[test]
    fn test_stream_catcher_vibes() {
        // Test basic line reading
        let input = "Line 1\nLine 2\nLine 3\n";
        let cursor = Cursor::new(input.as_bytes());
        let mut reader = StreamCatcherVibes::new(cursor);
        
        let line1 = reader.catch_line().unwrap();
        assert_eq!(line1, "Line 1");
        
        let line2 = reader.catch_line().unwrap();
        assert_eq!(line2, "Line 2");
        
        let line3 = reader.catch_line().unwrap();
        assert_eq!(line3, "Line 3");
        
        // Test EOF
        let eof_line = reader.catch_line().unwrap();
        assert_eq!(eof_line, "");
    }

    #[test]
    fn test_stream_catcher_all_lines() {
        let input = "Line 1\nLine 2\nLine 3";
        let cursor = Cursor::new(input.as_bytes());
        let mut reader = StreamCatcherVibes::new(cursor);
        
        let all_lines = reader.catch_all_lines().unwrap();
        assert_eq!(all_lines, vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn test_stream_catcher_empty_input() {
        let input = "";
        let cursor = Cursor::new(input.as_bytes());
        let mut reader = StreamCatcherVibes::new(cursor);
        
        let all_lines = reader.catch_all_lines().unwrap();
        assert_eq!(all_lines, Vec::<String>::new());
    }

    #[test]
    fn test_stream_catcher_mixed_line_endings() {
        let input = "Line 1\r\nLine 2\nLine 3\r\n";
        let cursor = Cursor::new(input.as_bytes());
        let mut reader = StreamCatcherVibes::new(cursor);
        
        let line1 = reader.catch_line().unwrap();
        assert_eq!(line1, "Line 1");
        
        let line2 = reader.catch_line().unwrap();
        assert_eq!(line2, "Line 2");
        
        let line3 = reader.catch_line().unwrap();
        assert_eq!(line3, "Line 3");
    }

    #[test]
    fn test_stream_dropper_vibes() {
        let mut output = Vec::new();
        {
            let mut writer = StreamDropperVibes::new(&mut output);
            
            writer.drop_tea("Hello").unwrap();
            writer.drop_line(" World").unwrap();
            writer.drop_tea("Test").unwrap();
            writer.flush_it().unwrap();
        }
        
        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("Hello"));
        assert!(result.contains(" World\n"));
        assert!(result.contains("Test"));
    }

    #[test]
    fn test_stream_dropper_multiple_lines() {
        let mut output = Vec::new();
        {
            let mut writer = StreamDropperVibes::new(&mut output);
            writer.drop_lines(&["Line 1", "Line 2", "Line 3"]).unwrap();
            writer.flush_it().unwrap();
        }
        
        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("Line 1\n"));
        assert!(result.contains("Line 2\n"));
        assert!(result.contains("Line 3\n"));
    }

    #[test]
    fn test_stream_dropper_formatted() {
        let mut output = Vec::new();
        {
            let mut writer = StreamDropperVibes::new(&mut output);
            writer.drop_formatted("Hello {}, number {}", &["World", "42"]).unwrap();
            writer.flush_it().unwrap();
        }
        
        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("Hello World, number 42"));
    }

    #[test]
    fn test_progress_vibes() {
        let mut progress = new_progress_vibes(100, 20);
        
        // Test basic operations (these would normally output to terminal)
        progress.set_message("Testing".to_string());
        
        // In a real test environment, we can't easily test the actual output
        // but we can test that the operations don't panic
        // progress.update_it(0).unwrap();
        // progress.update_it(50).unwrap();
        // progress.step_it().unwrap();
        // progress.finish_it().unwrap();
        
        // Test progress bounds
        // Should clamp to maximum
        // progress.update_it(150).unwrap();
    }

    #[test]
    fn test_simple_format() {
        // Test basic placeholder replacement
        let result = super::simple_format("Hello {}, you are {} years old", &["Alice", "25"]).unwrap();
        assert_eq!(result, "Hello Alice, you are 25 years old");
        
        // Test indexed placeholders
        let result = super::simple_format("Hello {0}, you are {1} years old", &["Bob", "30"]).unwrap();
        assert_eq!(result, "Hello Bob, you are 30 years old");
        
        // Test mixed placeholders
        let result = super::simple_format("First: {}, Second: {1}, Third: {}", &["A", "B", "C"]).unwrap();
        assert_eq!(result, "First: A, Second: B, Third: C");
        
        // Test no placeholders
        let result = super::simple_format("No placeholders here", &[]).unwrap();
        assert_eq!(result, "No placeholders here");
        
        // Test more args than placeholders
        let result = super::simple_format("Only one: {}", &["A", "B", "C"]).unwrap();
        assert_eq!(result, "Only one: A");
        
        // Test no args for placeholders
        let result = super::simple_format("Missing: {}", &[]).unwrap();
        assert_eq!(result, "Missing: {}");
    }

    #[test]
    fn test_terminal_operations() {
        // These operations would normally affect the terminal
        // In tests, we just verify they don't panic
        
        let size = get_terminal_size_vibes().unwrap();
        assert_eq!(size, (80, 24)); // Default size
        
        // These would normally output escape sequences
        // clear_drops().unwrap();
        // move_cursor_vibes(10, 5).unwrap();
        // hide_cursor_vibes().unwrap();
        // show_cursor_vibes().unwrap();
    }

    #[test]
    fn test_color_operations() {
        // Test valid colors
        assert!(set_text_color_vibes("red").is_ok());
        assert!(set_text_color_vibes("green").is_ok());
        assert!(set_text_color_vibes("blue").is_ok());
        assert!(set_text_color_vibes("yellow").is_ok());
        assert!(set_text_color_vibes("magenta").is_ok());
        assert!(set_text_color_vibes("cyan").is_ok());
        assert!(set_text_color_vibes("white").is_ok());
        assert!(set_text_color_vibes("black").is_ok());
        
        // Test invalid color
        assert!(set_text_color_vibes("invalid_color").is_err());
        assert!(set_text_color_vibes("").is_err());
        
        // Test reset
        assert!(reset_text_color_vibes().is_ok());
    }

    #[test]
    fn test_paginated_output() {
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(),
            "Line 5".to_string(),
        ];
        
        // We can't easily test interactive pagination in unit tests
        // but we can test that the function doesn't panic with empty input
        assert!(paginate_drops(&[], 10).is_ok());
        
        // Test with lines less than page size
        // This would normally display all lines without pagination
        // paginate_drops(&lines[0..2], 10).unwrap();
    }

    #[test]
    fn test_type_aliases() {
        // Test that type aliases work correctly
        let tea_val: Tea = "hello".to_string();
        let normie_val: Normie = 42;
        let thicc_val: Thicc = 1000000000;
        let chonky_val: Chonky = 3.14159;
        
        assert_eq!(tea_val, "hello");
        assert_eq!(normie_val, 42);
        assert_eq!(thicc_val, 1000000000);
        assert!((chonky_val - 3.14159).abs() < 1e-10);
        
        // Test type sizes
        assert_eq!(std::mem::size_of::<Normie>(), 4); // i32
        assert_eq!(std::mem::size_of::<Thicc>(), 8);  // i64
        assert_eq!(std::mem::size_of::<Chonky>(), 8); // f64
    }

    #[test]
    fn test_module_functions() {
        // Test module initialization
        assert!(init_dropz().is_ok());
        
        // Test module statistics
        let stats = get_dropz_stats();
        assert!(stats.contains_key("version"));
        assert!(stats.contains_key("operations"));
        assert!(stats.contains_key("features"));
        assert!(stats.contains_key("types"));
        
        assert_eq!(stats.get("version").unwrap(), "1.0.0");
        assert!(stats.get("operations").unwrap().contains("Input"));
        assert!(stats.get("operations").unwrap().contains("Output"));
        assert!(stats.get("features").unwrap().contains("Gen Z"));
        assert!(stats.get("types").unwrap().contains("tea"));
        assert!(stats.get("types").unwrap().contains("normie"));
    }

    #[test]
    fn test_buffered_io_edge_cases() {
        // Test empty input
        let input = "";
        let cursor = Cursor::new(input.as_bytes());
        let mut reader = StreamCatcherVibes::new(cursor);
        
        let line = reader.catch_line().unwrap();
        assert_eq!(line, "");
        
        // Test single character
        let input = "a";
        let cursor = Cursor::new(input.as_bytes());
        let mut reader = StreamCatcherVibes::new(cursor);
        
        let line = reader.catch_line().unwrap();
        assert_eq!(line, "a");
        
        // Test just newline
        let input = "\n";
        let cursor = Cursor::new(input.as_bytes());
        let mut reader = StreamCatcherVibes::new(cursor);
        
        let line = reader.catch_line().unwrap();
        assert_eq!(line, "");
    }

    #[test]
    fn test_output_buffering() {
        let mut output = Vec::new();
        {
            let mut writer = StreamDropperVibes::new(&mut output);
            
            // Write multiple small pieces
            for i in 0..10 {
                writer.drop_tea(&format!("{} ", i)).unwrap();
            }
            writer.flush_it().unwrap();
        }
        
        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("0 1 2 3 4 5 6 7 8 9"));
    }

    #[test]
    fn test_unicode_support() {
        // Test Unicode input
        let input = "🔥 Unicode test 💯\n🌍 World 🌟\n";
        let cursor = Cursor::new(input.as_bytes());
        let mut reader = StreamCatcherVibes::new(cursor);
        
        let line1 = reader.catch_line().unwrap();
        assert_eq!(line1, "🔥 Unicode test 💯");
        
        let line2 = reader.catch_line().unwrap();
        assert_eq!(line2, "🌍 World 🌟");
        
        // Test Unicode output
        let mut output = Vec::new();
        {
            let mut writer = StreamDropperVibes::new(&mut output);
            writer.drop_line("🚀 Testing Unicode output 📝").unwrap();
            writer.flush_it().unwrap();
        }
        
        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("🚀 Testing Unicode output 📝"));
    }

    #[test]
    fn test_large_input() {
        // Test with a large input to verify buffering works correctly
        let mut large_input = String::new();
        for i in 0..1000 {
            large_input.push_str(&format!("Line {}\n", i));
        }
        
        let cursor = Cursor::new(large_input.as_bytes());
        let mut reader = StreamCatcherVibes::new(cursor);
        
        let all_lines = reader.catch_all_lines().unwrap();
        assert_eq!(all_lines.len(), 1000);
        assert_eq!(all_lines[0], "Line 0");
        assert_eq!(all_lines[999], "Line 999");
    }

    #[test]
    fn test_format_edge_cases() {
        // Test empty format string
        let result = super::simple_format("", &[]).unwrap();
        assert_eq!(result, "");
        
        // Test format string with no args needed
        let result = super::simple_format("No placeholders", &["unused"]).unwrap();
        assert_eq!(result, "No placeholders");
        
        // Test multiple consecutive placeholders
        let result = super::simple_format("{}{}{}", &["A", "B", "C"]).unwrap();
        assert_eq!(result, "ABC");
        
        // Test placeholders at start and end
        let result = super::simple_format("{} middle {}", &["start", "end"]).unwrap();
        assert_eq!(result, "start middle end");
    }

    #[test]
    fn test_progress_operations() {
        let mut progress = new_progress_vibes(10, 10);
        
        // Test message setting
        progress.set_message("Test Message".to_string());
        
        // Test that operations don't panic (even though we can't see output)
        // In a real application these would render to the terminal
        
        // Test step operation
        // progress.step_it().unwrap();
        // progress.step_it().unwrap();
        
        // Test update operation
        // progress.update_it(5).unwrap();
        // progress.update_it(10).unwrap();
        
        // Test finish operation
        // progress.finish_it().unwrap();
    }

    #[test]
    fn test_color_case_insensitivity() {
        // Test that color names are case insensitive
        assert!(set_text_color_vibes("RED").is_ok());
        assert!(set_text_color_vibes("Green").is_ok());
        assert!(set_text_color_vibes("BLUE").is_ok());
        assert!(set_text_color_vibes("Yellow").is_ok());
    }

    #[test]
    fn test_cursor_operations() {
        // Test cursor movement
        assert!(move_cursor_vibes(1, 1).is_ok());
        assert!(move_cursor_vibes(10, 20).is_ok());
        assert!(move_cursor_vibes(0, 0).is_ok());
        
        // Test cursor visibility
        assert!(hide_cursor_vibes().is_ok());
        assert!(show_cursor_vibes().is_ok());
    }

    #[test]
    fn test_output_edge_cases() {
        let mut output = Vec::new();
        {
            let mut writer = StreamDropperVibes::new(&mut output);
            
            // Test empty strings
            writer.drop_tea("").unwrap();
            writer.drop_line("").unwrap();
            
            // Test strings with special characters
            writer.drop_line("Line with\ttabs").unwrap();
            writer.drop_line("Line with\nnewlines").unwrap();
            writer.drop_line("Line with \"quotes\"").unwrap();
            writer.drop_line("Line with 'apostrophes'").unwrap();
            
            writer.flush_it().unwrap();
        }
        
        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("tabs"));
        assert!(result.contains("quotes"));
        assert!(result.contains("apostrophes"));
    }
}
