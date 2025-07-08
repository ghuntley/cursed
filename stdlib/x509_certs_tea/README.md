# X.509 Certificates Tea Module

The `x509_certs_tea` module provides comprehensive X.509 certificate handling functionality for the CURSED programming language. This module builds on the `asn1_mood` and `pem_drip` modules to provide a complete X.509 certificate processing solution.

## Features

- **Certificate Parsing**: Parse X.509 certificates from PEM/DER format
- **Key Management**: Handle private keys, public keys, and certificate requests
- **Certificate Verification**: Verify certificate signatures and chains
- **Field Extraction**: Extract certificate fields like subject, issuer, serial number
- **Hostname Verification**: Check hostname validation against certificate
- **Email Verification**: Validate email addresses against certificate
- **IP Address Verification**: Check IP address validation against certificate
- **Encoding Support**: Encode certificates and keys to PEM format

## Data Structures

### X509Cert
Represents an X.509 certificate with all standard fields:
- `subject`: Certificate subject name
- `issuer`: Certificate issuer name
- `serial_number`: Certificate serial number
- `not_before`: Validity start date
- `not_after`: Validity end date
- `public_key`: Public key data
- `extensions`: Certificate extensions
- `signature`: Certificate signature

### X509Key
Represents an X.509 private key:
- `algorithm`: Key algorithm (RSA, ECDSA, etc.)
- `key_data`: Private key data
- `public_key`: Associated public key

### X509PubKey
Represents an X.509 public key:
- `algorithm`: Key algorithm
- `key_data`: Public key data
- `parameters`: Algorithm parameters

### X509CSR
Represents an X.509 certificate request:
- `subject`: Requested subject name
- `public_key`: Public key for certificate
- `extensions`: Requested extensions
- `signature`: Request signature

## Functions

### Certificate Parsing Functions

#### `x509_parse_cert(data tea) X509Cert`
Parse an X.509 certificate from PEM or DER format.

```cursed
sus cert_pem tea = "-----BEGIN CERTIFICATE-----\n..."
sus cert X509Cert = x509_parse_cert(cert_pem)
```

#### `x509_parse_key(data tea) X509Key`
Parse a private key from PEM or DER format.

```cursed
sus key_pem tea = "-----BEGIN PRIVATE KEY-----\n..."
sus key X509Key = x509_parse_key(key_pem)
```

#### `x509_parse_pubkey(data tea) X509PubKey`
Parse a public key from PEM or DER format.

```cursed
sus pubkey_pem tea = "-----BEGIN PUBLIC KEY-----\n..."
sus pubkey X509PubKey = x509_parse_pubkey(pubkey_pem)
```

#### `x509_parse_csr(data tea) X509CSR`
Parse a certificate request from PEM or DER format.

```cursed
sus csr_pem tea = "-----BEGIN CERTIFICATE REQUEST-----\n..."
sus csr X509CSR = x509_parse_csr(csr_pem)
```

### Certificate Encoding Functions

#### `x509_encode_cert(cert X509Cert) tea`
Encode an X.509 certificate to PEM format.

```cursed
sus encoded_cert tea = x509_encode_cert(cert)
```

#### `x509_encode_key(key X509Key) tea`
Encode a private key to PEM format.

```cursed
sus encoded_key tea = x509_encode_key(key)
```

#### `x509_encode_pubkey(pubkey X509PubKey) tea`
Encode a public key to PEM format.

```cursed
sus encoded_pubkey tea = x509_encode_pubkey(pubkey)
```

### Certificate Verification Functions

#### `x509_verify_cert(cert X509Cert, ca X509Cert) lit`
Verify a certificate against a CA certificate.

```cursed
sus is_valid lit = x509_verify_cert(cert, ca_cert)
```

#### `x509_verify_chain(certs []X509Cert) lit`
Verify a certificate chain from leaf to root.

```cursed
sus chain []X509Cert = [leaf_cert, intermediate_cert, root_cert]
sus is_valid lit = x509_verify_chain(chain)
```

### Certificate Field Extraction Functions

#### `x509_get_subject(cert X509Cert) tea`
Get the subject name from a certificate.

```cursed
sus subject tea = x509_get_subject(cert)
```

#### `x509_get_issuer(cert X509Cert) tea`
Get the issuer name from a certificate.

```cursed
sus issuer tea = x509_get_issuer(cert)
```

#### `x509_get_serial(cert X509Cert) tea`
Get the serial number from a certificate.

```cursed
sus serial tea = x509_get_serial(cert)
```

#### `x509_get_validity(cert X509Cert) (tea, tea)`
Get the validity period from a certificate.

```cursed
sus (not_before, not_after) := x509_get_validity(cert)
```

#### `x509_get_extensions(cert X509Cert) tea`
Get the extensions from a certificate.

```cursed
sus extensions tea = x509_get_extensions(cert)
```

### Certificate Validation Functions

#### `x509_check_hostname(cert X509Cert, hostname tea) lit`
Check if a hostname is valid for a certificate.

```cursed
sus is_valid lit = x509_check_hostname(cert, "example.com")
```

#### `x509_check_email(cert X509Cert, email tea) lit`
Check if an email address is valid for a certificate.

```cursed
sus is_valid lit = x509_check_email(cert, "user@example.com")
```

#### `x509_check_ip(cert X509Cert, ip tea) lit`
Check if an IP address is valid for a certificate.

```cursed
sus is_valid lit = x509_check_ip(cert, "192.168.1.1")
```

## Usage Examples

### Basic Certificate Processing

```cursed
yeet "x509_certs_tea"

# Parse a certificate
sus cert_pem tea = load_certificate_from_file("server.crt")
sus cert X509Cert = x509_parse_cert(cert_pem)

# Extract information
sus subject tea = x509_get_subject(cert)
sus issuer tea = x509_get_issuer(cert)
sus serial tea = x509_get_serial(cert)

vibez.spill("Certificate Subject: " + subject)
vibez.spill("Certificate Issuer: " + issuer)
vibez.spill("Serial Number: " + serial)
```

### Certificate Verification

```cursed
yeet "x509_certs_tea"

# Load certificate and CA
sus cert X509Cert = x509_parse_cert(cert_pem)
sus ca X509Cert = x509_parse_cert(ca_pem)

# Verify certificate
sus is_valid lit = x509_verify_cert(cert, ca)
mood is_valid {
    vibez.spill("Certificate is valid")
} else {
    vibez.spill("Certificate is invalid")
}
```

### Hostname Validation

```cursed
yeet "x509_certs_tea"

# Load certificate
sus cert X509Cert = x509_parse_cert(cert_pem)

# Check hostname
sus hostname tea = "example.com"
sus is_valid lit = x509_check_hostname(cert, hostname)

mood is_valid {
    vibez.spill("Hostname is valid for certificate")
} else {
    vibez.spill("Hostname is NOT valid for certificate")
}
```

### Certificate Chain Validation

```cursed
yeet "x509_certs_tea"

# Load certificate chain
sus leaf_cert X509Cert = x509_parse_cert(leaf_pem)
sus intermediate_cert X509Cert = x509_parse_cert(intermediate_pem)
sus root_cert X509Cert = x509_parse_cert(root_pem)

# Create chain array
sus chain []X509Cert = [leaf_cert, intermediate_cert, root_cert]

# Verify chain
sus is_valid lit = x509_verify_chain(chain)
mood is_valid {
    vibez.spill("Certificate chain is valid")
} else {
    vibez.spill("Certificate chain is invalid")
}
```

## Security Considerations

1. **Certificate Validation**: Always verify certificates against trusted CAs
2. **Hostname Verification**: Check hostname validation for TLS connections
3. **Validity Period**: Verify certificate validity periods
4. **Key Strength**: Ensure adequate key lengths for security
5. **Algorithm Security**: Use secure algorithms (avoid MD5, SHA1)

## Dependencies

- `asn1_mood`: ASN.1 parsing and encoding
- `pem_drip`: PEM format processing
- `crypto`: Cryptographic operations
- `string`: String manipulation functions
- `time`: Date/time operations

## Testing

Run the comprehensive test suite:

```bash
# Test in interpretation mode
cargo run --bin cursed stdlib/x509_certs_tea/test_x509_certs_tea.csd

# Test in compilation mode
cargo run --bin cursed -- compile stdlib/x509_certs_tea/test_x509_certs_tea.csd
./test_x509_certs_tea
```

## Standards Compliance

This module implements X.509 certificate processing according to:
- RFC 5280: Internet X.509 Public Key Infrastructure Certificate and CRL Profile
- RFC 3280: Internet X.509 Public Key Infrastructure Certificate and CRL Profile
- RFC 2459: Internet X.509 Public Key Infrastructure Certificate and CRL Profile

## Error Handling

The module provides robust error handling for:
- Invalid PEM/DER format
- Malformed certificates
- Signature verification failures
- Invalid certificate chains
- Expired certificates
- Hostname validation failures

## Performance

The module is optimized for:
- Fast certificate parsing
- Efficient chain validation
- Minimal memory usage
- Scalable certificate processing

## Future Enhancements

Planned features include:
- Certificate Revocation List (CRL) support
- Online Certificate Status Protocol (OCSP) support
- Advanced certificate validation policies
- Custom certificate extensions
- Hardware security module (HSM) integration
