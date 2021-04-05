//! Delays

use core::convert::Infallible;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

/// Use RISCV machine-mode cycle counter (`mcycle`) as a delay provider.
///
/// This can be used for high resolution delays for device initialization,
/// bit-banging protocols, etc
#[derive(Copy, Clone)]
pub struct McycleDelay {
    core_frequency: u32,
}

impl McycleDelay {
    /// Constructs the delay provider based on core clock frequency `freq`
    pub fn new(freq: u32) -> Self {
        Self {
            /// System clock frequency, used to convert clock cycles
            /// into real-world time values
            core_frequency: freq,
        }
    }

    /// Retrieves the cycle count for the current HART
    #[inline]
    pub fn get_cycle_count() -> u64 {
        riscv::register::mcycle::read64()
    }

    /// Returns the number of elapsed cycles since `previous_cycle_count`
    #[inline]
    pub fn cycles_since(previous_cycle_count: u64) -> u64 {
        riscv::register::mcycle::read64().wrapping_sub(previous_cycle_count)
    }

    /// Performs a busy-wait loop until the number of cycles `cycle_count` has elapsed
    #[inline]
    pub fn delay_cycles(cycle_count: u64) {
        let start_cycle_count = McycleDelay::get_cycle_count();

        while McycleDelay::cycles_since(start_cycle_count) <= cycle_count {}
    }
}

impl DelayUs<i8> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of microseconds `us` has elapsed
    #[inline]
    fn try_delay_us(&mut self, us: i8) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((us as u64 * (self.core_frequency as u64)) / 1_000_000);

        Ok(())
    }
}

impl DelayMs<i8> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of milliseconds `ms` has elapsed
    #[inline]
    fn try_delay_ms(&mut self, ms: i8) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((ms as u64* (self.core_frequency as u64)) / 1000);

        Ok(())
    }
}

impl DelayUs<i16> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of microseconds `us` has elapsed
    #[inline]
    fn try_delay_us(&mut self, us: i16) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((us as u64 * (self.core_frequency as u64)) / 1_000_000);

        Ok(())
    }
}

impl DelayMs<i16> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of milliseconds `ms` has elapsed
    #[inline]
    fn try_delay_ms(&mut self, ms: i16) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((ms as u64* (self.core_frequency as u64)) / 1000);

        Ok(())
    }
}

impl DelayUs<i32> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of microseconds `us` has elapsed
    #[inline]
    fn try_delay_us(&mut self, us: i32) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((us as u64 * (self.core_frequency as u64)) / 1_000_000);

        Ok(())
    }
}

impl DelayMs<i32> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of milliseconds `ms` has elapsed
    #[inline]
    fn try_delay_ms(&mut self, ms: i32) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((ms as u64* (self.core_frequency as u64)) / 1000);

        Ok(())
    }
}

impl DelayUs<i64> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of microseconds `us` has elapsed
    #[inline]
    fn try_delay_us(&mut self, us: i64) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((us as u64 * (self.core_frequency as u64)) / 1_000_000);

        Ok(())
    }
}

impl DelayMs<i64> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of milliseconds `ms` has elapsed
    #[inline]
    fn try_delay_ms(&mut self, ms: i64) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((ms as u64 * (self.core_frequency as u64)) / 1000);

        Ok(())
    }
}


impl DelayUs<u8> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of microseconds `us` has elapsed
    #[inline]
    fn try_delay_us(&mut self, us: u8) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((us as u64 * (self.core_frequency as u64)) / 1_000_000);

        Ok(())
    }
}

impl DelayMs<u8> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of milliseconds `ms` has elapsed
    #[inline]
    fn try_delay_ms(&mut self, ms: u8) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((ms as u64* (self.core_frequency as u64)) / 1000);

        Ok(())
    }
}

impl DelayUs<u16> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of microseconds `us` has elapsed
    #[inline]
    fn try_delay_us(&mut self, us: u16) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((us as u64 * (self.core_frequency as u64)) / 1_000_000);

        Ok(())
    }
}

impl DelayMs<u16> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of milliseconds `ms` has elapsed
    #[inline]
    fn try_delay_ms(&mut self, ms: u16) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((ms as u64* (self.core_frequency as u64)) / 1000);

        Ok(())
    }
}

impl DelayUs<u32> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of microseconds `us` has elapsed
    #[inline]
    fn try_delay_us(&mut self, us: u32) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((us as u64 * (self.core_frequency as u64)) / 1_000_000);

        Ok(())
    }
}

impl DelayMs<u32> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of milliseconds `ms` has elapsed
    #[inline]
    fn try_delay_ms(&mut self, ms: u32) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((ms as u64* (self.core_frequency as u64)) / 1000);

        Ok(())
    }
}

impl DelayUs<u64> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of microseconds `us` has elapsed
    #[inline]
    fn try_delay_us(&mut self, us: u64) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((us * (self.core_frequency as u64)) / 1_000_000);

        Ok(())
    }
}

impl DelayMs<u64> for McycleDelay {
    type Error = Infallible;

    /// Performs a busy-wait loop until the number of milliseconds `ms` has elapsed
    #[inline]
    fn try_delay_ms(&mut self, ms: u64) -> Result<(), Infallible> {
        McycleDelay::delay_cycles((ms * (self.core_frequency as u64)) / 1000);

        Ok(())
    }
}


