// Audio system with sound management
import "../../utils/time_manager.csd" as Time
export SoundManager, AudioClip, SoundEffect, MusicTrack

struct SoundManager {
    audio_clips: Map<int, AudioClip>,
    playing_sounds: Vec<PlayingSound>,
    music_tracks: Map<int, MusicTrack>,
    current_music: Option<int>,
    master_volume: float,
    sound_volume: float,
    music_volume: float,
    is_initialized: bool
}

struct AudioClip {
    id: int,
    name: string,
    duration: float,
    sample_rate: int,
    channels: int,
    format: AudioFormat
}

struct PlayingSound {
    clip_id: int,
    start_time: float,
    volume: float,
    pitch: float,
    is_looping: bool,
    is_finished: bool
}

struct MusicTrack {
    id: int,
    name: string,
    file_path: string,
    duration: float,
    is_looping: bool
}

enum AudioFormat {
    WAV,
    MP3,
    OGG,
    FLAC
}

enum SoundEffect {
    PlayerJump,
    EnemyHit,
    ItemPickup,
    ButtonClick,
    DoorOpen,
    Explosion,
    Footstep,
    WeaponFire
}

impl SoundManager {
    func new() -> SoundManager {
        println("Initializing Sound Manager")
        
        let mut manager = SoundManager {
            audio_clips: Map.new(),
            playing_sounds: Vec.new(),
            music_tracks: Map.new(),
            current_music: None,
            master_volume: 1.0,
            sound_volume: 1.0,
            music_volume: 0.8,
            is_initialized: false
        }
        
        manager.initialize_default_sounds()
        manager.is_initialized = true
        manager
    }
    
    func initialize_default_sounds(&mut self) {
        // Load common sound effects
        self.load_sound_effect(SoundEffect::PlayerJump, "jump.wav")
        self.load_sound_effect(SoundEffect::EnemyHit, "hit.wav")
        self.load_sound_effect(SoundEffect::ItemPickup, "pickup.wav")
        self.load_sound_effect(SoundEffect::ButtonClick, "click.wav")
        self.load_sound_effect(SoundEffect::DoorOpen, "door.wav")
        self.load_sound_effect(SoundEffect::Explosion, "explosion.wav")
        self.load_sound_effect(SoundEffect::Footstep, "footstep.wav")
        self.load_sound_effect(SoundEffect::WeaponFire, "gunshot.wav")
        
        // Load background music
        self.load_music_track(1, "background_music.mp3", true)
        self.load_music_track(2, "boss_battle.mp3", true)
        self.load_music_track(3, "menu_theme.mp3", true)
        
        println("Default audio assets loaded")
    }
    
    func load_sound_effect(&mut self, effect: SoundEffect, file_path: string) {
        let clip_id = self.get_sound_effect_id(effect)
        let audio_clip = AudioClip {
            id: clip_id,
            name: file_path.clone(),
            duration: self.get_mock_duration(&file_path),
            sample_rate: 44100,
            channels: 2,
            format: self.get_format_from_path(&file_path)
        }
        
        self.audio_clips.insert(clip_id, audio_clip)
        println("Loaded sound effect: " + file_path)
    }
    
    func load_music_track(&mut self, track_id: int, file_path: string, is_looping: bool) {
        let music_track = MusicTrack {
            id: track_id,
            name: file_path.clone(),
            file_path: file_path.clone(),
            duration: self.get_mock_duration(&file_path) * 10.0, // Music is longer
            is_looping: is_looping
        }
        
        self.music_tracks.insert(track_id, music_track)
        println("Loaded music track: " + file_path)
    }
    
    func play_sound_effect(&mut self, effect: SoundEffect) -> int {
        self.play_sound_effect_with_params(effect, 1.0, 1.0, false)
    }
    
    func play_sound_effect_with_params(&mut self, effect: SoundEffect, volume: float, pitch: float, is_looping: bool) -> int {
        let clip_id = self.get_sound_effect_id(effect)
        
        if !self.audio_clips.contains_key(&clip_id) {
            println("Warning: Sound effect not loaded: " + clip_id.to_string())
            return -1
        }
        
        let playing_sound = PlayingSound {
            clip_id: clip_id,
            start_time: Time.mock_time(),
            volume: volume * self.sound_volume * self.master_volume,
            pitch: pitch,
            is_looping: is_looping,
            is_finished: false
        }
        
        self.playing_sounds.push(playing_sound)
        let sound_instance_id = self.playing_sounds.len() - 1
        
        println("Playing sound effect: " + clip_id.to_string())
        return sound_instance_id as int
    }
    
    func play_music(&mut self, track_id: int) {
        if !self.music_tracks.contains_key(&track_id) {
            println("Warning: Music track not found: " + track_id.to_string())
            return
        }
        
        // Stop current music if playing
        if let Some(current_id) = self.current_music {
            self.stop_music()
        }
        
        self.current_music = Some(track_id)
        println("Playing music track: " + track_id.to_string())
    }
    
    func stop_music(&mut self) {
        if let Some(track_id) = self.current_music {
            println("Stopping music track: " + track_id.to_string())
            self.current_music = None
        }
    }
    
    func set_master_volume(&mut self, volume: float) {
        self.master_volume = clamp(volume, 0.0, 1.0)
        println("Master volume set to: " + self.master_volume.to_string())
    }
    
    func set_sound_volume(&mut self, volume: float) {
        self.sound_volume = clamp(volume, 0.0, 1.0)
        println("Sound volume set to: " + self.sound_volume.to_string())
    }
    
    func set_music_volume(&mut self, volume: float) {
        self.music_volume = clamp(volume, 0.0, 1.0)
        println("Music volume set to: " + self.music_volume.to_string())
    }
    
    func update(&mut self) {
        // Update playing sounds and remove finished ones
        let current_time = Time.mock_time()
        
        self.playing_sounds.retain_mut(|sound| {
            if sound.is_finished {
                return false
            }
            
            if let Some(clip) = self.audio_clips.get(&sound.clip_id) {
                let elapsed = current_time - sound.start_time
                if elapsed >= clip.duration && !sound.is_looping {
                    sound.is_finished = true
                    return false
                }
            }
            
            true
        })
    }
    
    func shutdown(&mut self) {
        println("Shutting down Sound Manager")
        self.stop_music()
        self.playing_sounds.clear()
        self.is_initialized = false
    }
    
    // Helper functions
    func get_sound_effect_id(&self, effect: SoundEffect) -> int {
        match effect {
            SoundEffect::PlayerJump => 1,
            SoundEffect::EnemyHit => 2,
            SoundEffect::ItemPickup => 3,
            SoundEffect::ButtonClick => 4,
            SoundEffect::DoorOpen => 5,
            SoundEffect::Explosion => 6,
            SoundEffect::Footstep => 7,
            SoundEffect::WeaponFire => 8
        }
    }
    
    func get_mock_duration(&self, file_path: &string) -> float {
        // Mock duration based on file name
        if file_path.contains("music") {
            return 180.0  // 3 minutes for music
        } else {
            return 1.5    // 1.5 seconds for sound effects
        }
    }
    
    func get_format_from_path(&self, file_path: &string) -> AudioFormat {
        if file_path.ends_with(".wav") {
            AudioFormat::WAV
        } else if file_path.ends_with(".mp3") {
            AudioFormat::MP3
        } else if file_path.ends_with(".ogg") {
            AudioFormat::OGG
        } else {
            AudioFormat::WAV
        }
    }
}

// Utility functions
func clamp(value: float, min: float, max: float) -> float {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
