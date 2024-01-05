use serde::{Serialize, Deserialize};

/// Configuration for program
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct PomdConfig {
    /// Length of work phases in seconds
    pub work_duration: f32,
    /// Length of short breaks in seconds
    pub short_break_duration: f32,
    /// Length of long breaks in seconds
    pub long_break_duration: f32,
    /// Number of iterations between long breaks
    pub num_iterations: u8,
    /// Whether to show system notifications
    pub notify: bool,
}

impl Default for PomdConfig {
    fn default() -> Self {
        Self {
            work_duration: 15.0 * 60.0,
            short_break_duration: 5.0 * 60.0,
            long_break_duration: 25.0 * 60.0,
            num_iterations: 4,
            notify: true,
        }
    }
}
