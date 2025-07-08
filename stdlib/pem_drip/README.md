# PEM Drip Module

A comprehensive RFC 7468 compliant PEM (Privacy-Enhanced Mail) encoding/decoding module for CURSED.

## Overview

The PEM Drip module provides complete PEM format support for encoding and decoding cryptographic data, certificates, keys, and other security-related information. This module implements the PEM format specification as defined in RFC 7468.

## Features

- **RFC 7468 Compliance**: Full compliance with PEM format specification
- **Pure CURSED Implementation**: No external dependencies or FFI bridges
- **Comprehensive Format Support**: Handles certificates, private keys, public keys, CSRs, and CRLs
- **Base64 Encoding/Decoding**: Built-in base64 operations for PEM data
- **Security Focused**: Proper padding and validation handling
- **Performance Optimized**: Efficient parsing and processing of large PEM data
- **Multiple Block Support**: Parse and process multiple PEM blocks in a single input

## Functions

### Core Operations

#### `pem_encode(data tea, label tea) tea`
Encodes binary data into PEM format with the specified label.

```cursed
sus cert_data tea = "certificate binary data"
sus pem_cert tea = pem_encode(cert_data, "CERTIFICATE")
```

#### `pem_decode(data tea) tea`
Decodes PEM formatted data back to binary format.

```cursed
sus binary_data tea = pem_decode(pem_cert)
```

#### `pem_parse(data tea) tea`
Parses PEM data and returns structured block information.

```cursed
sus blocks tea = pem_parse(pem_data)
```

### Block Operations

#### `pem_encode_block(block tea) tea`
Encodes a PEM block structure back to PEM format.

```cursed
sus pem_formatted tea = pem_encode_block(block)
```

#### `pem_decode_block(block tea) tea`
Decodes a PEM block structure to binary data.

```cursed
sus binary_data tea = pem_decode_block(block)
```

#### `pem_block_create(label tea, headers tea, body tea) tea`
Creates a new PEM block structure.

```cursed
sus block tea = pem_block_create("CERTIFICATE", "", base64_data)
```

### Data Extraction

#### `pem_extract_cert(data tea) tea`
Extracts certificate data from PEM formatted input.

```cursed
sus cert_binary tea = pem_extract_cert(pem_data)
```

#### `pem_extract_key(data tea) tea`
Extracts private key data from PEM formatted input.

```cursed
sus key_binary tea = pem_extract_key(pem_data)
```

#### `pem_extract_pubkey(data tea) tea`
Extracts public key data from PEM formatted input.

```cursed
sus pubkey_binary tea = pem_extract_pubkey(pem_data)
```

#### `pem_extract_csr(data tea) tea`
Extracts Certificate Signing Request data from PEM formatted input.

```cursed
sus csr_binary tea = pem_extract_csr(pem_data)
```

#### `pem_extract_crl(data tea) tea`
Extracts Certificate Revocation List data from PEM formatted input.

```cursed
sus crl_binary tea = pem_extract_crl(pem_data)
```

### Validation and Inspection

#### `pem_validate(data tea) lit`
Validates whether the input data is in proper PEM format.

```cursed
sus is_valid lit = pem_validate(pem_data)
```

#### `pem_get_label(block tea) tea`
Gets the label from a PEM block.

```cursed
sus label tea = pem_get_label(block)
```

#### `pem_get_headers(block tea) tea`
Gets the headers from a PEM block.

```cursed
sus headers tea = pem_get_headers(block)
```

#### `pem_get_body(block tea) tea`
Gets the body (base64 data) from a PEM block.

```cursed
sus body tea = pem_get_body(block)
```

### Base64 Operations

#### `base64_encode(data tea) tea`
Encodes binary data to base64 format.

```cursed
sus encoded tea = base64_encode(binary_data)
```

#### `base64_decode(data tea) tea`
Decodes base64 formatted data to binary.

```cursed
sus decoded tea = base64_decode(encoded_data)
```

## Usage Examples

### Basic Certificate Processing

```cursed
yeet "pem_drip"

# Load certificate from PEM format
sus cert_pem tea = "-----BEGIN CERTIFICATE-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END CERTIFICATE-----"

# Validate PEM format
sus is_valid lit = pem_validate(cert_pem)
sis is_valid {
    # Extract certificate data
    sus cert_data tea = pem_extract_cert(cert_pem)
    vibez.spill("Certificate extracted successfully")
}
```

### Multiple Block Processing

```cursed
yeet "pem_drip"

# Parse multiple PEM blocks
sus multi_pem tea = "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----\n-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----"

sus blocks tea = pem_parse(multi_pem)
sus cert_data tea = pem_extract_cert(multi_pem)
sus key_data tea = pem_extract_key(multi_pem)
```

### Creating PEM Data

```cursed
yeet "pem_drip"

# Create PEM formatted certificate
sus cert_binary tea = "certificate binary data"
sus pem_cert tea = pem_encode(cert_binary, "CERTIFICATE")

# Create PEM formatted private key
sus key_binary tea = "private key binary data"
sus pem_key tea = pem_encode(key_binary, "PRIVATE KEY")
```

### Working with Headers

```cursed
yeet "pem_drip"

# Create block with headers
sus block tea = pem_block_create("CERTIFICATE", "Version: 3\nSerial: 12345", base64_data)
sus headers tea = pem_get_headers(block)
```

## PEM Format Support

The module supports the following PEM block types:

- `CERTIFICATE` - X.509 certificates
- `PRIVATE KEY` - PKCS#8 private keys
- `RSA PRIVATE KEY` - RSA private keys
- `PUBLIC KEY` - X.509 public keys
- `RSA PUBLIC KEY` - RSA public keys
- `CERTIFICATE REQUEST` - Certificate Signing Requests
- `X509 CRL` - Certificate Revocation Lists

## Technical Details

### Base64 Encoding
- Uses standard Base64 alphabet (RFC 4648)
- Proper padding with '=' characters
- Line wrapping at 64 characters per RFC 7468

### PEM Structure
- Proper BEGIN/END markers
- Support for headers between markers and data
- Whitespace handling and normalization
- Multiple block parsing capability

### Security Considerations
- Proper input validation
- Safe string handling
- Memory-efficient processing
- No external dependencies

## Performance Characteristics

- **Encoding**: O(n) time complexity where n is data size
- **Decoding**: O(n) time complexity where n is PEM data size
- **Parsing**: O(n) time complexity where n is input size
- **Memory**: Linear memory usage proportional to input size

## Error Handling

The module provides graceful error handling:
- Invalid PEM format returns empty strings
- Missing block types return empty strings
- Malformed base64 data is handled safely
- Invalid input parameters are validated

## Testing

The module includes comprehensive tests covering:
- Basic encoding/decoding operations
- Multiple PEM block processing
- All supported block types
- Edge cases and error conditions
- RFC 7468 compliance verification
- Performance with large data sets
- Security features and validation

### Running Tests

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/pem_drip/test_pem_drip.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/pem_drip/test_pem_drip.csd
./test_pem_drip
```

## Implementation Notes

This module is implemented in pure CURSED without any external dependencies:
- No FFI bridges required
- No external cryptographic libraries
- Portable across all CURSED-supported platforms
- Self-contained base64 implementation
- Efficient string processing algorithms

## Standards Compliance

- **RFC 7468**: Privacy-Enhanced Mail (PEM) Textual Encodings
- **RFC 4648**: The Base16, Base32, and Base64 Data Encodings
- **X.509**: Certificate and CRL profile standards
- **PKCS#8**: Private key information syntax standard

## License

This module is part of the CURSED standard library and follows the same licensing terms as the CURSED compiler.
