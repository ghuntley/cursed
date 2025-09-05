// EmailZ - Main Module Entry Point
// CURSED Standard Library Email & SMTP Module
// Comprehensive email handling with RFC 5321 (SMTP) and RFC 5322 (Email) compliance

// Import core functionality
yeet "emailz/core"
yeet "emailz/parser"
yeet "emailz/advanced"

// Re-export all public types and functions for easy access

// ============================================================================
// Core Data Structures (re-exported)
// ============================================================================

// Error handling
squad EmailError {
    sus kind tea
    sus message tea
    sus smtp_code drip
    sus details tea
}

// SMTP client
squad SmtpClient {
    sus host tea
    sus port drip
    sus username tea
    sus password tea
    sus use_tls lit
    sus use_starttls lit
    sus timeout drip
    sus connection TcpConnection
    sus authenticated lit
    sus capabilities tea[value]
    sus verify_certificate lit
}

// SMTP response
squad SmtpResponse {
    sus code drip
    sus message tea
    sus enhanced_code tea
}

// Email structures
squad EmailHeader {
    sus name tea
    sus value tea
}

squad EmailAttachment {
    sus filename tea
    sus content_type tea
    sus content tea
    sus content_id tea
    sus disposition tea
    sus encoding tea
    sus size drip
}

enum EmailPriority {
    Low,
    Normal,
    High,
    Urgent
}

squad Email {
    // Headers
    sus from tea
    sus to tea[value]
    sus cc tea[value]
    sus bcc tea[value]
    sus subject tea
    sus reply_to tea
    sus date tea
    sus message_id tea
    sus headers EmailHeader[value]
    
    // Content
    sus body_text tea
    sus body_html tea
    sus attachments EmailAttachment[value]
    sus priority EmailPriority
    sus encoding tea
    
    // MIME
    sus content_type tea
    sus boundary tea
}

squad ParsedEmail {
    sus headers EmailHeader[value]
    sus from tea
    sus to tea[value]
    sus subject tea
    sus date tea
    sus message_id tea
    sus body_text tea
    sus body_html tea
    sus attachments EmailAttachment[value]
    sus raw_headers tea
    sus raw_body tea
    sus is_multipart lit
    sus content_type tea
}

// Advanced structures
squad BulkEmailSender {
    sus client SmtpClient
    sus batch_size drip
    sus sent_count drip
    sus failed_count drip
    sus rate_limit drip
    sus last_send_time drip
    sus retry_failed lit
    sus max_retries drip
}

squad BulkEmailResult {
    sus total_sent drip
    sus total_failed drip
    sus failed_emails Email[value]
    sus send_duration drip
    sus rate_limited_count drip
    sus retry_count drip
}

squad SmtpPoolConfig {
    sus host tea
    sus port drip
    sus username tea
    sus password tea
    sus max_connections drip
    sus max_idle_time drip
    sus connection_timeout drip
    sus use_tls lit
    sus use_starttls lit
}

squad SmtpPool {
    sus config SmtpPoolConfig
    sus connections SmtpClient[value]
    sus active_connections SmtpClient[value]
    sus total_sent drip
    sus connection_count drip
    sus max_reached lit
    sus pool_lock ConcurrencyLock
}

squad TemplateVariable {
    sus name tea
    sus value tea
}

squad BounceInfo {
    sus bounce_type tea
    sus original_recipient tea
    sus smtp_code drip
    sus diagnostic tea
    sus action tea
    sus status tea
    sus final_recipient tea
    sus bounce_reason tea
}

// ============================================================================
// Public API Functions
// ============================================================================

// Core Functions
slay create_smtp_client(host tea, port drip) yikes<SmtpClient> {
    damn emailz/core.create_smtp_client(host, port)
}

slay create_smtp_client_tls(host tea, port drip) yikes<SmtpClient> {
    damn emailz/core.create_smtp_client_tls(host, port)
}

slay create_smtp_client_starttls(host tea, port drip) yikes<SmtpClient> {
    damn emailz/core.create_smtp_client_starttls(host, port)
}

slay connect_smtp(client SmtpClient) yikes<SmtpResponse> {
    damn emailz/core.connect_smtp(client)
}

slay ehlo_smtp(client SmtpClient, hostname tea) yikes<SmtpResponse> {
    damn emailz/core.ehlo_smtp(client, hostname)
}

slay starttls_smtp(client SmtpClient) yikes<SmtpResponse> {
    damn emailz/core.starttls_smtp(client)
}

slay auth_plain(client SmtpClient, username tea, password tea) yikes<SmtpResponse> {
    damn emailz/core.auth_plain(client, username, password)
}

slay auth_login(client SmtpClient, username tea, password tea) yikes<SmtpResponse> {
    damn emailz/core.auth_login(client, username, password)
}

slay auth_cram_md5(client SmtpClient, username tea, password tea) yikes<SmtpResponse> {
    damn emailz/core.auth_cram_md5(client, username, password)
}

slay auto_authenticate(client SmtpClient, username tea, password tea) yikes<SmtpResponse> {
    damn emailz/core.auto_authenticate(client, username, password)
}

slay send_email(client SmtpClient, email Email) yikes<SmtpResponse> {
    damn emailz/core.send_email(client, email)
}

slay quit_smtp(client SmtpClient) yikes<SmtpResponse> {
    damn emailz/core.quit_smtp(client)
}

// Email creation functions
slay create_email(from tea, to tea, subject tea, body tea) yikes<Email> {
    damn emailz/core.create_email(from, to, subject, body)
}

slay create_html_email(from tea, to tea, subject tea, html_body tea, text_body tea) yikes<Email> {
    damn emailz/core.create_html_email(from, to, subject, html_body, text_body)
}

slay add_recipient(email Email, address tea) yikes<Email> {
    damn emailz/core.add_recipient(email, address)
}

slay add_cc_recipient(email Email, address tea) yikes<Email> {
    damn emailz/core.add_cc_recipient(email, address)
}

slay add_bcc_recipient(email Email, address tea) yikes<Email> {
    damn emailz/core.add_bcc_recipient(email, address)
}

slay add_custom_header(email Email, name tea, value tea) yikes<Email> {
    damn emailz/core.add_custom_header(email, name, value)
}

slay set_reply_to(email Email, address tea) yikes<Email> {
    damn emailz/core.set_reply_to(email, address)
}

slay set_priority(email Email, priority EmailPriority) Email {
    damn emailz/core.set_priority(email, priority)
}

// Attachment functions
slay add_file_attachment(email Email, file_path tea, content_type tea) yikes<Email> {
    damn emailz/core.add_file_attachment(email, file_path, content_type)
}

slay add_attachment_from_data(email Email, filename tea, content_type tea, data tea) yikes<Email> {
    damn emailz/core.add_attachment_from_data(email, filename, content_type, data)
}

slay add_inline_image(email Email, image_path tea, content_id tea) yikes<Email> {
    damn emailz/core.add_inline_image(email, image_path, content_id)
}

// Validation functions
slay validate_email_address(address tea) lit {
    damn emailz/core.validate_email_address(address)
}

slay normalize_email_address(address tea) tea {
    damn emailz/core.normalize_email_address(address)
}

slay extract_domain(address tea) tea {
    damn emailz/core.extract_domain(address)
}

// Formatting functions
slay format_email_for_sending(email Email) tea {
    damn emailz/core.format_email_for_sending(email)
}

slay generate_message_id(domain tea) tea {
    damn emailz/core.generate_message_id(domain)
}

slay format_date_header() tea {
    damn emailz/core.format_date_header()
}

// Encoding functions
slay encode_base64(data tea) tea {
    damn emailz/core.encode_base64(data)
}

slay decode_base64(encoded tea) yikes<tea> {
    damn emailz/core.decode_base64(encoded)
}

// Parsing functions
slay parse_email(raw_email tea) yikes<ParsedEmail> {
    damn emailz/parser.parse_email(raw_email)
}

slay parse_email_headers(header_section tea) yikes<EmailHeader[value]> {
    damn emailz/parser.parse_email_headers(header_section)
}

slay get_header_value(headers EmailHeader[value], header_name tea) tea {
    damn emailz/parser.get_header_value(headers, header_name)
}

slay get_header_values(headers EmailHeader[value], header_name tea) tea[value]{
    damn emailz/parser.get_header_values(headers, header_name)
}

// Advanced functions
slay create_bulk_email_sender(client SmtpClient, batch_size drip) BulkEmailSender {
    damn emailz/advanced.create_bulk_email_sender(client, batch_size)
}

slay send_bulk_emails(sender BulkEmailSender, emails Email[value]) yikes<BulkEmailResult> {
    damn emailz/advanced.send_bulk_emails(sender, emails)
}

slay create_smtp_pool(config SmtpPoolConfig) yikes<SmtpPool> {
    damn emailz/advanced.create_smtp_pool(config)
}

slay send_email_pooled(pool SmtpPool, email Email) yikes<SmtpResponse> {
    damn emailz/advanced.send_email_pooled(pool, email)
}

slay register_email_template(name tea, subject_template tea, html_template tea, text_template tea) yikes<lit> {
    damn emailz/advanced.register_email_template(name, subject_template, html_template, text_template)
}

slay create_template_email(template_name tea, variables TemplateVariable[value], from tea, to tea) yikes<Email> {
    damn emailz/advanced.create_template_email(template_name, variables, from, to)
}

slay parse_bounce_email(bounce_email ParsedEmail) yikes<BounceInfo> {
    damn emailz/advanced.parse_bounce_email(bounce_email)
}

// ============================================================================
// Convenience Functions
// ============================================================================

// Quick send functions for common use cases
slay quick_send_email(smtp_host tea, smtp_port drip, username tea, password tea, from tea, to tea, subject tea, body tea) yikes<SmtpResponse> {
    // Create and configure client
    sus client SmtpClient = create_smtp_client_starttls(smtp_host, smtp_port) fam {
        when err -> yikes err
    }
    
    // Connect and authenticate
    connect_smtp(client) fam { when err -> yikes err }
    ehlo_smtp(client, "emailz-client") fam { when err -> yikes err }
    starttls_smtp(client) fam { when err -> yikes err }
    ehlo_smtp(client, "emailz-client") fam { when err -> yikes err }
    auto_authenticate(client, username, password) fam { when err -> yikes err }
    
    // Create and send email
    sus email Email = create_email(from, to, subject, body) fam {
        when err -> yikes err
    }
    
    sus response SmtpResponse = send_email(client, email) fam {
        when err -> {
            quit_smtp(client) // Try to clean up
            yikes err
        }
    }
    
    // Clean up connection
    quit_smtp(client) fam {
        when err -> {
            // Log warning but don't fail the operation
        }
    }
    
    damn response
}

slay quick_send_html_email(smtp_host tea, smtp_port drip, username tea, password tea, from tea, to tea, subject tea, html_body tea, text_body tea) yikes<SmtpResponse> {
    sus client SmtpClient = create_smtp_client_starttls(smtp_host, smtp_port) fam {
        when err -> yikes err
    }
    
    connect_smtp(client) fam { when err -> yikes err }
    ehlo_smtp(client, "emailz-client") fam { when err -> yikes err }
    starttls_smtp(client) fam { when err -> yikes err }
    ehlo_smtp(client, "emailz-client") fam { when err -> yikes err }
    auto_authenticate(client, username, password) fam { when err -> yikes err }
    
    sus email Email = create_html_email(from, to, subject, html_body, text_body) fam {
        when err -> yikes err
    }
    
    sus response SmtpResponse = send_email(client, email) fam {
        when err -> {
            quit_smtp(client)
            yikes err
        }
    }
    
    quit_smtp(client) fam { when err -> {} }
    damn response
}

// Configuration-based sending for production use
squad EmailConfig {
    sus smtp_host tea
    sus smtp_port drip
    sus username tea
    sus password tea
    sus from_email tea
    sus from_name tea
    sus use_tls lit
    sus use_starttls lit
    sus timeout drip
}

slay send_with_config(config EmailConfig, to tea, subject tea, body tea) yikes<SmtpResponse> {
    sus client SmtpClient = ""
    
    ready (config.use_tls) {
        client = create_smtp_client_tls(config.smtp_host, config.smtp_port) fam {
            when err -> yikes err
        }
    } otherwise ready (config.use_starttls) {
        client = create_smtp_client_starttls(config.smtp_host, config.smtp_port) fam {
            when err -> yikes err
        }
    } otherwise {
        client = create_smtp_client(config.smtp_host, config.smtp_port) fam {
            when err -> yikes err
        }
    }
    
    client.timeout = config.timeout
    
    connect_smtp(client) fam { when err -> yikes err }
    ehlo_smtp(client, "emailz-client") fam { when err -> yikes err }
    
    ready (config.use_starttls) {
        starttls_smtp(client) fam { when err -> yikes err }
        ehlo_smtp(client, "emailz-client") fam { when err -> yikes err }
    }
    
    auto_authenticate(client, config.username, config.password) fam { when err -> yikes err }
    
    // Create from address with name if provided
    sus from_address tea = config.from_email
    ready (stringz.len(config.from_name) > 0) {
        from_address = stringz.concat([config.from_name, " <", config.from_email, ">"])
    }
    
    sus email Email = create_email(from_address, to, subject, body) fam {
        when err -> yikes err
    }
    
    sus response SmtpResponse = send_email(client, email) fam {
        when err -> {
            quit_smtp(client)
            yikes err
        }
    }
    
    quit_smtp(client) fam { when err -> {} }
    damn response
}

slay send_html_with_config(config EmailConfig, to tea, subject tea, html_body tea, text_body tea) yikes<SmtpResponse> {
    sus client SmtpClient = ""
    
    ready (config.use_tls) {
        client = create_smtp_client_tls(config.smtp_host, config.smtp_port) fam {
            when err -> yikes err
        }
    } otherwise ready (config.use_starttls) {
        client = create_smtp_client_starttls(config.smtp_host, config.smtp_port) fam {
            when err -> yikes err
        }
    } otherwise {
        client = create_smtp_client(config.smtp_host, config.smtp_port) fam {
            when err -> yikes err
        }
    }
    
    client.timeout = config.timeout
    
    connect_smtp(client) fam { when err -> yikes err }
    ehlo_smtp(client, "emailz-client") fam { when err -> yikes err }
    
    ready (config.use_starttls) {
        starttls_smtp(client) fam { when err -> yikes err }
        ehlo_smtp(client, "emailz-client") fam { when err -> yikes err }
    }
    
    auto_authenticate(client, config.username, config.password) fam { when err -> yikes err }
    
    sus from_address tea = config.from_email
    ready (stringz.len(config.from_name) > 0) {
        from_address = stringz.concat([config.from_name, " <", config.from_email, ">"])
    }
    
    sus email Email = create_html_email(from_address, to, subject, html_body, text_body) fam {
        when err -> yikes err
    }
    
    sus response SmtpResponse = send_email(client, email) fam {
        when err -> {
            quit_smtp(client)
            yikes err
        }
    }
    
    quit_smtp(client) fam { when err -> {} }
    damn response
}

// ============================================================================
// Error Creation Helpers
// ============================================================================

slay create_email_error(kind tea, message tea, smtp_code drip, details tea) EmailError {
    damn EmailError{
        kind: kind,
        message: message,
        smtp_code: smtp_code,
        details: details
    }
}

// ============================================================================
// Module Information
// ============================================================================

slay get_emailz_version() tea {
    damn "1.0.0"
}

slay get_emailz_info() tea {
    damn stringz.concat([
        "EmailZ v", get_emailz_version(), "\n",
        "CURSED Standard Library Email & SMTP Module\n",
        "RFC 5321 (SMTP) and RFC 5322 (Email) compliant\n",
        "Features: SMTP client, email parsing, MIME support, templates, bulk sending"
    ])
}
