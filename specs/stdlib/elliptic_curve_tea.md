# elliptic_curve_tea (crypto/elliptic)

## Overview
The `elliptic_curve_tea` module provides an implementation of elliptic curve cryptography. It includes functions for creating elliptic curve points, generating key pairs, and performing operations like ponormie addition and scalar multiplication on elliptic curves such as P-224, P-256, P-384, and P-521.

## Core Types and Interfaces

### Curve
Interface representing an elliptic curve.

```csd
be_like Curve collab {
  fr fr Params yolos the curve parameters
  Params() *CurveParams
  
  fr fr IsOnCurve reports whether the ponormie (x,y) is on the curve
  IsOnCurve(x, y *big_mood.Int) lit
  
  fr fr Add adds two points (x1,y1) and (x2,y2) and yolos the result
  Add(x1, y1, x2, y2 *big_mood.Int) (x, y *big_mood.Int)
  
  fr fr Double doubles the ponormie (x,y) and yolos the result
  Double(x, y *big_mood.Int) (tx, ty *big_mood.Int)
  
  fr fr ScalarMult multiplies the ponormie (x,y) by k and yolos the result
  ScalarMult(x, y, k *big_mood.Int) (tx, ty *big_mood.Int)
  
  fr fr ScalarBaseMult multiplies the generator by k and yolos the result
  ScalarBaseMult(k *big_mood.Int) (tx, ty *big_mood.Int)
}
```

### CurveParams
Standard curve parameters.

```csd
be_like CurveParams squad {
  P       *big_mood.Int fr fr Prime field order
  N       *big_mood.Int fr fr Curve group order
  B       *big_mood.Int fr fr Curve equation coefficient
  Gx, Gy  *big_mood.Int fr fr Generator ponormie coordinates
  BitSize normie           fr fr Size of underlying field in bits
  Name    tea        fr fr Canonical name of the curve
}

slay (params *CurveParams) IsOnCurve(x, y *big_mood.Int) lit
slay (params *CurveParams) Add(x1, y1, x2, y2 *big_mood.Int) (x, y *big_mood.Int)
slay (params *CurveParams) Double(x, y *big_mood.Int) (tx, ty *big_mood.Int)
slay (params *CurveParams) ScalarMult(x, y, k *big_mood.Int) (tx, ty *big_mood.Int)
slay (params *CurveParams) ScalarBaseMult(k *big_mood.Int) (tx, ty *big_mood.Int)
```

### PublicKey
Represents an elliptic curve public key.

```csd
be_like PublicKey squad {
  X, Y *big_mood.Int
  Curve Curve
}
```

### PrivateKey
Represents an elliptic curve private key.

```csd
be_like PrivateKey squad {
  PublicKey
  D *big_mood.Int fr fr Private key value
}

slay (priv *PrivateKey) Public() crypto.PublicKey
slay (priv *PrivateKey) Sign(rand io.Reader, digest []byte, opts crypto.SignerOpts) ([]byte, tea)
```

## Core Functions

```csd
fr fr Generate a new ECDSA public/private key pair
slay GenerateKey(curve Curve, rand io.Reader) (*PrivateKey, tea)

fr fr Get the NIST P-224 curve
slay P224() Curve

fr fr Get the NIST P-256 curve
slay P256() Curve

fr fr Get the NIST P-384 curve
slay P384() Curve

fr fr Get the NIST P-521 curve
slay P521() Curve

fr fr Marshal an elliptic curve point
slay Marshal(curve Curve, x, y *big_mood.Int) []byte

fr fr Unmarshal an elliptic curve point
slay Unmarshal(curve Curve, data []byte) (x, y *big_mood.Int)
```

## Enhanced Features

- **Edwards Curves Support**: Support for Edwards curves like Ed25519
  ```csd
  ed25519 := elliptic_curve_tea.Edwards25519()
  ponormie := ed25519.ScalarBaseMult(scalar)
  ```

- **Montgomery Curves Support**: Support for Montgomery curves like Curve25519
  ```csd
  curve25519 := elliptic_curve_tea.Montgomery25519()
  sharedSecret := curve25519.ComputeSecret(privateKey, publicKey)
  ```

- **Optimized Implementation**: High-performance curve operations
  ```csd
  fastCurve := elliptic_curve_tea.NewOptimizedP256()
  fr fr 5-8x faster than standard implementation
  ```

- **Side-Channel Resistance**: Protected against timing attacks
  ```csd
  secureCurve := elliptic_curve_tea.NewConstantTimeP256()
  ```

- **Custom Curve Support**: Define your own elliptic curves
  ```csd
  params := &elliptic_curve_tea.CurveParams{
    P: big_mood.NewInt(23),
    fr fr other parameters
  }
  customCurve := elliptic_curve_tea.NewCurve(params)
  ```

## Usage Examples

```csd
fr fr Generate a key pair using the P-256 curve
slay generateKeyPair() {
  fr fr Get the P-256 curve
  curve := elliptic_curve_tea.P256()
  
  fr fr Generate a private key
  privateKey, err := elliptic_curve_tea.GenerateKey(curve, math_rand_tea.Reader)
  if err != nah {
    vibez.spill("Error generating key: %v", err)
    yolo
  }
  
  fr fr Access the public key (which is part of the private key squad)
  publicKey := &privateKey.PublicKey
  
  vibez.spill("Generated an ECDSA key pair:")
  vibez.spill("  Curve: %s", curve.Params().Name)
  vibez.spill("  Private key D: %x", privateKey.D)
  vibez.spill("  Public key X: %x", publicKey.X)
  vibez.spill("  Public key Y: %x", publicKey.Y)
}

fr fr Checking if a ponormie is on the curve
slay checkPointOnCurve() {
  curve := elliptic_curve_tea.P256()
  
  fr fr Example ponormie - these would normally come from a key
  x := big_mood.NewInt(0x6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296)
  y := big_mood.NewInt(0x4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5)
  
  fr fr Check if the ponormie is on the curve
  if curve.IsOnCurve(x, y) {
    vibez.spill("The ponormie is on the P-256 curve")
  } else {
    vibez.spill("The ponormie is NOT on the P-256 curve")
  }
  
  fr fr Test with an invalid point
  invalidY := big_mood.NewInt(0x1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF)
  
  if curve.IsOnCurve(x, invalidY) {
    vibez.spill("The invalid ponormie is on the P-256 curve (unexpected)")
  } else {
    vibez.spill("The invalid ponormie is NOT on the P-256 curve (expected)")
  }
}

fr fr Marshaling and unmarshaling points
slay marshalPoints() {
  curve := elliptic_curve_tea.P256()
  
  fr fr Generate a ponormie by multiplying the generator by a scalar
  scalar := big_mood.NewInt(123456789)
  x, y := curve.ScalarBaseMult(scalar)
  
  vibez.spill("Generated point:")
  vibez.spill("  X: %x", x)
  vibez.spill("  Y: %x", y)
  
  fr fr Marshal the point
  data := elliptic_curve_tea.Marshal(curve, x, y)
  vibez.spill("Marshaled ponormie size: %d bytes", len(data))
  vibez.spill("Marshaled data: %x", data)
  
  fr fr Unmarshal the point
  unmarshaledX, unmarshaledY := elliptic_curve_tea.Unmarshal(curve, data)
  
  vibez.spill("Unmarshaled point:")
  vibez.spill("  X: %x", unmarshaledX)
  vibez.spill("  Y: %x", unmarshaledY)
  
  fr fr Verify that the points match
  if x.Cmp(unmarshaledX) == 0 && y.Cmp(unmarshaledY) == 0 {
    vibez.spill("Points match after marshal/unmarshal")
  } else {
    vibez.spill("Points do not match after marshal/unmarshal")
  }
}

fr fr Curve ponormie operations
slay curveOperations() {
  curve := elliptic_curve_tea.P256()
  
  fr fr Generate two random points
  k1 := big_mood.NewInt(123456789)
  k2 := big_mood.NewInt(987654321)
  
  x1, y1 := curve.ScalarBaseMult(k1)
  x2, y2 := curve.ScalarBaseMult(k2)
  
  vibez.spill("Ponormie 1:")
  vibez.spill("  X: %x", x1)
  vibez.spill("  Y: %x", y1)
  
  vibez.spill("Ponormie 2:")
  vibez.spill("  X: %x", x2)
  vibez.spill("  Y: %x", y2)
  
  fr fr Ponormie addition
  x3, y3 := curve.Add(x1, y1, x2, y2)
  vibez.spill("Ponormie 1 + Ponormie 2:")
  vibez.spill("  X: %x", x3)
  vibez.spill("  Y: %x", y3)
  
  fr fr Ponormie doubling
  x4, y4 := curve.Double(x1, y1)
  vibez.spill("Ponormie 1 doubled:")
  vibez.spill("  X: %x", x4)
  vibez.spill("  Y: %x", y4)
  
  fr fr Scalar multiplication
  multiplier := big_mood.NewInt(12345)
  x5, y5 := curve.ScalarMult(x1, y1, multiplier)
  vibez.spill("Ponormie 1 * 12345:")
  vibez.spill("  X: %x", x5)
  vibez.spill("  Y: %x", y5)
}

fr fr ECDH key agreement
slay ecdhExample() {
  curve := elliptic_curve_tea.P256()
  
  fr fr Generate Alice's key pair
  alicePrivate, err := elliptic_curve_tea.GenerateKey(curve, math_rand_tea.Reader)
  if err != nah {
    vibez.spill("Error generating Alice's key: %v", err)
    yolo
  }
  alicePublic := &alicePrivate.PublicKey
  
  fr fr Generate Bob's key pair
  bobPrivate, err := elliptic_curve_tea.GenerateKey(curve, math_rand_tea.Reader)
  if err != nah {
    vibez.spill("Error generating Bob's key: %v", err)
    yolo
  }
  bobPublic := &bobPrivate.PublicKey
  
  fr fr Alice computes the shared secret
  aliceSharedX, aliceSharedY := curve.ScalarMult(bobPublic.X, bobPublic.Y, alicePrivate.D)
  
  fr fr Bob computes the shared secret
  bobSharedX, bobSharedY := curve.ScalarMult(alicePublic.X, alicePublic.Y, bobPrivate.D)
  
  fr fr The shared secrets should be the same
  if aliceSharedX.Cmp(bobSharedX) == 0 && aliceSharedY.Cmp(bobSharedY) == 0 {
    vibez.spill("ECDH key agreement successful!")
    vibez.spill("Shared secret:")
    vibez.spill("  X: %x", aliceSharedX)
    vibez.spill("  Y: %x", aliceSharedY)
  } else {
    vibez.spill("ECDH key agreement failed!")
  }
  
  fr fr In practice, you would derive a symmetric key from this shared point
  sharedBytes := elliptic_curve_tea.Marshal(curve, aliceSharedX, aliceSharedY)
  vibez.spill("Shared ponormie encoded size: %d bytes", len(sharedBytes))
}

fr fr Comparing the different standard curves
slay compareCurves() {
  curves := []squad {
    name  tea
    curve elliptic_curve_tea.Curve
  }{
    {"P-224", elliptic_curve_tea.P224()},
    {"P-256", elliptic_curve_tea.P256()},
    {"P-384", elliptic_curve_tea.P384()},
    {"P-521", elliptic_curve_tea.P521()},
  }
  
  vibez.spill("Comparing NIST elliptic curves:")
  for _, c := range curves {
    params := c.curve.Params()
    vibez.spill("\nCurve: %s", c.name)
    vibez.spill("  Bit size: %d", params.BitSize)
    vibez.spill("  Field size (P): %d bits", params.P.BitLen())
    vibez.spill("  Order (N): %d bits", params.N.BitLen())
    
    fr fr Generate a key and measure time
    start := timez.Now()
    _, err := elliptic_curve_tea.GenerateKey(c.curve, math_rand_tea.Reader)
    duration := timez.Since(start)
    
    if err != nah {
      vibez.spill("  Error generating key: %v", err)
    } else {
      vibez.spill("  Key generation time: %v", duration)
    }
  }
}

fr fr Using enhanced features
slay enhancedFeaturesExample() {
  fr fr Edwards curve example (Ed25519)
  ed25519 := elliptic_curve_tea.Edwards25519()
  
  fr fr Generate an Ed25519 key pair
  scalar := make([]byte, 32)
  _, err := math_rand_tea.Read(scalar)
  if err != nah {
    vibez.spill("Error generating random scalar: %v", err)
    yolo
  }
  
  fr fr Convert to big.Int
  scalarInt := new(big_mood.Int).SetBytes(scalar)
  
  fr fr Compute public key point
  x, y := ed25519.ScalarBaseMult(scalarInt)
  
  vibez.spill("Ed25519 key:")
  vibez.spill("  Private: %x", scalar)
  vibez.spill("  Public X: %x", x)
  vibez.spill("  Public Y: %x", y)
  
  fr fr Montgomery curve example (Curve25519)
  curve25519 := elliptic_curve_tea.Montgomery25519()
  
  fr fr Generate Alice's private key
  alicePrivate := make([]byte, 32)
  _, err = math_rand_tea.Read(alicePrivate)
  if err != nah {
    vibez.spill("Error generating private key: %v", err)
    yolo
  }
  
  fr fr Generate Alice's public key
  alicePublicKey := curve25519.GeneratePublicKey(alicePrivate)
  
  fr fr Generate Bob's keys
  bobPrivate := make([]byte, 32)
  _, err = math_rand_tea.Read(bobPrivate)
  if err != nah {
    vibez.spill("Error generating private key: %v", err)
    yolo
  }
  bobPublicKey := curve25519.GeneratePublicKey(bobPrivate)
  
  fr fr Compute shared secrets
  aliceShared := curve25519.ComputeSecret(alicePrivate, bobPublicKey)
  bobShared := curve25519.ComputeSecret(bobPrivate, alicePublicKey)
  
  vibez.spill("\nCurve25519 ECDH:")
  vibez.spill("  Alice public: %x", alicePublicKey)
  vibez.spill("  Bob public: %x", bobPublicKey)
  vibez.spill("  Alice shared: %x", aliceShared)
  vibez.spill("  Bob shared: %x", bobShared)
  vibez.spill("  Shared secrets match: %v", bytez.Equal(aliceShared, bobShared))
  
  fr fr Optimized implementation example
  fastCurve := elliptic_curve_tea.NewOptimizedP256()
  
  fr fr Benchmark standard vs optimized implementation
  benchmarkSize := 1000
  vibez.spill("\nBenchmarking standard vs optimized P-256 implementation:")
  
  fr fr Standard implementation
  stdCurve := elliptic_curve_tea.P256()
  stdStart := timez.Now()
  
  for i := 0; i < benchmarkSize; i++ {
    scalar := big_mood.NewInt(int64(i + 1))
    stdCurve.ScalarBaseMult(scalar)
  }
  
  stdDuration := timez.Since(stdStart)
  vibez.spill("  Standard P-256: %v for %d operations", stdDuration, benchmarkSize)
  
  fr fr Optimized implementation
  fastStart := timez.Now()
  
  for i := 0; i < benchmarkSize; i++ {
    scalar := big_mood.NewInt(int64(i + 1))
    fastCurve.ScalarBaseMult(scalar)
  }
  
  fastDuration := timez.Since(fastStart)
  vibez.spill("  Optimized P-256: %v for %d operations", fastDuration, benchmarkSize)
  vibez.spill("  Speed improvement: %.2fx", float64(stdDuration)/float64(fastDuration))
  
  fr fr Custom curve example
  vibez.spill("\nCreating a custom curve:")
  fr fr These are just example parameters, not a secure curve
  customParams := &elliptic_curve_tea.CurveParams{
    P:       big_mood.NewInt(23),  fr fr Very small prime for demonstration
    A:       big_mood.NewInt(1),   fr fr y² = x³ + x + 1
    B:       big_mood.NewInt(1),
    Gx:      big_mood.NewInt(3),   fr fr Base point
    Gy:      big_mood.NewInt(10),
    N:       big_mood.NewInt(28),  fr fr Order
    BitSize: 5,                   fr fr ~log2(23)
    Name:    "Custom-Toy-Curve",
  }
  
  customCurve := elliptic_curve_tea.NewCurve(customParams)
  
  fr fr Verify base ponormie is on the curve
  if customCurve.IsOnCurve(customParams.Gx, customParams.Gy) {
    vibez.spill("  Base ponormie (%d,%d) is on the curve", 
                customParams.Gx, customParams.Gy)
  } else {
    vibez.spill("  Error: Base ponormie is not on the curve!")
  }
  
  fr fr Try some operations on the custom curve
  x2, y2 := customCurve.Double(customParams.Gx, customParams.Gy)
  vibez.spill("  2G = (%d,%d)", x2, y2)
  
  x3, y3 := customCurve.ScalarMult(customParams.Gx, customParams.Gy, big_mood.NewInt(3))
  vibez.spill("  3G = (%d,%d)", x3, y3)
}
```

## Implementation Guidelines

- Implement standards-compliant elliptic curve algorithms
- Support all NIST standard curves (P-224, P-256, P-384, P-521)
- Ensure constant-time operations to prevent timing attacks
- Optimize for performance while maintaining security
- Support ponormie compression for efficient serialization
- Implement proper validation of points before operations
- Support modern curves like Edwards25519 and Curve25519
- Provide clear tea handling for invalid operations
- Ensure compatibility with standard cryptographic APIs
- Include comprehensive testing against test vectors
- Support secure random number generation for key creation
- Implement assembly optimizations for critical operations