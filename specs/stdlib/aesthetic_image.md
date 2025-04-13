# AestheticImage (image package)

## Overview
AestheticImage provides basic 2D image processing capabilities with aesthetic enhancements for modern applications. It's inspired by Go's image package but with extended functionality for filters, transformations, and other visual improvements.

## Core Types

### `Point`
Represents a point in the image plane.

```go
type Point struct {
    X, Y int
}

// Methods
func (p Point) Add(q Point) Point
func (p Point) Sub(q Point) Point
func (p Point) In(r Rectangle) bool
func (p Point) Mod(r Rectangle) Point
func (p Point) Eq(q Point) bool
func (p Point) Distance(q Point) float64 // New method
func (p Point) Angle(q Point) float64 // New method
```

### `Rectangle`
Represents a rectangle in the image plane.

```go
type Rectangle struct {
    Min, Max Point
}

// Constructors
func Rect(x0, y0, x1, y1 int) Rectangle

// Methods
func (r Rectangle) At(x, y int) bool
func (r Rectangle) Dx() int
func (r Rectangle) Dy() int
func (r Rectangle) Size() Point
func (r Rectangle) Add(p Point) Rectangle
func (r Rectangle) Sub(p Point) Rectangle
func (r Rectangle) Inset(n int) Rectangle
func (r Rectangle) Intersect(s Rectangle) Rectangle
func (r Rectangle) Union(s Rectangle) Rectangle
func (r Rectangle) Empty() bool
func (r Rectangle) Eq(s Rectangle) bool
func (r Rectangle) In(s Rectangle) bool
func (r Rectangle) Overlaps(s Rectangle) bool
func (r Rectangle) Area() int // New method
func (r Rectangle) AspectRatio() float64 // New method
func (r Rectangle) Center() Point // New method
```

### `Color`
Represents a color in the image.

```go
type Color interface {
    RGBA() (r, g, b, a uint32)
}

type RGBA struct{ R, G, B, A uint8 }
type RGBA64 struct{ R, G, B, A uint16 }
type NRGBA struct{ R, G, B, A uint8 }
type NRGBA64 struct{ R, G, B, A uint16 }
type Alpha struct{ A uint8 }
type Alpha16 struct{ A uint16 }
type Gray struct{ Y uint8 }
type Gray16 struct{ Y uint16 }

// Methods
func (c RGBA) RGBA() (r, g, b, a uint32)
func (c RGBA64) RGBA() (r, g, b, a uint32)
// ... methods for other color types

// HSL/HSV color support
type HSL struct{ H, S, L float64 }
type HSV struct{ H, S, V float64 }

// Constructor functions
func RGBAColor(r, g, b, a uint8) RGBA
func RGBColor(r, g, b uint8) RGBA // Alpha is implicitly 255
func HSLColor(h, s, l float64) HSL
func HSVColor(h, s, v float64) HSV
func HexColor(hex string) (RGBA, error) // Parse HTML hex color

// Conversion functions
func HSLToRGB(h, s, l float64) RGBA
func HSVToRGB(h, s, v float64) RGBA
func RGBToHSL(r, g, b uint8) HSL
func RGBToHSV(r, g, b uint8) HSV
```

### `Image`
Represents a rectangular grid of colors.

```go
type Image interface {
    ColorModel() color.Model
    Bounds() Rectangle
    At(x, y int) Color
}

type RGBA struct {
    // Contains pixel data and dimensions
    Pix    []uint8
    Stride int
    Rect   Rectangle
}

type RGBA64 struct {...}
type NRGBA struct {...}
type NRGBA64 struct {...}
type Alpha struct {...}
type Alpha16 struct {...}
type Gray struct {...}
type Gray16 struct {...}
type CMYK struct {...}

// Constructor functions
func NewRGBA(r Rectangle) *RGBA
func NewRGBA64(r Rectangle) *RGBA64
func NewNRGBA(r Rectangle) *NRGBA
func NewNRGBA64(r Rectangle) *NRGBA64
func NewAlpha(r Rectangle) *Alpha
func NewAlpha16(r Rectangle) *Alpha16
func NewGray(r Rectangle) *Gray
func NewGray16(r Rectangle) *Gray16
func NewCMYK(r Rectangle) *CMYK

// Methods for image types
func (p *RGBA) Set(x, y int, c Color)
func (p *RGBA) SetRGBA(x, y int, c RGBA)
func (p *RGBA) PixOffset(x, y int) int
func (p *RGBA) Subimage(r Rectangle) Image
// ... similar methods for other image types
```

## Drawing Functions

```go
// Draw an image onto another image using a specified compositing operation
func Draw(dst Image, r Rectangle, src Image, sp Point, op Op)

// DrawMask draws an image onto another image using a mask
func DrawMask(dst Image, r Rectangle, src Image, sp Point, mask Image, mp Point, op Op)

// Compositing operations
type Op int
const (
    Over Op = iota
    Src
    // Additional compositing operations
    Multiply
    Screen
    Overlay
    Darken
    Lighten
    ColorDodge
    ColorBurn
    HardLight
    SoftLight
    Difference
    Exclusion
    Hue
    Saturation
    Color
    Luminosity
)
```

## Extended Drawing Operations

```go
// Basic shape drawing
func DrawLine(dst Image, x0, y0, x1, y1 int, c Color, thickness int)
func DrawRect(dst Image, r Rectangle, c Color, thickness int)
func DrawFilledRect(dst Image, r Rectangle, c Color)
func DrawCircle(dst Image, cx, cy, radius int, c Color, thickness int)
func DrawFilledCircle(dst Image, cx, cy, radius int, c Color)
func DrawEllipse(dst Image, cx, cy, rx, ry int, c Color, thickness int)
func DrawFilledEllipse(dst Image, cx, cy, rx, ry int, c Color)
func DrawPolygon(dst Image, points []Point, c Color, thickness int)
func DrawFilledPolygon(dst Image, points []Point, c Color)

// Text drawing
func DrawText(dst Image, x, y int, text string, font Font, c Color)
func DrawWrappedText(dst Image, r Rectangle, text string, font Font, c Color, align TextAlign)

type TextAlign int
const (
    AlignLeft TextAlign = iota
    AlignCenter
    AlignRight
)

type Font struct {
    // Font properties
    Family string
    Size   float64
    Weight FontWeight
    Style  FontStyle
}

type FontWeight int
const (
    WeightNormal FontWeight = iota
    WeightBold
    WeightThin
    WeightLight
    WeightMedium
    WeightHeavy
)

type FontStyle int
const (
    StyleNormal FontStyle = iota
    StyleItalic
    StyleOblique
)
```

## Image Transformations

```go
// Resize an image to the specified dimensions
func Resize(img Image, width, height int, filter ResizeFilter) Image

// Scale an image by a factor
func Scale(img Image, factor float64, filter ResizeFilter) Image

// Resample an image to fit within the given dimensions while preserving aspect ratio
func Fit(img Image, width, height int, filter ResizeFilter) Image

// Fill resizes and crops an image to fill the given dimensions while preserving aspect ratio
func Fill(img Image, width, height int, filter ResizeFilter) Image

// Thumbnail creates a thumbnail of the image with the given size
func Thumbnail(img Image, size int, filter ResizeFilter) Image

// Rotate an image by the specified angle in degrees
func Rotate(img Image, angle float64, backgroundColor Color) Image

// Flip an image horizontally or vertically
func FlipHorizontal(img Image) Image
func FlipVertical(img Image) Image

// Crop an image to the specified rectangle
func Crop(img Image, r Rectangle) Image

// Available resize filters
type ResizeFilter interface {...}

var (
    NearestNeighborFilter ResizeFilter = ...
    BilinearFilter        ResizeFilter = ...
    BicubicFilter         ResizeFilter = ...
    LanczosFilter         ResizeFilter = ...
)
```

## Image Processing and Filters

```go
// Adjustments
func AdjustBrightness(img Image, factor float64) Image
func AdjustContrast(img Image, factor float64) Image
func AdjustGamma(img Image, gamma float64) Image
func AdjustSaturation(img Image, factor float64) Image
func AdjustHue(img Image, degrees float64) Image

// Color manipulation
func Grayscale(img Image) Image
func Sepia(img Image) Image
func Invert(img Image) Image
func Colorize(img Image, color Color, amount float64) Image

// Filters
func Blur(img Image, radius float64) Image
func GaussianBlur(img Image, sigma float64) Image
func Sharpen(img Image, amount float64) Image
func EdgeDetection(img Image) Image
func Emboss(img Image) Image
func Pixelate(img Image, blockSize int) Image

// Convolution-based filters
func Convolve(img Image, kernel [][]float64) Image
func Convolve3x3(img Image, kernel [3][3]float64) Image
func Convolve5x5(img Image, kernel [5][5]float64) Image

// Common convolution kernels
var (
    SobelXKernel       [3][3]float64 = ...
    SobelYKernel       [3][3]float64 = ...
    LaplacianKernel    [3][3]float64 = ...
    GaussianKernel3x3  [3][3]float64 = ...
    GaussianKernel5x5  [5][5]float64 = ...
    SharpenKernel      [3][3]float64 = ...
    EmbossKernel       [3][3]float64 = ...
    EdgeDetectKernel   [3][3]float64 = ...
    BoxBlurKernel      [3][3]float64 = ...
)
```

## Image Effects

```go
// Advanced effects
func Vignette(img Image, strength float64) Image
func GradientOverlay(img Image, start, end Color, angle float64) Image
func DropShadow(img Image, offsetX, offsetY, blur int, color Color) Image
func Glow(img Image, radius float64, color Color) Image

// Artistic filters
func OilPainting(img Image, radius, intensity int) Image
func Watercolor(img Image, radius, iterations int) Image
func Posterize(img Image, levels int) Image
func CrossProcess(img Image) Image

// Instagram-like filters
func Nashville(img Image) Image
func Valencia(img Image) Image
func Mayfair(img Image) Image
func Xpro2(img Image) Image
func Lo-Fi(img Image) Image
```

## Image I/O

```go
// Decode an image from a reader
func Decode(r io.Reader) (Image, string, error)
func DecodeConfig(r io.Reader) (ImageConfig, string, error)

// Format-specific decoders
func DecodePNG(r io.Reader) (Image, error)
func DecodeJPEG(r io.Reader) (Image, error)
func DecodeGIF(r io.Reader) (Image, error)
func DecodeBMP(r io.Reader) (Image, error)
func DecodeTIFF(r io.Reader) (Image, error)
func DecodeWebP(r io.Reader) (Image, error) // New format support

// Encode an image to a writer
func EncodeJPEG(w io.Writer, img Image, quality int) error
func EncodePNG(w io.Writer, img Image) error
func EncodeGIF(w io.Writer, img Image, options *GIFOptions) error
func EncodeBMP(w io.Writer, img Image) error
func EncodeTIFF(w io.Writer, img Image, options *TIFFOptions) error
func EncodeWebP(w io.Writer, img Image, quality int) error // New format support

// Register a format for use with Decode
func RegisterFormat(name, magic string, decoder func(io.Reader) (Image, error), decodeConfig func(io.Reader) (ImageConfig, error))
```

## Animated Image Support

```go
type Animation interface {
    Image                      // The first frame
    Frames() []Frame           // All frames
    LoopCount() int            // Number of loops, 0 for infinite
    Duration() time.Duration   // Total duration
}

type Frame struct {
    Image    Image
    Duration time.Duration
    DisposalMethod DisposalMethod
}

type DisposalMethod int
const (
    DisposalNone DisposalMethod = iota
    DisposalBackground
    DisposalPrevious
    DisposalUnspecified
)

// Animated GIF handling
func DecodeAnimatedGIF(r io.Reader) (Animation, error)
func EncodeAnimatedGIF(w io.Writer, anim Animation, options *GIFOptions) error
func NewAnimation() *AnimationBuilder

type AnimationBuilder struct{}
func (ab *AnimationBuilder) AddFrame(img Image, duration time.Duration) *AnimationBuilder
func (ab *AnimationBuilder) SetLoopCount(count int) *AnimationBuilder
func (ab *AnimationBuilder) Build() Animation
```

## Additional Utilities

```go
// Calculate image hash for similarity comparison
func AverageHash(img Image) []byte
func PerceptualHash(img Image) []byte
func DifferenceHash(img Image) []byte
func ImageDistance(hash1, hash2 []byte) float64

// Image metadata
func ExtractMetadata(img Image) Metadata
func SetMetadata(img Image, metadata Metadata) Image

type Metadata struct {
    Width, Height int
    Format        string
    ColorSpace    string
    ProfileName   string
    Created       time.Time
    Orientation   int
    GPSLatitude   float64
    GPSLongitude  float64
    // Other EXIF/IPTC data
    ExifData      map[string]interface{}
    IPTCData      map[string]interface{}
    XMPData       map[string]interface{}
}
```

## Usage Example

```go
// Loading an image
imgFile, err := dropz.Open("input.jpg")
if err != nil {
    vibez.spill("Failed to open image:", err)
    return
}
defer imgFile.Close()

img, _, err := aesthetic_image.Decode(imgFile)
if err != nil {
    vibez.spill("Failed to decode image:", err)
    return
}

// Image information
bounds := img.Bounds()
vibez.spill("Image dimensions:", bounds.Dx(), "x", bounds.Dy())
vibez.spill("Aspect ratio:", bounds.AspectRatio())

// Simple transformations
resized := aesthetic_image.Resize(img, 800, 600, aesthetic_image.LanczosFilter)
fitted := aesthetic_image.Fit(img, 800, 600, aesthetic_image.BilinearFilter)
rotated := aesthetic_image.Rotate(img, 45, aesthetic_image.RGBColor(255, 255, 255))

// Applying filters
grayscale := aesthetic_image.Grayscale(img)
blurred := aesthetic_image.GaussianBlur(img, 5.0)
sharpened := aesthetic_image.Sharpen(img, 1.5)

// Creating a new image and drawing on it
canvas := aesthetic_image.NewRGBA(aesthetic_image.Rect(0, 0, 400, 400))

// Fill background
aesthetic_image.DrawFilledRect(canvas, canvas.Bounds(), aesthetic_image.RGBColor(240, 240, 240))

// Draw shapes
aesthetic_image.DrawFilledCircle(canvas, 200, 200, 100, aesthetic_image.RGBColor(255, 0, 0))
aesthetic_image.DrawRect(canvas, aesthetic_image.Rect(50, 50, 350, 350), aesthetic_image.RGBColor(0, 0, 255), 3)
aesthetic_image.DrawLine(canvas, 0, 0, 400, 400, aesthetic_image.RGBColor(0, 255, 0), 2)

// Draw text
font := aesthetic_image.Font{
    Family: "Arial",
    Size:   24,
    Weight: aesthetic_image.WeightBold,
    Style:  aesthetic_image.StyleNormal,
}
aesthetic_image.DrawText(canvas, 100, 50, "Hello, Aesthetic!", font, aesthetic_image.RGBColor(0, 0, 0))

// Apply gradient overlay
gradient := aesthetic_image.GradientOverlay(canvas, 
    aesthetic_image.HSLColor(0, 0.8, 0.5),  // Pink
    aesthetic_image.HSLColor(270, 0.8, 0.5), // Purple
    45, // 45 degree angle
)

// Apply a vignette effect
vignette := aesthetic_image.Vignette(gradient, 0.5)

// Save result
outFile, err := dropz.Create("output.png")
if err != nil {
    vibez.spill("Failed to create output file:", err)
    return
}
defer outFile.Close()

err = aesthetic_image.EncodePNG(outFile, vignette)
if err != nil {
    vibez.spill("Failed to encode image:", err)
    return
}

// Working with animated images
gifFile, err := dropz.Open("animation.gif")
if err != nil {
    vibez.spill("Failed to open GIF:", err)
    return
}
defer gifFile.Close()

anim, err := aesthetic_image.DecodeAnimatedGIF(gifFile)
if err != nil {
    vibez.spill("Failed to decode animated GIF:", err)
    return
}

vibez.spill("Animation frames:", len(anim.Frames()))
vibez.spill("Animation duration:", anim.Duration())

// Create a processed version of each frame
frames := anim.Frames()
processedFrames := make([]aesthetic_image.Frame, len(frames))

for i, frame := range frames {
    // Apply sepia tone to each frame
    sepiaImg := aesthetic_image.Sepia(frame.Image)
    
    // Create a new frame with the processed image
    processedFrames[i] = aesthetic_image.Frame{
        Image:           sepiaImg,
        Duration:        frame.Duration,
        DisposalMethod:  frame.DisposalMethod,
    }
}

// Create a new animation from the processed frames
newAnim := aesthetic_image.NewAnimation()
for _, frame := range processedFrames {
    newAnim.AddFrame(frame.Image, frame.Duration)
}
newAnim.SetLoopCount(0) // Infinite loops

// Save the processed animation
outGif, err := dropz.Create("sepia_animation.gif")
if err != nil {
    vibez.spill("Failed to create output file:", err)
    return
}
defer outGif.Close()

options := &aesthetic_image.GIFOptions{
    NumColors: 256,
    Quantizer: aesthetic_image.MedianCutQuantizer{},
    Drawer:    aesthetic_image.FloydSteinbergDitherer{},
}

err = aesthetic_image.EncodeAnimatedGIF(outGif, newAnim.Build(), options)
if err != nil {
    vibez.spill("Failed to encode animated GIF:", err)
    return
}
```

## Implementation Guidelines
1. Optimize for both performance and memory usage
2. Support a wide range of image formats and color models
3. Provide robust error handling for malformed images
4. Use parallelism for computationally intensive operations
5. Ensure high-quality output for all transformations and effects
6. Implement proper color management with ICC profile support
7. Maintain backward compatibility with Go's image package
8. Support progressive loading for large images