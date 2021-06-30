extern crate rusty_audio;
use rusty_audio::Audio;
use once_cell::sync::OnceCell;

pub struct AudioManager {
    audio:Audio
}

static mut AUDIO_MANAGER:OnceCell<AudioManager> = OnceCell::new();

impl AudioManager {
    pub fn create_instance() {
        let manager = AudioManager {
            audio: Audio::new()
        };

        unsafe {
            AUDIO_MANAGER.set(manager);
        }
    }

    pub fn get_instance()->&'static mut AudioManager {
        unsafe {
            AUDIO_MANAGER.get_mut().expect("Audio Manager has not been initialized")
        }
    }

    pub fn load_sound(&mut self, src: &str, name:&'static str) {
        self.audio.add(name, src);
    }

    pub fn play_sound(&mut self, name:&str) {
        self.audio.play(name);
    }
}