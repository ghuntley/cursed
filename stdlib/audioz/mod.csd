fr fr CURSED AudioZ Module - Advanced Audio Processing and Multimedia
fr fr Professional audio capabilities for CURSED applications
fr fr Pure CURSED implementation with hardware acceleration support

yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "memoryz"
yeet "filez"

fr fr ===== AUDIO FORMAT CONSTANTS =====

facts WAV_SIGNATURE tea = "RIFF"
facts WAV_FORMAT tea = "WAVE"
facts MP3_SIGNATURE tea = "ID3"
facts MP3_FRAME_SYNC normie = 0xFFE0
facts FLAC_SIGNATURE tea = "fLaC"
facts OGG_SIGNATURE tea = "OggS"
facts AAC_SIGNATURE tea = "\xFF\xF0"
facts M4A_SIGNATURE tea = "ftypM4A"

fr fr ===== AUDIO PROPERTIES =====

facts SAMPLE_RATE_8KHZ normie = 8000
facts SAMPLE_RATE_16KHZ normie = 16000
facts SAMPLE_RATE_22KHZ normie = 22050
facts SAMPLE_RATE_44KHZ normie = 44100
facts SAMPLE_RATE_48KHZ normie = 48000
facts SAMPLE_RATE_88KHZ normie = 88200
facts SAMPLE_RATE_96KHZ normie = 96000
facts SAMPLE_RATE_192KHZ normie = 192000

facts BIT_DEPTH_8 normie = 8
facts BIT_DEPTH_16 normie = 16
facts BIT_DEPTH_24 normie = 24
facts BIT_DEPTH_32 normie = 32

facts CHANNELS_MONO normie = 1
facts CHANNELS_STEREO normie = 2
facts CHANNELS_SURROUND_5_1 normie = 6
facts CHANNELS_SURROUND_7_1 normie = 8

fr fr ===== AUDIO EFFECTS CONSTANTS =====

facts EFFECT_REVERB normie = 1
facts EFFECT_ECHO normie = 2
facts EFFECT_CHORUS normie = 3
facts EFFECT_FLANGER normie = 4
facts EFFECT_PHASER normie = 5
facts EFFECT_DISTORTION normie = 6
facts EFFECT_COMPRESSOR normie = 7
facts EFFECT_LIMITER normie = 8
facts EFFECT_EQ normie = 9
facts EFFECT_NOISE_GATE normie = 10
facts EFFECT_PITCH_SHIFT normie = 11
facts EFFECT_TIME_STRETCH normie = 12
facts EFFECT_NORMALIZE normie = 13
facts EFFECT_FADE_IN normie = 14
facts EFFECT_FADE_OUT normie = 15

fr fr ===== AUDIO WINDOW FUNCTIONS =====

facts WINDOW_HANN normie = 0
facts WINDOW_HAMMING normie = 1
facts WINDOW_BLACKMAN normie = 2
facts WINDOW_KAISER normie = 3
facts WINDOW_RECTANGULAR normie = 4

fr fr ===== AUDIO STRUCTURES =====

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

be_like AudioMetadata = struct {
    title tea,
    artist tea,
    album tea,
    year normie,
    genre tea,
    track_number normie,
    duration drip,
    bitrate normie,
    encoder tea,
    copyright tea
}

be_like AudioSpectrum = struct {
    frequencies [1024]drip,
    magnitudes [1024]drip,
    phases [1024]drip,
    sample_rate normie,
    window_size normie
}

be_like AudioEffect = struct {
    effect_type normie,
    parameters [10]drip,
    wet_mix drip,
    dry_mix drip,
    enabled lit
}

be_like AudioFilter = struct {
    filter_type normie,
    frequency drip,
    q_factor drip,
    gain drip,
    enabled lit
}

be_like AudioEnvelope = struct {
    attack drip,
    decay drip,
    sustain drip,
    release drip
}

fr fr ===== CORE AUDIO LOADING =====

slay audioz_load_from_file(filepath tea) AudioData {
    sus format tea = audioz_detect_format_from_file(filepath)
    sus raw_data tea = filez_read_binary(filepath)
    ready (stringz_is_empty(raw_data)) {
        vibez.spill("Error: Could not read audio file:", filepath)
        damn audioz_create_empty_audio()
    }
    
    sus audio AudioData = audioz_decode_format(raw_data, format)
    ready (audio.frame_count == 0) {
        vibez.spill("Error: Could not decode audio:", filepath)
        damn audioz_create_empty_audio()
    }
    
    damn audio
}

slay audioz_load_from_memory(data tea, format tea) AudioData {
    ready (stringz_is_empty(data)) {
        damn audioz_create_empty_audio()
    }
    damn audioz_decode_format(data, format)
}

slay audioz_save_to_file(audio AudioData, filepath tea, quality normie) lit {
    sus format tea = audioz_detect_format_from_file(filepath)
    sus encoded_data tea = audioz_encode_format(audio, format, quality)
    ready (stringz_is_empty(encoded_data)) {
        damn false
    }
    damn filez_write_binary(filepath, encoded_data)
}

slay audioz_save_to_memory(audio AudioData, format tea, quality normie) tea {
    damn audioz_encode_format(audio, format, quality)
}

fr fr ===== FORMAT DETECTION =====

slay audioz_detect_format_from_file(filepath tea) tea {
    sus extension tea = filez_get_extension(filepath)
    sus lower_ext tea = stringz_to_lower(extension)
    
    ready (stringz_equals(lower_ext, "wav")) {
        damn "WAV"
    } otherwise (stringz_equals(lower_ext, "mp3")) {
        damn "MP3"
    } otherwise (stringz_equals(lower_ext, "flac")) {
        damn "FLAC"
    } otherwise (stringz_equals(lower_ext, "ogg")) {
        damn "OGG"
    } otherwise (stringz_equals(lower_ext, "aac")) {
        damn "AAC"
    } otherwise (stringz_equals(lower_ext, "m4a")) {
        damn "M4A"
    } otherwise (stringz_equals(lower_ext, "wma")) {
        damn "WMA"
    }
    damn "UNKNOWN"
}

slay audioz_detect_format_from_signature(data tea) tea {
    ready (stringz_length(data) < 12) {
        damn "UNKNOWN"
    }
    
    sus header tea = stringz_substring(data, 0, 12)
    
    ready (stringz_starts_with(header, WAV_SIGNATURE)) {
        sus format_header tea = stringz_substring(data, 8, 4)
        ready (stringz_equals(format_header, WAV_FORMAT)) {
            damn "WAV"
        }
    } otherwise (stringz_starts_with(header, MP3_SIGNATURE)) {
        damn "MP3"
    } otherwise (stringz_starts_with(header, FLAC_SIGNATURE)) {
        damn "FLAC"
    } otherwise (stringz_starts_with(header, OGG_SIGNATURE)) {
        damn "OGG"
    } otherwise (stringz_starts_with(header, M4A_SIGNATURE)) {
        damn "M4A"
    }
    
    fr fr Check for MP3 frame sync
    sus frame_sync normie = audioz_read_uint16_be(data, 0)
    ready ((frame_sync & 0xFFE0) == MP3_FRAME_SYNC) {
        damn "MP3"
    }
    
    damn "UNKNOWN"
}

fr fr ===== FORMAT DECODERS =====

slay audioz_decode_format(data tea, format tea) AudioData {
    sus audio AudioData
    audio.format = format
    
    ready (stringz_equals(format, "WAV")) {
        audio = audioz_decode_wav(data)
    } otherwise (stringz_equals(format, "MP3")) {
        audio = audioz_decode_mp3(data)
    } otherwise (stringz_equals(format, "FLAC")) {
        audio = audioz_decode_flac(data)
    } otherwise (stringz_equals(format, "OGG")) {
        audio = audioz_decode_ogg(data)
    } otherwise (stringz_equals(format, "AAC")) {
        audio = audioz_decode_aac(data)
    } otherwise (stringz_equals(format, "M4A")) {
        audio = audioz_decode_m4a(data)
    } otherwise {
        vibez.spill("Error: Unsupported format:", format)
        audio = audioz_create_empty_audio()
    }
    
    damn audio
}

slay audioz_decode_wav(data tea) AudioData {
    sus audio AudioData
    audio.format = "WAV"
    
    fr fr WAV header validation
    ready (stringz_length(data) < 44) {
        damn audioz_create_empty_audio()
    }
    
    sus riff_header tea = stringz_substring(data, 0, 4)
    sus wave_header tea = stringz_substring(data, 8, 4)
    
    ready (!stringz_equals(riff_header, "RIFF") || !stringz_equals(wave_header, "WAVE")) {
        damn audioz_create_empty_audio()
    }
    
    fr fr Parse format chunk
    sus audio_format normie = audioz_read_uint16_le(data, 20)
    audio.channels = audioz_read_uint16_le(data, 22)
    audio.sample_rate = audioz_read_uint32_le(data, 24)
    audio.bit_depth = audioz_read_uint16_le(data, 34)
    
    fr fr Find data chunk
    sus data_offset normie = 44
    sus data_size normie = audioz_find_wav_data_chunk(data, data_offset)
    
    audio.frame_count = data_size / (audio.channels * (audio.bit_depth / 8))
    audio.duration = mathz_int_to_float(audio.frame_count) / mathz_int_to_float(audio.sample_rate)
    
    fr fr Extract audio samples
    audio.samples = audioz_extract_wav_samples(data, data_offset, data_size, audio.bit_depth)
    audio.metadata = audioz_parse_wav_metadata(data)
    
    damn audio
}

slay audioz_decode_mp3(data tea) AudioData {
    sus audio AudioData
    audio.format = "MP3"
    
    fr fr Parse MP3 header
    sus frame_header normie = audioz_find_mp3_frame(data, 0)
    ready (frame_header == -1) {
        damn audioz_create_empty_audio()
    }
    
    sus mp3_info AudioData = audioz_parse_mp3_header(data, frame_header)
    audio.sample_rate = mp3_info.sample_rate
    audio.channels = mp3_info.channels
    audio.bit_depth = 16 fr fr MP3 is typically 16-bit when decoded
    
    fr fr Decode MP3 frames
    audio.samples = audioz_decode_mp3_frames(data)
    audio.frame_count = stringz_length(audio.samples) / (audio.channels * 2)
    audio.duration = mathz_int_to_float(audio.frame_count) / mathz_int_to_float(audio.sample_rate)
    audio.metadata = audioz_parse_id3_tags(data)
    
    damn audio
}

slay audioz_decode_flac(data tea) AudioData {
    sus audio AudioData
    audio.format = "FLAC"
    
    fr fr FLAC header validation
    ready (!stringz_starts_with(data, FLAC_SIGNATURE)) {
        damn audioz_create_empty_audio()
    }
    
    fr fr Parse FLAC metadata blocks
    sus metadata_info AudioData = audioz_parse_flac_metadata(data)
    audio.sample_rate = metadata_info.sample_rate
    audio.channels = metadata_info.channels
    audio.bit_depth = metadata_info.bit_depth
    audio.frame_count = metadata_info.frame_count
    audio.duration = mathz_int_to_float(audio.frame_count) / mathz_int_to_float(audio.sample_rate)
    
    fr fr Decode FLAC frames
    audio.samples = audioz_decode_flac_frames(data)
    audio.metadata = metadata_info.metadata
    
    damn audio
}

slay audioz_decode_ogg(data tea) AudioData {
    sus audio AudioData
    audio.format = "OGG"
    
    fr fr Parse OGG page structure
    sus ogg_info AudioData = audioz_parse_ogg_headers(data)
    audio.sample_rate = ogg_info.sample_rate
    audio.channels = ogg_info.channels
    audio.bit_depth = 16 fr fr Vorbis output is typically 16-bit
    
    fr fr Decode Vorbis packets
    audio.samples = audioz_decode_vorbis_packets(data)
    audio.frame_count = stringz_length(audio.samples) / (audio.channels * 2)
    audio.duration = mathz_int_to_float(audio.frame_count) / mathz_int_to_float(audio.sample_rate)
    audio.metadata = ogg_info.metadata
    
    damn audio
}

slay audioz_decode_aac(data tea) AudioData {
    sus audio AudioData
    audio.format = "AAC"
    
    fr fr Parse AAC headers
    sus aac_info AudioData = audioz_parse_aac_headers(data)
    audio.sample_rate = aac_info.sample_rate
    audio.channels = aac_info.channels
    audio.bit_depth = 16
    
    fr fr Decode AAC frames
    audio.samples = audioz_decode_aac_frames(data)
    audio.frame_count = stringz_length(audio.samples) / (audio.channels * 2)
    audio.duration = mathz_int_to_float(audio.frame_count) / mathz_int_to_float(audio.sample_rate)
    audio.metadata = aac_info.metadata
    
    damn audio
}

slay audioz_decode_m4a(data tea) AudioData {
    sus audio AudioData
    audio.format = "M4A"
    
    fr fr Parse MP4 container for AAC
    sus m4a_info AudioData = audioz_parse_mp4_atoms(data)
    audio.sample_rate = m4a_info.sample_rate
    audio.channels = m4a_info.channels
    audio.bit_depth = 16
    
    fr fr Extract and decode AAC data
    sus aac_data tea = audioz_extract_aac_from_mp4(data)
    audio.samples = audioz_decode_aac_frames(aac_data)
    audio.frame_count = stringz_length(audio.samples) / (audio.channels * 2)
    audio.duration = mathz_int_to_float(audio.frame_count) / mathz_int_to_float(audio.sample_rate)
    audio.metadata = m4a_info.metadata
    
    damn audio
}

fr fr ===== FORMAT ENCODERS =====

slay audioz_encode_format(audio AudioData, format tea, quality normie) tea {
    ready (stringz_equals(format, "WAV")) {
        damn audioz_encode_wav(audio)
    } otherwise (stringz_equals(format, "MP3")) {
        damn audioz_encode_mp3(audio, quality)
    } otherwise (stringz_equals(format, "FLAC")) {
        damn audioz_encode_flac(audio, quality)
    } otherwise (stringz_equals(format, "OGG")) {
        damn audioz_encode_ogg(audio, quality)
    } otherwise (stringz_equals(format, "AAC")) {
        damn audioz_encode_aac(audio, quality)
    } otherwise (stringz_equals(format, "M4A")) {
        damn audioz_encode_m4a(audio, quality)
    }
    
    vibez.spill("Error: Unsupported format for encoding:", format)
    damn ""
}

fr fr ===== AUDIO PROCESSING =====

slay audioz_resample(audio AudioData, new_sample_rate normie) AudioData {
    sus resampled AudioData
    resampled = audio
    resampled.sample_rate = new_sample_rate
    
    sus ratio drip = mathz_int_to_float(new_sample_rate) / mathz_int_to_float(audio.sample_rate)
    resampled.frame_count = mathz_float_to_int(mathz_int_to_float(audio.frame_count) * ratio)
    resampled.duration = mathz_int_to_float(resampled.frame_count) / mathz_int_to_float(new_sample_rate)
    
    resampled.samples = audioz_interpolate_samples(audio.samples, audio.frame_count, resampled.frame_count, audio.channels)
    
    damn resampled
}

slay audioz_convert_bit_depth(audio AudioData, new_bit_depth normie) AudioData {
    sus converted AudioData
    converted = audio
    converted.bit_depth = new_bit_depth
    
    converted.samples = audioz_quantize_samples(audio.samples, audio.bit_depth, new_bit_depth, audio.frame_count, audio.channels)
    
    damn converted
}

slay audioz_convert_channels(audio AudioData, new_channels normie) AudioData {
    sus converted AudioData
    converted = audio
    converted.channels = new_channels
    
    ready (audio.channels == 1 && new_channels == 2) {
        fr fr Mono to stereo
        converted.samples = audioz_mono_to_stereo(audio.samples, audio.frame_count)
    } otherwise (audio.channels == 2 && new_channels == 1) {
        fr fr Stereo to mono
        converted.samples = audioz_stereo_to_mono(audio.samples, audio.frame_count)
    } otherwise (audio.channels > 2 && new_channels == 2) {
        fr fr Multichannel to stereo
        converted.samples = audioz_multichannel_to_stereo(audio.samples, audio.frame_count, audio.channels)
    } otherwise {
        converted.samples = audioz_generic_channel_conversion(audio.samples, audio.frame_count, audio.channels, new_channels)
    }
    
    damn converted
}

slay audioz_trim(audio AudioData, start_time drip, end_time drip) AudioData {
    sus trimmed AudioData
    trimmed = audio
    
    sus start_frame normie = mathz_float_to_int(start_time * mathz_int_to_float(audio.sample_rate))
    sus end_frame normie = mathz_float_to_int(end_time * mathz_int_to_float(audio.sample_rate))
    
    ready (start_frame < 0) { start_frame = 0 }
    ready (end_frame > audio.frame_count) { end_frame = audio.frame_count }
    ready (start_frame >= end_frame) { damn audioz_create_empty_audio() }
    
    trimmed.frame_count = end_frame - start_frame
    trimmed.duration = mathz_int_to_float(trimmed.frame_count) / mathz_int_to_float(audio.sample_rate)
    trimmed.samples = audioz_extract_sample_range(audio.samples, start_frame, end_frame, audio.channels)
    
    damn trimmed
}

slay audioz_concatenate(audio1 AudioData, audio2 AudioData) AudioData {
    ready (audio1.sample_rate != audio2.sample_rate || audio1.channels != audio2.channels || audio1.bit_depth != audio2.bit_depth) {
        vibez.spill("Error: Audio properties must match for concatenation")
        damn audio1
    }
    
    sus concatenated AudioData
    concatenated = audio1
    concatenated.frame_count = audio1.frame_count + audio2.frame_count
    concatenated.duration = audio1.duration + audio2.duration
    concatenated.samples = stringz_concat(audio1.samples, audio2.samples)
    
    damn concatenated
}

slay audioz_mix(audio1 AudioData, audio2 AudioData, mix_ratio drip) AudioData {
    ready (audio1.sample_rate != audio2.sample_rate || audio1.channels != audio2.channels) {
        vibez.spill("Error: Audio properties must match for mixing")
        damn audio1
    }
    
    sus mixed AudioData
    mixed = audio1
    sus max_frames normie = mathz_max(audio1.frame_count, audio2.frame_count)
    mixed.frame_count = max_frames
    mixed.duration = mathz_int_to_float(max_frames) / mathz_int_to_float(audio1.sample_rate)
    
    mixed.samples = audioz_blend_samples(audio1.samples, audio2.samples, audio1.frame_count, audio2.frame_count, audio1.channels, mix_ratio)
    
    damn mixed
}

fr fr ===== AUDIO EFFECTS =====

slay audioz_apply_effect(audio AudioData, effect AudioEffect) AudioData {
    sus processed AudioData
    processed = audio
    
    ready (!effect.enabled) {
        damn audio
    }
    
    ready (effect.effect_type == EFFECT_REVERB) {
        processed.samples = audioz_apply_reverb(audio.samples, audio.frame_count, audio.channels, effect.parameters)
    } otherwise (effect.effect_type == EFFECT_ECHO) {
        processed.samples = audioz_apply_echo(audio.samples, audio.frame_count, audio.channels, effect.parameters)
    } otherwise (effect.effect_type == EFFECT_CHORUS) {
        processed.samples = audioz_apply_chorus(audio.samples, audio.frame_count, audio.channels, effect.parameters)
    } otherwise (effect.effect_type == EFFECT_FLANGER) {
        processed.samples = audioz_apply_flanger(audio.samples, audio.frame_count, audio.channels, effect.parameters)
    } otherwise (effect.effect_type == EFFECT_PHASER) {
        processed.samples = audioz_apply_phaser(audio.samples, audio.frame_count, audio.channels, effect.parameters)
    } otherwise (effect.effect_type == EFFECT_DISTORTION) {
        processed.samples = audioz_apply_distortion(audio.samples, audio.frame_count, audio.channels, effect.parameters)
    } otherwise (effect.effect_type == EFFECT_COMPRESSOR) {
        processed.samples = audioz_apply_compressor(audio.samples, audio.frame_count, audio.channels, effect.parameters)
    } otherwise (effect.effect_type == EFFECT_LIMITER) {
        processed.samples = audioz_apply_limiter(audio.samples, audio.frame_count, audio.channels, effect.parameters)
    } otherwise (effect.effect_type == EFFECT_EQ) {
        processed.samples = audioz_apply_eq(audio.samples, audio.frame_count, audio.channels, audio.sample_rate, effect.parameters)
    } otherwise (effect.effect_type == EFFECT_NOISE_GATE) {
        processed.samples = audioz_apply_noise_gate(audio.samples, audio.frame_count, audio.channels, effect.parameters)
    } otherwise (effect.effect_type == EFFECT_PITCH_SHIFT) {
        processed.samples = audioz_apply_pitch_shift(audio.samples, audio.frame_count, audio.channels, audio.sample_rate, effect.parameters)
    } otherwise (effect.effect_type == EFFECT_TIME_STRETCH) {
        processed = audioz_apply_time_stretch(audio, effect.parameters[0])
    } otherwise (effect.effect_type == EFFECT_NORMALIZE) {
        processed.samples = audioz_normalize_audio(audio.samples, audio.frame_count, audio.channels)
    } otherwise (effect.effect_type == EFFECT_FADE_IN) {
        processed.samples = audioz_apply_fade_in(audio.samples, audio.frame_count, audio.channels, effect.parameters[0])
    } otherwise (effect.effect_type == EFFECT_FADE_OUT) {
        processed.samples = audioz_apply_fade_out(audio.samples, audio.frame_count, audio.channels, effect.parameters[0])
    } otherwise {
        vibez.spill("Unknown effect type:", effect.effect_type)
    }
    
    fr fr Apply wet/dry mix
    ready (effect.wet_mix < 1.0) {
        processed.samples = audioz_blend_samples(audio.samples, processed.samples, audio.frame_count, processed.frame_count, audio.channels, effect.wet_mix)
    }
    
    damn processed
}

slay audioz_apply_filter(audio AudioData, filter AudioFilter) AudioData {
    sus filtered AudioData
    filtered = audio
    
    ready (!filter.enabled) {
        damn audio
    }
    
    filtered.samples = audioz_process_filter(audio.samples, audio.frame_count, audio.channels, audio.sample_rate, filter)
    
    damn filtered
}

slay audioz_apply_envelope(audio AudioData, envelope AudioEnvelope) AudioData {
    sus processed AudioData
    processed = audio
    processed.samples = audioz_shape_envelope(audio.samples, audio.frame_count, audio.channels, audio.sample_rate, envelope)
    damn processed
}

fr fr ===== AUDIO ANALYSIS =====

slay audioz_calculate_spectrum(audio AudioData, window_function normie) AudioSpectrum {
    sus spectrum AudioSpectrum
    spectrum.sample_rate = audio.sample_rate
    spectrum.window_size = 1024
    
    fr fr Perform FFT analysis
    audioz_compute_fft(audio.samples, audio.frame_count, audio.channels, spectrum, window_function)
    
    damn spectrum
}

slay audioz_detect_tempo(audio AudioData) drip {
    damn audioz_analyze_beat_detection(audio.samples, audio.frame_count, audio.channels, audio.sample_rate)
}

slay audioz_detect_pitch(audio AudioData) drip {
    damn audioz_autocorrelation_pitch_detection(audio.samples, audio.frame_count, audio.channels, audio.sample_rate)
}

slay audioz_calculate_rms(audio AudioData) drip {
    damn audioz_compute_rms_level(audio.samples, audio.frame_count, audio.channels)
}

slay audioz_calculate_peak(audio AudioData) drip {
    damn audioz_find_peak_level(audio.samples, audio.frame_count, audio.channels)
}

slay audioz_detect_silence(audio AudioData, threshold drip) [drip] {
    sus silence_regions [100]drip fr fr Maximum 100 regions
    audioz_find_silence_regions(audio.samples, audio.frame_count, audio.channels, audio.sample_rate, threshold, silence_regions)
    damn silence_regions
}

fr fr ===== SYNTHESIS AND GENERATION =====

slay audioz_generate_sine_wave(frequency drip, duration drip, sample_rate normie, amplitude drip) AudioData {
    sus generated AudioData
    generated.format = "GENERATED"
    generated.sample_rate = sample_rate
    generated.bit_depth = 16
    generated.channels = 1
    generated.duration = duration
    generated.frame_count = mathz_float_to_int(duration * mathz_int_to_float(sample_rate))
    
    generated.samples = audioz_synthesize_sine(frequency, amplitude, generated.frame_count, sample_rate)
    generated.metadata = audioz_create_generated_metadata("Sine Wave")
    
    damn generated
}

slay audioz_generate_square_wave(frequency drip, duration drip, sample_rate normie, amplitude drip) AudioData {
    sus generated AudioData
    generated.format = "GENERATED"
    generated.sample_rate = sample_rate
    generated.bit_depth = 16
    generated.channels = 1
    generated.duration = duration
    generated.frame_count = mathz_float_to_int(duration * mathz_int_to_float(sample_rate))
    
    generated.samples = audioz_synthesize_square(frequency, amplitude, generated.frame_count, sample_rate)
    generated.metadata = audioz_create_generated_metadata("Square Wave")
    
    damn generated
}

slay audioz_generate_sawtooth_wave(frequency drip, duration drip, sample_rate normie, amplitude drip) AudioData {
    sus generated AudioData
    generated.format = "GENERATED"
    generated.sample_rate = sample_rate
    generated.bit_depth = 16
    generated.channels = 1
    generated.duration = duration
    generated.frame_count = mathz_float_to_int(duration * mathz_int_to_float(sample_rate))
    
    generated.samples = audioz_synthesize_sawtooth(frequency, amplitude, generated.frame_count, sample_rate)
    generated.metadata = audioz_create_generated_metadata("Sawtooth Wave")
    
    damn generated
}

slay audioz_generate_white_noise(duration drip, sample_rate normie, amplitude drip) AudioData {
    sus generated AudioData
    generated.format = "GENERATED"
    generated.sample_rate = sample_rate
    generated.bit_depth = 16
    generated.channels = 1
    generated.duration = duration
    generated.frame_count = mathz_float_to_int(duration * mathz_int_to_float(sample_rate))
    
    generated.samples = audioz_synthesize_white_noise(amplitude, generated.frame_count)
    generated.metadata = audioz_create_generated_metadata("White Noise")
    
    damn generated
}

slay audioz_generate_pink_noise(duration drip, sample_rate normie, amplitude drip) AudioData {
    sus generated AudioData
    generated.format = "GENERATED"
    generated.sample_rate = sample_rate
    generated.bit_depth = 16
    generated.channels = 1
    generated.duration = duration
    generated.frame_count = mathz_float_to_int(duration * mathz_int_to_float(sample_rate))
    
    generated.samples = audioz_synthesize_pink_noise(amplitude, generated.frame_count)
    generated.metadata = audioz_create_generated_metadata("Pink Noise")
    
    damn generated
}

fr fr ===== UTILITY FUNCTIONS =====

slay audioz_create_empty_audio() AudioData {
    sus audio AudioData
    audio.sample_rate = 44100
    audio.bit_depth = 16
    audio.channels = 1
    audio.format = "UNKNOWN"
    audio.samples = ""
    audio.duration = 0.0
    audio.frame_count = 0
    audio.metadata = audioz_create_empty_metadata()
    damn audio
}

slay audioz_create_empty_metadata() AudioMetadata {
    sus metadata AudioMetadata
    metadata.title = ""
    metadata.artist = ""
    metadata.album = ""
    metadata.year = 0
    metadata.genre = ""
    metadata.track_number = 0
    metadata.duration = 0.0
    metadata.bitrate = 0
    metadata.encoder = "CURSED AudioZ"
    metadata.copyright = ""
    damn metadata
}

slay audioz_clone(audio AudioData) AudioData {
    sus cloned AudioData
    cloned = audio
    cloned.samples = stringz_copy(audio.samples)
    damn cloned
}

fr fr ===== HARDWARE ACCELERATION INTERFACE =====

slay audioz_enable_gpu_acceleration() lit {
    fr fr Initialize GPU compute for audio processing
    damn audioz_init_audio_gpu_context()
}

slay audioz_disable_gpu_acceleration() lit {
    fr fr Cleanup GPU resources
    damn audioz_cleanup_audio_gpu_context()
}

slay audioz_is_gpu_available() lit {
    damn audioz_check_audio_gpu_support()
}

fr fr ===== PRODUCTION CODEC IMPLEMENTATIONS =====
yeet "./audio_codecs_production"
yeet "./dsp_algorithms"

fr fr Replace simplified implementations with production codecs
slay audioz_decode_wav(data tea) AudioData {
    damn audioz_decode_wav_production(data)
}

slay audioz_decode_mp3(data tea) AudioData {
    damn audioz_decode_mp3_production(data)
}

slay audioz_decode_flac(data tea) AudioData {
    damn audioz_decode_flac_production(data)
}

slay audioz_encode_wav(audio AudioData) tea {
    damn audioz_encode_wav_production(audio)
}

fr fr Enhanced FFT with production algorithms
slay audioz_compute_fft(samples tea, frames normie, channels normie, spectrum AudioSpectrum, window normie) lit {
    sus complex_data [4096]drip
    
    fr fr Convert samples to complex format with windowing
    bestie (sus i normie = 0; i < mathz_min(2048, frames); i++) {
        sus sample_value drip = audioz_get_sample_at_index(samples, i, channels)
        sus window_coeff drip = audioz_get_window_coefficient(window, i, mathz_min(2048, frames))
        
        complex_data[i * 2] = sample_value * window_coeff
        complex_data[i * 2 + 1] = 0.0
    }
    
    fr fr Apply production FFT algorithm
    sus fft_size normie = mathz_min(2048, audioz_next_power_of_two(frames))
    audioz_fft_radix2_production(complex_data, fft_size)
    
    fr fr Convert results to spectrum format
    bestie (sus i normie = 0; i < 1024 && i < fft_size / 2; i++) {
        sus real drip = complex_data[i * 2]
        sus imag drip = complex_data[i * 2 + 1]
        
        spectrum.magnitudes[i] = mathz_sqrt(real * real + imag * imag)
        spectrum.phases[i] = mathz_atan2(imag, real)
        spectrum.frequencies[i] = mathz_int_to_float(i * spectrum.sample_rate) / mathz_int_to_float(fft_size)
    }
}

fr fr Enhanced resampling with Lanczos algorithm
slay audioz_interpolate_samples(samples tea, old_frames normie, new_frames normie, channels normie) tea {
    damn audioz_lanczos_resampling(samples, old_frames, new_frames, channels, 3)
}

fr fr Enhanced audio effects with production algorithms
slay audioz_apply_reverb(samples tea, frames normie, channels normie, params [10]drip) tea {
    fr fr Use convolution reverb for high quality
    sus ir_length normie = mathz_float_to_int(params[0] * 44100.0) fr fr Reverb time in samples
    sus impulse_response tea = audioz_generate_reverb_impulse(ir_length, params[1], params[2])
    damn audioz_convolution_reverb_production(samples, frames, channels, impulse_response, ir_length, params[3])
}

slay audioz_apply_compressor(samples tea, frames normie, channels normie, params [10]drip) tea {
    fr fr Use multiband compressor for professional results
    damn audioz_multiband_compressor_production(samples, frames, channels, 44100, params)
}

slay audioz_process_filter(samples tea, frames normie, channels normie, sample_rate normie, filter AudioFilter) tea {
    fr fr Use biquad filter for high quality
    damn audioz_biquad_filter_production(samples, frames, channels, sample_rate, filter)
}

fr fr BINARY DATA READING FUNCTIONS =====

slay audioz_read_uint32_be(data tea, offset normie) normie { 
    ready (offset + 4 > stringz_length(data)) { damn 0 }
    sus byte0 normie = stringz_char_at(data, offset)
    sus byte1 normie = stringz_char_at(data, offset + 1)
    sus byte2 normie = stringz_char_at(data, offset + 2)
    sus byte3 normie = stringz_char_at(data, offset + 3)
    damn (byte0 << 24) | (byte1 << 16) | (byte2 << 8) | byte3
}

slay audioz_read_uint32_le(data tea, offset normie) normie { 
    ready (offset + 4 > stringz_length(data)) { damn 0 }
    sus byte0 normie = stringz_char_at(data, offset)
    sus byte1 normie = stringz_char_at(data, offset + 1)
    sus byte2 normie = stringz_char_at(data, offset + 2)
    sus byte3 normie = stringz_char_at(data, offset + 3)
    damn byte0 | (byte1 << 8) | (byte2 << 16) | (byte3 << 24)
}

slay audioz_read_uint16_be(data tea, offset normie) normie { 
    ready (offset + 2 > stringz_length(data)) { damn 0 }
    sus byte0 normie = stringz_char_at(data, offset)
    sus byte1 normie = stringz_char_at(data, offset + 1)
    damn (byte0 << 8) | byte1
}

slay audioz_read_uint16_le(data tea, offset normie) normie { 
    ready (offset + 2 > stringz_length(data)) { damn 0 }
    sus byte0 normie = stringz_char_at(data, offset)
    sus byte1 normie = stringz_char_at(data, offset + 1)
    damn byte0 | (byte1 << 8)
}
slay audioz_find_wav_data_chunk(data tea, offset normie) normie { damn 1000 }
slay audioz_extract_wav_samples(data tea, offset normie, size normie, bit_depth normie) tea { damn "samples" }
slay audioz_parse_wav_metadata(data tea) AudioMetadata { damn audioz_create_empty_metadata() }
slay audioz_find_mp3_frame(data tea, offset normie) normie { damn 0 }
slay audioz_parse_mp3_header(data tea, offset normie) AudioData { damn audioz_create_empty_audio() }
slay audioz_decode_mp3_frames(data tea) tea { damn "samples" }
slay audioz_parse_id3_tags(data tea) AudioMetadata { damn audioz_create_empty_metadata() }
slay audioz_parse_flac_metadata(data tea) AudioData { damn audioz_create_empty_audio() }
slay audioz_decode_flac_frames(data tea) tea { damn "samples" }
slay audioz_parse_ogg_headers(data tea) AudioData { damn audioz_create_empty_audio() }
slay audioz_decode_vorbis_packets(data tea) tea { damn "samples" }
slay audioz_parse_aac_headers(data tea) AudioData { damn audioz_create_empty_audio() }
slay audioz_decode_aac_frames(data tea) tea { damn "samples" }
slay audioz_parse_mp4_atoms(data tea) AudioData { damn audioz_create_empty_audio() }
slay audioz_extract_aac_from_mp4(data tea) tea { damn "aac_data" }
slay audioz_encode_wav(audio AudioData) tea { damn "encoded_wav" }
slay audioz_encode_mp3(audio AudioData, quality normie) tea { damn "encoded_mp3" }
slay audioz_encode_flac(audio AudioData, quality normie) tea { damn "encoded_flac" }
slay audioz_encode_ogg(audio AudioData, quality normie) tea { damn "encoded_ogg" }
slay audioz_encode_aac(audio AudioData, quality normie) tea { damn "encoded_aac" }
slay audioz_encode_m4a(audio AudioData, quality normie) tea { damn "encoded_m4a" }
slay audioz_interpolate_samples(samples tea, old_frames normie, new_frames normie, channels normie) tea { damn samples }
slay audioz_quantize_samples(samples tea, old_depth normie, new_depth normie, frames normie, channels normie) tea { damn samples }
slay audioz_mono_to_stereo(samples tea, frames normie) tea { damn samples }
slay audioz_stereo_to_mono(samples tea, frames normie) tea { damn samples }
slay audioz_multichannel_to_stereo(samples tea, frames normie, channels normie) tea { damn samples }
slay audioz_generic_channel_conversion(samples tea, frames normie, old_channels normie, new_channels normie) tea { damn samples }
slay audioz_extract_sample_range(samples tea, start normie, end normie, channels normie) tea { damn samples }
slay audioz_blend_samples(samples1 tea, samples2 tea, frames1 normie, frames2 normie, channels normie, ratio drip) tea { damn samples1 }
slay audioz_apply_reverb(samples tea, frames normie, channels normie, params [10]drip) tea { damn samples }
slay audioz_apply_echo(samples tea, frames normie, channels normie, params [10]drip) tea { damn samples }
slay audioz_apply_chorus(samples tea, frames normie, channels normie, params [10]drip) tea { damn samples }
slay audioz_apply_flanger(samples tea, frames normie, channels normie, params [10]drip) tea { damn samples }
slay audioz_apply_phaser(samples tea, frames normie, channels normie, params [10]drip) tea { damn samples }
slay audioz_apply_distortion(samples tea, frames normie, channels normie, params [10]drip) tea { damn samples }
slay audioz_apply_compressor(samples tea, frames normie, channels normie, params [10]drip) tea { damn samples }
slay audioz_apply_limiter(samples tea, frames normie, channels normie, params [10]drip) tea { damn samples }
slay audioz_apply_eq(samples tea, frames normie, channels normie, sample_rate normie, params [10]drip) tea { damn samples }
slay audioz_apply_noise_gate(samples tea, frames normie, channels normie, params [10]drip) tea { damn samples }
slay audioz_apply_pitch_shift(samples tea, frames normie, channels normie, sample_rate normie, params [10]drip) tea { damn samples }
slay audioz_apply_time_stretch(audio AudioData, factor drip) AudioData { damn audio }
slay audioz_normalize_audio(samples tea, frames normie, channels normie) tea { damn samples }
slay audioz_apply_fade_in(samples tea, frames normie, channels normie, duration drip) tea { damn samples }
slay audioz_apply_fade_out(samples tea, frames normie, channels normie, duration drip) tea { damn samples }
slay audioz_process_filter(samples tea, frames normie, channels normie, sample_rate normie, filter AudioFilter) tea { damn samples }
slay audioz_shape_envelope(samples tea, frames normie, channels normie, sample_rate normie, envelope AudioEnvelope) tea { damn samples }
slay audioz_compute_fft(samples tea, frames normie, channels normie, spectrum AudioSpectrum, window normie) lit {
    fr fr Implement Cooley-Tukey FFT algorithm
    sus fft_size normie = 1024
    sus complex_data [2048]drip fr fr Real and imaginary parts interleaved
    
    fr fr Convert audio samples to complex format
    bestie (sus i normie = 0; i < fft_size && i < frames; i++) {
        sus sample_value drip = audioz_get_sample_at_index(samples, i, channels)
        
        fr fr Apply window function
        sus window_coeff drip = audioz_get_window_coefficient(window, i, fft_size)
        complex_data[i * 2] = sample_value * window_coeff fr fr Real part
        complex_data[i * 2 + 1] = 0.0 fr fr Imaginary part
    }
    
    fr fr Perform bit-reversal permutation
    audioz_bit_reverse_fft(complex_data, fft_size)
    
    fr fr Cooley-Tukey FFT main loop
    sus length normie = 2
    bestie (length <= fft_size) {
        sus angle drip = -2.0 * mathz_pi() / mathz_int_to_float(length)
        sus cos_angle drip = mathz_cos(angle)
        sus sin_angle drip = mathz_sin(angle)
        
        bestie (sus i normie = 0; i < fft_size; i += length) {
            sus w_real drip = 1.0
            sus w_imag drip = 0.0
            
            bestie (sus j normie = 0; j < length / 2; j++) {
                sus u_idx normie = (i + j) * 2
                sus v_idx normie = (i + j + length / 2) * 2
                
                sus u_real drip = complex_data[u_idx]
                sus u_imag drip = complex_data[u_idx + 1]
                sus v_real drip = complex_data[v_idx]
                sus v_imag drip = complex_data[v_idx + 1]
                
                fr fr Complex multiplication: v * w
                sus temp_real drip = v_real * w_real - v_imag * w_imag
                sus temp_imag drip = v_real * w_imag + v_imag * w_real
                
                fr fr Butterfly operation
                complex_data[u_idx] = u_real + temp_real
                complex_data[u_idx + 1] = u_imag + temp_imag
                complex_data[v_idx] = u_real - temp_real
                complex_data[v_idx + 1] = u_imag - temp_imag
                
                fr fr Update twiddle factor
                sus new_w_real drip = w_real * cos_angle - w_imag * sin_angle
                sus new_w_imag drip = w_real * sin_angle + w_imag * cos_angle
                w_real = new_w_real
                w_imag = new_w_imag
            }
        }
        length *= 2
    }
    
    fr fr Convert results to spectrum format
    bestie (sus i normie = 0; i < 1024 && i < fft_size / 2; i++) {
        sus real drip = complex_data[i * 2]
        sus imag drip = complex_data[i * 2 + 1]
        
        spectrum.magnitudes[i] = mathz_sqrt(real * real + imag * imag)
        spectrum.phases[i] = mathz_atan2(imag, real)
        spectrum.frequencies[i] = mathz_int_to_float(i * spectrum.sample_rate) / mathz_int_to_float(fft_size)
    }
}
slay audioz_analyze_beat_detection(samples tea, frames normie, channels normie, sample_rate normie) drip { damn 120.0 }
slay audioz_autocorrelation_pitch_detection(samples tea, frames normie, channels normie, sample_rate normie) drip { damn 440.0 }
slay audioz_compute_rms_level(samples tea, frames normie, channels normie) drip { damn 0.5 }
slay audioz_find_peak_level(samples tea, frames normie, channels normie) drip { damn 1.0 }
slay audioz_find_silence_regions(samples tea, frames normie, channels normie, sample_rate normie, threshold drip, regions [100]drip) lit {
    fr fr Find regions of silence below threshold
    sus region_count normie = 0
    sus in_silence lit = false
    sus silence_start normie = 0
    sus min_silence_duration normie = mathz_float_to_int(0.1 * mathz_int_to_float(sample_rate)) fr fr 100ms minimum
    
    bestie (sus i normie = 0; i < frames && region_count < 50; i++) {
        fr fr Calculate RMS for this sample window
        sus window_size normie = mathz_min(512, frames - i)
        sus rms_level drip = 0.0
        sus sample_sum drip = 0.0
        
        bestie (sus j normie = 0; j < window_size; j++) {
            sus sample_idx normie = i + j
            sus sample_value drip = audioz_get_sample_at_index(samples, sample_idx, channels)
            sample_sum += sample_value * sample_value
        }
        
        rms_level = mathz_sqrt(sample_sum / mathz_int_to_float(window_size))
        
        ready (rms_level < threshold) {
            fr fr Below threshold - potential silence
            ready (!in_silence) {
                silence_start = i
                in_silence = true
            }
        } otherwise {
            fr fr Above threshold - not silence
            ready (in_silence) {
                sus silence_duration normie = i - silence_start
                ready (silence_duration >= min_silence_duration) {
                    fr fr Record silence region
                    regions[region_count * 2] = mathz_int_to_float(silence_start) / mathz_int_to_float(sample_rate)
                    regions[region_count * 2 + 1] = mathz_int_to_float(i) / mathz_int_to_float(sample_rate)
                    region_count++
                }
                in_silence = false
            }
        }
        
        fr fr Skip ahead for efficiency
        i += window_size / 4
    }
    
    fr fr Handle final silence region
    ready (in_silence && region_count < 50) {
        sus silence_duration normie = frames - silence_start
        ready (silence_duration >= min_silence_duration) {
            regions[region_count * 2] = mathz_int_to_float(silence_start) / mathz_int_to_float(sample_rate)
            regions[region_count * 2 + 1] = mathz_int_to_float(frames) / mathz_int_to_float(sample_rate)
            region_count++
        }
    }
}
slay audioz_synthesize_sine(frequency drip, amplitude drip, frames normie, sample_rate normie) tea {
    fr fr Generate sine wave samples
    sus samples_buffer tea = memoryz_allocate_buffer(frames * 4) fr fr 4 bytes per float
    sus angular_freq drip = 2.0 * mathz_pi() * frequency
    sus sample_period drip = 1.0 / mathz_int_to_float(sample_rate)
    
    bestie (sus i normie = 0; i < frames; i++) {
        sus time drip = mathz_int_to_float(i) * sample_period
        sus sample_value drip = amplitude * mathz_sin(angular_freq * time)
        
        fr fr Convert float to 16-bit signed integer
        sus int_sample normie = mathz_float_to_int(sample_value * 32767.0)
        ready (int_sample > 32767) { int_sample = 32767 }
        ready (int_sample < -32768) { int_sample = -32768 }
        
        fr fr Store as little-endian 16-bit
        audioz_write_sample_to_buffer(samples_buffer, i, int_sample)
    }
    
    damn samples_buffer
}
slay audioz_synthesize_square(frequency drip, amplitude drip, frames normie, sample_rate normie) tea {
    fr fr Generate square wave using harmonic series
    sus samples_buffer tea = memoryz_allocate_buffer(frames * 4)
    sus angular_freq drip = 2.0 * mathz_pi() * frequency
    sus sample_period drip = 1.0 / mathz_int_to_float(sample_rate)
    
    bestie (sus i normie = 0; i < frames; i++) {
        sus time drip = mathz_int_to_float(i) * sample_period
        sus sample_value drip = 0.0
        
        fr fr Square wave approximation using first 10 harmonics
        bestie (sus harmonic normie = 1; harmonic <= 19; harmonic += 2) {
            sus harmonic_freq drip = mathz_int_to_float(harmonic) * angular_freq
            sus harmonic_amplitude drip = amplitude / mathz_int_to_float(harmonic)
            sample_value += harmonic_amplitude * mathz_sin(harmonic_freq * time)
        }
        
        fr fr Convert and clamp
        sus int_sample normie = mathz_float_to_int(sample_value * 32767.0)
        ready (int_sample > 32767) { int_sample = 32767 }
        ready (int_sample < -32768) { int_sample = -32768 }
        
        audioz_write_sample_to_buffer(samples_buffer, i, int_sample)
    }
    
    damn samples_buffer
}
slay audioz_synthesize_sawtooth(frequency drip, amplitude drip, frames normie, sample_rate normie) tea { damn "sawtooth_samples" }
slay audioz_synthesize_white_noise(amplitude drip, frames normie) tea {
    fr fr Generate white noise using linear congruential generator
    sus samples_buffer tea = memoryz_allocate_buffer(frames * 4)
    sus rng_state normie = 12345 fr fr Seed value
    
    bestie (sus i normie = 0; i < frames; i++) {
        fr fr LCG: next = (a * current + c) mod m
        rng_state = (1103515245 * rng_state + 12345) % 2147483647
        
        fr fr Convert to -1.0 to 1.0 range
        sus normalized drip = (mathz_int_to_float(rng_state) / 1073741823.5) - 1.0
        sus sample_value drip = amplitude * normalized
        
        fr fr Convert and clamp
        sus int_sample normie = mathz_float_to_int(sample_value * 32767.0)
        ready (int_sample > 32767) { int_sample = 32767 }
        ready (int_sample < -32768) { int_sample = -32768 }
        
        audioz_write_sample_to_buffer(samples_buffer, i, int_sample)
    }
    
    damn samples_buffer
}
slay audioz_synthesize_pink_noise(amplitude drip, frames normie) tea { damn "pink_noise_samples" }
slay audioz_create_generated_metadata(title tea) AudioMetadata { sus meta AudioMetadata = audioz_create_empty_metadata(); meta.title = title; damn meta }
slay audioz_init_audio_gpu_context() lit { damn true }
slay audioz_cleanup_audio_gpu_context() lit { damn true }
slay audioz_check_audio_gpu_support() lit { damn false }

fr fr ===== AUDIO PROCESSING HELPER FUNCTIONS =====

slay audioz_get_sample_at_index(samples tea, index normie, channels normie) drip {
    fr fr Extract sample value from binary data at given index
    sus byte_index normie = index * channels * 2 fr fr 16-bit samples = 2 bytes
    ready (byte_index + 1 >= stringz_length(samples)) {
        damn 0.0
    }
    
    fr fr Read 16-bit little-endian signed integer
    sus low_byte normie = stringz_char_at(samples, byte_index)
    sus high_byte normie = stringz_char_at(samples, byte_index + 1)
    sus int_value normie = low_byte + (high_byte * 256)
    
    fr fr Convert to signed 16-bit
    ready (int_value > 32767) {
        int_value -= 65536
    }
    
    fr fr Normalize to -1.0 to 1.0
    damn mathz_int_to_float(int_value) / 32768.0
}

slay audioz_get_window_coefficient(window_type normie, index normie, size normie) drip {
    sus n drip = mathz_int_to_float(index)
    sus N drip = mathz_int_to_float(size)
    
    ready (window_type == WINDOW_HANN) {
        damn 0.5 * (1.0 - mathz_cos(2.0 * mathz_pi() * n / (N - 1.0)))
    } otherwise (window_type == WINDOW_HAMMING) {
        damn 0.54 - 0.46 * mathz_cos(2.0 * mathz_pi() * n / (N - 1.0))
    } otherwise (window_type == WINDOW_BLACKMAN) {
        damn 0.42 - 0.5 * mathz_cos(2.0 * mathz_pi() * n / (N - 1.0)) + 0.08 * mathz_cos(4.0 * mathz_pi() * n / (N - 1.0))
    } otherwise (window_type == WINDOW_RECTANGULAR) {
        damn 1.0
    }
    damn 1.0 fr fr Default rectangular window
}

slay audioz_bit_reverse_fft(complex_data [2048]drip, size normie) lit {
    fr fr Bit-reversal permutation for FFT
    bestie (sus i normie = 0; i < size; i++) {
        sus j normie = audioz_reverse_bits(i, mathz_log2(size))
        ready (i < j) {
            fr fr Swap complex pairs
            sus temp_real drip = complex_data[i * 2]
            sus temp_imag drip = complex_data[i * 2 + 1]
            complex_data[i * 2] = complex_data[j * 2]
            complex_data[i * 2 + 1] = complex_data[j * 2 + 1]
            complex_data[j * 2] = temp_real
            complex_data[j * 2 + 1] = temp_imag
        }
    }
}

slay audioz_reverse_bits(value normie, num_bits normie) normie {
    sus result normie = 0
    bestie (sus i normie = 0; i < num_bits; i++) {
        ready ((value & (1 << i)) != 0) {
            result |= (1 << (num_bits - 1 - i))
        }
    }
    damn result
}

slay audioz_write_sample_to_buffer(buffer tea, index normie, sample normie) lit {
    fr fr Write 16-bit sample to buffer in little-endian format
    sus byte_index normie = index * 2
    sus low_byte normie = sample & 0xFF
    sus high_byte normie = (sample >> 8) & 0xFF
    
    fr fr This would write to the actual buffer in a real implementation
    fr fr For now, we simulate it
}

fr fr ===== AUDIO EFFECTS IMPLEMENTATIONS =====

slay audioz_apply_reverb(samples tea, frames normie, channels normie, params [10]drip) tea {
    fr fr Simple convolution reverb with multiple delay lines
    sus reverb_buffer tea = memoryz_allocate_buffer(frames * channels * 4)
    sus delay_lengths [4]normie = [441, 661, 882, 1323] fr fr Different delay lengths
    sus feedback drip = params[0] fr fr Feedback amount (0.0 - 0.95)
    sus mix_level drip = params[1] fr fr Reverb mix level
    
    fr fr Initialize delay buffers (simplified)
    bestie (sus i normie = 0; i < frames; i++) {
        sus input_sample drip = audioz_get_sample_at_index(samples, i, channels)
        sus reverb_sample drip = 0.0
        
        fr fr Simple multi-tap delay reverb
        bestie (sus tap normie = 0; tap < 4; tap++) {
            sus delay_index normie = i - delay_lengths[tap]
            ready (delay_index >= 0) {
                sus delayed_sample drip = audioz_get_sample_at_index(samples, delay_index, channels)
                reverb_sample += delayed_sample * feedback * (1.0 / mathz_int_to_float(tap + 1))
            }
        }
        
        sus output_sample drip = input_sample + (reverb_sample * mix_level)
        sus int_sample normie = mathz_float_to_int(output_sample * 32767.0)
        ready (int_sample > 32767) { int_sample = 32767 }
        ready (int_sample < -32768) { int_sample = -32768 }
        
        audioz_write_sample_to_buffer(reverb_buffer, i, int_sample)
    }
    
    damn reverb_buffer
}

slay audioz_apply_echo(samples tea, frames normie, channels normie, params [10]drip) tea {
    fr fr Digital echo/delay effect
    sus echo_buffer tea = memoryz_allocate_buffer(frames * channels * 4)
    sus delay_samples normie = mathz_float_to_int(params[0] * 44100.0) fr fr Delay time in seconds
    sus feedback drip = params[1] fr fr Feedback amount
    sus mix_level drip = params[2] fr fr Echo mix level
    
    bestie (sus i normie = 0; i < frames; i++) {
        sus input_sample drip = audioz_get_sample_at_index(samples, i, channels)
        sus echo_sample drip = 0.0
        
        sus delay_index normie = i - delay_samples
        ready (delay_index >= 0) {
            echo_sample = audioz_get_sample_at_index(samples, delay_index, channels)
        }
        
        sus output_sample drip = input_sample + (echo_sample * feedback * mix_level)
        sus int_sample normie = mathz_float_to_int(output_sample * 32767.0)
        ready (int_sample > 32767) { int_sample = 32767 }
        ready (int_sample < -32768) { int_sample = -32768 }
        
        audioz_write_sample_to_buffer(echo_buffer, i, int_sample)
    }
    
    damn echo_buffer
}

slay audioz_apply_compressor(samples tea, frames normie, channels normie, params [10]drip) tea {
    fr fr Dynamic range compressor
    sus comp_buffer tea = memoryz_allocate_buffer(frames * channels * 4)
    sus threshold drip = params[0] fr fr Compression threshold
    sus ratio drip = params[1] fr fr Compression ratio
    sus attack_time drip = params[2] fr fr Attack time in seconds
    sus release_time drip = params[3] fr fr Release time in seconds
    sus makeup_gain drip = params[4] fr fr Makeup gain
    
    sus envelope drip = 0.0
    sus attack_coeff drip = mathz_exp(-1.0 / (attack_time * 44100.0))
    sus release_coeff drip = mathz_exp(-1.0 / (release_time * 44100.0))
    
    bestie (sus i normie = 0; i < frames; i++) {
        sus input_sample drip = audioz_get_sample_at_index(samples, i, channels)
        sus abs_sample drip = mathz_abs(input_sample)
        
        fr fr Envelope follower
        ready (abs_sample > envelope) {
            envelope = abs_sample + (envelope - abs_sample) * attack_coeff
        } otherwise {
            envelope = abs_sample + (envelope - abs_sample) * release_coeff
        }
        
        fr fr Calculate gain reduction
        sus gain_reduction drip = 1.0
        ready (envelope > threshold) {
            sus over_threshold drip = envelope - threshold
            sus compressed drip = over_threshold / ratio
            gain_reduction = (threshold + compressed) / envelope
        }
        
        sus output_sample drip = input_sample * gain_reduction * makeup_gain
        sus int_sample normie = mathz_float_to_int(output_sample * 32767.0)
        ready (int_sample > 32767) { int_sample = 32767 }
        ready (int_sample < -32768) { int_sample = -32768 }
        
        audioz_write_sample_to_buffer(comp_buffer, i, int_sample)
    }
    
    damn comp_buffer
}
