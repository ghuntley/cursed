# SMTP Tea Module

Enterprise-grade SMTP email functionality for CURSED applications. This module provides comprehensive email sending capabilities with pure CURSED implementation (no FFI dependencies).

## Features

- **SMTP Protocol Support**: Full RFC 5321 compliant SMTP client
- **Authentication**: Username/password and OAuth2 authentication
- **TLS/SSL Security**: Secure email transmission with encryption
- **Multiple Formats**: Plain text, HTML, and multipart emails
- **Attachments**: Support for file attachments
- **Bulk Sending**: Efficient bulk email operations
- **Error Handling**: Comprehensive error reporting and recovery
- **Debug Mode**: Detailed logging for troubleshooting

## Quick Start

```cursed
yeet "smtp_tea"

// Connect to SMTP server
smtp_connect("smtp.gmail.com", 587)

// Authenticate
smtp_auth("your_email@gmail.com", "your_password")

// Send email
smtp_send_email("sender@example.com", "recipient@example.com", "Hello", "World!")

// Disconnect
smtp_disconnect()
```

## Core Functions

### Connection Management

#### `smtp_connect(server tea, port normie) lit`
Establishes connection to SMTP server.
- `server`: SMTP server hostname
- `port`: SMTP port (25, 587, 465)
- Returns: `based` on success, `cap` on failure

#### `smtp_disconnect() lit`
Closes SMTP connection and cleans up resources.

#### `smtp_is_connected() lit`
Checks if SMTP connection is active.

### Authentication

#### `smtp_auth(username tea, password tea) lit`
Authenticates with SMTP server using credentials.
- `username`: Email address or username
- `password`: Account password
- Returns: `based` on successful authentication

#### `smtp_enable_tls() lit`
Enables TLS encryption for secure communication.

### Email Sending

#### `smtp_send_email(from tea, to tea, subject tea, body tea) lit`
Sends basic email message.
- `from`: Sender email address
- `to`: Recipient email address
- `subject`: Email subject line
- `body`: Email message body

#### `smtp_send_full_email(from tea, to tea, cc tea, bcc tea, subject tea, body tea, headers tea) lit`
Sends email with full header control.
- `cc`: Carbon copy recipients
- `bcc`: Blind carbon copy recipients
- `headers`: Additional email headers

#### `smtp_send_html_email(from tea, to tea, subject tea, html_body tea) lit`
Sends HTML formatted email.
- `html_body`: HTML content with tags

#### `smtp_send_with_attachments(from tea, to tea, subject tea, body tea, attachments tea) lit`
Sends email with file attachments.
- `attachments`: Comma-separated file paths

### Bulk Operations

#### `smtp_send_bulk_emails(from tea, recipients tea, subject tea, body tea) normie`
Sends email to multiple recipients efficiently.
- `recipients`: Comma-separated email addresses
- Returns: Number of emails sent

### Validation & Formatting

#### `smtp_validate_email(email tea) lit`
Validates email address format.
- Returns: `based` for valid addresses

#### `smtp_format_message(from tea, to tea, subject tea, body tea) tea`
Formats email message with proper headers.

#### `smtp_parse_headers(headers tea) tea`
Parses and validates email headers.

### Configuration

#### `smtp_set_timeout(seconds normie) lit`
Sets connection timeout in seconds.

#### `smtp_set_debug(enabled lit) lit`
Enables/disables debug logging.

#### `smtp_get_capabilities() tea`
Returns server capabilities (EHLO response).

### Error Handling

#### `smtp_get_last_error() tea`
Returns description of last error.

#### `smtp_get_status() tea`
Returns current SMTP server status.

### Encoding Utilities

#### `smtp_encode_base64(data tea) tea`
Encodes data to base64 for authentication.

#### `smtp_decode_base64(encoded tea) tea`
Decodes base64 encoded data.

## Constants

- `smtp_default_port`: 25 (Standard SMTP)
- `smtp_tls_port`: 587 (SMTP with TLS)
- `smtp_ssl_port`: 465 (SMTP over SSL)

## Examples

### Basic Email

```cursed
yeet "smtp_tea"

smtp_connect("smtp.gmail.com", smtp_tls_port)
smtp_enable_tls()
smtp_auth("sender@gmail.com", "app_password")
smtp_send_email("sender@gmail.com", "recipient@example.com", "Hello", "This is a test message")
smtp_disconnect()
```

### HTML Email with Attachments

```cursed
yeet "smtp_tea"

smtp_connect("smtp.office365.com", 587)
smtp_auth("user@company.com", "password")

sus html_content tea = "<html><body><h1>Report</h1><p>See attached files.</p></body></html>"
smtp_send_with_attachments("user@company.com", "manager@company.com", "Monthly Report", html_content, "report.pdf,charts.xlsx")

smtp_disconnect()
```

### Bulk Email Campaign

```cursed
yeet "smtp_tea"

smtp_connect("smtp.mailgun.org", 587)
smtp_auth("api_key", "your_api_key")

sus recipients tea = "user1@example.com,user2@example.com,user3@example.com"
sus count normie = smtp_send_bulk_emails("newsletter@company.com", recipients, "Weekly Newsletter", "This week's updates...")

vibez.spill("Sent " + count + " emails")
smtp_disconnect()
```

### Error Handling

```cursed
yeet "smtp_tea"

sus connected lit = smtp_connect("invalid.server.com", 587)
if (connected == cap) {
    sus error tea = smtp_get_last_error()
    vibez.spill("Connection failed: " + error)
}
```

## Server Configuration

### Gmail
- Server: `smtp.gmail.com`
- Port: 587 (TLS) or 465 (SSL)
- Requires: App password or OAuth2

### Outlook/Hotmail
- Server: `smtp.office365.com`
- Port: 587 (TLS)
- Requires: Account password

### Custom SMTP
- Configure server hostname and port
- Set authentication credentials
- Enable TLS if supported

## Security Best Practices

1. **Use TLS**: Always enable TLS encryption
2. **App Passwords**: Use app-specific passwords instead of account passwords
3. **Validate Inputs**: Validate email addresses before sending
4. **Rate Limiting**: Implement sending limits to avoid spam detection
5. **Error Logging**: Log errors without exposing sensitive information

## Performance Tips

- Use `smtp_send_bulk_emails()` for multiple recipients
- Reuse connections for multiple emails
- Set appropriate timeouts for network conditions
- Enable debug mode only during development

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/smtp_tea/test_smtp_tea.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/smtp_tea/test_smtp_tea.csd
./test_smtp_tea
```

## Compatibility

- **CURSED Version**: Compatible with all CURSED compiler versions
- **SMTP Standards**: RFC 5321 compliant
- **Character Encoding**: UTF-8 support
- **Platforms**: Cross-platform implementation

## Contributing

When extending the SMTP Tea module:

1. Follow pure CURSED implementation patterns
2. Add comprehensive tests for new functions
3. Update documentation with examples
4. Ensure cross-platform compatibility
5. Test both interpretation and compilation modes

## License

Part of the CURSED standard library. See main project license.
