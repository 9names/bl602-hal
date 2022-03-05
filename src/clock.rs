//! SoC clock configuration

// The clocking in this chip is split into several peripheral sections oriented around low power modes
//
// Here is a quick overview of the peripheral sections as relates to those modes
//
// The GLB (global register) portion of the chip controls most clock enable/division circuits
//   as well as the GPIO
// The AON (always on) section is parts of the SOC that are active in all but the deepest
//   hibernate mode (HBN3). This section controls power to external high frequency crystal
// The PDS (power-down state, sleep) is the smallest level of power saving.
//   It always keeps CORE SRAM and timer power enabled.
//   Power to CPU, Wireless PHY+MAC, and digital/analog pins is optionally turned off at different pre-set levels
//   Peripherals that relate to clocking in this module: PLL
// The HBN (hibernate, deep sleep) section is the largest level of power saving.
//   It always turns off CPU, Wireless PHY+MAC, CORE SRAM and timers, and optionally sections or all of AON
//   It contains the root clock source selection (sysclk/flck)
// The L1C (level 1 cache) section maps tightly-coupled ram/cache SRAM in front of slower buses
//   (ROM, flash). It contains configuration for internal ROM access latency
//
// Currently implemented clock tree configuration options:
//   - internal 32Mhz RC oscillator for sysclock
//   - XTAL driving PLL, sysclock frequencies of 48/80/120/160/192Mhz
//   - UART using PLL if sysclock is using PLL

use crate::gpio::ClkCfg;
use crate::pac;
use crate::rom;
use core::num::NonZeroU32;
use embedded_time::rate::{Extensions, Hertz};

/// Internal high-speed RC oscillator frequency
pub const RC32M: u32 = 32_000_000;
/// UART peripheral clock frequency when PLL selected
pub const UART_PLL_FREQ: u32 = 160_000_000;

#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum SysclkFreq {
    Rc32Mhz = 32_000_000,
    Pll48Mhz = 48_000_000,
    Pll120Mhz = 120_000_000,
    Pll160Mhz = 160_000_000,
}

/// Frozen clock frequencies
///
/// The existance of this value indicates that the clock configuration can no longer be changed
#[derive(Clone, Copy)]
pub struct Clocks {
    sysclk: Hertz,
    uart_clk: Hertz,
    spi_clk: Hertz,
    i2c_clk: Hertz,
    _xtal_freq: Option<Hertz>,
    pll_enable: bool,
}

impl Clocks {
    pub fn new() -> Self {
        Clocks {
            sysclk: Hertz(RC32M),
            uart_clk: Hertz(RC32M),
            spi_clk: Hertz(RC32M),
            i2c_clk: Hertz(RC32M),
            _xtal_freq: None,
            pll_enable: false,
        }
    }

    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }

    pub fn pll_enable(&self) -> bool {
        self.pll_enable
    }

    pub const fn uart_clk(&self) -> Hertz {
        self.uart_clk
    }

    pub const fn spi_clk(&self) -> Hertz {
        self.spi_clk
    }

    pub const fn i2c_clk(&self) -> Hertz {
        self.i2c_clk
    }
}

impl Default for Clocks {
    fn default() -> Self {
        Self::new()
    }
}

/// Strict clock configurator
///
/// This configurator only accepts strictly accurate value. If all available frequency
/// values after configurated does not strictly equal to the desired value, the `freeze`
/// function panics. Users must be careful to ensure that the output frequency values
/// can be strictly configurated into using input frequency values and internal clock
/// frequencies.
///
/// If you need to get most precise frequenct possible (other than the stictly accutare
/// value only), use configurator `Precise` instead.
///
/// For example if 49.60MHz and 50.20MHz are able to be configurated prefectly, input
/// 50MHz into `Strict` would result in a panic when performing `freeze`; however input
/// same 50MHz into `Precise` it would not panic, but would set and freeze into
/// 50.20MHz as the frequency error is smallest.
pub struct Strict {
    target_i2c_clk: Option<NonZeroU32>,
    target_spi_clk: Option<NonZeroU32>,
    target_uart_clk: Option<NonZeroU32>,
    pll_xtal_freq: Option<u32>,
    sysclk: SysclkFreq,
}

impl Strict {
    /// Create a strict configurator
    pub fn new() -> Self {
        Strict {
            target_i2c_clk: None,
            target_spi_clk: None,
            target_uart_clk: None,
            pll_xtal_freq: None,
            sysclk: SysclkFreq::Rc32Mhz,
        }
    }

    /// Sets the desired frequency for the I2C-CLK clock
    pub fn i2c_clk(mut self, freq: impl Into<Hertz>) -> Self {
        let freq_hz = freq.into().0;

        self.target_i2c_clk = NonZeroU32::new(freq_hz);

        self
    }

    /// Sets the desired frequency for the SPI-CLK clock
    pub fn spi_clk(mut self, freq: impl Into<Hertz>) -> Self {
        let freq_hz = freq.into().0;

        self.target_spi_clk = NonZeroU32::new(freq_hz);

        self
    }

    /// Sets the desired frequency for the UART-CLK clock
    pub fn uart_clk(mut self, freq: impl Into<Hertz>) -> Self {
        let freq_hz = freq.into().0;

        self.target_uart_clk = NonZeroU32::new(freq_hz);

        self
    }

    /// Enables PLL clock source, using external XTAL frequency provided
    pub fn use_pll(mut self, freq: impl Into<Hertz>) -> Self {
        self.pll_xtal_freq = Some(freq.into().0);

        self
    }

    /// Set the system clock frequency (fclk/hclk)
    ///
    /// Supported frequencies:
    ///   `32_000_000`, `48_000_000`, `80_000_000`, `120_000_000`, `160_000_000`
    pub fn sys_clk(mut self, freq: SysclkFreq) -> Self {
        self.sysclk = freq;

        self
    }

    /// Calculate and balance clock registers to configure into the given clock value.
    /// If accurate value is not possible, this function panics.
    ///
    /// Be aware that Rust's panic is sometimes not obvious on embedded devices; if your
    /// program didn't execute as expected, or the `pc` is pointing to somewhere weird
    /// (usually `abort: j abort`), it's likely that this function have panicked.
    /// Breakpoint on `rust_begin_unwind` may help debugging.
    ///
    /// # Panics
    ///
    /// If strictly accurate value of given `ck_sys` etc. is not reachable, this function
    /// panics.
    pub fn freeze(self, _clk_cfg: &mut ClkCfg) -> Clocks {
        // Default to not using the PLL, and selecting the internal RC oscillator if nothing selected
        let pll_xtal_freq = self.pll_xtal_freq.unwrap_or(0);
        let pll_enabled = pll_xtal_freq != 0;
        let sysclk = self.sysclk;

        // If sysclk isn't 32Mhz but PLL isn't enabled, panic
        assert!(pll_enabled || sysclk == SysclkFreq::Rc32Mhz);

        // If PLL is available we'll be using the PLL_160Mhz clock, otherwise sysclk
        let uart_clk_src = if pll_enabled {
            UART_PLL_FREQ
        } else {
            sysclk as u32
        };

        // UART config
        let uart_clk = self
            .target_uart_clk
            .map(|f| f.get())
            .unwrap_or(uart_clk_src as u32);

        let uart_clk_div = {
            let ans = uart_clk_src / uart_clk;

            if !(1..=7).contains(&ans) || ans * uart_clk != uart_clk_src {
                panic!("unreachable uart_clk")
            }

            ans as u8
        };

        // Enable system clock, PLL + crystal if required
        set_sys_clk(sysclk, pll_xtal_freq);

        // If PLL is enabled, use that for the UART base clock
        // Otherwise, use sysclk as the UART clock
        unsafe { &*pac::HBN::ptr() }
            .hbn_glb
            .modify(|_, w| w.hbn_uart_clk_sel().bit(pll_enabled));

        // Write UART clock divider
        unsafe { &*pac::GLB::ptr() }.clk_cfg2.modify(|_, w| unsafe {
            w.uart_clk_div()
                .bits(uart_clk_div - 1_u8)
                .uart_clk_en()
                .set_bit()
        });

        // SPI config
        let spi_clk = self
            .target_spi_clk
            .map(|f| f.get())
            .unwrap_or(32_000_000u32);

        // SPI Clock Divider (BUS_CLK/(N+1)), default BUS_CLK/4
        let bus_clock = calculate_bus_clock();
        let spi_clk_div = bus_clock.0 / spi_clk;

        if spi_clk_div == 0 || spi_clk_div > 0b100000 {
            panic!("Unreachable SPI_CLK");
        }

        let spi_clk_div = ((spi_clk_div - 1) & 0b11111) as u8;
        // Write SPI clock divider
        unsafe { &*pac::GLB::ptr() }
            .clk_cfg3
            .modify(|_, w| unsafe { w.spi_clk_en().set_bit().spi_clk_div().bits(spi_clk_div) });

        // I2C config
        let i2c_clk = self
            .target_i2c_clk
            .map(|f| f.get())
            .unwrap_or(32_000_000u32);

        // I2C Clock Divider (BUS_CLK/(N+1)), default BUS_CLK/255
        let i2c_clk_div = bus_clock.0 / i2c_clk;

        if i2c_clk_div == 0 || i2c_clk_div > 255 {
            panic!("Unreachable I2C_CLK");
        }

        let i2c_clk_div = ((i2c_clk_div - 1) & 0xff) as u8;
        // Write I2C clock divider
        unsafe { &*pac::GLB::ptr() }
            .clk_cfg3
            .modify(|_, w| unsafe { w.i2c_clk_en().set_bit().i2c_clk_div().bits(i2c_clk_div) });

        Clocks {
            sysclk: Hertz(sysclk as u32),
            uart_clk: Hertz(uart_clk),
            spi_clk: Hertz(spi_clk),
            i2c_clk: Hertz(i2c_clk),
            _xtal_freq: Some(Hertz(pll_xtal_freq)),
            pll_enable: pll_enabled,
        }
    }
}

impl Default for Strict {
    fn default() -> Self {
        Self::new()
    }
}

/// Gets the current bus clock rate
fn calculate_bus_clock() -> Hertz {
    let root_clk_sel = unsafe { &*pac::GLB::ptr() }
        .clk_cfg0
        .read()
        .hbn_root_clk_sel()
        .bits();
    let pll_clk_sel = unsafe { &*pac::GLB::ptr() }
        .clk_cfg0
        .read()
        .reg_pll_sel()
        .bits();
    let hclk_div = unsafe { &*pac::GLB::ptr() }
        .clk_cfg0
        .read()
        .reg_hclk_div()
        .bits();
    let bclk_div = unsafe { &*pac::GLB::ptr() }
        .clk_cfg0
        .read()
        .reg_bclk_div()
        .bits();

    let root = match root_clk_sel {
        0 => 32_000_000_u32.Hz(),
        1 => 40_000_000_u32.Hz(),
        _ => match pll_clk_sel {
            0 => 48_000_000_u32.Hz(),
            1 => 120_000_000_u32.Hz(),
            2 => 160_000_000_u32.Hz(),
            _ => 192_000_000_u32.Hz(),
        },
    };

    root / (hclk_div as u32 + 1) / (bclk_div as u32 + 1)
}

/// Gets the system clock in the (undocumented) system_core_clock register
pub fn system_core_clock_get() -> u32 {
    unsafe { &*pac::HBN::ptr() }.hbn_rsv2.read().bits()
}

fn set_sys_clk(sysclk: SysclkFreq, pll_xtal_freq: u32) {
    use rom::clock::types::{GLB_PLL_XTAL_Type, GLB_SYS_CLK_Type};
    let sysclk = match sysclk {
        SysclkFreq::Rc32Mhz => GLB_SYS_CLK_Type::RC32M,
        SysclkFreq::Pll48Mhz => GLB_SYS_CLK_Type::PLL48M,
        SysclkFreq::Pll120Mhz => GLB_SYS_CLK_Type::PLL120M,
        SysclkFreq::Pll160Mhz => GLB_SYS_CLK_Type::PLL160M,
    };
    let xtal = match pll_xtal_freq {
        0 => GLB_PLL_XTAL_Type::None,
        24_000_000 => GLB_PLL_XTAL_Type::XTAL_24M,
        32_000_000 => GLB_PLL_XTAL_Type::XTAL_32M,
        38_400_000 => GLB_PLL_XTAL_Type::XTAL_38P4M,
        40_000_000 => GLB_PLL_XTAL_Type::XTAL_40M,
        26_000_000 => GLB_PLL_XTAL_Type::XTAL_26M,
        _ => panic!("Invalid PLL setup"),
    };
    rom::clock::GLB_Set_System_CLK(xtal, sysclk);
}
