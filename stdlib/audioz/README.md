# CURSED AudioZ Module - Professional Audio Processing

## Overview

The AudioZ module provides comprehensive audio processing capabilities for CURSED applications, enabling professional-grade audio manipulation, format conversion, effects processing, and synthesis. This module supports multiple audio formats and offers both CPU and GPU-accelerated processing for real-time applications.

## Features

### Supported Audio Formats
- **WAV** - Uncompressed PCM audio with metadata support
- **MP3** - MPEG-1 Audio Layer III with quality control
- **FLAC** - Free Lossless Audio Codec for high-quality archival
- **OGG** - Ogg Vorbis for efficient compression
- **AAC** - Advanced Audio Coding for modern applications
- **M4A** - MPEG-4 Audio with iTunes compatibility

### Core Functionality
- **Loading and Saving** - Read/write audio from files or memory
- **Format Conversion** - Convert between formats with quality control
- **Sample Rate Conversion** - High-quality resampling algorithms
- **Channel Conversion** - Mono/stereo/surround sound conversion
- **Audio Effects** - Reverb, echo, distortion, compression, EQ
- **Audio Synthesis** - Generate waveforms and noise
- **Analysis Tools** - Spectrum analysis, tempo/pitch detection
- **Real-time Processing** - Low-latency audio processing

### Hardware Acceleration
- **GPU Support** - CUDA and OpenCL compute shaders
- **SIMD Optimization** - Vectorized audio processing
- **Multi-threading** - Parallel processing for complex operations

## Quick Start

```cursed
yeet "audioz"

# Load an audio file
sus audio audioz.AudioData = audioz_load_from_file("music.wav")

# Apply effects
sus reverb audioz.AudioEffect
reverb.effect_type = audioz.EFFECT_REVERB
reverb.parameters[0] = 0.3  # Room size
reverb.enabled = true

audio = audioz_apply_effect(audio, reverb)

# Save in different format
audioz_save_to_file(audio, "processed.mp3", 320)
```

## API Reference

### Core Functions

#### Loading and Saving
```cursed
slay audioz_load_from_file(filepath tea) AudioData
slay audioz_load_from_memory(data tea, format tea) AudioData
slay audioz_save_to_file(audio AudioData, filepath tea, quality normie) lit
slay audioz_save_to_memory(audio AudioData, format tea, quality normie) tea
```

#### Format Detection
```cursed
slay audioz_detect_format_from_file(filepath tea) tea
slay audioz_detect_format_from_signature(data tea) tea
```

#### Audio Conversion
```cursed
slay audioz_resample(audio AudioData, new_sample_rate normie) AudioData
slay audioz_convert_bit_depth(audio AudioData, new_bit_depth normie) AudioData
slay audioz_convert_channels(audio AudioData, new_channels normie) AudioData
```

#### Audio Editing
```cursed
slay audioz_trim(audio AudioData, start_time drip, end_time drip) AudioData
slay audioz_concatenate(audio1 AudioData, audio2 AudioData) AudioData
slay audioz_mix(audio1 AudioData, audio2 AudioData, mix_ratio drip) AudioData
```

#### Effects Processing
```cursed
slay audioz_apply_effect(audio AudioData, effect AudioEffect) AudioData
slay audioz_apply_filter(audio AudioData, filter AudioFilter) AudioData
slay audioz_apply_envelope(audio AudioData, envelope AudioEnvelope) AudioData
```

#### Audio Synthesis
```cursed
slay audioz_generate_sine_wave(frequency drip, duration drip, sample_rate normie, amplitude drip) AudioData
slay audioz_generate_square_wave(frequency drip, duration drip, sample_rate normie, amplitude drip) AudioData
slay audioz_generate_sawtooth_wave(frequency drip, duration drip, sample_rate normie, amplitude drip) AudioData
slay audioz_generate_white_noise(duration drip, sample_rate normie, amplitude drip) AudioData
slay audioz_generate_pink_noise(duration drip, sample_rate normie, amplitude drip) AudioData
```

#### Analysis Functions
```cursed
slay audioz_calculate_spectrum(audio AudioData, window_function normie) AudioSpectrum
slay audioz_detect_tempo(audio AudioData) drip
slay audioz_detect_pitch(audio AudioData) drip
slay audioz_calculate_rms(audio AudioData) drip
slay audioz_calculate_peak(audio AudioData) drip
slay audioz_detect_silence(audio AudioData, threshold drip) [drip]
```

#### Hardware Acceleration
```cursed
slay audioz_enable_gpu_acceleration() lit
slay audioz_disable_gpu_acceleration() lit
slay audioz_is_gpu_available() lit
```

### Data Structures

#### AudioData
```cursed
be_like AudioData = struct {
    sample_rate normie,
    bit_depth normie,
    channels normie,
    format tea,
    samples tea,
    duration drip,
    frame_count normie,
    metadata AudioMetadata
}
```

#### AudioEffect
```cursed
be_like AudioEffect = struct {
    effect_type normie,
    parameters [10]drip,
    wet_mix drip,
    dry_mix drip,
    enabled lit
}
```

#### AudioFilter
```cursed
be_like AudioFilter = struct {
    filter_type normie,
    frequency drip,
    q_factor drip,
    gain drip,
    enabled lit
}
```

#### AudioSpectrum
```cursed
be_like AudioSpectrum = struct {
    frequencies [1024]drip,
    magnitudes [1024]drip,
    phases [1024]drip,
    sample_rate normie,
    window_size normie
}
```

### Constants

#### Sample Rates
```cursed
facts SAMPLE_RATE_8KHZ normie = 8000
facts SAMPLE_RATE_16KHZ normie = 16000
facts SAMPLE_RATE_22KHZ normie = 22050
facts SAMPLE_RATE_44KHZ normie = 44100
facts SAMPLE_RATE_48KHZ normie = 48000
facts SAMPLE_RATE_96KHZ normie = 96000
facts SAMPLE_RATE_192KHZ normie = 192000
```

#### Bit Depths
```cursed
facts BIT_DEPTH_8 normie = 8
facts BIT_DEPTH_16 normie = 16
facts BIT_DEPTH_24 normie = 24
facts BIT_DEPTH_32 normie = 32
```

#### Effect Types
```cursed
facts EFFECT_REVERB normie = 1
facts EFFECT_ECHO normie = 2
facts EFFECT_CHORUS normie = 3
facts EFFECT_DISTORTION normie = 6
facts EFFECT_COMPRESSOR normie = 7
facts EFFECT_EQ normie = 9
facts EFFECT_PITCH_SHIFT normie = 11
facts EFFECT_NORMALIZE normie = 13
```

## Usage Examples

### Basic Audio Processing
```cursed
yeet "audioz"

# Load audio file
sus audio audioz.AudioData = audioz_load_from_file("input.wav")

# Convert to high-quality format
audio = audioz_resample(audio, audioz.SAMPLE_RATE_48KHZ)
audio = audioz_convert_bit_depth(audio, audioz.BIT_DEPTH_24)

# Apply compression
sus compressor audioz.AudioEffect
compressor.effect_type = audioz.EFFECT_COMPRESSOR
compressor.parameters[0] = 4.0     # Ratio
compressor.parameters[1] = -18.0   # Threshold
compressor.parameters[2] = 5.0     # Attack (ms)
compressor.parameters[3] = 50.0    # Release (ms)
compressor.enabled = true

audio = audioz_apply_effect(audio, compressor)

# Save as FLAC
audioz_save_to_file(audio, "output.flac", 100)
```

### Audio Synthesis and Mixing
```cursed
# Generate musical notes
sus note_c audioz.AudioData = audioz_generate_sine_wave(261.63, 2.0, audioz.SAMPLE_RATE_44KHZ, 0.3)
sus note_e audioz.AudioData = audioz_generate_sine_wave(329.63, 2.0, audioz.SAMPLE_RATE_44KHZ, 0.3)
sus note_g audioz.AudioData = audioz_generate_sine_wave(392.00, 2.0, audioz.SAMPLE_RATE_44KHZ, 0.3)

# Mix notes to create chord
sus chord audioz.AudioData = audioz_mix(note_c, note_e, 0.5)
chord = audioz_mix(chord, note_g, 0.5)

# Add reverb to chord
sus reverb audioz.AudioEffect
reverb.effect_type = audioz.EFFECT_REVERB
reverb.parameters[0] = 0.4    # Room size
reverb.parameters[1] = 0.6    # Damping
reverb.wet_mix = 0.3
reverb.enabled = true

chord = audioz_apply_effect(chord, reverb)

# Save result
audioz_save_to_file(chord, "chord.wav", 100)
```

### Real-time Effects Chain
```cursed
# Create effects chain for live processing
sus noise_gate audioz.AudioEffect
noise_gate.effect_type = audioz.EFFECT_NOISE_GATE
noise_gate.parameters[0] = -40.0   # Threshold (dB)
noise_gate.parameters[1] = 10.0    # Attack (ms)
noise_gate.parameters[2] = 100.0   # Release (ms)
noise_gate.enabled = true

sus eq audioz.AudioEffect
eq.effect_type = audioz.EFFECT_EQ
eq.parameters[0] = 100.0    # Low freq
eq.parameters[1] = 1.2      # Low gain
eq.parameters[2] = 1000.0   # Mid freq
eq.parameters[3] = 0.8      # Mid gain
eq.parameters[4] = 8000.0   # High freq
eq.parameters[5] = 1.5      # High gain
eq.enabled = true

sus limiter audioz.AudioEffect
limiter.effect_type = audioz.EFFECT_LIMITER
limiter.parameters[0] = -1.0    # Ceiling (dB)
limiter.parameters[1] = 5.0     # Release (ms)
limiter.enabled = true

# Process audio through chain
sus input audioz.AudioData = audioz_load_from_file("vocal.wav")
input = audioz_apply_effect(input, noise_gate)
input = audioz_apply_effect(input, eq)
input = audioz_apply_effect(input, limiter)

audioz_save_to_file(input, "processed_vocal.wav", 100)
```

### Audio Analysis
```cursed
sus audio audioz.AudioData = audioz_load_from_file("music.wav")

# Perform spectrum analysis
sus spectrum audioz.AudioSpectrum = audioz_calculate_spectrum(audio, audioz.WINDOW_HANN)

# Detect musical properties
sus tempo drip = audioz_detect_tempo(audio)
sus pitch drip = audioz_detect_pitch(audio)

# Calculate audio levels
sus rms_level drip = audioz_calculate_rms(audio)
sus peak_level drip = audioz_calculate_peak(audio)

# Find silence regions
sus silence_regions [100]drip = audioz_detect_silence(audio, 0.01)

vibez.spill("Tempo:", tempo, "BPM")
vibez.spill("Dominant pitch:", pitch, "Hz")
vibez.spill("RMS level:", rms_level)
vibez.spill("Peak level:", peak_level)
```

### Batch Audio Conversion
```cursed
# Convert multiple files to different formats
sus input_files [5]tea = ["song1.wav", "song2.wav", "song3.wav", "song4.wav", "song5.wav"]

sus i normie = 0
bestie (i < 5) {
    sus audio audioz.AudioData = audioz_load_from_file(input_files[i])
    
    # Normalize audio levels
    audio = audioz_apply_effect(audio, normalize_effect)
    
    # Convert to streaming quality
    audio = audioz_resample(audio, audioz.SAMPLE_RATE_44KHZ)
    audio = audioz_convert_bit_depth(audio, audioz.BIT_DEPTH_16)
    
    # Save as MP3
    sus output_name tea = stringz_replace(input_files[i], ".wav", ".mp3")
    audioz_save_to_file(audio, output_name, 192)
    
    i = i + 1
}
```

### Advanced Signal Processing
```cursed
# Time-domain effects
sus audio audioz.AudioData = audioz_load_from_file("drums.wav")

# Pitch shifting without changing duration
sus pitch_shift audioz.AudioEffect
pitch_shift.effect_type = audioz.EFFECT_PITCH_SHIFT
pitch_shift.parameters[0] = 2.0    # Semitones
pitch_shift.enabled = true

sus pitched audioz.AudioData = audioz_apply_effect(audio, pitch_shift)

# Time stretching without changing pitch
sus time_stretch audioz.AudioEffect
time_stretch.effect_type = audioz.EFFECT_TIME_STRETCH
time_stretch.parameters[0] = 1.5   # Stretch factor
time_stretch.enabled = true

sus stretched audioz.AudioData = audioz_apply_effect(audio, time_stretch)

# Save results
audioz_save_to_file(pitched, "drums_pitched.wav", 100)
audioz_save_to_file(stretched, "drums_stretched.wav", 100)
```

## Performance Considerations

### Memory Usage
- Audio data can be very large (sample_rate × duration × channels × bytes_per_sample)
- Use streaming processing for long audio files
- Consider resampling to lower rates for preview operations

### Real-time Processing
- Use smaller buffer sizes for lower latency
- GPU acceleration provides significant speedup for complex effects
- Optimize effect chains to minimize computational overhead

### Quality vs Performance
- Higher sample rates and bit depths improve quality but increase processing time
- Use appropriate window functions for spectrum analysis
- Balance quality settings with real-time requirements

## Dependencies

The AudioZ module depends on:
- `vibez` - For output and logging
- `mathz` - For mathematical operations and FFT
- `stringz` - For string manipulation
- `memoryz` - For memory management
- `filez` - For file I/O operations

## Error Handling

The module follows CURSED error handling conventions:
- Functions return empty/invalid data structures on error
- Use validation functions to check audio properties
- Error messages are logged through the vibez module

## Platform Support

AudioZ is designed to work across all CURSED-supported platforms:
- **Linux** - Full feature support with ALSA/PulseAudio integration
- **macOS** - Full feature support with Core Audio backend
- **Windows** - Full feature support with WASAPI/DirectSound
- **WebAssembly** - Core features with Web Audio API

## Thread Safety

AudioZ provides thread-safe operations for:
- Concurrent audio processing
- Real-time audio callbacks
- Multi-threaded batch processing

Use appropriate synchronization when sharing audio data between threads.

## License

This module is part of the CURSED standard library and follows the same licensing terms as the core language.
