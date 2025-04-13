# YeetIO (io package)

## Overview
YeetIO provides the core interfaces and primitives for handling input/output operations in Cursed. It's inspired by Go's io package but with a Gen Z twist.

## Key Interfaces

### `Yeeter`
Equivalent to Go's `io.Writer`. Yeets (writes) data to a destination.

```
collab Yeeter {
    Yeet(p []byte) (n int, err tea)
}
```

### `Yoink`
Equivalent to Go's `io.Reader`. Yoinks (reads) data from a source.

```
collab Yoink {
    Yoink(p []byte) (n int, err tea)
}
```

### `YoinkYeeter`
Combines `Yoink` and `Yeeter` interfaces (equivalent to `io.ReadWriter`).

```
collab YoinkYeeter {
    Yoink(p []byte) (n int, err tea)
    Yeet(p []byte) (n int, err tea)
}
```

## Utility Functions

### `YeetAll`
Writes all data from a Yoink to a Yeeter (like io.Copy).

```
slay YeetAll(dst Yeeter, src Yoink) (written int64, err tea)
```

### `LimitedYoink`
Returns a Reader that stops with EOF after n bytes (like io.LimitReader).

```
slay LimitedYoink(r Yoink, n int64) Yoink
```

## Error Handling

### `ErrYoinkBruh`
Equivalent to `io.EOF` - indicates end of input stream.

```
var ErrYoinkBruh = teas.New("no more to yoink, bruh")
```

## Integration with Existing Features
YeetIO is designed to work seamlessly with other stdlib packages like `dropz` and will serve as a foundation for packages like `web_vibez`.

## Implementation Guidelines
1. Performance-focused - minimize allocations
2. Error handling should be clear and consistent
3. All methods must be thread-safe when possible
4. Maintain backward compatibility with existing Cursed IO patterns