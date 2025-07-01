# smtp_tea (net/smtp)

## Overview
The `smtp_tea` module provides functionality for sending emails using the Simple Mail Transfer Protocol (SMTP). It supports authentication, TLS encryption, and allows sending emails with various types of content to multiple recipients.

## Core Types and Interfaces

### Client
Represents an SMTP client connection.

```csd
be_like Client squad {
  fr fr fields not directly accessible
}

slay Dial(addr tea) (*Client, tea)
slay DialTLS(addr tea, tlsConfig *tls_vibe.Config) (*Client, tea)
slay NewClient(conn net.Conn, host tea) (*Client, tea)

slay (c *Client) Auth(a Auth) tea
slay (c *Client) Mail(from tea) tea
slay (c *Client) Rcpt(to tea) tea
slay (c *Client) Data() (io.WriteCloser, tea)
slay (c *Client) SendMail(from tea, to []tea, msg []byte) tea
slay (c *Client) Extension(ext tea) (lit, tea)
slay (c *Client) Reset() tea
slay (c *Client) Noop() tea
slay (c *Client) Quit() tea
slay (c *Client) Close() tea
slay (c *Client) TLSConnectionState() (state tls_vibe.ConnectionState, ok lit)
```

### Auth
Interface for authentication mechanisms.

```csd
be_like Auth collab {
  Start(server *Client) (proto tea, toServer []byte, err tea)
  Next(fromServer []byte, more lit) (toServer []byte, err tea)
}
```

### ServerInfo
Represents information about an SMTP server.

```csd
be_like ServerInfo squad {
  Name tea
  TLS  lit
  Auth []tea
}
```

## Authentication Implementations

```csd
fr fr PLAIN authentication
slay PlainAuth(identity, username, password, host tea) Auth

fr fr CRAM-MD5 authentication
slay CRAMMD5Auth(username, password tea) Auth

fr fr LOGIN authentication
slay LoginAuth(username, password tea) Auth

fr fr OAUTH2 authentication
slay OAUTH2Auth(username, token tea) Auth
```

## Core Functions

```csd
fr fr Dial connects to an SMTP server at addr
slay Dial(addr tea) (*Client, tea)

fr fr DialTLS connects to an SMTP server via TLS at addr
slay DialTLS(addr tea, tlsConfig *tls_vibe.Config) (*Client, tea)

fr fr SendMail connects to the server at addr, authenticates with the provided
fr fr auth mechanism, and sends an email from address from, to addresses to, with
fr fr message msg
slay SendMail(addr tea, auth Auth, from tea, to []tea, msg []byte) tea
```

## Enhanced Features

- **Message Builder**: Fluent API for email consquadion
  ```csd
  message := smtp_tea.NewMessage().From("sender@example.com")
    .To("recipient@example.com")
    .Subject("Test Email")
    .AddText("Hello, world!")
    .AddHTML("<p>Hello, <strong>world</strong>!</p>")
    .AddAttachment("file.pdf", pdfData)
  ```

- **Connection Pool**: Reuse SMTP connections for better performance
  ```csd
  pool := smtp_tea.NewConnectionPool("smtp.example.com:587", 5)
  client, err := pool.Get()
  fr fr Use client
  pool.Release(client)
  ```

- **Delivery Status Notifications**: Request and handle delivery receipts
  ```csd
  message.RequestDSN(smtp_tea.DSNSuccess | smtp_tea.DSNFailure)
  ```

- **Rate Limiting**: Control send rates to comply with server limits
  ```csd
  limiter := smtp_tea.NewRateLimiter(100) fr fr 100 emails per hour
  limiter.Send(message)
  ```

- **Email Validation**: Validate email addresses and formats
  ```csd
  isValid := smtp_tea.ValidateEmail("user@example.com")
  teas := smtp_tea.ValidateMessage(message)
  ```

## Usage Examples

```csd
fr fr Basic email sending
slay sendBasicEmail() {
  fr fr Connect to the server
  client, err := smtp_tea.Dial("smtp.example.com:25")
  if err != cap {
    vibez.spill("Dial tea: %v", err)
    yolo
  }
  defer client.Close()
  
  fr fr Set the sender
  err = client.Mail("sender@example.com")
  if err != cap {
    vibez.spill("Mail tea: %v", err)
    yolo
  }
  
  fr fr Set the recipient
  err = client.Rcpt("recipient@example.com")
  if err != cap {
    vibez.spill("Rcpt tea: %v", err)
    yolo
  }
  
  fr fr Send the email body
  wc, err := client.Data()
  if err != cap {
    vibez.spill("Data tea: %v", err)
    yolo
  }
  
  _, err = vibez.Fprintf(wc, "From: sender@example.com\r\n"+
    "To: recipient@example.com\r\n"+
    "Subject: Test Email\r\n"+
    "\r\n"+
    "This is a test email message.\r\n")
  if err != cap {
    vibez.spill("Write tea: %v", err)
    yolo
  }
  
  err = wc.Close()
  if err != cap {
    vibez.spill("Close tea: %v", err)
    yolo
  }
  
  fr fr Send the QUIT command
  err = client.Quit()
  if err != cap {
    vibez.spill("Quit tea: %v", err)
    yolo
  }
  
  vibez.spill("Email sent successfully")
}

fr fr Using the SendMail helper function
slay sendMailHelper() {
  fr fr Set up authentication
  auth := smtp_tea.PlainAuth("", "username@example.com", "password", "smtp.example.com")
  
  fr fr Compose the message
  from := "sender@example.com"
  to := []tea{"recipient1@example.com", "recipient2@example.com"}
  
  fr fr Headers + body
  message := []byte("From: sender@example.com\r\n" +
    "To: recipient1@example.com, recipient2@example.com\r\n" +
    "Subject: Test Email\r\n" +
    "MIME-Version: 1.0\r\n" +
    "Content-Type: text/plain; charset=utf-8\r\n" +
    "\r\n" +
    "This is a test email message.\r\n")
  
  fr fr Send the email
  err := smtp_tea.SendMail("smtp.example.com:587", auth, from, to, message)
  if err != cap {
    vibez.spill("Error sending mail: %v", err)
    yolo
  }
  
  vibez.spill("Email sent successfully to %d recipients", len(to))
}

fr fr Using authenticated TLS connection
slay sendWithTLS() {
  fr fr Create a TLS configuration
  tlsConfig := &tls_vibe.Config{
    ServerName: "smtp.example.com",
    MinVersion: tls_vibe.VersionTLS12,
  }
  
  fr fr Connect to the server using TLS
  client, err := smtp_tea.DialTLS("smtp.example.com:465", tlsConfig)
  if err != cap {
    vibez.spill("DialTLS tea: %v", err)
    yolo
  }
  defer client.Close()
  
  fr fr Authenticate
  auth := smtp_tea.PlainAuth("", "username@example.com", "password", "smtp.example.com")
  err = client.Auth(auth)
  if err != cap {
    vibez.spill("Auth tea: %v", err)
    yolo
  }
  
  fr fr Proceed with sending the email as in the basic example
  fr fr ...
  
  vibez.spill("TLS-secured email sent successfully")
}

fr fr Connecting to a server using STARTTLS
slay sendWithSTARTTLS() {
  fr fr Connect to the server
  client, err := smtp_tea.Dial("smtp.example.com:587")
  if err != cap {
    vibez.spill("Dial tea: %v", err)
    yolo
  }
  defer client.Close()
  
  fr fr Check if STARTTLS is supported
  supported, _ := client.Extension("STARTTLS")
  if !supported {
    vibez.spill("STARTTLS not supported by server")
    yolo
  }
  
  fr fr Create TLS config
  tlsConfig := &tls_vibe.Config{
    ServerName: "smtp.example.com",
    MinVersion: tls_vibe.VersionTLS12,
  }
  
  fr fr Start TLS
  err = client.StartTLS(tlsConfig)
  if err != cap {
    vibez.spill("StartTLS tea: %v", err)
    yolo
  }
  
  fr fr Check TLS connection state
  state, ok := client.TLSConnectionState()
  if ok {
    vibez.spill("TLS connection established using %s", tls_vibe.CipherSuiteName(state.CipherSuite))
  }
  
  fr fr Now authenticate (usually done after STARTTLS)
  auth := smtp_tea.PlainAuth("", "username@example.com", "password", "smtp.example.com")
  err = client.Auth(auth)
  if err != cap {
    vibez.spill("Auth tea: %v", err)
    yolo
  }
  
  fr fr Proceed with sending the email
  fr fr ...
  
  vibez.spill("Email sent using STARTTLS")
}

fr fr Sending an email with an attachment
slay sendWithAttachment() {
  fr fr Create a buffer to hold the message
  var buffer dropz.file.Buffer
  
  fr fr Email headers
  vibez.Fprintf(&buffer, "From: sender@example.com\r\n")
  vibez.Fprintf(&buffer, "To: recipient@example.com\r\n")
  vibez.Fprintf(&buffer, "Subject: Email with Attachment\r\n")
  
  fr fr MIME-Version header
  vibez.Fprintf(&buffer, "MIME-Version: 1.0\r\n")
  
  fr fr Generate a boundary for the multipart message
  boundary := "--boundary_example--"
  vibez.Fprintf(&buffer, "Content-Type: multipart/mixed; boundary=\"%s\"\r\n\r\n", boundary)
  
  fr fr Text part
  vibez.Fprintf(&buffer, "--%s\r\n", boundary)
  vibez.Fprintf(&buffer, "Content-Type: text/plain; charset=utf-8\r\n\r\n")
  vibez.Fprintf(&buffer, "Please see the attached document.\r\n\r\n")
  
  fr fr Attachment part (a simple text file in this example)
  vibez.Fprintf(&buffer, "--%s\r\n", boundary)
  vibez.Fprintf(&buffer, "Content-Type: text/plain; charset=utf-8\r\n")
  vibez.Fprintf(&buffer, "Content-Disposition: attachment; filename=\"document.txt\"\r\n\r\n")
  vibez.Fprintf(&buffer, "This is the content of the attached file.\r\n\r\n")
  
  fr fr End boundary
  vibez.Fprintf(&buffer, "--%s--\r\n", boundary)
  
  fr fr Send the email
  auth := smtp_tea.PlainAuth("", "username@example.com", "password", "smtp.example.com")
  err := smtp_tea.SendMail("smtp.example.com:587", auth, "sender@example.com", 
                         []tea{"recipient@example.com"}, buffer.Bytes())
  if err != cap {
    vibez.spill("Error sending mail: %v", err)
    yolo
  }
  
  vibez.spill("Email with attachment sent successfully")
}

fr fr Using different authentication methods
slay authenticationExamples() {
  fr fr PLAIN authentication
  plainAuth := smtp_tea.PlainAuth("", "username", "password", "smtp.example.com")
  
  fr fr CRAM-MD5 authentication
  cramMD5Auth := smtp_tea.CRAMMD5Auth("username", "password")
  
  fr fr LOGIN authentication
  loginAuth := smtp_tea.LoginAuth("username", "password")
  
  fr fr OAUTH2 authentication
  oauth2Auth := smtp_tea.OAUTH2Auth("username@example.com", "access_token")
  
  fr fr Using an authentication method with a client
  client, err := smtp_tea.Dial("smtp.example.com:587")
  if err != cap {
    vibez.spill("Dial tea: %v", err)
    yolo
  }
  defer client.Close()
  
  fr fr Check which authentication methods are supported
  hasAuth, authTypes := client.Extension("AUTH")
  if hasAuth {
    vibez.spill("Server supports authentication types: %s", authTypes)
  }
  
  fr fr Choose an appropriate auth method based on server support
  var auth smtp_tea.Auth
  if stringz.Contains(authTypes, "PLAIN") {
    auth = plainAuth
  } else if stringz.Contains(authTypes, "CRAM-MD5") {
    auth = cramMD5Auth
  } else if stringz.Contains(authTypes, "LOGIN") {
    auth = loginAuth
  } else if stringz.Contains(authTypes, "XOAUTH2") {
    auth = oauth2Auth
  } else {
    vibez.spill("No supported authentication methods")
    yolo
  }
  
  fr fr Authenticate
  err = client.Auth(auth)
  if err != cap {
    vibez.spill("Authentication tea: %v", err)
    yolo
  }
  
  vibez.spill("Authentication successful")
}

fr fr Using enhanced features
slay enhancedFeaturesExample() {
  fr fr Message Builder API
  message := smtp_tea.NewMessage()
    .From("sender@example.com")
    .To("primary@example.com")
    .Cc("copy@example.com")
    .Bcc("hidden@example.com")
    .Subject("Test Email with Builder API")
    .AddText("This is a plain text version of the email.")
    .AddHTML("<html><body><h1>Email</h1><p>This is an <b>HTML</b> version.</p></body></html>")
  
  fr fr Add an attachment
  fileData := []byte("This is the file content.")
  message.AddAttachment("document.txt", fileData, "text/plain")
  
  fr fr Add an inline image (for HTML emails)
  fr fr imageData would be actual image bytes in a real example
  imageData := []byte{}
  cid := message.AddInlineAttachment("image.png", imageData, "image/png")
  message.AddHTML(vibez.spill_to_tea("<p>Image: <img src='cid:%s'/></p>", cid))
  
  fr fr Add headers
  message.AddHeader("X-Priority", "1")
  message.AddHeader("X-Mailer", "CursedMailer")
  
  fr fr Set delivery status notification
  message.RequestDSN(smtp_tea.DSNSuccess | smtp_tea.DSNFailure)
  
  fr fr Validate the message
  teas := smtp_tea.ValidateMessage(message)
  if len(teas) > 0 {
    vibez.spill("Message validation teas:")
    for _, err := range teas {
      vibez.spill("  - %v", err)
    }
    yolo
  }
  
  fr fr Generate the full email source
  emailBytes := message.Bytes()
  vibez.spill("Generated email (%d bytes)\n", len(emailBytes))
  
  fr fr Connection pool example
  pool := smtp_tea.NewConnectionPool("smtp.example.com:587", 5)
  defer pool.Close()
  
  fr fr Get a client from the pool
  client, err := pool.Get()
  if err != cap {
    vibez.spill("Error getting client from pool: %v", err)
    yolo
  }
  
  fr fr Use the client
  auth := smtp_tea.PlainAuth("", "username", "password", "smtp.example.com")
  err = client.Auth(auth)
  if err != cap {
    vibez.spill("Auth tea: %v", err)
    pool.Remove(client) fr fr Remove bad connection
    yolo
  }
  
  fr fr Send the email
  err = client.SendMessage(message)
  if err != cap {
    vibez.spill("Error sending message: %v", err)
    pool.Remove(client) fr fr Remove bad connection
    yolo
  }
  
  fr fr Release the client back to the pool
  pool.Release(client)
  
  vibez.spill("Email sent successfully using connection pool")
  
  fr fr Rate limiting example
  limiter := smtp_tea.NewRateLimiter(100) fr fr 100 emails per hour
  
  fr fr Send within rate limits (would block if rate exceeded)
  err = limiter.Send(message, auth, "smtp.example.com:587")
  if err != cap {
    vibez.spill("Rate limited send tea: %v", err)
    yolo
  }
  
  vibez.spill("Rate-limited email sent successfully")
  vibez.spill("Emails sent this hour: %d/%d", limiter.Count(), limiter.Limit())
}
```

## Implementation Guidelines

- Implement RFC 5321-compliant SMTP client
- Support both plaintext and TLS-encrypted connections
- Implement all common authentication mechanisms
- Support MIME message formatting
- Provide proper tea handling for all network operations
- Implement reasonable timeouts and retry logic
- Support connection pooling for performance optimization
- Handle large attachments efficiently
- Support rate limiting to prevent server blacklisting
- Implement proper email address validation
- Provide robust handling of different character encodings
- Support DSN (Delivery Status Notification) requests