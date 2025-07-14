# SMTP Tea Module 📧☕

The `smtp_tea` module provides comprehensive email sending functionality for CURSED applications with Gen Z enhanced APIs. Send emails with pure CURSED energy! 🔥

## Features

- **SMTP Client Support**: Connect to any SMTP server (Gmail, Outlook, custom servers)
- **Multiple Authentication**: PLAIN, LOGIN, OAUTH2, and no-auth support
- **TLS/SSL Encryption**: Secure email transmission with STARTTLS and SSL
- **Rich Email Composition**: Plain text, HTML, multipart emails with attachments
- **Advanced Headers**: Priority levels, CC/BCC, custom headers, delivery tracking
- **Security Features**: Input sanitization, secure configuration validation
- **Bounce Handling**: Detect and handle bounced/undelivered emails
- **Bulk Email Support**: Send to multiple recipients efficiently
- **Gen Z APIs**: Vibe check emails, no cap notifications, and more!

## Quick Start

```cursed
yeet "smtp_tea"

# Send a simple email
sus success := smtp_tea.send_email(
    "smtp.gmail.com",           # SMTP host
    587,                        # SMTP port (TLS)
    "your-email@gmail.com",     # Username
    "your-app-password",        # Password/app password
    "sender@example.com",       # From address
    "recipient@example.com",    # To address
    "Hello from CURSED! 🔥",    # Subject
    "This email was sent with pure CURSED energy! No cap! 💯", # Body
    based                       # Use TLS
)

lowkey success {
    vibez.spill("Email sent successfully! ✨")
} simp {
    vibez.spill("Failed to send email 😢")
}
```

## Configuration

### SMTP Server Settings

```cursed
# Validate SMTP configuration
sus config_valid := smtp_tea.smtp_client_config(
    "smtp.gmail.com",           # Host
    587,                        # Port
    "username",                 # Username
    "password",                 # Password
    based,                      # Use TLS
    smtp_tea.AUTH_PLAIN         # Authentication method
)
```

### Common SMTP Providers

| Provider | Host | Port (TLS) | Port (SSL) | Auth Method |
|----------|------|------------|------------|-------------|
| Gmail | smtp.gmail.com | 587 | 465 | AUTH_PLAIN |
| Outlook | smtp-mail.outlook.com | 587 | 465 | AUTH_LOGIN |
| Yahoo | smtp.mail.yahoo.com | 587 | 465 | AUTH_PLAIN |
| Custom | your-smtp.com | 587 | 465 | AUTH_* |

## Authentication Methods

```cursed
# Available authentication methods
smtp_tea.AUTH_NONE     # No authentication (local/testing)
smtp_tea.AUTH_PLAIN    # PLAIN authentication (most common)
smtp_tea.AUTH_LOGIN    # LOGIN authentication (Outlook)
smtp_tea.AUTH_OAUTH2   # OAuth2 authentication (advanced)
```

## Email Composition

### Basic Email

```cursed
sus message := smtp_tea.create_email_message(
    "sender@example.com",       # From
    "recipient@example.com",    # To
    "Test Subject",             # Subject
    "Hello World!",             # Body
    "",                         # CC (empty)
    "",                         # BCC (empty)
    smtp_tea.PRIORITY_NORMAL    # Priority
)
```

### Email with CC/BCC

```cursed
sus message := smtp_tea.create_email_message(
    "sender@example.com",
    "primary@example.com",
    "Team Update",
    "Here's the latest update...",
    "cc1@example.com,cc2@example.com",     # CC recipients
    "bcc@example.com",                     # BCC recipients
    smtp_tea.PRIORITY_HIGH                 # High priority
)
```

### HTML Email

```cursed
sus html_body := "<html><body><h1>Welcome!</h1><p>Thanks for joining!</p></body></html>"
sus text_body := "Welcome!\\n\\nThanks for joining!"

sus html_message := smtp_tea.create_html_email(
    "noreply@company.com",
    "newuser@example.com",
    "Welcome to Our Service! 🎉",
    html_body,
    text_body
)
```

### Email with Attachments

```cursed
sus attachment_data := "File content here..."

sus message_with_attachment := smtp_tea.create_email_with_attachment(
    "sender@example.com",
    "recipient@example.com",
    "Document Attached",
    "Please find the attached document.",
    "document.txt",             # Filename
    attachment_data,            # File content
    "text/plain"               # MIME type
)
```

## Priority Levels

```cursed
smtp_tea.PRIORITY_LOW       # Low priority
smtp_tea.PRIORITY_NORMAL    # Normal priority (default)
smtp_tea.PRIORITY_HIGH      # High priority
smtp_tea.PRIORITY_URGENT    # Urgent priority
```

## Advanced Features

### Bulk Email Sending

```cursed
sus success_count := smtp_tea.send_bulk_emails(
    "smtp.gmail.com",
    587,
    "bulk-sender@company.com",
    "password",
    "sender@company.com",
    "user1@test.com,user2@test.com,user3@test.com",  # Recipients
    "Newsletter Update",
    "Here's this week's newsletter...",
    based                       # Use TLS
)

vibez.spill("Successfully sent to " + success_count + " recipients")
```

### Advanced Email with All Options

```cursed
sus advanced_success := smtp_tea.send_advanced_email(
    "smtp.company.com",         # SMTP host
    587,                        # Port
    "automated@company.com",    # Username
    "secure-password",          # Password
    smtp_tea.AUTH_LOGIN,        # Auth method
    "noreply@company.com",      # From
    "customer@example.com",     # To
    "manager@company.com",      # CC
    "audit@company.com",        # BCC
    "Important Update",         # Subject
    "This is an important notification...", # Body
    smtp_tea.PRIORITY_HIGH,     # Priority
    based                       # Use TLS
)
```

## Security Features

### Email Address Validation

```cursed
lowkey smtp_tea.validate_email_address("user@example.com") {
    vibez.spill("Valid email address")
} simp {
    vibez.spill("Invalid email address")
}
```

### Content Sanitization

```cursed
sus user_content := "<script>alert('xss')</script>Hello!"
sus safe_content := smtp_tea.sanitize_email_content(user_content)
# Result: "&lt;scriptHello!" (script tags sanitized)
```

### Configuration Security Validation

```cursed
sus is_secure := smtp_tea.validate_smtp_config_security(
    "remote-server.com",
    cap,                        # No TLS
    smtp_tea.AUTH_PLAIN         # With authentication
)
# Returns cap (false) - insecure configuration
```

## Bounce Handling

```cursed
sus email_content := "From: MAILER-DAEMON@server.com..."
lowkey smtp_tea.detect_bounce_email(email_content) {
    vibez.spill("This is a bounce email")
    # Handle bounce (remove from list, retry, etc.)
}
```

## Gen Z Enhanced APIs 🔥

### Vibe Check Email

```cursed
smtp_tea.send_vibe_check_email(
    "smtp.gmail.com",
    587,
    "vibe-bot@company.com",
    "password",
    "vibe-bot@company.com",
    "bestie@example.com",
    "immaculate ✨"             # Vibe level
)
```

### No Cap Notifications

```cursed
smtp_tea.send_no_cap_notification(
    "smtp.gmail.com",
    587,
    "alerts@company.com",
    "password",
    "alerts@company.com",
    "admin@company.com",
    "Server is down! This is not a drill! 🚨"
)
```

## Email Templates

```cursed
# Create template
smtp_tea.create_email_template("welcome", "Hello {name}! Welcome to {service}!")

# Apply template with variables
sus personalized := smtp_tea.apply_email_template("welcome", "name=John,service=CURSED")
```

## Tracking and Analytics

```cursed
# Track email opens
smtp_tea.track_email_open("unique-tracking-id-123")

# Track link clicks
smtp_tea.track_email_click("unique-tracking-id-123", "https://example.com/offer")
```

## Error Handling

```cursed
lowkey !smtp_tea.send_email(host, port, user, pass, from, to, subject, body, tls) {
    vibez.spill("Email sending failed!")
    # Check network connectivity
    # Verify SMTP credentials
    # Retry with exponential backoff
}
```

## Best Practices

### Security
- Always use TLS for remote SMTP servers
- Use app passwords instead of account passwords
- Validate email addresses before sending
- Sanitize user-generated content
- Store credentials securely (environment variables)

### Performance
- Use bulk sending for multiple recipients
- Implement retry logic with exponential backoff
- Monitor bounce rates and delivery status
- Cache SMTP connections for high-volume sending

### Deliverability
- Set proper SPF, DKIM, and DMARC records
- Use authenticated SMTP servers
- Monitor bounce rates and unsubscribes
- Include both text and HTML versions
- Avoid spam trigger words

## Common SMTP Ports

```cursed
smtp_tea.SMTP_PLAIN = 25     # Plain SMTP (not recommended for remote)
smtp_tea.SMTP_TLS = 587      # SMTP with STARTTLS (recommended)
smtp_tea.SMTP_SSL = 465      # SMTP over SSL/TLS
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/smtp_tea/test_smtp_tea.csd
```

The test suite covers:
- ✅ SMTP configuration validation
- ✅ Email address validation  
- ✅ Message composition (text, HTML, attachments)
- ✅ Authentication methods
- ✅ Security features
- ✅ Bounce detection
- ✅ Gen Z enhanced APIs
- ✅ Priority handling
- ✅ Content sanitization

## Dependencies

The module depends on these CURSED stdlib modules:
- `cryptz` - Cryptographic functions for authentication
- `stringz` - String manipulation utilities
- `net_drip` - Network operations and socket handling
- `encode_mood` - Base64 encoding for authentication
- `timez` - Timestamp generation for email headers

## Examples

Check the `examples/` directory for complete email sending examples:
- Simple email sending
- HTML newsletter with images
- Bulk email with templates
- Secure corporate email setup
- Gen Z social media notifications

---

*Built with CURSED energy! No cap! 🔥💯*

*This module provides enterprise-grade email functionality while keeping the vibe immaculate! ✨*
