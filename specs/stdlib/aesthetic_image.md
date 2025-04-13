# AestheticImage (image package)

## Overview
AestheticImage provides basic 2D image processing capabilities with aesthetic enhancements for modern applications. It's inspired by Go's image package but with extended functionality for filters, transformations, and other visual improvements.

## Core Types

### `Point`
Represents a ponormie in the image plane.

```
be_like Ponormie squad {
    X, Y int
}

fr fr Methods
slay (p Ponormie) Add(q Ponormie) Point
slay (p Ponormie) Sub(q Ponormie) Point
slay (p Ponormie) In(r Rectangle) lit
slay (p Ponormie) Mod(r Rectangle) Point
slay (p Ponormie) Eq(q Ponormie) lit
slay (p Ponormie) Distance(q Ponormie) float64 fr fr New method
slay (p Ponormie) Angle(q Ponormie) float64 fr fr New method
```

### `Rectangle`
Represents a rectangle in the image plane.

```
be_like Rectangle squad {
    Min, Max Point
}

fr fr Consquadors
slay Rect(x0, y0, x1, y1 normie) Rectangle

fr fr Methods
slay (r Rectangle) At(x, y normie) lit
slay (r Rectangle) Dx() int
slay (r Rectangle) Dy() int
slay (r Rectangle) Size() Point
slay (r Rectangle) Add(p Ponormie) Rectangle
slay (r Rectangle) Sub(p Ponormie) Rectangle
slay (r Rectangle) Inset(n normie) Rectangle
slay (r Rectangle) Intersect(s Rectangle) Rectangle
slay (r Rectangle) Union(s Rectangle) Rectangle
slay (r Rectangle) Empty() lit
slay (r Rectangle) Eq(s Rectangle) lit
slay (r Rectangle) In(s Rectangle) lit
slay (r Rectangle) Overlaps(s Rectangle) lit
slay (r Rectangle) Area() normie fr fr New method
slay (r Rectangle) AspectRatio() float64 fr fr New method
slay (r Rectangle) Center() Ponormie fr fr New method
```

### `Color`
Represents a color in the image.

```
be_like Color collab {
    RGBA() (r, g, b, a uint32)
}

be_like RGBA squad{ R, G, B, A uint8 }
be_like RGBA64 squad{ R, G, B, A uint16 }
be_like NRGBA squad{ R, G, B, A uint8 }
be_like NRGBA64 squad{ R, G, B, A uint16 }
be_like Alpha squad{ A uint8 }
be_like Alpha16 squad{ A uint16 }
be_like Gray squad{ Y uint8 }
be_like Gray16 squad{ Y uint16 }

fr fr Methods
slay (c RGBA) RGBA() (r, g, b, a uint32)
slay (c RGBA64) RGBA() (r, g, b, a uint32)
fr fr ... methods for other color types

fr fr HSL/HSV color support
be_like HSL squad{ H, S, L float64 }
be_like HSV squad{ H, S, V float64 }

fr fr Consquador functions
slay RGBAColor(r, g, b, a uint8) RGBA
slay RGBColor(r, g, b uint8) RGBA fr fr Alpha is implicitly 255
slay HSLColor(h, s, l float64) HSL
slay HSVColor(h, s, v float64) HSV
slay HexColor(hex tea) (RGBA, tea) fr fr Parse HTML hex color

fr fr Conversion functions
slay HSLToRGB(h, s, l float64) RGBA
slay HSVToRGB(h, s, v float64) RGBA
slay RGBToHSL(r, g, b uint8) HSL
slay RGBToHSV(r, g, b uint8) HSV
```

### `Image`
Represents a rectangular grid of colors.

```
be_like Image collab {
    ColorModel() color.Model
    Bounds() Rectangle
    At(x, y normie) Color
}

be_like RGBA squad {
    fr fr Contains pixel data and dimensions
    Pix    []uint8
    Stride int
    Rect   Rectangle
}

be_like RGBA64 squad {...}
be_like NRGBA squad {...}
be_like NRGBA64 squad {...}
be_like Alpha squad {...}
be_like Alpha16 squad {...}
be_like Gray squad {...}
be_like Gray16 squad {...}
be_like CMYK squad {...}

fr fr Consquador functions
slay NewRGBA(r Rectangle) *RGBA
slay NewRGBA64(r Rectangle) *RGBA64
slay NewNRGBA(r Rectangle) *NRGBA
slay NewNRGBA64(r Rectangle) *NRGBA64
slay NewAlpha(r Rectangle) *Alpha
slay NewAlpha16(r Rectangle) *Alpha16
slay NewGray(r Rectangle) *Gray
slay NewGray16(r Rectangle) *Gray16
slay NewCMYK(r Rectangle) *CMYK

fr fr Methods for image types
slay (p *RGBA) Set(x, y int, c Color)
slay (p *RGBA) SetRGBA(x, y int, c RGBA)
slay (p *RGBA) PixOffset(x, y normie) int
slay (p *RGBA) Subimage(r Rectangle) Image
fr fr ... similar methods for other image types
```

## Drawing Functions

```
fr fr Draw an image onto another image using a specified compositing operation
slay Draw(dst Image, r Rectangle, src Image, sp Point, op Op)

fr fr DrawMask draws an image onto another image using a mask
slay DrawMask(dst Image, r Rectangle, src Image, sp Point, mask Image, mp Point, op Op)

fr fr Compositing operations
be_like Op int
const (
    Over Op = iota
    Src
    fr fr Additional compositing operations
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

```
fr fr Basic shape drawing
slay DrawLine(dst Image, x0, y0, x1, y1 int, c Color, thickness normie)
slay DrawRect(dst Image, r Rectangle, c Color, thickness normie)
slay DrawFilledRect(dst Image, r Rectangle, c Color)
slay DrawCircle(dst Image, cx, cy, radius int, c Color, thickness normie)
slay DrawFilledCircle(dst Image, cx, cy, radius int, c Color)
slay DrawEllipse(dst Image, cx, cy, rx, ry int, c Color, thickness normie)
slay DrawFilledEllipse(dst Image, cx, cy, rx, ry int, c Color)
slay DrawPolygon(dst Image, points []Point, c Color, thickness normie)
slay DrawFilledPolygon(dst Image, points []Point, c Color)

fr fr Text drawing
slay DrawText(dst Image, x, y int, text tea, font Font, c Color)
slay DrawWrappedText(dst Image, r Rectangle, text tea, font Font, c Color, align TextAlign)

be_like TextAlign int
const (
    AlignLeft TextAlign = iota
    AlignCenter
    AlignRight
)

be_like Font squad {
    fr fr Font properties
    Family tea
    Size   float64
    Weight FontWeight
    Style  FontStyle
}

be_like FontWeight int
const (
    WeightNormal FontWeight = iota
    WeightBold
    WeightThin
    WeightLight
    WeightMedium
    WeightHeavy
)

be_like FontStyle int
const (
    StyleNormal FontStyle = iota
    StyleItalic
    StyleOblique
)
```

## Image Transformations

```
fr fr Resize an image to the specified dimensions
slay Resize(img Image, width, height int, filter ResizeFilter) Image

fr fr Scale an image by a factor
slay Scale(img Image, factor float64, filter ResizeFilter) Image

fr fr Resample an image to fit within the given dimensions while preserving aspect ratio
slay Fit(img Image, width, height int, filter ResizeFilter) Image

fr fr Fill resizes and crops an image to fill the given dimensions while preserving aspect ratio
slay Fill(img Image, width, height int, filter ResizeFilter) Image

fr fr Thumbnail creates a thumbnail of the image with the given size
slay Thumbnail(img Image, size int, filter ResizeFilter) Image

fr fr Rotate an image by the specified angle in degrees
slay Rotate(img Image, angle float64, backgroundColor Color) Image

fr fr Flip an image horizontally or vertically
slay FlipHorizontal(img Image) Image
slay FlipVertical(img Image) Image

fr fr Crop an image to the specified rectangle
slay Crop(img Image, r Rectangle) Image

fr fr Available resize filters
be_like ResizeFilter collab {...}

var (
    NearestNeighborFilter ResizeFilter = ...
    BilinearFilter        ResizeFilter = ...
    BicubicFilter         ResizeFilter = ...
    LanczosFilter         ResizeFilter = ...
)
```

## Image Processing and Filters

```
fr fr Adjustments
slay AdjustBrightness(img Image, factor float64) Image
slay AdjustContrast(img Image, factor float64) Image
slay AdjustGamma(img Image, gamma float64) Image
slay AdjustSaturation(img Image, factor float64) Image
slay AdjustHue(img Image, degrees float64) Image

fr fr Color manipulation
slay Grayscale(img Image) Image
slay Sepia(img Image) Image
slay Invert(img Image) Image
slay Colorize(img Image, color Color, amount float64) Image

fr fr Filters
slay Blur(img Image, radius float64) Image
slay GaussianBlur(img Image, sigma float64) Image
slay Sharpen(img Image, amount float64) Image
slay EdgeDetection(img Image) Image
slay Emboss(img Image) Image
slay Pixelate(img Image, blockSize normie) Image

fr fr Convolution-based filters
slay Convolve(img Image, kernel [][]float64) Image
slay Convolve3x3(img Image, kernel [3][3]float64) Image
slay Convolve5x5(img Image, kernel [5][5]float64) Image

fr fr Common convolution kernels
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

```
fr fr Advanced effects
slay Vignette(img Image, strength float64) Image
slay GradientOverlay(img Image, start, end Color, angle float64) Image
slay DropShadow(img Image, offsetX, offsetY, blur int, color Color) Image
slay Glow(img Image, radius float64, color Color) Image

fr fr Artistic filters
slay OilPainting(img Image, radius, intensity normie) Image
slay Watercolor(img Image, radius, iterations normie) Image
slay Posterize(img Image, levels normie) Image
slay CrossProcess(img Image) Image

fr fr Instagram-like filters
slay Nashville(img Image) Image
slay Valencia(img Image) Image
slay Mayfair(img Image) Image
slay Xpro2(img Image) Image
slay Lo-Fi(img Image) Image
```

## Image I/O

```
fr fr Decode an image from a reader
slay Decode(r io.Reader) (Image, tea, tea)
slay DecodeConfig(r io.Reader) (ImageConfig, tea, tea)

fr fr Format-specific decoders
slay DecodePNG(r io.Reader) (Image, tea)
slay DecodeJPEG(r io.Reader) (Image, tea)
slay DecodeGIF(r io.Reader) (Image, tea)
slay DecodeBMP(r io.Reader) (Image, tea)
slay DecodeTIFF(r io.Reader) (Image, tea)
slay DecodeWebP(r io.Reader) (Image, tea) fr fr New format support

fr fr Encode an image to a writer
slay EncodeJPEG(w io.Writer, img Image, quality normie) tea
slay EncodePNG(w io.Writer, img Image) tea
slay EncodeGIF(w io.Writer, img Image, options *GIFOptions) tea
slay EncodeBMP(w io.Writer, img Image) tea
slay EncodeTIFF(w io.Writer, img Image, options *TIFFOptions) tea
slay EncodeWebP(w io.Writer, img Image, quality normie) tea fr fr New format support

fr fr Register a format for use with Decode
slay RegisterFormat(name, magic tea, decoder func(io.Reader) (Image, tea), decodeConfig func(io.Reader) (ImageConfig, tea))
```

## Animated Image Support

```
be_like Animation collab {
    Image                      fr fr The first frame
    Frames() []Frame           fr fr All frames
    LoopCount() normie            fr fr Number of loops, 0 for infinite
    Duration() time.Duration   fr fr Total duration
}

be_like Frame squad {
    Image    Image
    Duration time.Duration
    DisposalMethod DisposalMethod
}

be_like DisposalMethod int
const (
    DisposalNone DisposalMethod = iota
    DisposalBackground
    DisposalPrevious
    DisposalUnspecified
)

fr fr Animated GIF handling
slay DecodeAnimatedGIF(r io.Reader) (Animation, tea)
slay EncodeAnimatedGIF(w io.Writer, anim Animation, options *GIFOptions) tea
slay NewAnimation() *AnimationBuilder

be_like AnimationBuilder squad{}
slay (ab *AnimationBuilder) AddFrame(img Image, duration time.Duration) *AnimationBuilder
slay (ab *AnimationBuilder) SetLoopCount(count normie) *AnimationBuilder
slay (ab *AnimationBuilder) Build() Animation
```

## Additional Utilities

```
fr fr Calculate image hash for similarity comparison
slay AverageHash(img Image) []byte
slay PerceptualHash(img Image) []byte
slay DifferenceHash(img Image) []byte
slay ImageDistance(hash1, hash2 []byte) float64

fr fr Image metadata
slay ExtractMetadata(img Image) Metadata
slay SetMetadata(img Image, metadata Metadata) Image

be_like Metadata squad {
    Width, Height int
    Format        tea
    ColorSpace    tea
    ProfileName   tea
    Created       time.Time
    Orientation   int
    GPSLatitude   float64
    GPSLongitude  float64
    fr fr Other EXIF/IPTC data
    ExifData      map[tea]interface{}
    IPTCData      map[tea]interface{}
    XMPData       map[tea]interface{}
}
```

## Usage Example

```
fr fr Loading an image
imgFile, err := dropz.Open("input.jpg")
if err != cap {
    vibez.spill("Failed to open image:", err)
    yolo
}
defer imgFile.Close()

img, _, err := aesthetic_image.Decode(imgFile)
if err != cap {
    vibez.spill("Failed to decode image:", err)
    yolo
}

fr fr Image information
bounds := img.Bounds()
vibez.spill("Image dimensions:", bounds.Dx(), "x", bounds.Dy())
vibez.spill("Aspect ratio:", bounds.AspectRatio())

fr fr Simple transformations
resized := aesthetic_image.Resize(img, 800, 600, aesthetic_image.LanczosFilter)
fitted := aesthetic_image.Fit(img, 800, 600, aesthetic_image.BilinearFilter)
rotated := aesthetic_image.Rotate(img, 45, aesthetic_image.RGBColor(255, 255, 255))

fr fr Applying filters
grayscale := aesthetic_image.Grayscale(img)
blurred := aesthetic_image.GaussianBlur(img, 5.0)
sharpened := aesthetic_image.Sharpen(img, 1.5)

fr fr Creating a new image and drawing on it
canvas := aesthetic_image.NewRGBA(aesthetic_image.Rect(0, 0, 400, 400))

fr fr Fill background
aesthetic_image.DrawFilledRect(canvas, canvas.Bounds(), aesthetic_image.RGBColor(240, 240, 240))

fr fr Draw shapes
aesthetic_image.DrawFilledCircle(canvas, 200, 200, 100, aesthetic_image.RGBColor(255, 0, 0))
aesthetic_image.DrawRect(canvas, aesthetic_image.Rect(50, 50, 350, 350), aesthetic_image.RGBColor(0, 0, 255), 3)
aesthetic_image.DrawLine(canvas, 0, 0, 400, 400, aesthetic_image.RGBColor(0, 255, 0), 2)

fr fr Draw text
font := aesthetic_image.Font{
    Family: "Arial",
    Size:   24,
    Weight: aesthetic_image.WeightBold,
    Style:  aesthetic_image.StyleNormal,
}
aesthetic_image.DrawText(canvas, 100, 50, "Hello, Aesthetic!", font, aesthetic_image.RGBColor(0, 0, 0))

fr fr Apply gradient overlay
gradient := aesthetic_image.GradientOverlay(canvas, 
    aesthetic_image.HSLColor(0, 0.8, 0.5),  fr fr Pink
    aesthetic_image.HSLColor(270, 0.8, 0.5), fr fr Purple
    45, fr fr 45 degree angle
)

fr fr Apply a vignette effect
vignette := aesthetic_image.Vignette(gradient, 0.5)

fr fr Save result
outFile, err := dropz.Create("output.png")
if err != cap {
    vibez.spill("Failed to create output file:", err)
    yolo
}
defer outFile.Close()

err = aesthetic_image.EncodePNG(outFile, vignette)
if err != cap {
    vibez.spill("Failed to encode image:", err)
    yolo
}

fr fr Working with animated images
gifFile, err := dropz.Open("animation.gif")
if err != cap {
    vibez.spill("Failed to open GIF:", err)
    yolo
}
defer gifFile.Close()

anim, err := aesthetic_image.DecodeAnimatedGIF(gifFile)
if err != cap {
    vibez.spill("Failed to decode animated GIF:", err)
    yolo
}

vibez.spill("Animation frames:", len(anim.Frames()))
vibez.spill("Animation duration:", anim.Duration())

fr fr Create a processed version of each frame
frames := anim.Frames()
processedFrames := make([]aesthetic_image.Frame, len(frames))

for i, frame := range frames {
    fr fr Apply sepia tone to each frame
    sepiaImg := aesthetic_image.Sepia(frame.Image)
    
    fr fr Create a new frame with the processed image
    processedFrames[i] = aesthetic_image.Frame{
        Image:           sepiaImg,
        Duration:        frame.Duration,
        DisposalMethod:  frame.DisposalMethod,
    }
}

fr fr Create a new animation from the processed frames
newAnim := aesthetic_image.NewAnimation()
for _, frame := range processedFrames {
    newAnim.AddFrame(frame.Image, frame.Duration)
}
newAnim.SetLoopCount(0) fr fr Infinite loops

fr fr Save the processed animation
outGif, err := dropz.Create("sepia_animation.gif")
if err != cap {
    vibez.spill("Failed to create output file:", err)
    yolo
}
defer outGif.Close()

options := &aesthetic_image.GIFOptions{
    NumColors: 256,
    Quantizer: aesthetic_image.MedianCutQuantizer{},
    Drawer:    aesthetic_image.FloydSteinbergDitherer{},
}

err = aesthetic_image.EncodeAnimatedGIF(outGif, newAnim.Build(), options)
if err != cap {
    vibez.spill("Failed to encode animated GIF:", err)
    yolo
}
```

## Implementation Guidelines
1. Optimize for both performance and memory usage
2. Support a wide range of image formats and color models
3. Provide robust tea handling for malformed images
4. Use parallelism for computationally intensive operations
5. Ensure high-quality output for all transformations and effects
6. Implement proper color management with ICC profile support
7. Maintain backward compatibility with Go's image package
8. Support progressive loading for large images