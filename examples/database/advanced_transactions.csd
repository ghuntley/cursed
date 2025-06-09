// fr fr Advanced transaction management example - showing ACID properties periodt
//
// This example demonstrates:
// - Transaction basics (begin, commit, rollback)
// - Savepoints for nested transactions
// - Error handling within transactions
// - Batch processing with transactions
// - Concurrent transaction scenarios

sus main() {
    // Setup test database with accounts table
    let connection = sql_connect("sqlite", ":memory:")?;
    println!("🏦 Setting up banking database...");
    
    let create_accounts = "
        CREATE TABLE accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            account_number TEXT UNIQUE NOT NULL,
            owner_name TEXT NOT NULL,
            balance DECIMAL(15,2) NOT NULL CHECK(balance >= 0),
            account_type TEXT DEFAULT 'checking',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    ";
    
    let create_transactions = "
        CREATE TABLE transaction_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            from_account TEXT,
            to_account TEXT,
            amount DECIMAL(15,2) NOT NULL,
            transaction_type TEXT NOT NULL,
            description TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    ";
    
    connection.execute(create_accounts, [])?;
    connection.execute(create_transactions, [])?;
    println!("✅ Created accounts and transaction_log tables");
    
    // Insert initial account data
    initial_account_setup(&connection)?;
    
    // Example 1: Basic transaction - Money transfer
    println!("\n💸 Example 1: Basic money transfer transaction");
    money_transfer_example(&connection)?;
    
    // Example 2: Savepoints - Complex business logic
    println!("\n💼 Example 2: Complex transaction with savepoints");
    complex_business_transaction(&connection)?;
    
    // Example 3: Batch processing with transactions
    println!("\n📦 Example 3: Batch processing with transaction optimization");
    batch_processing_example(&connection)?;
    
    // Example 4: Transaction rollback on error
    println!("\n🚨 Example 4: Transaction rollback on error");
    error_rollback_example(&connection)?;
    
    // Example 5: Read-only transaction for consistency
    println!("\n📊 Example 5: Read-only transaction for reporting");
    reporting_transaction_example(&connection)?;
    
    // Final summary
    print_account_summary(&connection)?;
    
    connection.close()?;
    println!("\n✅ Advanced transaction examples completed!");
}

// Setup initial account data
sus initial_account_setup(connection: &DatabaseConnection) -> Result<(), DatabaseError> {
    println!("🏗️ Setting up initial accounts...");
    
    let insert_account = "INSERT INTO accounts (account_number, owner_name, balance, account_type) VALUES (?, ?, ?, ?)";
    
    // Create several test accounts
    let accounts = [
        ("ACC001", "Alice Johnson", 5000.00, "checking"),
        ("ACC002", "Bob Smith", 3500.00, "checking"),
        ("ACC003", "Charlie Brown", 10000.00, "savings"),
        ("ACC004", "Diana Prince", 2500.00, "checking"),
        ("ACC005", "Eve Wilson", 15000.00, "savings"),
    ];
    
    periodt (account_num, owner, balance, acc_type) in accounts {
        connection.execute(insert_account, [account_num, owner, balance, acc_type])?;
    }
    
    println!("✅ Created {} accounts", accounts.len());
    facts
}

// Example 1: Basic money transfer with transaction
sus money_transfer_example(connection: &DatabaseConnection) -> Result<(), DatabaseError> {
    let from_account = "ACC001";
    let to_account = "ACC002";
    let amount = 1000.00;
    
    println!("💰 Transferring ${} from {} to {}", amount, from_account, to_account);
    
    // Start transaction
    let mut transaction = connection.begin_transaction()?;
    
    bestie {
        // Check sender balance first
        let balance_check = "SELECT balance FROM accounts WHERE account_number = ?";
        let result = transaction.query(balance_check, [from_account])?;
        
        lowkey result.row_count() == 0 {
            yolo DatabaseError::new("Account not found");
        }
        
        let sender_balance: f64 = result.rows()[0].get("balance")?;
        
        lowkey sender_balance < amount {
            yolo DatabaseError::new("Insufficient funds");
        }
        
        println!("✅ Sender has sufficient balance: ${}", sender_balance);
        
        // Debit sender account
        let debit_sql = "UPDATE accounts SET balance = balance - ? WHERE account_number = ?";
        let debit_result = transaction.execute(debit_sql, [amount, from_account])?;
        
        lowkey debit_result.rows_affected() != 1 {
            yolo DatabaseError::new("Failed to debit sender account");
        }
        
        // Credit receiver account  
        let credit_sql = "UPDATE accounts SET balance = balance + ? WHERE account_number = ?";
        let credit_result = transaction.execute(credit_sql, [amount, to_account])?;
        
        lowkey credit_result.rows_affected() != 1 {
            yolo DatabaseError::new("Failed to credit receiver account");
        }
        
        // Log the transaction
        let log_sql = "INSERT INTO transaction_log (from_account, to_account, amount, transaction_type, description) VALUES (?, ?, ?, ?, ?)";
        transaction.execute(log_sql, [
            from_account,
            to_account, 
            amount,
            "transfer",
            format!("Transfer from {} to {}", from_account, to_account)
        ])?;
        
        // Commit transaction
        transaction.commit()?;
        println!("✅ Transfer completed successfully");
        
        // Verify final balances
        let verify_sql = "SELECT account_number, balance FROM accounts WHERE account_number IN (?, ?)";
        let verify_result = connection.query(verify_sql, [from_account, to_account])?;
        
        periodt row in verify_result.rows() {
            let acc_num: tea = row.get("account_number")?;
            let balance: f64 = row.get("balance")?;
            println!("  {} balance: ${:.2}", acc_num, balance);
        }
        
    } flex error {
        // Rollback on any error
        transaction.rollback()?;
        println!("❌ Transfer failed, transaction rolled back: {}", error);
    }
    
    facts
}

// Example 2: Complex transaction with savepoints
sus complex_business_transaction(connection: &DatabaseConnection) -> Result<(), DatabaseError> {
    println!("🏢 Processing complex business transaction with multiple steps...");
    
    let mut transaction = connection.begin_transaction()?;
    
    bestie {
        // Step 1: Apply monthly fees to all checking accounts
        println!("Step 1: Applying monthly maintenance fees...");
        
        let fee_amount = 10.00;
        let apply_fees = "UPDATE accounts SET balance = balance - ? WHERE account_type = 'checking' AND balance >= ?";
        let fee_result = transaction.execute(apply_fees, [fee_amount, fee_amount])?;
        
        println!("✅ Applied fees to {} accounts", fee_result.rows_affected());
        
        // Create savepoint after fees
        let savepoint1 = transaction.savepoint("after_fees")?;
        
        // Step 2: Apply interest to savings accounts
        println!("Step 2: Applying interest to savings accounts...");
        
        let interest_rate = 0.02; // 2% monthly (example)
        let apply_interest = "UPDATE accounts SET balance = balance * (1 + ?) WHERE account_type = 'savings'";
        let interest_result = transaction.execute(apply_interest, [interest_rate])?;
        
        println!("✅ Applied interest to {} savings accounts", interest_result.rows_affected());
        
        // Create savepoint after interest
        let savepoint2 = transaction.savepoint("after_interest")?;
        
        // Step 3: Bonus processing (potentially risky operation)
        println!("Step 3: Processing account bonuses...");
        
        bestie {
            // Find accounts eligible for bonus (balance > 5000)
            let bonus_eligible = "SELECT account_number FROM accounts WHERE balance > ?";
            let eligible_result = transaction.query(bonus_eligible, [5000.00])?;
            
            lowkey eligible_result.row_count() == 0 {
                println!("No accounts eligible for bonus");
            } highkey {
                let bonus_amount = 100.00;
                
                // Apply bonus to eligible accounts
                periodt row in eligible_result.rows() {
                    let account_num: tea = row.get("account_number")?;
                    
                    let bonus_sql = "UPDATE accounts SET balance = balance + ? WHERE account_number = ?";
                    transaction.execute(bonus_sql, [bonus_amount, account_num])?;
                    
                    // Log bonus transaction
                    let log_sql = "INSERT INTO transaction_log (to_account, amount, transaction_type, description) VALUES (?, ?, ?, ?)";
                    transaction.execute(log_sql, [
                        account_num,
                        bonus_amount,
                        "bonus",
                        "Monthly account bonus"
                    ])?;
                }
                
                println!("✅ Applied bonus to {} accounts", eligible_result.row_count());
            }
            
        } flex bonus_error {
            // Rollback only the bonus step
            println!("⚠️ Bonus processing failed, rolling back to after interest: {}", bonus_error);
            transaction.rollback_to_savepoint(&savepoint2)?;
        }
        
        // Step 4: Final validation
        println!("Step 4: Validating all account balances...");
        
        let validation_sql = "SELECT COUNT(*) as negative_count FROM accounts WHERE balance < 0";
        let validation_result = transaction.query(validation_sql, [])?;
        let negative_count: normie = validation_result.rows()[0].get("negative_count")?;
        
        lowkey negative_count > 0 {
            println!("❌ Found {} accounts with negative balance, rolling back to after fees", negative_count);
            transaction.rollback_to_savepoint(&savepoint1)?;
        } highkey {
            println!("✅ All account balances valid");
        }
        
        // Commit the entire transaction
        transaction.commit()?;
        println!("✅ Complex business transaction completed successfully");
        
    } flex error {
        transaction.rollback()?;
        println!("❌ Complex transaction failed, all changes rolled back: {}", error);
    }
    
    facts
}

// Example 3: Batch processing with transaction optimization
sus batch_processing_example(connection: &DatabaseConnection) -> Result<(), DatabaseError> {
    println!("📦 Processing large batch of transactions...");
    
    // Simulate processing many small transactions
    let transaction_batch = [
        ("ACC003", "ACC004", 50.00, "Small transfer 1"),
        ("ACC004", "ACC005", 75.00, "Small transfer 2"),
        ("ACC005", "ACC003", 100.00, "Small transfer 3"),
        ("ACC002", "ACC003", 25.00, "Small transfer 4"),
        ("ACC003", "ACC001", 200.00, "Small transfer 5"),
    ];
    
    println!("Processing {} transactions in batch...", transaction_batch.len());
    
    // Use transaction for batch processing (much faster than individual commits)
    let mut transaction = connection.begin_transaction()?;
    
    let mut successful_transfers = 0;
    let mut failed_transfers = 0;
    
    periodt (from_acc, to_acc, amount, description) in transaction_batch {
        bestie {
            // Check balance
            let balance_check = "SELECT balance FROM accounts WHERE account_number = ?";
            let balance_result = transaction.query(balance_check, [from_acc])?;
            
            lowkey balance_result.row_count() == 0 {
                failed_transfers += 1;
                continue;
            }
            
            let current_balance: f64 = balance_result.rows()[0].get("balance")?;
            
            lowkey current_balance < amount {
                failed_transfers += 1;
                continue;
            }
            
            // Process transfer
            transaction.execute("UPDATE accounts SET balance = balance - ? WHERE account_number = ?", [amount, from_acc])?;
            transaction.execute("UPDATE accounts SET balance = balance + ? WHERE account_number = ?", [amount, to_acc])?;
            
            // Log transaction
            transaction.execute("INSERT INTO transaction_log (from_account, to_account, amount, transaction_type, description) VALUES (?, ?, ?, ?, ?)", [
                from_acc, to_acc, amount, "batch_transfer", description
            ])?;
            
            successful_transfers += 1;
            
        } flex transfer_error {
            println!("⚠️ Individual transfer failed: {}", transfer_error);
            failed_transfers += 1;
        }
    }
    
    transaction.commit()?;
    
    println!("✅ Batch processing completed:");
    println!("  Successful transfers: {}", successful_transfers);
    println!("  Failed transfers: {}", failed_transfers);
    
    facts
}

// Example 4: Transaction rollback on error
sus error_rollback_example(connection: &DatabaseConnection) -> Result<(), DatabaseError> {
    println!("🚨 Demonstrating transaction rollback on error...");
    
    let mut transaction = connection.begin_transaction()?;
    
    bestie {
        // Valid operation
        transaction.execute("UPDATE accounts SET balance = balance + 500 WHERE account_number = ?", ["ACC001"])?;
        println!("✅ Added $500 to ACC001");
        
        // Another valid operation
        transaction.execute("UPDATE accounts SET balance = balance + 300 WHERE account_number = ?", ["ACC002"])?;
        println!("✅ Added $300 to ACC002");
        
        // Intentionally cause an error (trying to set negative balance)
        transaction.execute("UPDATE accounts SET balance = -1000 WHERE account_number = ?", ["ACC003"])?;
        
        // This should not be reached due to CHECK constraint
        transaction.commit()?;
        
    } flex error {
        // Rollback all changes
        transaction.rollback()?;
        println!("❌ Error occurred, all changes rolled back: {}", error);
        println!("💡 Account balances remain unchanged due to rollback");
    }
    
    facts
}

// Example 5: Read-only transaction for consistent reporting
sus reporting_transaction_example(connection: &DatabaseConnection) -> Result<(), DatabaseError> {
    println!("📊 Generating consistent financial report...");
    
    // Use read-only transaction to ensure consistent snapshot
    let mut transaction = connection.begin_read_only_transaction()?;
    
    // Get account summary
    let account_summary = "
        SELECT 
            account_type,
            COUNT(*) as account_count,
            SUM(balance) as total_balance,
            AVG(balance) as average_balance,
            MIN(balance) as min_balance,
            MAX(balance) as max_balance
        FROM accounts 
        GROUP BY account_type
        ORDER BY account_type
    ";
    
    let summary_result = transaction.query(account_summary, [])?;
    
    println!("\n📈 Account Summary Report:");
    println!("─".repeat(80));
    
    periodt row in summary_result.rows() {
        let acc_type: tea = row.get("account_type")?;
        let count: normie = row.get("account_count")?;
        let total: f64 = row.get("total_balance")?;
        let avg: f64 = row.get("average_balance")?;
        let min: f64 = row.get("min_balance")?;
        let max: f64 = row.get("max_balance")?;
        
        println!("Account Type: {}", acc_type.to_uppercase());
        println!("  Count: {} accounts", count);
        println!("  Total Balance: ${:.2}", total);
        println!("  Average Balance: ${:.2}", avg);
        println!("  Balance Range: ${:.2} - ${:.2}", min, max);
        println!();
    }
    
    // Get recent transaction activity
    let recent_transactions = "
        SELECT 
            transaction_type,
            COUNT(*) as transaction_count,
            SUM(amount) as total_amount
        FROM transaction_log 
        WHERE created_at >= datetime('now', '-1 day')
        GROUP BY transaction_type
        ORDER BY total_amount DESC
    ";
    
    let activity_result = transaction.query(recent_transactions, [])?;
    
    println!("📊 Recent Transaction Activity:");
    println!("─".repeat(80));
    
    periodt row in activity_result.rows() {
        let txn_type: tea = row.get("transaction_type")?;
        let count: normie = row.get("transaction_count")?;
        let total: f64 = row.get("total_amount")?;
        
        println!("  {}: {} transactions, ${:.2} total", txn_type, count, total);
    }
    
    transaction.commit()?; // Read-only commit (releases locks)
    
    facts
}

// Helper function to print account summary
sus print_account_summary(connection: &DatabaseConnection) -> Result<(), DatabaseError> {
    println!("\n💰 Final Account Summary:");
    println!("─".repeat(60));
    
    let summary_sql = "SELECT account_number, owner_name, balance, account_type FROM accounts ORDER BY account_number";
    let result = connection.query(summary_sql, [])?;
    
    periodt row in result.rows() {
        let acc_num: tea = row.get("account_number")?;
        let owner: tea = row.get("owner_name")?;
        let balance: f64 = row.get("balance")?;
        let acc_type: tea = row.get("account_type")?;
        
        println!("{} | {} | ${:.2} | {}", acc_num, owner, balance, acc_type);
    }
    
    // Total balances
    let total_result = connection.query("SELECT SUM(balance) as total FROM accounts", [])?;
    let total_balance: f64 = total_result.rows()[0].get("total")?;
    
    println!("─".repeat(60));
    println!("Total Bank Balance: ${:.2}", total_balance);
    
    facts
}
