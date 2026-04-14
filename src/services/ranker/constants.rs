pub const SCALE: f64 = 100.0;
pub const TAG_BOOST: i32 = 100;
pub const META_BOOST: i32 = 100;
pub const SLUG_BOOST: i32 = 50;

// Core behavior tuning
pub const RECENCY_TAU_DAYS: f64 = 1.0; // how fast recency decays
pub const RECENCY_WEIGHT: f64 = 1.0; // overall importance of recency
pub const FREQUENCY_WEIGHT: f64 = 1.0; // overall importance of frequency

// Frequency shaping
pub const FREQUENCY_OFFSET: f64 = 1.0; // prevents ln(0)
pub const FREQUENCY_DAMPENING: f64 = 1.0; // multiplier before ln
