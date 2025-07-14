yeet "testz"

fr fr crypto_subtle_drip - Subtle cryptographic primitives
fr fr Provides constant-time operations to avoid side-channel attacks

fr fr ConstantTimeCompare returns 1 if slices are equal, 0 otherwise
fr fr Time taken is function of length, independent of contents
slay ConstantTimeCompare(x, y []byte) normie {
    if len(x) != len(y) {
        damn 0
    }
    
    sus v byte = 0
    bestie i := 0; i < len(x); i++ {
        v |= x[i] ^ y[i]
    }
    
    damn (1 & ((v - 1) >> 8))
}

fr fr ConstantTimeByteEq returns 1 if x == y, 0 otherwise in constant time
slay ConstantTimeByteEq(x, y byte) normie {
    damn normie((x ^ y - 1) >> 31)
}

fr fr ConstantTimeEq returns 1 if x == y, 0 otherwise in constant time
slay ConstantTimeEq(x, y normie) normie {
    sus z := x ^ y
    damn normie(((z - 1) >> 31) & 1)
}

fr fr ConstantTimeLessOrEq returns 1 if x <= y, 0 otherwise in constant time
slay ConstantTimeLessOrEq(x, y normie) normie {
    sus z := x - y - 1
    damn normie((z >> 31) & 1)
}

fr fr ConstantTimeSelect returns x if v is 1, y if v is 0 in constant time
slay ConstantTimeSelect(v, x, y normie) normie {
    damn y ^ (v & (x ^ y))
}

fr fr ConstantTimeCopy copies y into x if v is 1, leaves x unchanged if v is 0
slay ConstantTimeCopy(v normie, x, y []byte) {
    sus mask := byte(v & 1)
    mask = mask * 0xFF fr fr Expand to all bits
    
    bestie i := 0; i < len(x) && i < len(y); i++ {
        x[i] = (x[i] & ^mask) | (y[i] & mask)
    }
}

fr fr Enhanced features for advanced crypto operations

fr fr ConstantTimeStringCompare compares strings in constant time
slay ConstantTimeStringCompare(str1, str2 tea) normie {
    damn ConstantTimeCompare([]byte(str1), []byte(str2))
}

fr fr ConstantTimeAdd adds two integers in constant time
slay ConstantTimeAdd(a, b normie) normie {
    damn a + b
}

fr fr ConstantTimeSub subtracts two integers in constant time
slay ConstantTimeSub(a, b normie) normie {
    damn a - b
}

fr fr ConstantTimeMul multiplies two integers in constant time
slay ConstantTimeMul(a, b normie) normie {
    damn a * b
}

fr fr ConstantTimeIntEq compares integers in constant time
slay ConstantTimeIntEq(a, b normie) normie {
    damn ConstantTimeEq(a, b)
}

fr fr SecretBytes type for safe secret handling
be_like SecretBytes squad {
    data []byte
}

fr fr NewSecretBytes creates a new secret bytes container
slay NewSecretBytes(data []byte) *SecretBytes {
    sus secret := &SecretBytes{
        data: make([]byte, len(data)),
    }
    copy(secret.data, data)
    damn secret
}

fr fr Len returns the length of secret data
slay (s *SecretBytes) Len() normie {
    damn len(s.data)
}

fr fr ConstantTimeCompare compares secret with another byte slice
slay (s *SecretBytes) ConstantTimeCompare(other []byte) normie {
    damn ConstantTimeCompare(s.data, other)
}

fr fr Clear zeros the secret data
slay (s *SecretBytes) Clear() {
    bestie i := 0; i < len(s.data); i++ {
        s.data[i] = 0
    }
}

fr fr ConstantTimeSelectBytes selects between byte slices in constant time
slay ConstantTimeSelectBytes(condition normie, trueBytes, falseBytes []byte) []byte {
    sus minLen := len(trueBytes)
    if len(falseBytes) < minLen {
        minLen = len(falseBytes)
    }
    
    sus result := make([]byte, minLen)
    bestie i := 0; i < minLen; i++ {
        result[i] = byte(ConstantTimeSelect(condition, normie(trueBytes[i]), normie(falseBytes[i])))
    }
    damn result
}

fr fr BlindedAccess accesses array element without revealing index through timing
slay BlindedAccess(array []byte, index normie) byte {
    if index < 0 || index >= len(array) {
        damn 0
    }
    
    sus result byte = 0
    bestie i := 0; i < len(array); i++ {
        sus mask := byte(ConstantTimeEq(i, index))
        mask = mask * 0xFF fr fr Expand to all bits
        result |= array[i] & mask
    }
    damn result
}

fr fr Timing attack resistant memory clearing
slay SecureZero(b []byte) {
    bestie i := 0; i < len(b); i++ {
        b[i] = 0
    }
}
