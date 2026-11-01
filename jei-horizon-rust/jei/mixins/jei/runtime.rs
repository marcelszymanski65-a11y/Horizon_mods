use std::cell::RefCell;

pub const ID: &str = "jei";
pub const VERSION: &str = "31.17.0.108";
pub const DESCRIPTION: &str = "JEI is an item and recipe viewing mod for Minecraft, built from the ground up for stability and performance.";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub show_hud: bool,
    pub compact: bool,
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let show_hud = get("show_hud").map(|v| v.trim() != "false").unwrap_or(true);
        let compact = get("compact").map(|v| v.trim() == "true").unwrap_or(false);
        Self { show_hud, compact }
    }

    pub fn stored(&self) -> [(&'static str, String); 2] {
        [
            ("show_hud", self.show_hud.to_string()),
            ("compact", self.compact.to_string()),
        ]
    }

    pub fn labels(&self) -> [String; 2] {
        [
            format!("HUD Display: {}", if self.show_hud { "ENABLED" } else { "DISABLED" }),
            format!("Compact mode: {}", if self.compact { "ON" } else { "OFF" }),
        ]
    }

    pub fn click(&mut self, button: usize) {
        match button {
            0 => self.show_hud = !self.show_hud,
            1 => self.compact = !self.compact,
            _ => {}
        }
    }
}

thread_local! {
    static SETTINGS: RefCell<Settings> = RefCell::new(Settings {
        show_hud: true,
        compact: false,
    });
}

pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    let s = Settings::read(get);
    SETTINGS.with(|st| *st.borrow_mut() = s);
    s
}

pub fn set_settings(s: Settings) {
    SETTINGS.with(|st| *st.borrow_mut() = s);
}

pub fn format_overlay(x: f64, y: f64, z: f64, fps: u32) -> String {
    SETTINGS.with(|st| {
        let s = st.borrow();
        if !s.show_hud {
            return String::new();
        }
        if s.compact {
            format!("{:.1} / {:.1} / {:.1} [{} fps]", x, y, z, fps)
        } else {
            format!("XYZ: {:.2}, {:.2}, {:.2} | FPS: {}", x, y, z, fps)
        }
    })
}

pub fn handle_command(cmd: &str) -> bool {
    if cmd == "jei" || cmd == "jei toggle" {
        SETTINGS.with(|st| {
            let mut s = st.borrow_mut();
            s.show_hud = !s.show_hud;
        });
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hud_formatting() {
        let text = format_overlay(100.5, 64.0, -200.2, 120);
        assert!(text.contains("XYZ: 100.50, 64.00, -200.20"));
        assert!(text.contains("FPS: 120"));
    }

    #[test]
    fn test_command() {
        assert!(handle_command("jei"));
        assert_eq!(format_overlay(0.0, 0.0, 0.0, 60), "");
    }
}
