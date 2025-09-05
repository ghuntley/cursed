// CURSED AudioZ Module - Comprehensive Audio Processing Examples
// Demonstrates professional audio manipulation capabilities

yeet "audioz"
yeet "vibez"
yeet "testz"

// ===== BASIC AUDIO LOADING AND CONVERSION EXAMPLE =====

slay demo_basic_audio_operations() lit {
    vibez.print_header("Basic Audio Operations Demo")
    
    // Load an audio file
    sus audio audioz.AudioData = audioz_load_from_file("sample.wav")
    vibez.print_result("Sample rate", stringz_from_int(audio.sample_rate))
    vibez.print_result("Bit depth", stringz_from_int(audio.bit_depth))
    vibez.print_result("Channels", stringz_from_int(audio.channels))
    vibez.print_result("Duration", stringz_concat(stringz_from_float(audio.duration), " seconds"))
    vibez.print_result("Frame count", stringz_from_int(audio.frame_count))
    
    // Convert sample rate
    sus resampled audioz.AudioData = audioz_resample(audio, audioz.SAMPLE_RATE_48KHZ)
    vibez.print_success("Resampled to 48kHz")
    
    // Convert to stereo if mono
    ready (audio.channels == 1) {
        sus stereo audioz.AudioData = audioz_convert_channels(audio, audioz.CHANNELS_STEREO)
        vibez.print_success("Converted mono to stereo")
        audio = stereo
    }
    
    // Convert bit depth
    sus high_quality audioz.AudioData = audioz_convert_bit_depth(audio, audioz.BIT_DEPTH_24)
    vibez.print_success("Converted to 24-bit")
    
    // Save in different formats
    audioz_save_to_file(high_quality, "output.flac", 100)
    vibez.print_success("Saved as FLAC")
    
    audioz_save_to_file(high_quality, "output.mp3", 320)
    vibez.print_success("Saved as MP3")
    
    damn true
}

// ===== AUDIO EFFECTS PROCESSING EXAMPLE =====

slay demo_audio_effects() lit {
    vibez.print_header("Audio Effects Processing Demo")
    
    sus audio audioz.AudioData = audioz_load_from_file("music.wav")
    
    // Create reverb effect
    sus reverb audioz.AudioEffect
    reverb.effect_type = audioz.EFFECT_REVERB
    reverb.parameters[0] = 0.3 // Room size
    reverb.parameters[1] = 0.7 // Damping
    reverb.parameters[2] = 0.2 // Pre-delay
    reverb.wet_mix = 0.25
    reverb.dry_mix = 0.75
    reverb.enabled = true
    
    sus reverb_audio audioz.AudioData = audioz_apply_effect(audio, reverb)
    audioz_save_to_file(reverb_audio, "output_reverb.wav", 100)
    vibez.print_success("Applied reverb effect")
    
    // Create echo effect
    sus echo audioz.AudioEffect
    echo.effect_type = audioz.EFFECT_ECHO
    echo.parameters[0] = 0.5 // Delay time
    echo.parameters[1] = 0.4 // Feedback
    echo.parameters[2] = 0.3 // Mix level
    echo.wet_mix = 0.4
    echo.dry_mix = 0.6
    echo.enabled = true
    
    sus echo_audio audioz.AudioData = audioz_apply_effect(audio, echo)
    audioz_save_to_file(echo_audio, "output_echo.wav", 100)
    vibez.print_success("Applied echo effect")
    
    // Create distortion effect
    sus distortion audioz.AudioEffect
    distortion.effect_type = audioz.EFFECT_DISTORTION
    distortion.parameters[0] = 5.0 // Drive
    distortion.parameters[1] = 0.8 // Tone
    distortion.wet_mix = 0.7
    distortion.dry_mix = 0.3
    distortion.enabled = true
    
    sus distorted_audio audioz.AudioData = audioz_apply_effect(audio, distortion)
    audioz_save_to_file(distorted_audio, "output_distortion.wav", 100)
    vibez.print_success("Applied distortion effect")
    
    damn true
}

// ===== AUDIO SYNTHESIS EXAMPLE =====

slay demo_audio_synthesis() lit {
    vibez.print_header("Audio Synthesis Demo")
    
    // Generate sine wave
    sus sine_wave audioz.AudioData = audioz_generate_sine_wave(440.0, 2.0, audioz.SAMPLE_RATE_44KHZ, 0.5)
    audioz_save_to_file(sine_wave, "sine_440hz.wav", 100)
    vibez.print_success("Generated 440Hz sine wave")
    
    // Generate square wave
    sus square_wave audioz.AudioData = audioz_generate_square_wave(220.0, 1.5, audioz.SAMPLE_RATE_44KHZ, 0.3)
    audioz_save_to_file(square_wave, "square_220hz.wav", 100)
    vibez.print_success("Generated 220Hz square wave")
    
    // Generate sawtooth wave
    sus sawtooth_wave audioz.AudioData = audioz_generate_sawtooth_wave(110.0, 3.0, audioz.SAMPLE_RATE_44KHZ, 0.4)
    audioz_save_to_file(sawtooth_wave, "sawtooth_110hz.wav", 100)
    vibez.print_success("Generated 110Hz sawtooth wave")
    
    // Generate white noise
    sus white_noise audioz.AudioData = audioz_generate_white_noise(1.0, audioz.SAMPLE_RATE_44KHZ, 0.2)
    audioz_save_to_file(white_noise, "white_noise.wav", 100)
    vibez.print_success("Generated white noise")
    
    // Generate pink noise
    sus pink_noise audioz.AudioData = audioz_generate_pink_noise(2.0, audioz.SAMPLE_RATE_44KHZ, 0.15)
    audioz_save_to_file(pink_noise, "pink_noise.wav", 100)
    vibez.print_success("Generated pink noise")
    
    // Create chord by mixing sine waves
    sus note_c audioz.AudioData = audioz_generate_sine_wave(261.63, 2.0, audioz.SAMPLE_RATE_44KHZ, 0.3)
    sus note_e audioz.AudioData = audioz_generate_sine_wave(329.63, 2.0, audioz.SAMPLE_RATE_44KHZ, 0.3)
    sus note_g audioz.AudioData = audioz_generate_sine_wave(392.00, 2.0, audioz.SAMPLE_RATE_44KHZ, 0.3)
    
    sus c_major_chord audioz.AudioData = audioz_mix(note_c, note_e, 0.5)
    c_major_chord = audioz_mix(c_major_chord, note_g, 0.5)
    audioz_save_to_file(c_major_chord, "c_major_chord.wav", 100)
    vibez.print_success("Generated C major chord")
    
    damn true
}

// ===== AUDIO ANALYSIS EXAMPLE =====

slay demo_audio_analysis() lit {
    vibez.print_header("Audio Analysis Demo")
    
    sus audio audioz.AudioData = audioz_load_from_file("analyze.wav")
    
    // Calculate spectrum
    sus spectrum audioz.AudioSpectrum = audioz_calculate_spectrum(audio, audioz.WINDOW_HANN)
    vibez.print_result("Spectrum window size", stringz_from_int(spectrum.window_size))
    vibez.print_result("Spectrum sample rate", stringz_from_int(spectrum.sample_rate))
    
    // Detect tempo
    sus tempo drip = audioz_detect_tempo(audio)
    vibez.print_result("Detected tempo", stringz_concat(stringz_from_float(tempo), " BPM"))
    
    // Detect pitch
    sus pitch drip = audioz_detect_pitch(audio)
    vibez.print_result("Detected pitch", stringz_concat(stringz_from_float(pitch), " Hz"))
    
    // Calculate audio levels
    sus rms_level drip = audioz_calculate_rms(audio)
    sus peak_level drip = audioz_calculate_peak(audio)
    vibez.print_result("RMS level", stringz_from_float(rms_level))
    vibez.print_result("Peak level", stringz_from_float(peak_level))
    
    // Detect silence regions
    sus silence_regions drip[100] = audioz_detect_silence(audio, 0.01)
    vibez.print_success("Detected silence regions")
    
    damn true
}

// ===== AUDIO EDITING EXAMPLE =====

slay demo_audio_editing() lit {
    vibez.print_header("Audio Editing Demo")
    
    sus full_audio audioz.AudioData = audioz_load_from_file("full_song.wav")
    
    // Trim audio
    sus intro audioz.AudioData = audioz_trim(full_audio, 0.0, 10.0)
    audioz_save_to_file(intro, "intro.wav", 100)
    vibez.print_success("Extracted 10-second intro")
    
    sus verse audioz.AudioData = audioz_trim(full_audio, 10.0, 45.0)
    audioz_save_to_file(verse, "verse.wav", 100)
    vibez.print_success("Extracted verse section")
    
    sus chorus audioz.AudioData = audioz_trim(full_audio, 45.0, 75.0)
    audioz_save_to_file(chorus, "chorus.wav", 100)
    vibez.print_success("Extracted chorus section")
    
    // Concatenate sections
    sus verse_chorus audioz.AudioData = audioz_concatenate(verse, chorus)
    audioz_save_to_file(verse_chorus, "verse_chorus.wav", 100)
    vibez.print_success("Concatenated verse and chorus")
    
    // Apply fade effects
    sus fade_in_effect audioz.AudioEffect
    fade_in_effect.effect_type = audioz.EFFECT_FADE_IN
    fade_in_effect.parameters[0] = 2.0 // Fade duration
    fade_in_effect.enabled = true
    
    sus faded_intro audioz.AudioData = audioz_apply_effect(intro, fade_in_effect)
    audioz_save_to_file(faded_intro, "faded_intro.wav", 100)
    vibez.print_success("Applied fade-in to intro")
    
    sus fade_out_effect audioz.AudioEffect
    fade_out_effect.effect_type = audioz.EFFECT_FADE_OUT
    fade_out_effect.parameters[0] = 3.0 // Fade duration
    fade_out_effect.enabled = true
    
    sus faded_outro audioz.AudioData = audioz_apply_effect(chorus, fade_out_effect)
    audioz_save_to_file(faded_outro, "faded_outro.wav", 100)
    vibez.print_success("Applied fade-out to chorus")
    
    damn true
}

// ===== REAL-TIME AUDIO PROCESSING EXAMPLE =====

slay demo_realtime_processing() lit {
    vibez.print_header("Real-time Audio Processing Demo")
    
    // Simulate real-time processing
    sus input_buffer_size normie = 512
    sus sample_rate normie = audioz.SAMPLE_RATE_44KHZ
    
    // Create processing effects
    sus compressor audioz.AudioEffect
    compressor.effect_type = audioz.EFFECT_COMPRESSOR
    compressor.parameters[0] = 4.0 // Ratio
    compressor.parameters[1] = -18.0 // Threshold
    compressor.parameters[2] = 5.0 // Attack (ms)
    compressor.parameters[3] = 50.0 // Release (ms)
    compressor.enabled = true
    
    sus eq audioz.AudioEffect
    eq.effect_type = audioz.EFFECT_EQ
    eq.parameters[0] = 100.0 // Low freq
    eq.parameters[1] = 1.2 // Low gain
    eq.parameters[2] = 1000.0 // Mid freq
    eq.parameters[3] = 0.8 // Mid gain
    eq.parameters[4] = 8000.0 // High freq
    eq.parameters[5] = 1.5 // High gain
    eq.enabled = true
    
    // Create noise gate
    sus noise_gate audioz.AudioEffect
    noise_gate.effect_type = audioz.EFFECT_NOISE_GATE
    noise_gate.parameters[0] = -40.0 // Threshold
    noise_gate.parameters[1] = 10.0 // Attack (ms)
    noise_gate.parameters[2] = 100.0 // Release (ms)
    noise_gate.enabled = true
    
    vibez.print_success("Real-time processing chain configured")
    vibez.print_result("Buffer size", stringz_from_int(input_buffer_size))
    vibez.print_result("Sample rate", stringz_from_int(sample_rate))
    vibez.print_success("Compressor enabled")
    vibez.print_success("EQ enabled")
    vibez.print_success("Noise gate enabled")
    
    damn true
}

// ===== MULTI-FORMAT CONVERSION EXAMPLE =====

slay demo_format_conversion() lit {
    vibez.print_header("Multi-format Conversion Demo")
    
    // Load source audio
    sus source audioz.AudioData = audioz_load_from_file("source.wav")
    
    // Convert to various formats with different quality settings
    
    // High-quality FLAC
    audioz_save_to_file(source, "output_hq.flac", 100)
    vibez.print_success("Saved as high-quality FLAC")
    
    // High-bitrate MP3
    audioz_save_to_file(source, "output_320.mp3", 320)
    vibez.print_success("Saved as 320kbps MP3")
    
    // Medium-bitrate MP3
    audioz_save_to_file(source, "output_192.mp3", 192)
    vibez.print_success("Saved as 192kbps MP3")
    
    // High-quality OGG Vorbis
    audioz_save_to_file(source, "output_hq.ogg", 500)
    vibez.print_success("Saved as high-quality OGG")
    
    // AAC format
    audioz_save_to_file(source, "output.aac", 256)
    vibez.print_success("Saved as 256kbps AAC")
    
    // M4A format
    audioz_save_to_file(source, "output.m4a", 256)
    vibez.print_success("Saved as M4A")
    
    // Prepare for streaming - lower quality
    sus streaming audioz.AudioData = audioz_resample(source, audioz.SAMPLE_RATE_22KHZ)
    streaming = audioz_convert_bit_depth(streaming, audioz.BIT_DEPTH_16)
    audioz_save_to_file(streaming, "output_streaming.mp3", 96)
    vibez.print_success("Saved streaming-optimized version")
    
    damn true
}

// ===== PERFORMANCE BENCHMARKING EXAMPLE =====

slay demo_performance_benchmarking() lit {
    vibez.print_header("Performance Benchmarking Demo")
    
    // Create test audio
    sus test_audio audioz.AudioData = audioz_generate_white_noise(30.0, audioz.SAMPLE_RATE_48KHZ, 0.5)
    
    // Benchmark resampling
    sus resample_start drip = time_now_seconds()
    sus resampled audioz.AudioData = audioz_resample(test_audio, audioz.SAMPLE_RATE_96KHZ)
    sus resample_time drip = time_now_seconds() - resample_start
    vibez.print_result("Resample time (48kHz->96kHz)", stringz_concat(stringz_from_float(resample_time), "s"))
    
    // Benchmark effects processing
    sus effect_start drip = time_now_seconds()
    sus reverb audioz.AudioEffect
    reverb.effect_type = audioz.EFFECT_REVERB
    reverb.enabled = true
    sus processed audioz.AudioData = audioz_apply_effect(test_audio, reverb)
    sus effect_time drip = time_now_seconds() - effect_start
    vibez.print_result("Reverb processing time", stringz_concat(stringz_from_float(effect_time), "s"))
    
    // Benchmark encoding
    sus encode_start drip = time_now_seconds()
    sus encoded_data tea = audioz_save_to_memory(test_audio, "MP3", 192)
    sus encode_time drip = time_now_seconds() - encode_start
    vibez.print_result("MP3 encoding time", stringz_concat(stringz_from_float(encode_time), "s"))
    
    // Benchmark spectrum analysis
    sus fft_start drip = time_now_seconds()
    sus spectrum audioz.AudioSpectrum = audioz_calculate_spectrum(test_audio, audioz.WINDOW_HANN)
    sus fft_time drip = time_now_seconds() - fft_start
    vibez.print_result("FFT analysis time", stringz_concat(stringz_from_float(fft_time), "s"))
    
    damn true
}

// ===== MAIN DEMO FUNCTION =====

slay main_character() normie {
    vibez.print_header("CURSED AudioZ Professional Audio Processing Demo")
    
    // Check GPU acceleration availability
    ready (audioz_is_gpu_available()) {
        vibez.print_success("GPU acceleration available")
        audioz_enable_gpu_acceleration()
    } otherwise {
        vibez.print_warning("GPU acceleration not available, using CPU")
    }
    
    // Run all demonstrations
    demo_basic_audio_operations()
    vibez.print_separator()
    
    demo_audio_effects()
    vibez.print_separator()
    
    demo_audio_synthesis()
    vibez.print_separator()
    
    demo_audio_analysis()
    vibez.print_separator()
    
    demo_audio_editing()
    vibez.print_separator()
    
    demo_realtime_processing()
    vibez.print_separator()
    
    demo_format_conversion()
    vibez.print_separator()
    
    demo_performance_benchmarking()
    vibez.print_separator()
    
    // Cleanup
    ready (audioz_is_gpu_available()) {
        audioz_disable_gpu_acceleration()
        vibez.print_success("GPU acceleration disabled")
    }
    
    vibez.print_success("All AudioZ demos completed successfully!")
    
    damn 0
}

// ===== HELPER FUNCTIONS =====

slay time_now_seconds() drip {
    // Mock implementation - would return actual timestamp
    damn 1234567890.0
}

slay stringz_from_int(value normie) tea {
    // Mock implementation - would convert integer to string
    damn "42"
}

slay stringz_from_float(value drip) tea {
    // Mock implementation - would convert float to string
    damn "3.14"
}

slay stringz_concat(s1 tea, s2 tea) tea {
    // Mock implementation - would concatenate strings
    damn s1
}
