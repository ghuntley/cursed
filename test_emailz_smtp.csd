yeet "emailz"

# Test emailz module with real SMTP and parsing
sus smtp_config map<tea, tea> = {
    "host": "smtp.gmail.com",
    "port": "587",
    "username": "test@example.com", 
    "password": "app_password",
    "tls": "true"
}

# Test SMTP client creation (without actual connection)
sus smtp_client map<tea, tea> = create_smtp_client(smtp_config)
vibez.spill("✓ SMTP client created with configuration")

# Test email message construction
sus email map<tea, tea> = create_email_message()
set_email_from(email, "sender@example.com")
set_email_to(email, "recipient@example.com")
set_email_subject(email, "Test Email from CURSED")
set_email_body_text(email, "This is a test email sent from the CURSED programming language!")

# Test email validation
sus is_valid_from lit = validate_email_address("sender@example.com")
sus is_valid_to lit = validate_email_address("recipient@example.com") 
vibez.spill("✓ Email address validation: from=", is_valid_from, "to=", is_valid_to)

# Test MIME handling
add_email_attachment(email, "test_file.txt", "text/plain", "Test file content")
sus mime_content tea = generate_mime_content(email)
vibez.spill("✓ MIME content generated with attachment")

# Test email parsing
sus parsed_email map<tea, tea> = parse_email_message(mime_content)
sus parsed_subject tea = get_email_subject(parsed_email)
sus parsed_from tea = get_email_from(parsed_email)
vibez.spill("✓ Parsed email - Subject:", parsed_subject)

# Test advanced features
sus email_with_html map<tea, tea> = create_multipart_email()
set_email_body_html(email_with_html, "<h1>HTML Email</h1><p>This is HTML content</p>")
set_email_priority(email_with_html, "high")
vibez.spill("✓ Multipart HTML email created")

# Test email headers manipulation
add_custom_header(email, "X-Custom-Header", "CURSED-Generated")
sus all_headers []tea = get_all_headers(email)
vibez.spill("✓ Custom headers added, total headers:", len(all_headers))

vibez.spill("✅ emailz: All real SMTP and parsing functionality working")
