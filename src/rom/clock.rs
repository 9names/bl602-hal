// BL_Err_Type GLB_Set_System_CLK(GLB_PLL_XTAL_Type xtalType,GLB_SYS_CLK_Type clkFreq);

/**
 *  @brief PLL XTAL type definition
 */
/*
typedef enum {
    GLB_PLL_XTAL_NONE,                      /*!< XTAL is none */
    GLB_PLL_XTAL_24M,                       /*!< XTAL is 24M */
    GLB_PLL_XTAL_32M,                       /*!< XTAL is 32M */
    GLB_PLL_XTAL_38P4M,                     /*!< XTAL is 38.4M */
    GLB_PLL_XTAL_40M,                       /*!< XTAL is 40M */
    GLB_PLL_XTAL_26M,                       /*!< XTAL is 26M */
    GLB_PLL_XTAL_RC32M,                     /*!< XTAL is RC32M */
}GLB_PLL_XTAL_Type;

typedef enum {
    GLB_PLL_CLK_480M,                       /*!< PLL output clock:480M */
    GLB_PLL_CLK_240M,                       /*!< PLL output clock:240M */
    GLB_PLL_CLK_192M,                       /*!< PLL output clock:192M */
    GLB_PLL_CLK_160M,                       /*!< PLL output clock:160M */
    GLB_PLL_CLK_120M,                       /*!< PLL output clock:120M */
    GLB_PLL_CLK_96M,                        /*!< PLL output clock:96M */
    GLB_PLL_CLK_80M,                        /*!< PLL output clock:80M */
    GLB_PLL_CLK_48M,                        /*!< PLL output clock:48M */
    GLB_PLL_CLK_32M,                        /*!< PLL output clock:32M */
}GLB_PLL_CLK_Type;

/**
 *  @brief GLB system clock type definition
 */
typedef enum {
    GLB_SYS_CLK_RC32M,                      /*!< use RC32M as system clock frequency */
    GLB_SYS_CLK_XTAL,                       /*!< use XTAL as system clock */
    GLB_SYS_CLK_PLL48M,                     /*!< use PLL output 48M as system clock */
    GLB_SYS_CLK_PLL120M,                    /*!< use PLL output 120M as system clock */
    GLB_SYS_CLK_PLL160M,                    /*!< use PLL output 160M as system clock */
    GLB_SYS_CLK_PLL192M,                    /*!< use PLL output 192M as system clock */
}GLB_SYS_CLK_Type;

*/

pub mod types {
    pub enum GLB_PLL_XTAL_Type {
        None = 0,
        XTAL_24M = 1,
        XTAL_32M = 2,
        XTAL_38P4M = 3,
        XTAL_40M = 4,
        XTAL_26M = 5,
        RC32M = 6,
    }

    pub enum GLB_PLL_CLK_Type {
        GLB_PLL_CLK_480M = 0,
        GLB_PLL_CLK_240M = 1,
        GLB_PLL_CLK_192M = 2,
        GLB_PLL_CLK_160M = 3,
        GLB_PLL_CLK_120M = 4,
        GLB_PLL_CLK_96M = 5,
        GLB_PLL_CLK_80M = 6,
        GLB_PLL_CLK_48M = 7,
        GLB_PLL_CLK_32M = 8,
    }

    pub enum GLB_SYS_CLK_Type {
        RC32M,
        XTAL,
        PLL48M,
        PLL120M,
        PLL160M,
        PLL192M,
    }
}

use super::{rom_lookup::RomIndex, BL_Err_Type};
use crate::rom::rom_lookup::rom_lookup;
use types::*;
#[inline(always)]
pub fn GLB_Set_System_CLK(xtalType: GLB_PLL_XTAL_Type, clkFreq: GLB_SYS_CLK_Type) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(xtalType: u32, clkFreq: u32) -> BL_Err_Type>(
            rom_lookup(RomIndex::GLB_Set_System_CLK),
        )
    };
    romdriver_func(xtalType as u32, clkFreq as u32)
}
