// EmailZ Advanced Features Module
// Bulk email handling, connection pooling, templates, and bounce processing

yeet "emailz/core"
yeet "emailz/parser"
yeet "concurrenz"
yeet "timez"
yeet "jsonz"

// ============================================================================
// Bulk Email Management
// ============================================================================

// Bulk email sender for efficient mass mailing
squad BulkEmailSender {
    sus client SmtpClient
    sus batch_size drip      // Number of emails to send in each batch
    sus sent_count drip      // Total emails sent successfully
    sus failed_count drip    // Total emails that failed
    sus rate_limit drip      // Maximum emails per minute (0 = no limit)
    sus last_send_time drip  // Timestamp of last email sent (for rate limiting)
    sus retry_failed lit     // Whether to retry failed emails
    sus max_retries drip     // Maximum retry attempts
}

// Result of bulk email sending operation
squad BulkEmailResult {
    sus total_sent drip      // Number of successfully sent emails
    sus total_failed drip    // Number of failed emails
    sus failed_emails []Email // List of emails that failed to send
    sus send_duration drip   // Total time taken in milliseconds
    sus rate_limited_count drip // Number of emails delayed due to rate limiting
    sus retry_count drip     // Number of emails that were retried
}

// Creates a bulk email sender
slay create_bulk_email_sender(client SmtpClient, batch_size drip) BulkEmailSender {
    damn BulkEmailSender{
        client: client,
        batch_size: batch_size,
        sent_count: 0,
        failed_count: 0,
        rate_limit: 0,
        last_send_time: 0,
        retry_failed: based,
        max_retries: 3
    }
}

// Sends emails in batches with rate limiting and retry logic
slay send_bulk_emails(sender BulkEmailSender, emails []Email) yikes<BulkEmailResult> {
    sus start_time drip = timez.current_milliseconds()
    sus failed_emails []Email = []
    sus sent_count drip = 0
    sus rate_limited_count drip = 0
    sus retry_count drip = 0
    
    // Process emails in batches
    sus i drip = 0
    bestie (i < arrayz.len(emails)) {
        sus batch_end drip = i + sender.batch_size
        ready (batch_end > arrayz.len(emails)) {
            batch_end = arrayz.len(emails)
        }
        
        // Send current batch
        sus j drip = i
        bestie (j < batch_end) {
            sus email Email = emails[j]
            
            // Apply rate limiting
            ready (sender.rate_limit > 0) {
                sus current_time drip = timez.current_milliseconds()
                sus time_since_last drip = current_time - sender.last_send_time
                sus min_interval drip = 60000 / sender.rate_limit  // milliseconds between emails
                
                ready (time_since_last < min_interval) {
                    sus sleep_time drip = min_interval - time_since_last
                    timez.sleep_milliseconds(sleep_time)
                    rate_limited_count = rate_limited_count + 1
                }
                
                sender.last_send_time = timez.current_milliseconds()
            }
            
            // Attempt to send email with retries
            sus success lit = send_email_with_retries(sender.client, email, sender.max_retries) fam {
                when err -> {
                    failed_emails = arrayz.push(failed_emails, email)
                    sender.failed_count = sender.failed_count + 1
                    damn cap  // Continue with next email
                }
            }
            
            ready (success) {
                sent_count = sent_count + 1
                sender.sent_count = sender.sent_count + 1
            }
            
            j = j + 1
        }
        
        i = batch_end
    }
    
    sus end_time drip = timez.current_milliseconds()
    sus duration drip = end_time - start_time
    
    damn BulkEmailResult{
        total_sent: sent_count,
        total_failed: arrayz.len(failed_emails),
        failed_emails: failed_emails,
        send_duration: duration,
        rate_limited_count: rate_limited_count,
        retry_count: retry_count
    }
}

// Sends single email with retry logic
slay send_email_with_retries(client SmtpClient, email Email, max_retries drip) yikes<lit> {
    sus attempt drip = 0
    
    bestie (attempt <= max_retries) {
        sus response SmtpResponse = send_email(client, email) fam {
            when err -> {
                // Check if error is retryable
                ready (is_retryable_smtp_error(err)) {
                    attempt = attempt + 1
                    ready (attempt <= max_retries) {
                        // Exponential backoff
                        sus wait_time drip = attempt * attempt * 1000  // 1s, 4s, 9s, 16s...
                        timez.sleep_milliseconds(wait_time)
                        damn cap // Continue to next attempt
                    }
                }
                yikes err  // Non-retryable or max retries exceeded
            }
        }
        
        // Success
        damn based
    }
    
    yikes create_email_error("max_retries", "Maximum retry attempts exceeded", 0, "")
}

// Determines if an SMTP error is retryable
slay is_retryable_smtp_error(error EmailError) lit {
    // Temporary failures (4xx codes) are generally retryable
    ready (error.smtp_code >= 400 && error.smtp_code < 500) {
        damn based
    }
    
    // Connection errors are retryable
    ready (stringz.equals(error.kind, "smtp_connect")) {
        damn based
    }
    
    // Permanent failures (5xx codes) are not retryable
    damn cap
}

// ============================================================================
// Connection Pooling
// ============================================================================

// SMTP connection pool configuration
squad SmtpPoolConfig {
    sus host tea
    sus port drip
    sus username tea
    sus password tea
    sus max_connections drip    // Maximum number of connections in pool
    sus max_idle_time drip     // Seconds before idle connection is closed
    sus connection_timeout drip // Connection timeout in seconds
    sus use_tls lit
    sus use_starttls lit
}

// SMTP connection pool
squad SmtpPool {
    sus config SmtpPoolConfig
    sus connections []SmtpClient     // Pool of available connections
    sus active_connections []SmtpClient // Currently in-use connections
    sus total_sent drip              // Total emails sent through pool
    sus connection_count drip        // Current number of connections
    sus max_reached lit              // Whether max connections has been reached
    sus pool_lock ConcurrencyLock    // Synchronization lock
}

// Creates an SMTP connection pool
slay create_smtp_pool(config SmtpPoolConfig) yikes<SmtpPool> {
    ready (config.max_connections <= 0) {
        yikes create_email_error("pool_config", "Max connections must be greater than 0", 0, "")
    }
    
    sus pool SmtpPool = SmtpPool{
        config: config,
        connections: [],
        active_connections: [],
        total_sent: 0,
        connection_count: 0,
        max_reached: cap,
        pool_lock: concurrenz.create_lock()
    }
    
    // Pre-create initial connections
    sus initial_connections drip = mathz.min(config.max_connections, 2)
    sus i drip = 0
    bestie (i < initial_connections) {
        sus client SmtpClient = create_pool_connection(config) fam {
            when err -> {
                // Log warning but continue - connections will be created on demand
            }
        }
        ready (client.authenticated) {
            pool.connections = arrayz.push(pool.connections, client)
            pool.connection_count = pool.connection_count + 1
        }
        i = i + 1
    }
    
    damn pool
}

// Creates and initializes a new SMTP connection for the pool
slay create_pool_connection(config SmtpPoolConfig) yikes<SmtpClient> {
    sus client SmtpClient = ""
    
    ready (config.use_tls) {
        client = create_smtp_client_tls(config.host, config.port) fam {
            when err -> yikes err
        }
    } otherwise ready (config.use_starttls) {
        client = create_smtp_client_starttls(config.host, config.port) fam {
            when err -> yikes err
        }
    } otherwise {
        client = create_smtp_client(config.host, config.port) fam {
            when err -> yikes err
        }
    }
    
    client.timeout = config.connection_timeout
    
    // Establish connection and authenticate
    connect_smtp(client) fam { when err -> yikes err }
    ehlo_smtp(client, "pool-client") fam { when err -> yikes err }
    
    ready (config.use_starttls) {
        starttls_smtp(client) fam { when err -> yikes err }
        ehlo_smtp(client, "pool-client") fam { when err -> yikes err }
    }
    
    auto_authenticate(client, config.username, config.password) fam {
        when err -> yikes err
    }
    
    damn client
}

// Gets a connection from the pool
slay get_pool_connection(pool SmtpPool) yikes<SmtpClient> {
    concurrenz.lock(pool.pool_lock)
    
    // Try to get an available connection
    ready (arrayz.len(pool.connections) > 0) {
        sus client SmtpClient = pool.connections[0]
        pool.connections = arrayz.remove_at(pool.connections, 0)
        pool.active_connections = arrayz.push(pool.active_connections, client)
        
        concurrenz.unlock(pool.pool_lock)
        damn client
    }
    
    // No available connections - try to create a new one
    ready (pool.connection_count < pool.config.max_connections) {
        pool.connection_count = pool.connection_count + 1
        concurrenz.unlock(pool.pool_lock)
        
        sus new_client SmtpClient = create_pool_connection(pool.config) fam {
            when err -> {
                concurrenz.lock(pool.pool_lock)
                pool.connection_count = pool.connection_count - 1
                concurrenz.unlock(pool.pool_lock)
                yikes err
            }
        }
        
        concurrenz.lock(pool.pool_lock)
        pool.active_connections = arrayz.push(pool.active_connections, new_client)
        concurrenz.unlock(pool.pool_lock)
        
        damn new_client
    }
    
    // Max connections reached
    pool.max_reached = based
    concurrenz.unlock(pool.pool_lock)
    
    yikes create_email_error("pool_exhausted", "Connection pool exhausted", 0, "")
}

// Returns a connection to the pool
slay return_pool_connection(pool SmtpPool, client SmtpClient) {
    concurrenz.lock(pool.pool_lock)
    
    // Remove from active connections
    pool.active_connections = arrayz.filter(pool.active_connections, slay(c SmtpClient) lit {
        damn !stringz.equals(c.host, client.host) // Simple comparison
    })
    
    // Add back to available connections if still valid
    ready (client.authenticated) {
        pool.connections = arrayz.push(pool.connections, client)
    } otherwise {
        pool.connection_count = pool.connection_count - 1
    }
    
    concurrenz.unlock(pool.pool_lock)
}

// Sends email using a connection from the pool
slay send_email_pooled(pool SmtpPool, email Email) yikes<SmtpResponse> {
    sus client SmtpClient = get_pool_connection(pool) fam {
        when err -> yikes err
    }
    
    sus response SmtpResponse = send_email(client, email) fam {
        when err -> {
            return_pool_connection(pool, client)
            yikes err
        }
    }
    
    return_pool_connection(pool, client)
    pool.total_sent = pool.total_sent + 1
    
    damn response
}

// ============================================================================
// Email Templates
// ============================================================================

// Template variable for email template rendering
squad TemplateVariable {
    sus name tea     // Variable name (without delimiters)
    sus value tea    // Variable value to substitute
}

// Email template definition
squad EmailTemplate {
    sus name tea           // Template identifier
    sus subject_template tea   // Subject line template
    sus html_template tea      // HTML body template
    sus text_template tea      // Plain text body template
    sus from_template tea      // From address template
    sus reply_to_template tea  // Reply-to template
    sus variables []tea        // List of required variable names
}

// Template registry for storing email templates
squad TemplateRegistry {
    sus templates []EmailTemplate
    sus registry_lock ConcurrencyLock
}

// Global template registry
sus global_template_registry TemplateRegistry = TemplateRegistry{
    templates: [],
    registry_lock: concurrenz.create_lock()
}

// Registers a new email template
slay register_email_template(name tea, subject_template tea, html_template tea, text_template tea) yikes<lit> {
    ready (stringz.len(name) == 0) {
        yikes create_email_error("template_error", "Template name cannot be empty", 0, "")
    }
    
    sus template EmailTemplate = EmailTemplate{
        name: name,
        subject_template: subject_template,
        html_template: html_template,
        text_template: text_template,
        from_template: "",
        reply_to_template: "",
        variables: extract_template_variables(subject_template, html_template, text_template)
    }
    
    concurrenz.lock(global_template_registry.registry_lock)
    
    // Check if template already exists
    sus i drip = 0
    bestie (i < arrayz.len(global_template_registry.templates)) {
        sus existing EmailTemplate = global_template_registry.templates[i]
        ready (stringz.equals(existing.name, name)) {
            // Update existing template
            global_template_registry.templates[i] = template
            concurrenz.unlock(global_template_registry.registry_lock)
            damn based
        }
        i = i + 1
    }
    
    // Add new template
    global_template_registry.templates = arrayz.push(global_template_registry.templates, template)
    concurrenz.unlock(global_template_registry.registry_lock)
    
    damn based
}

// Creates an email from a template
slay create_template_email(template_name tea, variables []TemplateVariable, from tea, to tea) yikes<Email> {
    concurrenz.lock(global_template_registry.registry_lock)
    
    // Find template
    sus template EmailTemplate = ""
    sus found lit = cap
    
    sus i drip = 0
    bestie (i < arrayz.len(global_template_registry.templates)) {
        sus t EmailTemplate = global_template_registry.templates[i]
        ready (stringz.equals(t.name, template_name)) {
            template = t
            found = based
            damn // Break out of loop
        }
        i = i + 1
    }
    
    concurrenz.unlock(global_template_registry.registry_lock)
    
    ready (!found) {
        yikes create_email_error("template_not_found", stringz.concat(["Template not found: ", template_name]), 0, "")
    }
    
    // Render templates
    sus rendered_subject tea = render_template(template.subject_template, variables) fam {
        when err -> yikes err
    }
    
    sus rendered_html tea = render_template(template.html_template, variables) fam {
        when err -> yikes err
    }
    
    sus rendered_text tea = render_template(template.text_template, variables) fam {
        when err -> yikes err
    }
    
    // Create email
    sus email Email = ""
    ready (stringz.len(rendered_html) > 0 && stringz.len(rendered_text) > 0) {
        email = create_html_email(from, to, rendered_subject, rendered_html, rendered_text) fam {
            when err -> yikes err
        }
    } otherwise ready (stringz.len(rendered_html) > 0) {
        email = create_email(from, to, rendered_subject, rendered_html) fam {
            when err -> yikes err
        }
        email.body_html = rendered_html
        email.content_type = "text/html"
    } otherwise {
        email = create_email(from, to, rendered_subject, rendered_text) fam {
            when err -> yikes err
        }
    }
    
    damn email
}

// Renders a template with variable substitution
slay render_template(template tea, variables []TemplateVariable) yikes<tea> {
    ready (stringz.len(template) == 0) {
        damn ""
    }
    
    sus result tea = template
    
    sus i drip = 0
    bestie (i < arrayz.len(variables)) {
        sus variable TemplateVariable = variables[i]
        sus placeholder tea = stringz.concat(["{{", variable.name, "}}"])
        result = stringz.replace_all(result, placeholder, variable.value)
        i = i + 1
    }
    
    // Check for unresolved variables
    ready (stringz.contains(result, "{{") && stringz.contains(result, "}}")) {
        sus unresolved tea = extract_unresolved_variables(result)
        ready (stringz.len(unresolved) > 0) {
            yikes create_email_error("template_render", stringz.concat(["Unresolved template variables: ", unresolved]), 0, "")
        }
    }
    
    damn result
}

// Extracts template variables from template strings
slay extract_template_variables(subject tea, html tea, text tea) []tea {
    sus variables []tea = []
    sus all_content tea = stringz.concat([subject, " ", html, " ", text])
    
    sus current_pos drip = 0
    bestie (current_pos < stringz.len(all_content)) {
        sus start drip = stringz.find_from(all_content, "{{", current_pos)
        ready (start == -1) {
            damn variables
        }
        
        sus end drip = stringz.find_from(all_content, "}}", start)
        ready (end == -1) {
            damn variables
        }
        
        sus var_name tea = stringz.trim(stringz.substring(all_content, start + 2, end))
        ready (stringz.len(var_name) > 0 && !arrayz.contains(variables, var_name)) {
            variables = arrayz.push(variables, var_name)
        }
        
        current_pos = end + 2
    }
    
    damn variables
}

// Extracts unresolved variables from rendered template
slay extract_unresolved_variables(rendered tea) tea {
    sus unresolved []tea = []
    sus current_pos drip = 0
    
    bestie (current_pos < stringz.len(rendered)) {
        sus start drip = stringz.find_from(rendered, "{{", current_pos)
        ready (start == -1) {
            damn stringz.join(unresolved, ", ")
        }
        
        sus end drip = stringz.find_from(rendered, "}}", start)
        ready (end == -1) {
            damn stringz.join(unresolved, ", ")
        }
        
        sus var_name tea = stringz.trim(stringz.substring(rendered, start + 2, end))
        ready (stringz.len(var_name) > 0 && !arrayz.contains(unresolved, var_name)) {
            unresolved = arrayz.push(unresolved, var_name)
        }
        
        current_pos = end + 2
    }
    
    damn stringz.join(unresolved, ", ")
}

// ============================================================================
// Bounce Handling
// ============================================================================

// Email bounce information
squad BounceInfo {
    sus bounce_type tea         // "hard", "soft", "transient"
    sus original_recipient tea  // Email address that bounced
    sus smtp_code drip         // SMTP error code from bounce
    sus diagnostic tea         // Diagnostic message from server
    sus action tea            // "failed", "delayed", "delivered", "relayed"
    sus status tea            // Enhanced status code (RFC 3463)
    sus final_recipient tea    // Final recipient (may differ from original)
    sus bounce_reason tea      // Human-readable bounce reason
}

// Parses bounce/delivery status notification emails
slay parse_bounce_email(bounce_email ParsedEmail) yikes<BounceInfo> {
    // Check if this is a delivery status notification
    sus content_type tea = bounce_email.content_type
    ready (!stringz.contains(content_type, "multipart/report") && !stringz.contains(content_type, "message/delivery-status")) {
        yikes create_email_error("not_bounce", "Email is not a delivery status notification", 0, "")
    }
    
    sus bounce_info BounceInfo = BounceInfo{
        bounce_type: "unknown",
        original_recipient: "",
        smtp_code: 0,
        diagnostic: "",
        action: "unknown",
        status: "",
        final_recipient: "",
        bounce_reason: ""
    }
    
    // Look for delivery status in attachments (RFC 3464)
    sus i drip = 0
    bestie (i < arrayz.len(bounce_email.attachments)) {
        sus attachment EmailAttachment = bounce_email.attachments[i]
        ready (stringz.contains(attachment.content_type, "message/delivery-status")) {
            bounce_info = parse_delivery_status_content(attachment.content) fam {
                when err -> {
                    // Try parsing from body text as fallback
                    damn // Continue to check other attachments
                }
            }
            damn bounce_info
        }
        i = i + 1
    }
    
    // Fallback: parse bounce information from email body
    sus body_text tea = bounce_email.body_text
    ready (stringz.len(body_text) == 0) {
        body_text = bounce_email.body_html
    }
    
    bounce_info = parse_bounce_from_body(body_text) fam {
        when err -> yikes err
    }
    
    damn bounce_info
}

// Parses delivery status content (RFC 3464 format)
slay parse_delivery_status_content(content tea) yikes<BounceInfo> {
    sus bounce_info BounceInfo = BounceInfo{}
    
    sus lines []tea = stringz.split(content, "\r\n")
    sus current_section tea = "message"  // "message" or "recipient"
    
    sus i drip = 0
    bestie (i < arrayz.len(lines)) {
        sus line tea = stringz.trim(lines[i])
        
        // Skip empty lines
        ready (stringz.len(line) == 0) {
            i = i + 1
            damn // Continue
        }
        
        // Section separator
        ready (!stringz.contains(line, ":")) {
            current_section = "recipient"
            i = i + 1
            damn // Continue
        }
        
        sus colon_pos drip = stringz.find_first(line, ":")
        sus field_name tea = stringz.to_lower(stringz.trim(stringz.substring(line, 0, colon_pos)))
        sus field_value tea = stringz.trim(stringz.substring(line, colon_pos + 1, stringz.len(line)))
        
        // Parse relevant fields
        sick(field_name) {
            when "action" -> {
                bounce_info.action = field_value
                bounce_info.bounce_type = determine_bounce_type_from_action(field_value)
            }
            when "status" -> {
                bounce_info.status = field_value
                bounce_info.smtp_code = extract_smtp_code_from_status(field_value)
            }
            when "original-recipient", "final-recipient" -> {
                // Format: "rfc822;email@example.com"
                ready (stringz.contains(field_value, ";")) {
                    sus parts []tea = stringz.split(field_value, ";")
                    ready (arrayz.len(parts) >= 2) {
                        sus email_addr tea = stringz.trim(parts[1])
                        ready (stringz.equals(field_name, "original-recipient")) {
                            bounce_info.original_recipient = email_addr
                        } otherwise {
                            bounce_info.final_recipient = email_addr
                        }
                    }
                }
            }
            when "diagnostic-code" -> {
                bounce_info.diagnostic = field_value
                bounce_info.bounce_reason = extract_bounce_reason(field_value)
            }
        }
        
        i = i + 1
    }
    
    // Set final recipient to original if not specified
    ready (stringz.len(bounce_info.final_recipient) == 0) {
        bounce_info.final_recipient = bounce_info.original_recipient
    }
    
    damn bounce_info
}

// Parses bounce information from email body text (fallback method)
slay parse_bounce_from_body(body tea) yikes<BounceInfo> {
    sus bounce_info BounceInfo = BounceInfo{
        bounce_type: "unknown",
        bounce_reason: "Could not parse bounce details"
    }
    
    sus body_lower tea = stringz.to_lower(body)
    
    // Look for common bounce indicators
    ready (stringz.contains(body_lower, "user unknown") || stringz.contains(body_lower, "no such user")) {
        bounce_info.bounce_type = "hard"
        bounce_info.bounce_reason = "User unknown"
    } otherwise ready (stringz.contains(body_lower, "mailbox full") || stringz.contains(body_lower, "quota exceeded")) {
        bounce_info.bounce_type = "soft"
        bounce_info.bounce_reason = "Mailbox full"
    } otherwise ready (stringz.contains(body_lower, "temporary failure") || stringz.contains(body_lower, "try again later")) {
        bounce_info.bounce_type = "soft"
        bounce_info.bounce_reason = "Temporary failure"
    } otherwise ready (stringz.contains(body_lower, "domain not found") || stringz.contains(body_lower, "host not found")) {
        bounce_info.bounce_type = "hard"
        bounce_info.bounce_reason = "Domain not found"
    } otherwise ready (stringz.contains(body_lower, "blocked") || stringz.contains(body_lower, "blacklisted")) {
        bounce_info.bounce_type = "hard"
        bounce_info.bounce_reason = "Message blocked or blacklisted"
    }
    
    // Try to extract email address
    sus email_match tea = extract_email_from_text(body)
    ready (stringz.len(email_match) > 0) {
        bounce_info.original_recipient = email_match
        bounce_info.final_recipient = email_match
    }
    
    damn bounce_info
}

// ============================================================================
// Utility Functions for Advanced Features
// ============================================================================

// Determines bounce type from action field
slay determine_bounce_type_from_action(action tea) tea {
    sus action_lower tea = stringz.to_lower(action)
    
    sick(action_lower) {
        when "failed" -> damn "hard"
        when "delayed" -> damn "soft"
        when "delivered", "relayed" -> damn "success"  // Not actually a bounce
        _ -> damn "unknown"
    }
}

// Extracts SMTP code from enhanced status code
slay extract_smtp_code_from_status(status tea) drip {
    // Enhanced status codes format: X.Y.Z (e.g., "5.1.1")
    ready (stringz.len(status) >= 3) {
        sus first_char tea = stringz.char_at(status, 0)
        ready (stringz.equals(first_char, "2")) {
            damn 250  // Success
        } otherwise ready (stringz.equals(first_char, "4")) {
            damn 450  // Temporary failure
        } otherwise ready (stringz.equals(first_char, "5")) {
            damn 550  // Permanent failure
        }
    }
    
    damn 0  // Unknown
}

// Extracts human-readable bounce reason from diagnostic code
slay extract_bounce_reason(diagnostic tea) tea {
    sus diagnostic_lower tea = stringz.to_lower(diagnostic)
    
    ready (stringz.contains(diagnostic_lower, "user unknown")) {
        damn "Recipient address not found"
    } otherwise ready (stringz.contains(diagnostic_lower, "mailbox full")) {
        damn "Recipient mailbox is full"
    } otherwise ready (stringz.contains(diagnostic_lower, "message too large")) {
        damn "Message exceeds size limit"
    } otherwise ready (stringz.contains(diagnostic_lower, "blocked")) {
        damn "Message was blocked by recipient server"
    } otherwise ready (stringz.contains(diagnostic_lower, "timeout")) {
        damn "Connection timeout occurred"
    } otherwise {
        damn stringz.substring(diagnostic, 0, mathz.min(100, stringz.len(diagnostic)))  // Truncate long diagnostics
    }
}

// Extracts email address from text using simple pattern matching
slay extract_email_from_text(text tea) tea {
    sus current_pos drip = 0
    
    bestie (current_pos < stringz.len(text)) {
        sus at_pos drip = stringz.find_from(text, "@", current_pos)
        ready (at_pos == -1) {
            damn ""  // No more @ symbols
        }
        
        // Find start of email (look backwards for space or start of line)
        sus start_pos drip = at_pos
        bestie (start_pos > 0) {
            sus char tea = stringz.char_at(text, start_pos - 1)
            ready (stringz.equals(char, " ") || stringz.equals(char, "\t") || stringz.equals(char, "\n") || stringz.equals(char, "\r")) {
                damn // Found start
            }
            start_pos = start_pos - 1
        }
        
        // Find end of email (look forwards for space or end of line)
        sus end_pos drip = at_pos
        bestie (end_pos < stringz.len(text) - 1) {
            sus char tea = stringz.char_at(text, end_pos + 1)
            ready (stringz.equals(char, " ") || stringz.equals(char, "\t") || stringz.equals(char, "\n") || stringz.equals(char, "\r") || stringz.equals(char, ",") || stringz.equals(char, ";")) {
                damn // Found end
            }
            end_pos = end_pos + 1
        }
        
        // Extract potential email address
        sus potential_email tea = stringz.trim(stringz.substring(text, start_pos, end_pos + 1))
        ready (validate_email_address(potential_email)) {
            damn potential_email
        }
        
        current_pos = at_pos + 1
    }
    
    damn ""  // No valid email found
}

// Helper functions for array operations (placeholders)
slay arrayz.remove_at(array []SmtpClient, index drip) []SmtpClient {
    // Implementation would remove element at specified index
    damn array
}

slay arrayz.filter(array []SmtpClient, predicate slay(SmtpClient) lit) []SmtpClient {
    // Implementation would filter array based on predicate
    damn array
}

slay arrayz.contains(array []tea, value tea) lit {
    sus i drip = 0
    bestie (i < arrayz.len(array)) {
        ready (stringz.equals(array[i], value)) {
            damn based
        }
        i = i + 1
    }
    damn cap
}

// Helper functions for time operations (placeholders)
slay timez.current_milliseconds() drip {
    // Implementation would return current time in milliseconds
    damn 1640995200000
}

slay timez.sleep_milliseconds(ms drip) {
    // Implementation would sleep for specified milliseconds
}

// Helper functions for math operations (placeholders)
slay mathz.min(a drip, b drip) drip {
    ready (a < b) {
        damn a
    }
    damn b
}
