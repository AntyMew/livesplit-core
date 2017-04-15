use hotkey::KeyCode;
use std::sync::Arc;
use parking_lot::RwLock;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct HotkeyConfig {
    pub split: KeyCode,
    pub reset: KeyCode,
    pub skip: KeyCode,
    pub undo: KeyCode,
    pub redo: KeyCode,
    pub pause: KeyCode,
    pub previous_comparison: KeyCode,
    pub next_comparison: KeyCode,
}

pub type SharedHotkeyConfig = Arc<RwLock<HotkeyConfig>>;

impl HotkeyConfig {
    pub fn into_shared(self) -> SharedHotkeyConfig {
        Arc::new(RwLock::new(self))
    }
}

#[cfg(windows)]
impl Default for HotkeyConfig {
    fn default() -> Self {
        use hotkey::KeyCode::*;
        Self {
            split: NumPad1,
            reset: NumPad3,
            skip: NumPad2,
            undo: NumPad7,
            redo: NumPad9,
            pause: NumPad5,
            previous_comparison: NumPad4,
            next_comparison: NumPad6,
        }
    }
}

#[cfg(not(any(windows)))]
impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            split: KeyCode,
            reset: KeyCode,
            skip: KeyCode,
            undo: KeyCode,
            redo: KeyCode,
            pause: KeyCode,
            previous_comparison: KeyCode,
            next_comparison: KeyCode,
        }
    }
}
