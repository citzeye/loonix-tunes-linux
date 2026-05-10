/* --- loonixtunesv2/src/audio/samplerate.rs | Sample Rate Manager --- */
/* Professional-grade sample rate management for real-time audio engine  */

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;

/// Sanity constants for common sample rates
/// These are the only rates that won't cause DSP filters to produce NaN
pub const SAMPLE_RATE_22050: u32 = 22050;
pub const SAMPLE_RATE_32000: u32 = 32000;
pub const SAMPLE_RATE_44100: u32 = 44100;
pub const SAMPLE_RATE_48000: u32 = 48000;
pub const SAMPLE_RATE_88200: u32 = 88200;
pub const SAMPLE_RATE_96000: u32 = 96000;
pub const SAMPLE_RATE_176400: u32 = 176400;
pub const SAMPLE_RATE_192000: u32 = 192000;

/// Minimum valid sample rate (CD quality minimum)
pub const MIN_SAMPLE_RATE: u32 = 22050;

/// Maximum valid sample rate (high-res audio limit)
pub const MAX_SAMPLE_RATE: u32 = 192000;

/// Default sample rate if device query fails
pub const DEFAULT_SAMPLE_RATE: u32 = 48000;

/// The global sample rate manager instance
static SAMPLE_RATE_MANAGER: OnceLock<SampleRateManager> = OnceLock::new();

/// Dirty flag to notify DSP chain of sample rate changes
/// DSP processors check this flag to avoid recalculating coefficients in every audio frame
static RATE_CHANGED: AtomicU32 = AtomicU32::new(0);

/// Sample Rate Manager - Single Source of Truth (SSOT) for sample rate
struct SampleRateManager {
    rate: AtomicU32,
}

impl SampleRateManager {
    /// Create a new SampleRateManager with default rate
    fn new() -> Self {
        Self {
            rate: AtomicU32::new(DEFAULT_SAMPLE_RATE),
        }
    }
}

/// Initialize the global sample rate manager
/// Must be called once at startup (before audio threads start)
pub fn init_sample_rate() {
    SAMPLE_RATE_MANAGER.get_or_init(SampleRateManager::new);
}

/// Validate and clamp sample rate to valid range
/// Returns a sane sample rate, never returns 0 or invalid values
#[inline(always)]
pub fn validate_sample_rate(rate: u32) -> u32 {
    if rate < MIN_SAMPLE_RATE {
        return DEFAULT_SAMPLE_RATE;
    }
    if rate > MAX_SAMPLE_RATE {
        return DEFAULT_SAMPLE_RATE;
    }
    // Check if it's a common rate (optional strict mode)
    match rate {
        SAMPLE_RATE_22050
        | SAMPLE_RATE_32000
        | SAMPLE_RATE_44100
        | SAMPLE_RATE_48000
        | SAMPLE_RATE_88200
        | SAMPLE_RATE_96000
        | SAMPLE_RATE_176400
        | SAMPLE_RATE_192000 => rate,
        _ => {
            // For non-standard rates, still accept if in valid range
            // but log a warning in debug builds
            #[cfg(debug_assertions)]
            eprintln!(
                "[SampleRate] Non-standard sample rate: {} Hz, using anyway",
                rate
            );
            rate
        }
    }
}

/// Set the global sample rate
/// Updates the atomic value and sets the dirty flag for DSP notification
/// This is wait-free (no locks), safe to call from any thread
#[inline]
pub fn set_rate(new_rate: u32) {
    let validated = validate_sample_rate(new_rate);
    if let Some(manager) = SAMPLE_RATE_MANAGER.get() {
        let old_rate = manager.rate.load(Ordering::Relaxed);
        if old_rate != validated {
            manager.rate.store(validated, Ordering::SeqCst);
            // Set dirty flag - DSP chain will detect this
            RATE_CHANGED.store(1, Ordering::SeqCst);
        }
    }
}

/// Get the current sample rate as f32
/// Returns f32 because DSP math (sin, cos, tan) requires floating point
/// This is wait-free (no locks), safe to call from audio callback thread
/// Marked #[inline(always)] for zero-cost abstraction
#[inline(always)]
pub fn get_rate() -> f32 {
    if let Some(manager) = SAMPLE_RATE_MANAGER.get() {
        manager.rate.load(Ordering::Relaxed) as f32
    } else {
        DEFAULT_SAMPLE_RATE as f32
    }
}

/// Get the current sample rate as u32
/// Use this for integer calculations (buffer sizes, etc.)
#[inline(always)]
pub fn get_rate_u32() -> u32 {
    if let Some(manager) = SAMPLE_RATE_MANAGER.get() {
        manager.rate.load(Ordering::Relaxed)
    } else {
        DEFAULT_SAMPLE_RATE
    }
}

/// Check and consume the dirty flag
/// Returns true if sample rate changed since last check
/// DSP processors call this at the start of processing to know if they need
/// to recalculate filter coefficients (sin/cos/tan are expensive!)
/// This is wait-free (no locks)
#[inline(always)]
pub fn consume_rate_changed() -> bool {
    let changed = RATE_CHANGED.load(Ordering::SeqCst);
    if changed != 0 {
        RATE_CHANGED.store(0, Ordering::SeqCst);
        true
    } else {
        false
    }
}

/// Set the dirty flag manually (rarely needed, use set_rate instead)
#[inline(always)]
pub fn mark_rate_changed() {
    RATE_CHANGED.store(1, Ordering::SeqCst);
}

/// Get the raw atomic reference for advanced use cases
/// Most code should use get_rate() and set_rate() instead
pub fn get_rate_atomic() -> Option<&'static AtomicU32> {
    SAMPLE_RATE_MANAGER
        .get()
        .map(|manager| &manager.rate)
}

/// Convenience function: Check if rate changed, if so, update DSP chain
/// Returns the current rate (as f32) for immediate use
/// This is the function DSP processors should call in their process() method:
/// ```rust,ignore
/// let rate = check_and_get_rate();
/// if rate > 0.0 {
///     // use rate for DSP math
/// }
/// ```
#[inline(always)]
pub fn check_and_get_rate() -> f32 {
    let _ = consume_rate_changed(); // Clear flag if set
    get_rate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_sample_rate() {
        assert_eq!(validate_sample_rate(0), DEFAULT_SAMPLE_RATE);
        assert_eq!(validate_sample_rate(1000), DEFAULT_SAMPLE_RATE);
        assert_eq!(validate_sample_rate(44100), 44100);
        assert_eq!(validate_sample_rate(48000), 48000);
        assert_eq!(validate_sample_rate(192000), 192000);
        assert_eq!(validate_sample_rate(200000), DEFAULT_SAMPLE_RATE);
    }

    #[test]
    fn test_set_and_get_rate() {
        init_sample_rate();
        set_rate(44100);
        assert_eq!(get_rate_u32(), 44100);
        assert!((get_rate() - 44100.0).abs() < 0.01);
    }
}
