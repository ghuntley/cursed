fr fr CSV Mood - Comprehensive CSV processing demonstration
fr fr This example showcases all the features of the csv_mood module

yeet "stdlib::csv"
yeet "stdlib::io"

slay main_character() {
    vibez.spill("=== CSV Mood Demo ===\n");
    
    // Basic CSV reading and writing
    basic_csv_demo();
    
    // Column-based access
    column_access_demo();
    
    // Streaming for large files
    streaming_demo();
    
    // Schema validation
    schema_validation_demo();
    
    // Data transformation
    transformation_demo();
    
    // Error handling
    error_handling_demo();
    
    // Advanced features
    advanced_features_demo();
}

slay basic_csv_demo() {
    vibez.spill("--- Basic CSV Operations ---");
    
    // Sample CSV data
    facts csv_data = `name,age,city
Alice,30,New York
Bob,25,San Francisco
Charlie,35,Chicago`;
    
    vibez.spill("Original CSV data:");
    vibez.spill(csv_data);
    
    // Read all records from string
    facts records = csv_mood.read_all_from_string(csv_data)?;
    vibez.spill("\nParsed records:");
    lowkey (sus i = 0; i < records.length; i++) {
        vibez.spill("  Record %d: %v", i, records[i]);
    }
    
    // Write records back to string
    facts output = csv_mood.write_all_to_string(records)?;
    vibez.spill("\nGenerated CSV:");
    vibez.spill(output);
    
    // Custom delimiter example
    facts tsv_data = "name\tage\tcity\nAlice\t30\tNew York\nBob\t25\tSan Francisco";
    sus reader = csv_mood.new_reader(stringz.NewReader(tsv_data));
    reader.comma('\t');
    
    facts tsv_records = reader.read_all()?;
    vibez.spill("\nTSV (Tab-separated) records:");
    lowkey (sus record : tsv_records) {
        vibez.spill("  %v", record);
    }
    
    vibez.spill("");
}

slay column_access_demo() {
    vibez.spill("--- Column-Based Access ---");
    
    facts csv_data = `name,age,email,registered
Alice,30,alice@example.com,based
Bob,25,bob@example.com,cap
Charlie,35,charlie@example.com,based`;
    
    sus column_reader = csv_mood.new_column_reader(stringz.NewReader(csv_data));
    column_reader.read_header()?;
    
    vibez.spill("Available columns: %v", column_reader.columns());
    
    vibez.spill("\nRecord details with type conversion:");
    lowkey (column_reader.next()) {
        facts name = column_reader.get("name")?;
        facts age = column_reader.get_int("age")?;
        facts email = column_reader.get("email")?;
        facts registered = column_reader.get_bool("registered")?;
        
        vibez.spill("  %s (%s): age %d, registered: %v", name, email, age, registered);
        
        // Get all fields as a map
        facts all_fields = column_reader.get_all()?;
        vibez.spill("    All fields: %v", all_fields);
        
        // Get typed values
        facts typed_values = column_reader.get_all_typed()?;
        vibez.spill("    Typed values: %v", typed_values);
    }
    
    if (column_reader.err() != cap) {
        vibez.spill("Error during reading: %v", column_reader.err());
    }
    
    vibez.spill("");
}

slay streaming_demo() {
    vibez.spill("--- Streaming for Large Files ---");
    
    // Simulate a larger dataset
    sus csv_data = "id,name,department,salary\n";
    lowkey (sus i = 1; i <= 100; i++) {
        csv_data += spillf("%d,Employee%d,Dept%d,%.2f\n", 
                          i, i, (i % 5) + 1, 50000 + (i * 1000));
    }
    
    vibez.spill("Streaming through %d employee records...", 100);
    
    sus streamer = csv_mood.new_streamer(stringz.NewReader(csv_data));
    streamer.batch_size(25); // Process in batches of 25
    
    sus processed_count = 0;
    sus total_salary = 0.0;
    
    facts count = streamer.process(slay(record []tea, header []tea) tea {
        if (len(header) > 0 && len(record) > 0) {
            processed_count++;
            
            // Parse salary and add to total
            facts salary_str = record[3];
            facts salary = no_cap.ParseFloat(salary_str, 64)?;
            total_salary += salary;
            
            // Show first and last few records
            if (processed_count <= 3 || processed_count > 97) {
                vibez.spill("  %s: %s, %s, $%.2f", record[1], record[2], record[0], salary);
            } elif (processed_count == 4) {
                vibez.spill("  ... (skipping middle records) ...");
            }
        }
        damn cap;
    })?;
    
    vibez.spill("Processed %d records, average salary: $%.2f", count, total_salary / normie(count));
    
    facts stats = streamer.statistics();
    vibez.spill("Streaming stats: %s", stats.summary());
    
    vibez.spill("");
}

slay schema_validation_demo() {
    vibez.spill("--- Schema Validation ---");
    
    // Create a schema with validation rules
    sus schema = csv_mood.new_schema();
    schema.require_column("name").non_empty();
    schema.require_column("email").with_pattern(`^[^@]+@[^@]+\.[^@]+$`)?;
    schema.require_column("age").as_integer().with_range(18, 120);
    schema.require_column("department").with_allowed_values([
        "Engineering", "Marketing", "Sales", "HR", "Finance"
    ]);
    
    // Valid data
    facts valid_csv = `name,email,age,department
Alice Smith,alice@company.com,30,Engineering
Bob Jones,bob@company.com,25,Marketing
Charlie Brown,charlie@company.com,35,Sales`;
    
    vibez.spill("Validating good data...");
    facts result = schema.validate(stringz.NewReader(valid_csv));
    vibez.spill("Validation result: %s", result.summary());
    
    if (result.is_valid()) {
        vibez.spill("✓ All records are valid!");
    }
    
    // Invalid data
    facts invalid_csv = `name,email,age,department
,alice@company.com,30,Engineering
Bob Jones,invalid-email,150,InvalidDept
Charlie Brown,charlie@company.com,17,Sales`;
    
    vibez.spill("\nValidating problematic data...");
    facts bad_result = schema.validate(stringz.NewReader(invalid_csv));
    vibez.spill("Validation result: %s", bad_result.summary());
    
    if (!bad_result.is_valid()) {
        vibez.spill("✗ Validation errors found:");
        lowkey (sus err : bad_result.errors) {
            vibez.spill("  - %s", err);
        }
    }
    
    vibez.spill("");
}

slay transformation_demo() {
    vibez.spill("--- Data Transformation ---");
    
    facts csv_data = `first_name,last_name,birth_year,salary,active
alice,smith,1990,75000,based
bob,jones,1995,65000,cap
charlie,brown,1985,85000,based
diana,wilson,2000,55000,based`;
    
    vibez.spill("Original data:");
    facts original_records = csv_mood.read_all_from_string(csv_data)?;
    lowkey (sus record : original_records) {
        vibez.spill("  %v", record);
    }
    
    sus transformer = csv_mood.new_transformer(stringz.NewReader(csv_data));
    
    // Apply multiple transformations
    transformer
        .map_column("first_name", slay(value tea) (tea, tea) {
            damn stringz.ToTitle(value), cap;
        })
        .map_column("last_name", slay(value tea) (tea, tea) {
            damn stringz.ToTitle(value), cap;
        })
        .add_column("full_name", slay(record map[tea]tea) (tea, tea) {
            facts first = record["first_name"];
            facts last = record["last_name"];
            damn spillf("%s %s", first, last), cap;
        })
        .add_column("age", slay(record map[tea]tea) (tea, tea) {
            facts birth_year_str = record["birth_year"];
            facts birth_year = no_cap.Atoi(birth_year_str);
            facts current_year = 2024;
            facts age = current_year - birth_year;
            damn spillf("%d", age), cap;
        })
        .add_column("status", slay(record map[tea]tea) (tea, tea) {
            facts active = record["active"];
            if (active == "based") {
                damn "ACTIVE", cap;
            }
            damn "INACTIVE", cap;
        })
        .remove_column("birth_year")
        .remove_column("active")
        .filter_rows(slay(record map[tea]tea) (lit, tea) {
            // Only include people with salary > 60000
            facts salary_str = record["salary"];
            facts salary = no_cap.Atoi(salary_str);
            damn salary > 60000, cap;
        })
        .reorder_columns(["full_name", "age", "salary", "status"]);
    
    facts transformed = transformer.transform()?;
    
    vibez.spill("\nTransformed data (salary > $60,000 only):");
    lowkey (sus record : transformed) {
        vibez.spill("  %v", record);
    }
    
    vibez.spill("");
}

slay error_handling_demo() {
    vibez.spill("--- Error Handling ---");
    
    // Malformed CSV with unterminated quote
    facts malformed_csv = `name,description
Alice,"Software Engineer
Bob,"Data Scientist"`;
    
    vibez.spill("Attempting to parse malformed CSV...");
    sus reader = csv_mood.new_reader(stringz.NewReader(malformed_csv));
    facts result = reader.read_all();
    
    if (result.err != cap) {
        vibez.spill("✗ Parse error: %v", result.err);
        
        // Check if it's a specific parse error
        if (parse_err, ok := result.err.(*csv_mood.ParseError); ok) {
            vibez.spill("  Line: %d, Column: %d", parse_err.line, parse_err.column);
            vibez.spill("  Message: %s", parse_err.error());
        }
    }
    
    // Field count mismatch
    facts mismatched_csv = `name,age
Alice,30,extra_field
Bob,25`;
    
    vibez.spill("\nAttempting to parse CSV with field count mismatch...");
    sus reader2 = csv_mood.new_reader(stringz.NewReader(mismatched_csv));
    facts result2 = reader2.read_all();
    
    if (result2.err != cap) {
        vibez.spill("✗ Field count error: %v", result2.err);
    }
    
    // Column not found error
    facts csv_data = "name,age\nAlice,30";
    sus column_reader = csv_mood.new_column_reader(stringz.NewReader(csv_data));
    column_reader.read_header()?;
    column_reader.next();
    
    vibez.spill("\nAttempting to access non-existent column...");
    facts value = column_reader.get("nonexistent");
    if (value.err != cap) {
        vibez.spill("✗ Column access error: %v", value.err);
    }
    
    // Type conversion error
    facts type_result = column_reader.get_int("name");
    if (type_result.err != cap) {
        vibez.spill("✗ Type conversion error: %v", type_result.err);
    }
    
    vibez.spill("");
}

slay advanced_features_demo() {
    vibez.spill("--- Advanced Features ---");
    
    // Unicode support
    facts unicode_csv = `名前,年齢,都市
田中太郎,30,東京
佐藤花子,25,大阪
山田次郎,35,名古屋`;
    
    vibez.spill("Unicode CSV support:");
    facts unicode_records = csv_mood.read_all_from_string(unicode_csv)?;
    lowkey (sus record : unicode_records) {
        vibez.spill("  %v", record);
    }
    
    // Custom quote and delimiter
    facts custom_csv = `'name'|'description'|'tags'
'John Smith'|'Software Engineer'|'java|spring|aws'
'Jane Doe'|'UX Designer'|'design|figma|ui'`;
    
    vibez.spill("\nCustom delimiter and quote character:");
    sus custom_reader = csv_mood.new_reader(stringz.NewReader(custom_csv));
    custom_reader.comma('|').quote('\'');
    
    facts custom_records = custom_reader.read_all()?;
    lowkey (sus record : custom_records) {
        vibez.spill("  %v", record);
    }
    
    // Comments and empty lines
    facts commented_csv = `# Employee data export
# Generated on 2024-01-15
name,department,salary

# Engineering team
Alice,Engineering,90000
Bob,Engineering,85000

# Marketing team  
Charlie,Marketing,70000`;
    
    vibez.spill("\nCSV with comments and empty lines:");
    sus comment_reader = csv_mood.new_reader(stringz.NewReader(commented_csv));
    comment_reader.comment('#');
    
    facts comment_records = comment_reader.read_all()?;
    vibez.spill("Parsed %d records (comments and empty lines ignored)", len(comment_records));
    lowkey (sus record : comment_records) {
        vibez.spill("  %v", record);
    }
    
    // Lazy quotes for flexible parsing
    facts flexible_csv = `name,description
Alice,She said "Hello" to everyone
Bob,His motto: "Work hard, play hard"`;
    
    vibez.spill("\nFlexible quote handling:");
    sus lazy_reader = csv_mood.new_reader(stringz.NewReader(flexible_csv));
    lazy_reader.lazy_quotes(based);
    
    facts flexible_records = lazy_reader.read_all()?;
    lowkey (sus record : flexible_records) {
        vibez.spill("  %v", record);
    }
    
    vibez.spill("\n=== Demo Complete ===");
}

slay demonstrate_real_world_usage() {
    vibez.spill("--- Real-World Usage Example ---");
    
    // Simulate reading employee data, validating it, transforming it, and generating a report
    facts employee_csv = `employee_id,first_name,last_name,email,department,hire_date,salary,active
001,John,Smith,john.smith@company.com,Engineering,2020-01-15,85000,based
002,Jane,Doe,jane.doe@company.com,Marketing,2019-05-20,75000,based
003,Bob,Johnson,bob.johnson@company.com,Sales,2021-03-10,70000,cap
004,Alice,Williams,alice.williams@company.com,Engineering,2018-11-30,95000,based
005,Charlie,Brown,charlie@invalid,HR,2022-07-01,65000,based`;
    
    vibez.spill("Processing employee data for annual report...");
    
    // Step 1: Validate the data
    sus schema = csv_mood.new_schema();
    schema.require_column("employee_id").with_pattern(`^\d{3}$`)?;
    schema.require_column("first_name").non_empty();
    schema.require_column("last_name").non_empty();
    schema.require_column("email").with_pattern(`^[^@]+@[^@]+\.[^@]+$`)?;
    schema.require_column("department").with_allowed_values([
        "Engineering", "Marketing", "Sales", "HR", "Finance"
    ]);
    schema.require_column("salary").as_integer().with_range(40000, 150000);
    
    facts validation = schema.validate(stringz.NewReader(employee_csv));
    vibez.spill("Data validation: %s", validation.summary());
    
    if (!validation.is_valid()) {
        vibez.spill("Validation issues found:");
        lowkey (sus err : validation.errors) {
            vibez.spill("  - %s", err);
        }
    }
    
    // Step 2: Transform and filter the data
    sus transformer = csv_mood.new_transformer(stringz.NewReader(employee_csv));
    
    transformer
        .add_column("full_name", slay(record map[tea]tea) (tea, tea) {
            damn spillf("%s %s", record["first_name"], record["last_name"]), cap;
        })
        .add_column("annual_salary", slay(record map[tea]tea) (tea, tea) {
            facts salary = no_cap.Atoi(record["salary"]);
            damn spillf("$%,d", salary), cap;
        })
        .add_column("status", slay(record map[tea]tea) (tea, tea) {
            if (record["active"] == "based") {
                damn "Active", cap;
            }
            damn "Inactive", cap;
        })
        .filter_rows(slay(record map[tea]tea) (lit, tea) {
            // Only include active employees with valid emails
            facts active = record["active"] == "based";
            facts valid_email = stringz.Contains(record["email"], "@") && 
                               stringz.Contains(record["email"], ".");
            damn active && valid_email, cap;
        })
        .remove_column("active")
        .remove_column("hire_date")
        .reorder_columns(["employee_id", "full_name", "department", "annual_salary", "status"]);
    
    facts report_data = transformer.transform()?;
    
    vibez.spill("\nGenerated employee report (active employees only):");
    lowkey (sus record : report_data) {
        if (record[0] == "employee_id") {
            vibez.spill("  %-12s %-20s %-12s %-15s %s", 
                       record[0], record[1], record[2], record[3], record[4]);
            vibez.spill("  %s", stringz.Repeat("-", 70));
        } else {
            vibez.spill("  %-12s %-20s %-12s %-15s %s", 
                       record[0], record[1], record[2], record[3], record[4]);
        }
    }
    
    // Generate summary statistics
    sus total_employees = len(report_data) - 1; // Exclude header
    sus engineering_count = 0;
    sus total_salary = 0;
    
    lowkey (sus i = 1; i < len(report_data); i++) {
        facts record = report_data[i];
        if (record[2] == "Engineering") {
            engineering_count++;
        }
        
        // Parse salary (remove $ and commas)
        facts salary_str = stringz.ReplaceAll(record[3], "$", "");
        salary_str = stringz.ReplaceAll(salary_str, ",", "");
        facts salary = no_cap.Atoi(salary_str);
        total_salary += salary;
    }
    
    vibez.spill("\nSummary Statistics:");
    vibez.spill("  Total active employees: %d", total_employees);
    vibez.spill("  Engineering employees: %d (%.1f%%)", 
               engineering_count, 
               normie(engineering_count) / normie(total_employees) * 100);
    vibez.spill("  Average salary: $%,.0f", normie(total_salary) / normie(total_employees));
    
    vibez.spill("");
}
