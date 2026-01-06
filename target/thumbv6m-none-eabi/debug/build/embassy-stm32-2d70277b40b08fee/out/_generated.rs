embassy_hal_internal::peripherals_definition!(
    PA0,
    PA1,
    PA2,
    PA3,
    PA4,
    PA5,
    PA6,
    PA7,
    PA8,
    PA9,
    PA10,
    PA11,
    PA12,
    PA13,
    PA14,
    PA15,
    PB0,
    PB1,
    PB2,
    PB3,
    PB4,
    PB5,
    PB6,
    PB7,
    PB8,
    PB9,
    PB10,
    PB11,
    PB12,
    PB13,
    PB14,
    PB15,
    PC0,
    PC1,
    PC2,
    PC3,
    PC4,
    PC5,
    PC6,
    PC7,
    PC8,
    PC9,
    PC10,
    PC11,
    PC12,
    PC13,
    PC14,
    PC15,
    PD0,
    PD1,
    PD2,
    PD3,
    PD4,
    PD5,
    PD6,
    PD8,
    PD9,
    PF0,
    PF1,
    PF2,
    ADC1,
    ADC1_COMMON,
    CEC,
    COMP1,
    COMP2,
    COMP3,
    CRC,
    CRS,
    DAC1,
    DBGMCU,
    DMA1,
    DMA2,
    DMAMUX1,
    FDCAN1,
    FDCAN2,
    FDCANRAM1,
    FDCANRAM2,
    FLASH,
    I2C1,
    I2C2,
    I2C3,
    IWDG,
    LPTIM1,
    LPTIM2,
    LPUART1,
    LPUART2,
    PWR,
    MCO2,
    MCO1,
    RCC,
    RTC,
    SPI1,
    SPI2,
    SPI3,
    SYSCFG,
    TAMP,
    TIM1,
    TIM14,
    TIM15,
    TIM16,
    TIM17,
    TIM2,
    TIM3,
    TIM4,
    TIM6,
    TIM7,
    UCPD1,
    UCPD2,
    UID,
    USART1,
    USART2,
    USART3,
    USART4,
    USART5,
    USART6,
    USB,
    USBRAM,
    VREFBUF,
    WWDG,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    EXTI5,
    EXTI6,
    EXTI7,
    EXTI8,
    EXTI9,
    EXTI10,
    EXTI11,
    EXTI12,
    EXTI13,
    EXTI14,
    EXTI15,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    DMA2_CH1,
    DMA2_CH2,
    DMA2_CH3,
    DMA2_CH4,
    DMA2_CH5
);
embassy_hal_internal::peripherals_struct!(
    PA0,
    PA1,
    PA2,
    PA3,
    PA4,
    PA5,
    PA6,
    PA7,
    PA8,
    PA9,
    PA10,
    PA11,
    PA12,
    PA13,
    PA14,
    PA15,
    PB0,
    PB1,
    PB2,
    PB3,
    PB4,
    PB5,
    PB6,
    PB7,
    PB8,
    PB9,
    PB10,
    PB11,
    PB12,
    PB13,
    PB14,
    PB15,
    PC0,
    PC1,
    PC2,
    PC3,
    PC4,
    PC5,
    PC6,
    PC7,
    PC8,
    PC9,
    PC10,
    PC11,
    PC12,
    PC13,
    PC14,
    PC15,
    PD0,
    PD1,
    PD2,
    PD3,
    PD4,
    PD5,
    PD6,
    PD8,
    PD9,
    PF0,
    PF1,
    PF2,
    ADC1,
    ADC1_COMMON,
    CEC,
    COMP1,
    COMP2,
    COMP3,
    CRC,
    CRS,
    DAC1,
    DBGMCU,
    DMA1,
    DMA2,
    DMAMUX1,
    FDCAN1,
    FDCAN2,
    FDCANRAM1,
    FDCANRAM2,
    FLASH,
    I2C1,
    I2C2,
    I2C3,
    IWDG,
    LPTIM1,
    LPTIM2,
    LPUART1,
    LPUART2,
    PWR,
    MCO2,
    MCO1,
    RCC,
    RTC,
    SPI1,
    SPI2,
    SPI3,
    SYSCFG,
    TAMP,
    TIM1,
    TIM14,
    TIM16,
    TIM17,
    TIM2,
    TIM3,
    TIM4,
    TIM6,
    TIM7,
    UCPD1,
    UCPD2,
    UID,
    USART1,
    USART2,
    USART3,
    USART4,
    USART5,
    USART6,
    USB,
    USBRAM,
    VREFBUF,
    WWDG,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    EXTI5,
    EXTI6,
    EXTI7,
    EXTI8,
    EXTI9,
    EXTI10,
    EXTI11,
    EXTI12,
    EXTI13,
    EXTI14,
    EXTI15,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    DMA2_CH1,
    DMA2_CH2,
    DMA2_CH3,
    DMA2_CH4,
    DMA2_CH5
);
embassy_hal_internal::interrupt_mod!(
    WWDG,
    PVD_VDDIO2,
    RTC_TAMP,
    FLASH,
    RCC_CRS,
    EXTI0_1,
    EXTI2_3,
    EXTI4_15,
    USB_UCPD1_2,
    DMA1_CHANNEL1,
    DMA1_CHANNEL2_3,
    DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR,
    ADC1_COMP,
    TIM1_BRK_UP_TRG_COM,
    TIM1_CC,
    TIM2,
    TIM3_TIM4,
    TIM6_DAC_LPTIM1,
    TIM7_LPTIM2,
    TIM14,
    TIM15,
    TIM16_FDCAN_IT0,
    TIM17_FDCAN_IT1,
    I2C1,
    I2C2_3,
    SPI1,
    SPI2_3,
    USART1,
    USART2_LPUART2,
    USART3_4_5_6_LPUART1,
    CEC,
);
pub const MAX_ERASE_SIZE: usize = 2048u32 as usize;
pub mod flash_regions {
    pub const BANK1_REGION: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank1,
        base: 134217728u32,
        size: 262144u32,
        erase_size: 2048u32,
        write_size: 8u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank1Region<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    pub const BANK2_REGION: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank2,
        base: 134479872u32,
        size: 262144u32,
        erase_size: 2048u32,
        write_size: 8u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank2Region<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    #[cfg(flash)]
    pub struct FlashLayout<'d, MODE = crate::flash::Async> {
        pub bank1_region: Bank1Region<'d, MODE>,
        pub bank2_region: Bank2Region<'d, MODE>,
        _mode: core::marker::PhantomData<MODE>,
    }
    #[cfg(flash)]
    impl<'d, MODE> FlashLayout<'d, MODE> {
        pub(crate) fn new(p: embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>) -> Self {
            Self {
                bank1_region: Bank1Region(
                    &BANK1_REGION,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                bank2_region: Bank2Region(
                    &BANK2_REGION,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                _mode: core::marker::PhantomData,
            }
        }
    }
    pub const FLASH_REGIONS: [&crate::flash::FlashRegion; 2usize] = [&BANK1_REGION, &BANK2_REGION];
}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().adcsel() {
            crate::pac::rcc::vals::Adcsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "sys")
            },
            crate::pac::rcc::vals::Adcsel::PLL1_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "pll1_p")
            },
            crate::pac::rcc::vals::Adcsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "hsi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "ADC1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((12u8, 20u8)),
            (16u8, 20u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::CEC {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().cecsel() {
            crate::pac::rcc::vals::Cecsel::HSI_DIV_488 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CEC" , "hsi") / 488u32
            },
            crate::pac::rcc::vals::Cecsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CEC" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "CEC"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CEC" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 24u8)),
            (15u8, 24u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CEC {}
impl crate::rcc::SealedRccPeripheral for peripherals::CRC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRC" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRC" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 12u8)),
            (14u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CRC {}
impl crate::rcc::SealedRccPeripheral for peripherals::CRS {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRS" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRS" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 16u8)),
            (15u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CRS {}
impl crate::rcc::SealedRccPeripheral for peripherals::DAC1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DAC1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DAC1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 29u8)),
            (15u8, 29u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DAC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA1" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA1" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 0u8)),
            (14u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA2" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA2" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 1u8)),
            (14u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::FDCAN1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr2().read().fdcansel() {
            crate::pac::rcc::vals::Fdcansel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN1" , "pclk1")
            },
            crate::pac::rcc::vals::Fdcansel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN1" , "pll1_q")
            },
            crate::pac::rcc::vals::Fdcansel::HSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN1" , "hse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "FDCAN1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 12u8)),
            (15u8, 12u8),
            Some(0u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::FDCAN1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::FDCAN2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr2().read().fdcansel() {
            crate::pac::rcc::vals::Fdcansel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN2" , "pclk1")
            },
            crate::pac::rcc::vals::Fdcansel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN2" , "pll1_q")
            },
            crate::pac::rcc::vals::Fdcansel::HSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN2" , "hse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "FDCAN2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 12u8)),
            (15u8, 12u8),
            Some(0u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::FDCAN2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::FLASH {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FLASH" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FLASH" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((10u8, 8u8)),
            (14u8, 8u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::FLASH {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().i2c1sel() {
            crate::pac::rcc::vals::I2c1sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "pclk1")
            },
            crate::pac::rcc::vals::I2c1sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "sys")
            },
            crate::pac::rcc::vals::I2c1sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "hsi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 21u8)),
            (15u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 22u8)),
            (15u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 23u8)),
            (15u8, 23u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPTIM1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().lptim1sel() {
            crate::pac::rcc::vals::Lptim1sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "pclk1")
            },
            crate::pac::rcc::vals::Lptim1sel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "lsi")
            },
            crate::pac::rcc::vals::Lptim1sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "hsi")
            },
            crate::pac::rcc::vals::Lptim1sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "LPTIM1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 31u8)),
            (15u8, 31u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPTIM1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPTIM2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().lptim2sel() {
            crate::pac::rcc::vals::Lptim2sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "pclk1")
            },
            crate::pac::rcc::vals::Lptim2sel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "lsi")
            },
            crate::pac::rcc::vals::Lptim2sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "hsi")
            },
            crate::pac::rcc::vals::Lptim2sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "LPTIM2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 30u8)),
            (15u8, 30u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPTIM2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPUART1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().lpuart1sel() {
            crate::pac::rcc::vals::Lpuart1sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "pclk1")
            },
            crate::pac::rcc::vals::Lpuart1sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "sys")
            },
            crate::pac::rcc::vals::Lpuart1sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "hsi")
            },
            crate::pac::rcc::vals::Lpuart1sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "LPUART1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 20u8)),
            (15u8, 20u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPUART1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPUART2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().lpuart2sel() {
            crate::pac::rcc::vals::Lpuart2sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART2" , "pclk1")
            },
            crate::pac::rcc::vals::Lpuart2sel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART2" , "sys")
            },
            crate::pac::rcc::vals::Lpuart2sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART2" , "hsi")
            },
            crate::pac::rcc::vals::Lpuart2sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART2" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "LPUART2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 7u8)),
            (15u8, 7u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPUART2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::PWR {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "PWR" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "PWR" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 28u8)),
            (15u8, 28u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::PWR {}
impl crate::rcc::SealedRccPeripheral for peripherals::RTC {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.bdcr().read().rtcsel() {
            crate::pac::rcc::vals::Rtcsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "lse")
            },
            crate::pac::rcc::vals::Rtcsel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "lsi")
            },
            crate::pac::rcc::vals::Rtcsel::HSE_DIV_32 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "hse") / 32u32
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "RTC"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (15u8, 10u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Standby,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::RTC {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((12u8, 12u8)),
            (16u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 14u8)),
            (15u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 15u8)),
            (15u8, 15u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SYSCFG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SYSCFG" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SYSCFG" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((12u8, 0u8)),
            (16u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SYSCFG {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().tim1sel() {
            crate::pac::rcc::vals::Tim1sel::PCLK1_TIM => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "pclk1_tim")
            },
            crate::pac::rcc::vals::Tim1sel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "pll1_q")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "TIM1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((12u8, 11u8)),
            (16u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM14 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM14" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM14" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((12u8, 15u8)),
            (16u8, 15u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM14 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM15 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().tim15sel() {
            crate::pac::rcc::vals::Tim15sel::PCLK1_TIM => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM15" , "pclk1_tim")
            },
            crate::pac::rcc::vals::Tim15sel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM15" , "pll1_q")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "TIM15"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM15" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((12u8, 16u8)),
            (16u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM15 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM16 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM16" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM16" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((12u8, 17u8)),
            (16u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM16 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM17 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM17" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM17" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((12u8, 18u8)),
            (16u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM17 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM2" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 0u8)),
            (15u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM3" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 1u8)),
            (15u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM4 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM4" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM4" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 2u8)),
            (15u8, 2u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM6 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM6" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM6" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 4u8)),
            (15u8, 4u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM6 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM7 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM7" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM7" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 5u8)),
            (15u8, 5u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM7 {}
impl crate::rcc::SealedRccPeripheral for peripherals::UCPD1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UCPD1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UCPD1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 25u8)),
            (15u8, 25u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UCPD1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::UCPD2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UCPD2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UCPD2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 26u8)),
            (15u8, 26u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UCPD2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().usart1sel() {
            crate::pac::rcc::vals::Usartsel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "pclk1")
            },
            crate::pac::rcc::vals::Usartsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "sys")
            },
            crate::pac::rcc::vals::Usartsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "hsi")
            },
            crate::pac::rcc::vals::Usartsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((12u8, 14u8)),
            (16u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().usart2sel() {
            crate::pac::rcc::vals::Usartsel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "pclk1")
            },
            crate::pac::rcc::vals::Usartsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "sys")
            },
            crate::pac::rcc::vals::Usartsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "hsi")
            },
            crate::pac::rcc::vals::Usartsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 17u8)),
            (15u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART3 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr().read().usart3sel() {
            crate::pac::rcc::vals::Usartsel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "pclk1")
            },
            crate::pac::rcc::vals::Usartsel::SYS => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . sys . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "sys")
            },
            crate::pac::rcc::vals::Usartsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "hsi")
            },
            crate::pac::rcc::vals::Usartsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART3"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 18u8)),
            (15u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART4 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART4" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART4" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 19u8)),
            (15u8, 19u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART5 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART5" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART5" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 8u8)),
            (15u8, 8u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART5 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART6 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 9u8)),
            (15u8, 9u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART6 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USB {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.ccipr2().read().usbsel() {
            crate::pac::rcc::vals::Usbsel::HSI48 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi48 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB" , "hsi48")
            },
            crate::pac::rcc::vals::Usbsel::HSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB" , "hse")
            },
            crate::pac::rcc::vals::Usbsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB" , "pll1_q")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USB"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((11u8, 13u8)),
            (15u8, 13u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USB {}
impl crate::rcc::SealedRccPeripheral for peripherals::WWDG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "WWDG" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "WWDG" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (15u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::WWDG {}
pub(crate) static mut REFCOUNTS: [u8; 1usize] = [0u8];
pub mod mux {
    pub use crate::pac::rcc::vals::Adcsel;
    pub use crate::pac::rcc::vals::Cecsel;
    pub use crate::pac::rcc::vals::Fdcansel;
    pub use crate::pac::rcc::vals::I2c1sel;
    pub use crate::pac::rcc::vals::Lptim1sel;
    pub use crate::pac::rcc::vals::Lptim2sel;
    pub use crate::pac::rcc::vals::Lpuart1sel;
    pub use crate::pac::rcc::vals::Lpuart2sel;
    pub use crate::pac::rcc::vals::Rtcsel;
    pub use crate::pac::rcc::vals::Tim15sel;
    pub use crate::pac::rcc::vals::Tim1sel;
    pub use crate::pac::rcc::vals::Usartsel;
    pub use crate::pac::rcc::vals::Usbsel;
    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub struct ClockMux {
        pub rtcsel: Rtcsel,
        pub adcsel: Adcsel,
        pub cecsel: Cecsel,
        pub i2c1sel: I2c1sel,
        pub lptim1sel: Lptim1sel,
        pub lptim2sel: Lptim2sel,
        pub lpuart1sel: Lpuart1sel,
        pub lpuart2sel: Lpuart2sel,
        pub tim15sel: Tim15sel,
        pub tim1sel: Tim1sel,
        pub usart1sel: Usartsel,
        pub usart2sel: Usartsel,
        pub usart3sel: Usartsel,
        pub fdcansel: Fdcansel,
        pub usbsel: Usbsel,
    }
    impl ClockMux {
        pub(crate) const fn default() -> Self {
            unsafe { ::core::mem::zeroed() }
        }
    }
    impl Default for ClockMux {
        fn default() -> Self {
            Self::default()
        }
    }
    impl ClockMux {
        pub(crate) fn init(&self) {
            crate::pac::RCC.bdcr().modify(|w| {
                w.set_rtcsel(self.rtcsel);
            });
            crate::pac::RCC.ccipr().modify(|w| {
                w.set_adcsel(self.adcsel);
                w.set_cecsel(self.cecsel);
                w.set_i2c1sel(self.i2c1sel);
                w.set_lptim1sel(self.lptim1sel);
                w.set_lptim2sel(self.lptim2sel);
                w.set_lpuart1sel(self.lpuart1sel);
                w.set_lpuart2sel(self.lpuart2sel);
                w.set_tim15sel(self.tim15sel);
                w.set_tim1sel(self.tim1sel);
                w.set_usart1sel(self.usart1sel);
                w.set_usart2sel(self.usart2sel);
                w.set_usart3sel(self.usart3sel);
            });
            crate::pac::RCC.ccipr2().modify(|w| {
                w.set_fdcansel(self.fdcansel);
                w.set_usbsel(self.usbsel);
            });
        }
    }
}
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct Clocks {
    pub hclk1: crate::time::MaybeHertz,
    pub hse: crate::time::MaybeHertz,
    pub hsi: crate::time::MaybeHertz,
    pub hsi48: crate::time::MaybeHertz,
    pub lse: crate::time::MaybeHertz,
    pub lsi: crate::time::MaybeHertz,
    pub pclk1: crate::time::MaybeHertz,
    pub pclk1_tim: crate::time::MaybeHertz,
    pub pll1_p: crate::time::MaybeHertz,
    pub pll1_q: crate::time::MaybeHertz,
    pub rtc: crate::time::MaybeHertz,
    pub sys: crate::time::MaybeHertz,
}
pub unsafe fn init_dma() {}
pub unsafe fn init_bdma() {
    crate::pac::RCC.ahbenr().modify(|w| w.set_dma1en(true));
    crate::pac::RCC.ahbenr().modify(|w| w.set_dma2en(true));
}
pub unsafe fn init_dmamux() {}
pub unsafe fn init_gpdma() {}
pub unsafe fn init_gpio() {
    crate::pac::RCC.gpioenr().modify(|w| w.set_gpioaen(true));
    crate::pac::RCC.gpioenr().modify(|w| w.set_gpioben(true));
    crate::pac::RCC.gpioenr().modify(|w| w.set_gpiocen(true));
    crate::pac::RCC.gpioenr().modify(|w| w.set_gpioden(true));
    crate::pac::RCC.gpioenr().modify(|w| w.set_gpiofen(true));
}
impl_adc_pin!(ADC1, PA0, 0u8);
impl_adc_pin!(ADC1, PA1, 1u8);
impl_adc_pin!(ADC1, PA2, 2u8);
impl_adc_pin!(ADC1, PA3, 3u8);
impl_adc_pin!(ADC1, PA4, 4u8);
impl_adc_pin!(ADC1, PA5, 5u8);
impl_adc_pin!(ADC1, PA6, 6u8);
impl_adc_pin!(ADC1, PA7, 7u8);
impl_adc_pin!(ADC1, PB0, 8u8);
impl_adc_pin!(ADC1, PB1, 9u8);
impl_adc_pin!(ADC1, PB10, 11u8);
impl_adc_pin!(ADC1, PB11, 15u8);
impl_adc_pin!(ADC1, PB12, 16u8);
impl_adc_pin!(ADC1, PB2, 10u8);
impl_adc_pin!(ADC1, PC4, 17u8);
impl_adc_pin!(ADC1, PC5, 18u8);
pin_trait_impl!(crate::dac::DacPin<Ch1>, DAC1, PA4, 0u8);
pin_trait_impl!(crate::dac::DacPin<Ch2>, DAC1, PA5, 0u8);
pin_trait_impl!(crate::can::RxPin, FDCAN1, PA11, 3u8);
pin_trait_impl!(crate::can::TxPin, FDCAN1, PA12, 3u8);
pin_trait_impl!(crate::can::RxPin, FDCAN1, PB8, 3u8);
pin_trait_impl!(crate::can::TxPin, FDCAN1, PB9, 3u8);
pin_trait_impl!(crate::can::RxPin, FDCAN1, PC4, 3u8);
pin_trait_impl!(crate::can::TxPin, FDCAN1, PC5, 3u8);
pin_trait_impl!(crate::can::RxPin, FDCAN1, PD0, 3u8);
pin_trait_impl!(crate::can::TxPin, FDCAN1, PD1, 3u8);
pin_trait_impl!(crate::can::RxPin, FDCAN2, PB0, 3u8);
pin_trait_impl!(crate::can::TxPin, FDCAN2, PB1, 3u8);
pin_trait_impl!(crate::can::RxPin, FDCAN2, PB12, 3u8);
pin_trait_impl!(crate::can::TxPin, FDCAN2, PB13, 3u8);
pin_trait_impl!(crate::can::RxPin, FDCAN2, PB5, 3u8);
pin_trait_impl!(crate::can::TxPin, FDCAN2, PB6, 3u8);
pin_trait_impl!(crate::can::RxPin, FDCAN2, PC2, 3u8);
pin_trait_impl!(crate::can::TxPin, FDCAN2, PC3, 3u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C1, PA10, 6u8);
pin_trait_impl!(crate::i2c::SclPin, I2C1, PA9, 6u8);
pin_trait_impl!(crate::i2c::SclPin, I2C1, PB6, 6u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C1, PB7, 6u8);
pin_trait_impl!(crate::i2c::SclPin, I2C1, PB8, 6u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C1, PB9, 6u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C2, PA10, 8u8);
pin_trait_impl!(crate::i2c::SclPin, I2C2, PA11, 6u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C2, PA12, 6u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C2, PA6, 8u8);
pin_trait_impl!(crate::i2c::SclPin, I2C2, PA7, 8u8);
pin_trait_impl!(crate::i2c::SclPin, I2C2, PA9, 8u8);
pin_trait_impl!(crate::i2c::SclPin, I2C2, PB10, 6u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C2, PB11, 6u8);
pin_trait_impl!(crate::i2c::SclPin, I2C2, PB13, 6u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C2, PB14, 6u8);
pin_trait_impl!(crate::i2c::SclPin, I2C2, PB3, 8u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C2, PB4, 8u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C3, PA6, 9u8);
pin_trait_impl!(crate::i2c::SclPin, I2C3, PA7, 9u8);
pin_trait_impl!(crate::i2c::SclPin, I2C3, PB3, 6u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C3, PB4, 6u8);
pin_trait_impl!(crate::i2c::SclPin, I2C3, PC0, 6u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C3, PC1, 6u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM1, PA0, 5u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM1, PB0, 5u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM1, PB2, 5u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM1, PC1, 0u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM1, PD8, 2u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM2, PA4, 5u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM2, PA8, 5u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM2, PD6, 2u8);
pin_trait_impl!(crate::usart::TxPin, LPUART1, PA2, 6u8);
pin_trait_impl!(crate::usart::RxPin, LPUART1, PA3, 6u8);
pin_trait_impl!(crate::usart::CtsPin, LPUART1, PA6, 6u8);
pin_trait_impl!(crate::usart::DePin, LPUART1, PB1, 6u8);
pin_trait_impl!(crate::usart::RtsPin, LPUART1, PB1, 6u8);
pin_trait_impl!(crate::usart::RxPin, LPUART1, PB10, 1u8);
pin_trait_impl!(crate::usart::TxPin, LPUART1, PB11, 1u8);
pin_trait_impl!(crate::usart::DePin, LPUART1, PB12, 1u8);
pin_trait_impl!(crate::usart::RtsPin, LPUART1, PB12, 1u8);
pin_trait_impl!(crate::usart::CtsPin, LPUART1, PB13, 1u8);
pin_trait_impl!(crate::usart::RxPin, LPUART1, PC0, 1u8);
pin_trait_impl!(crate::usart::TxPin, LPUART1, PC1, 1u8);
pin_trait_impl!(crate::usart::RxPin, LPUART2, PA13, 10u8);
pin_trait_impl!(crate::usart::TxPin, LPUART2, PA14, 10u8);
pin_trait_impl!(crate::usart::CtsPin, LPUART2, PB0, 10u8);
pin_trait_impl!(crate::usart::DePin, LPUART2, PB1, 10u8);
pin_trait_impl!(crate::usart::RtsPin, LPUART2, PB1, 10u8);
pin_trait_impl!(crate::usart::TxPin, LPUART2, PB6, 10u8);
pin_trait_impl!(crate::usart::RxPin, LPUART2, PB7, 10u8);
pin_trait_impl!(crate::usart::TxPin, LPUART2, PC0, 3u8);
pin_trait_impl!(crate::usart::RxPin, LPUART2, PC1, 3u8);
pin_trait_impl!(crate::usart::TxPin, LPUART2, PC6, 3u8);
pin_trait_impl!(crate::usart::RxPin, LPUART2, PC7, 3u8);
pin_trait_impl!(crate::usart::CtsPin, LPUART2, PC8, 3u8);
pin_trait_impl!(crate::usart::DePin, LPUART2, PC9, 3u8);
pin_trait_impl!(crate::usart::RtsPin, LPUART2, PC9, 3u8);
pin_trait_impl!(crate::usart::DePin, LPUART2, PF2, 3u8);
pin_trait_impl!(crate::usart::RtsPin, LPUART2, PF2, 3u8);
pin_trait_impl!(crate::usart::TxPin, LPUART2, PF2, 1u8);
pin_trait_impl!(crate::rcc::McoPin, MCO2, PA10, 3u8);
pin_trait_impl!(crate::rcc::McoPin, MCO2, PA15, 3u8);
pin_trait_impl!(crate::rcc::McoPin, MCO1, PA8, 0u8);
pin_trait_impl!(crate::rcc::McoPin, MCO1, PA9, 0u8);
pin_trait_impl!(crate::rcc::McoPin, MCO2, PB2, 3u8);
pin_trait_impl!(crate::rcc::McoPin, MCO1, PF2, 0u8);
pin_trait_impl!(crate::spi::CkPin, SPI1, PA1, 0u8);
pin_trait_impl!(crate::spi::SckPin, SPI1, PA1, 0u8);
pin_trait_impl!(crate::spi::MckPin, SPI1, PA11, 0u8);
pin_trait_impl!(crate::spi::MisoPin, SPI1, PA11, 0u8);
pin_trait_impl!(crate::spi::MosiPin, SPI1, PA12, 0u8);
pin_trait_impl!(crate::spi::WsPin, SPI1, PA15, 0u8);
pin_trait_impl!(crate::spi::CsPin, SPI1, PA15, 0u8);
pin_trait_impl!(crate::spi::MosiPin, SPI1, PA2, 0u8);
pin_trait_impl!(crate::spi::WsPin, SPI1, PA4, 0u8);
pin_trait_impl!(crate::spi::CsPin, SPI1, PA4, 0u8);
pin_trait_impl!(crate::spi::CkPin, SPI1, PA5, 0u8);
pin_trait_impl!(crate::spi::SckPin, SPI1, PA5, 0u8);
pin_trait_impl!(crate::spi::MckPin, SPI1, PA6, 0u8);
pin_trait_impl!(crate::spi::MisoPin, SPI1, PA6, 0u8);
pin_trait_impl!(crate::spi::MosiPin, SPI1, PA7, 0u8);
pin_trait_impl!(crate::spi::WsPin, SPI1, PB0, 0u8);
pin_trait_impl!(crate::spi::CsPin, SPI1, PB0, 0u8);
pin_trait_impl!(crate::spi::CkPin, SPI1, PB3, 0u8);
pin_trait_impl!(crate::spi::SckPin, SPI1, PB3, 0u8);
pin_trait_impl!(crate::spi::MckPin, SPI1, PB4, 0u8);
pin_trait_impl!(crate::spi::MisoPin, SPI1, PB4, 0u8);
pin_trait_impl!(crate::spi::MosiPin, SPI1, PB5, 0u8);
pin_trait_impl!(crate::spi::MckPin, SPI1, PD5, 1u8);
pin_trait_impl!(crate::spi::MisoPin, SPI1, PD5, 1u8);
pin_trait_impl!(crate::spi::MosiPin, SPI1, PD6, 1u8);
pin_trait_impl!(crate::spi::CkPin, SPI1, PD8, 1u8);
pin_trait_impl!(crate::spi::SckPin, SPI1, PD8, 1u8);
pin_trait_impl!(crate::spi::WsPin, SPI1, PD9, 1u8);
pin_trait_impl!(crate::spi::CsPin, SPI1, PD9, 1u8);
pin_trait_impl!(crate::spi::CkPin, SPI2, PA0, 0u8);
pin_trait_impl!(crate::spi::SckPin, SPI2, PA0, 0u8);
pin_trait_impl!(crate::spi::MosiPin, SPI2, PA10, 0u8);
pin_trait_impl!(crate::spi::MckPin, SPI2, PA3, 0u8);
pin_trait_impl!(crate::spi::MisoPin, SPI2, PA3, 0u8);
pin_trait_impl!(crate::spi::MosiPin, SPI2, PA4, 1u8);
pin_trait_impl!(crate::spi::WsPin, SPI2, PA8, 1u8);
pin_trait_impl!(crate::spi::CsPin, SPI2, PA8, 1u8);
pin_trait_impl!(crate::spi::MckPin, SPI2, PA9, 4u8);
pin_trait_impl!(crate::spi::MisoPin, SPI2, PA9, 4u8);
pin_trait_impl!(crate::spi::CkPin, SPI2, PB10, 5u8);
pin_trait_impl!(crate::spi::SckPin, SPI2, PB10, 5u8);
pin_trait_impl!(crate::spi::MosiPin, SPI2, PB11, 0u8);
pin_trait_impl!(crate::spi::WsPin, SPI2, PB12, 0u8);
pin_trait_impl!(crate::spi::CsPin, SPI2, PB12, 0u8);
pin_trait_impl!(crate::spi::CkPin, SPI2, PB13, 0u8);
pin_trait_impl!(crate::spi::SckPin, SPI2, PB13, 0u8);
pin_trait_impl!(crate::spi::MckPin, SPI2, PB14, 0u8);
pin_trait_impl!(crate::spi::MisoPin, SPI2, PB14, 0u8);
pin_trait_impl!(crate::spi::MosiPin, SPI2, PB15, 0u8);
pin_trait_impl!(crate::spi::MckPin, SPI2, PB2, 1u8);
pin_trait_impl!(crate::spi::MisoPin, SPI2, PB2, 1u8);
pin_trait_impl!(crate::spi::MckPin, SPI2, PB6, 4u8);
pin_trait_impl!(crate::spi::MisoPin, SPI2, PB6, 4u8);
pin_trait_impl!(crate::spi::MosiPin, SPI2, PB7, 1u8);
pin_trait_impl!(crate::spi::CkPin, SPI2, PB8, 1u8);
pin_trait_impl!(crate::spi::SckPin, SPI2, PB8, 1u8);
pin_trait_impl!(crate::spi::WsPin, SPI2, PB9, 5u8);
pin_trait_impl!(crate::spi::CsPin, SPI2, PB9, 5u8);
pin_trait_impl!(crate::spi::MckPin, SPI2, PC2, 1u8);
pin_trait_impl!(crate::spi::MisoPin, SPI2, PC2, 1u8);
pin_trait_impl!(crate::spi::MosiPin, SPI2, PC3, 1u8);
pin_trait_impl!(crate::spi::WsPin, SPI2, PD0, 1u8);
pin_trait_impl!(crate::spi::CsPin, SPI2, PD0, 1u8);
pin_trait_impl!(crate::spi::CkPin, SPI2, PD1, 1u8);
pin_trait_impl!(crate::spi::SckPin, SPI2, PD1, 1u8);
pin_trait_impl!(crate::spi::MckPin, SPI2, PD3, 1u8);
pin_trait_impl!(crate::spi::MisoPin, SPI2, PD3, 1u8);
pin_trait_impl!(crate::spi::MosiPin, SPI2, PD4, 1u8);
pin_trait_impl!(crate::spi::CsPin, SPI3, PA15, 9u8);
pin_trait_impl!(crate::spi::CsPin, SPI3, PA4, 9u8);
pin_trait_impl!(crate::spi::SckPin, SPI3, PB3, 9u8);
pin_trait_impl!(crate::spi::MisoPin, SPI3, PB4, 9u8);
pin_trait_impl!(crate::spi::MosiPin, SPI3, PB5, 9u8);
pin_trait_impl!(crate::spi::SckPin, SPI3, PC10, 4u8);
pin_trait_impl!(crate::spi::MisoPin, SPI3, PC11, 4u8);
pin_trait_impl!(crate::spi::MosiPin, SPI3, PC12, 4u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM1, PA10, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch4>, TIM1, PA11, 2u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM1, PA12, 2u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch1>, TIM1, PA7, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM1, PA8, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM1, PA9, 2u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch2>, TIM1, PB0, 2u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch3>, TIM1, PB1, 2u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch1>, TIM1, PB13, 2u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch2>, TIM1, PB14, 2u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch3>, TIM1, PB15, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM1, PB3, 1u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM1, PB6, 1u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM1, PC10, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch4>, TIM1, PC11, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM1, PC8, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM1, PC9, 2u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch1>, TIM1, PD2, 2u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch2>, TIM1, PD3, 2u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch3>, TIM1, PD4, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM14, PA4, 4u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM14, PA7, 4u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM14, PB1, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM14, PC12, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM14, PF0, 2u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch1>, TIM15, PA1, 5u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM15, PA2, 5u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM15, PA3, 5u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch1>, TIM15, PB13, 5u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM15, PB14, 5u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch1>, TIM15, PB15, 4u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM15, PB15, 5u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM15, PC1, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM15, PC2, 2u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch1>, TIM15, PF1, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM16, PA6, 5u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch1>, TIM16, PB6, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM16, PB8, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM16, PD0, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM17, PA7, 5u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch1>, TIM17, PB7, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM17, PB9, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM17, PD1, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM2, PA0, 2u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM2, PA0, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM2, PA1, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM2, PA15, 2u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM2, PA15, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM2, PA2, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch4>, TIM2, PA3, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM2, PA5, 2u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM2, PA5, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM2, PB10, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch4>, TIM2, PB11, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM2, PB3, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM2, PC4, 2u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM2, PC4, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM2, PC5, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM2, PC6, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch4>, TIM2, PC7, 2u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM3, PA6, 1u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM3, PA7, 1u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM3, PB0, 1u8);
pin_trait_impl!(crate::timer::TimerPin<Ch4>, TIM3, PB1, 1u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM3, PB4, 1u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM3, PB5, 1u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM3, PC6, 1u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM3, PC7, 1u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM3, PC8, 1u8);
pin_trait_impl!(crate::timer::TimerPin<Ch4>, TIM3, PC9, 1u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM3, PD2, 1u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM4, PB6, 9u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM4, PB7, 9u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM4, PB8, 9u8);
pin_trait_impl!(crate::timer::TimerPin<Ch4>, TIM4, PB9, 9u8);
pin_trait_impl!(crate::ucpd::Cc1Pin, UCPD1, PA8, 0u8);
pin_trait_impl!(crate::ucpd::Cc2Pin, UCPD1, PB15, 0u8);
pin_trait_impl!(crate::ucpd::Cc1Pin, UCPD2, PD0, 0u8);
pin_trait_impl!(crate::ucpd::Cc2Pin, UCPD2, PD2, 0u8);
pin_trait_impl!(crate::usart::RxPin, USART1, PA10, 1u8);
pin_trait_impl!(crate::usart::CtsPin, USART1, PA11, 1u8);
pin_trait_impl!(crate::usart::CkPin, USART1, PA12, 1u8);
pin_trait_impl!(crate::usart::DePin, USART1, PA12, 1u8);
pin_trait_impl!(crate::usart::RtsPin, USART1, PA12, 1u8);
pin_trait_impl!(crate::usart::TxPin, USART1, PA9, 1u8);
pin_trait_impl!(crate::usart::CkPin, USART1, PB3, 4u8);
pin_trait_impl!(crate::usart::DePin, USART1, PB3, 4u8);
pin_trait_impl!(crate::usart::RtsPin, USART1, PB3, 4u8);
pin_trait_impl!(crate::usart::CtsPin, USART1, PB4, 4u8);
pin_trait_impl!(crate::usart::TxPin, USART1, PB6, 0u8);
pin_trait_impl!(crate::usart::RxPin, USART1, PB7, 0u8);
pin_trait_impl!(crate::usart::TxPin, USART1, PC4, 1u8);
pin_trait_impl!(crate::usart::RxPin, USART1, PC5, 1u8);
pin_trait_impl!(crate::usart::CtsPin, USART2, PA0, 1u8);
pin_trait_impl!(crate::usart::CkPin, USART2, PA1, 1u8);
pin_trait_impl!(crate::usart::DePin, USART2, PA1, 1u8);
pin_trait_impl!(crate::usart::RtsPin, USART2, PA1, 1u8);
pin_trait_impl!(crate::usart::TxPin, USART2, PA14, 1u8);
pin_trait_impl!(crate::usart::RxPin, USART2, PA15, 1u8);
pin_trait_impl!(crate::usart::TxPin, USART2, PA2, 1u8);
pin_trait_impl!(crate::usart::RxPin, USART2, PA3, 1u8);
pin_trait_impl!(crate::usart::CtsPin, USART2, PD3, 0u8);
pin_trait_impl!(crate::usart::CkPin, USART2, PD4, 0u8);
pin_trait_impl!(crate::usart::DePin, USART2, PD4, 0u8);
pin_trait_impl!(crate::usart::RtsPin, USART2, PD4, 0u8);
pin_trait_impl!(crate::usart::TxPin, USART2, PD5, 0u8);
pin_trait_impl!(crate::usart::RxPin, USART2, PD6, 0u8);
pin_trait_impl!(crate::usart::CkPin, USART3, PA15, 5u8);
pin_trait_impl!(crate::usart::DePin, USART3, PA15, 5u8);
pin_trait_impl!(crate::usart::RtsPin, USART3, PA15, 5u8);
pin_trait_impl!(crate::usart::TxPin, USART3, PA5, 4u8);
pin_trait_impl!(crate::usart::CtsPin, USART3, PA6, 4u8);
pin_trait_impl!(crate::usart::RxPin, USART3, PB0, 4u8);
pin_trait_impl!(crate::usart::CkPin, USART3, PB1, 4u8);
pin_trait_impl!(crate::usart::DePin, USART3, PB1, 4u8);
pin_trait_impl!(crate::usart::RtsPin, USART3, PB1, 4u8);
pin_trait_impl!(crate::usart::TxPin, USART3, PB10, 4u8);
pin_trait_impl!(crate::usart::RxPin, USART3, PB11, 4u8);
pin_trait_impl!(crate::usart::CtsPin, USART3, PB13, 4u8);
pin_trait_impl!(crate::usart::CkPin, USART3, PB14, 4u8);
pin_trait_impl!(crate::usart::DePin, USART3, PB14, 4u8);
pin_trait_impl!(crate::usart::RtsPin, USART3, PB14, 4u8);
pin_trait_impl!(crate::usart::TxPin, USART3, PB2, 4u8);
pin_trait_impl!(crate::usart::TxPin, USART3, PB8, 4u8);
pin_trait_impl!(crate::usart::RxPin, USART3, PB9, 4u8);
pin_trait_impl!(crate::usart::TxPin, USART3, PC10, 0u8);
pin_trait_impl!(crate::usart::RxPin, USART3, PC11, 0u8);
pin_trait_impl!(crate::usart::TxPin, USART3, PC4, 0u8);
pin_trait_impl!(crate::usart::RxPin, USART3, PC5, 0u8);
pin_trait_impl!(crate::usart::CkPin, USART3, PD2, 0u8);
pin_trait_impl!(crate::usart::DePin, USART3, PD2, 0u8);
pin_trait_impl!(crate::usart::RtsPin, USART3, PD2, 0u8);
pin_trait_impl!(crate::usart::TxPin, USART3, PD8, 0u8);
pin_trait_impl!(crate::usart::RxPin, USART3, PD9, 0u8);
pin_trait_impl!(crate::usart::TxPin, USART4, PA0, 4u8);
pin_trait_impl!(crate::usart::RxPin, USART4, PA1, 4u8);
pin_trait_impl!(crate::usart::CkPin, USART4, PA15, 4u8);
pin_trait_impl!(crate::usart::DePin, USART4, PA15, 4u8);
pin_trait_impl!(crate::usart::RtsPin, USART4, PA15, 4u8);
pin_trait_impl!(crate::usart::CtsPin, USART4, PB7, 4u8);
pin_trait_impl!(crate::usart::TxPin, USART4, PC10, 1u8);
pin_trait_impl!(crate::usart::RxPin, USART4, PC11, 1u8);
pin_trait_impl!(crate::usart::TxPin, USART5, PB0, 8u8);
pin_trait_impl!(crate::usart::RxPin, USART5, PB1, 8u8);
pin_trait_impl!(crate::usart::TxPin, USART5, PB3, 3u8);
pin_trait_impl!(crate::usart::RxPin, USART5, PB4, 3u8);
pin_trait_impl!(crate::usart::CkPin, USART5, PB5, 8u8);
pin_trait_impl!(crate::usart::DePin, USART5, PB5, 8u8);
pin_trait_impl!(crate::usart::RtsPin, USART5, PB5, 8u8);
pin_trait_impl!(crate::usart::CtsPin, USART5, PB6, 8u8);
pin_trait_impl!(crate::usart::TxPin, USART5, PC12, 3u8);
pin_trait_impl!(crate::usart::RxPin, USART5, PD2, 3u8);
pin_trait_impl!(crate::usart::TxPin, USART5, PD3, 3u8);
pin_trait_impl!(crate::usart::CkPin, USART5, PD4, 3u8);
pin_trait_impl!(crate::usart::DePin, USART5, PD4, 3u8);
pin_trait_impl!(crate::usart::RtsPin, USART5, PD4, 3u8);
pin_trait_impl!(crate::usart::CtsPin, USART5, PD5, 3u8);
pin_trait_impl!(crate::usart::TxPin, USART6, PA4, 3u8);
pin_trait_impl!(crate::usart::RxPin, USART6, PA5, 3u8);
pin_trait_impl!(crate::usart::CtsPin, USART6, PA6, 3u8);
pin_trait_impl!(crate::usart::CkPin, USART6, PA7, 3u8);
pin_trait_impl!(crate::usart::DePin, USART6, PA7, 3u8);
pin_trait_impl!(crate::usart::RtsPin, USART6, PA7, 3u8);
pin_trait_impl!(crate::usart::CkPin, USART6, PB14, 8u8);
pin_trait_impl!(crate::usart::DePin, USART6, PB14, 8u8);
pin_trait_impl!(crate::usart::RtsPin, USART6, PB14, 8u8);
pin_trait_impl!(crate::usart::CtsPin, USART6, PB15, 8u8);
pin_trait_impl!(crate::usart::TxPin, USART6, PB8, 8u8);
pin_trait_impl!(crate::usart::RxPin, USART6, PB9, 8u8);
pin_trait_impl!(crate::usart::TxPin, USART6, PC0, 4u8);
pin_trait_impl!(crate::usart::RxPin, USART6, PC1, 4u8);
pin_trait_impl!(crate::usb::DmPin, USB, PA11, 0u8);
pin_trait_impl!(crate::usb::DpPin, USB, PA12, 0u8);
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH1, 5u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH2, 5u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH3, 5u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH4, 5u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH5, 5u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH6, 5u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH7, 5u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH1, 5u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH2, 5u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH3, 5u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH4, 5u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH5, 5u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH1, 8u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH2, 8u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH3, 8u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH4, 8u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH5, 8u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH6, 8u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH7, 8u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH1, 8u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH2, 8u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH3, 8u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH4, 8u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH5, 8u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH1, 9u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH2, 9u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH3, 9u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH4, 9u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH5, 9u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH6, 9u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH7, 9u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH1, 9u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH2, 9u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH3, 9u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH4, 9u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH5, 9u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH1, 10u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH2, 10u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH3, 10u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH4, 10u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH5, 10u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH6, 10u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH7, 10u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH1, 10u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH2, 10u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH3, 10u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH4, 10u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH5, 10u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH1, 11u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH2, 11u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH3, 11u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH4, 11u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH5, 11u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH6, 11u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH7, 11u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH1, 11u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH2, 11u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH3, 11u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH4, 11u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH5, 11u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH1, 12u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH2, 12u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH3, 12u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH4, 12u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH5, 12u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH6, 12u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH7, 12u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA2_CH1, 12u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA2_CH2, 12u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA2_CH3, 12u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA2_CH4, 12u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA2_CH5, 12u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH1, 13u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH2, 13u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH3, 13u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH4, 13u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH5, 13u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH6, 13u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH7, 13u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA2_CH1, 13u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA2_CH2, 13u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA2_CH3, 13u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA2_CH4, 13u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA2_CH5, 13u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH1, 62u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH2, 62u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH3, 62u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH4, 62u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH5, 62u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH6, 62u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH7, 62u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA2_CH1, 62u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA2_CH2, 62u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA2_CH3, 62u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA2_CH4, 62u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA2_CH5, 62u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH1, 63u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH2, 63u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH3, 63u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH4, 63u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH5, 63u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH6, 63u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH7, 63u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA2_CH1, 63u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA2_CH2, 63u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA2_CH3, 63u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA2_CH4, 63u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA2_CH5, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, DMA1_CH1, 14u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, DMA1_CH2, 14u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, DMA1_CH3, 14u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, DMA1_CH4, 14u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, DMA1_CH5, 14u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, DMA1_CH6, 14u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, DMA1_CH7, 14u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, DMA2_CH1, 14u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, DMA2_CH2, 14u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, DMA2_CH3, 14u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, DMA2_CH4, 14u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, DMA2_CH5, 14u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, DMA1_CH1, 15u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, DMA1_CH2, 15u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, DMA1_CH3, 15u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, DMA1_CH4, 15u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, DMA1_CH5, 15u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, DMA1_CH6, 15u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, DMA1_CH7, 15u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, DMA2_CH1, 15u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, DMA2_CH2, 15u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, DMA2_CH3, 15u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, DMA2_CH4, 15u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, DMA2_CH5, 15u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART2, DMA1_CH1, 64u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART2, DMA1_CH2, 64u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART2, DMA1_CH3, 64u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART2, DMA1_CH4, 64u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART2, DMA1_CH5, 64u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART2, DMA1_CH6, 64u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART2, DMA1_CH7, 64u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART2, DMA2_CH1, 64u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART2, DMA2_CH2, 64u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART2, DMA2_CH3, 64u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART2, DMA2_CH4, 64u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART2, DMA2_CH5, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART2, DMA1_CH1, 65u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART2, DMA1_CH2, 65u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART2, DMA1_CH3, 65u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART2, DMA1_CH4, 65u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART2, DMA1_CH5, 65u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART2, DMA1_CH6, 65u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART2, DMA1_CH7, 65u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART2, DMA2_CH1, 65u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART2, DMA2_CH2, 65u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART2, DMA2_CH3, 65u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART2, DMA2_CH4, 65u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART2, DMA2_CH5, 65u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH1, 16u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH2, 16u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH3, 16u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH4, 16u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH5, 16u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH6, 16u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH7, 16u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH1, 16u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH2, 16u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH3, 16u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH4, 16u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH5, 16u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH1, 17u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH2, 17u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH3, 17u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH4, 17u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH5, 17u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH6, 17u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH7, 17u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH1, 17u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH2, 17u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH3, 17u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH4, 17u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH5, 17u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH1, 18u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH2, 18u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH3, 18u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH4, 18u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH5, 18u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH6, 18u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH7, 18u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA2_CH1, 18u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA2_CH2, 18u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA2_CH3, 18u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA2_CH4, 18u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA2_CH5, 18u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH1, 19u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH2, 19u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH3, 19u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH4, 19u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH5, 19u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH6, 19u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH7, 19u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA2_CH1, 19u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA2_CH2, 19u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA2_CH3, 19u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA2_CH4, 19u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA2_CH5, 19u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH1, 66u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH2, 66u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH3, 66u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH4, 66u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH5, 66u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH6, 66u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH7, 66u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH1, 66u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH2, 66u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH3, 66u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH4, 66u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH5, 66u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH1, 67u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH2, 67u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH3, 67u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH4, 67u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH5, 67u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH6, 67u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH7, 67u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH1, 67u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH2, 67u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH3, 67u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH4, 67u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH5, 67u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH1, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH2, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH3, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH4, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH5, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH6, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH7, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH1, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH2, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH3, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH4, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH5, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH1, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH2, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH3, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH4, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH5, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH6, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH7, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH1, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH2, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH3, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH4, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH5, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH1, 22u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH2, 22u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH3, 22u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH4, 22u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH5, 22u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH6, 22u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH7, 22u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA2_CH1, 22u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA2_CH2, 22u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA2_CH3, 22u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA2_CH4, 22u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA2_CH5, 22u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH1, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH2, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH3, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH4, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH5, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH6, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH7, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA2_CH1, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA2_CH2, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA2_CH3, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA2_CH4, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA2_CH5, 23u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH1, 25u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH2, 25u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH3, 25u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH4, 25u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH5, 25u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH6, 25u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH7, 25u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH1, 25u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH2, 25u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH3, 25u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH4, 25u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH5, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH1, 40u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH2, 40u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH3, 40u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH4, 40u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH5, 40u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH6, 40u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH7, 40u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA2_CH1, 40u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA2_CH2, 40u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA2_CH3, 40u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA2_CH4, 40u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA2_CH5, 40u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM15, DMA1_CH1, 41u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM15, DMA1_CH2, 41u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM15, DMA1_CH3, 41u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM15, DMA1_CH4, 41u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM15, DMA1_CH5, 41u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM15, DMA1_CH6, 41u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM15, DMA1_CH7, 41u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM15, DMA2_CH1, 41u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM15, DMA2_CH2, 41u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM15, DMA2_CH3, 41u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM15, DMA2_CH4, 41u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM15, DMA2_CH5, 41u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH1, 43u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH2, 43u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH3, 43u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH4, 43u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH5, 43u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH6, 43u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH7, 43u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA2_CH1, 43u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA2_CH2, 43u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA2_CH3, 43u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA2_CH4, 43u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA2_CH5, 43u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH1, 44u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH2, 44u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH3, 44u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH4, 44u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH5, 44u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH6, 44u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH7, 44u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA2_CH1, 44u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA2_CH2, 44u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA2_CH3, 44u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA2_CH4, 44u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA2_CH5, 44u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH1, 46u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH2, 46u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH3, 46u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH4, 46u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH5, 46u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH6, 46u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH7, 46u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA2_CH1, 46u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA2_CH2, 46u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA2_CH3, 46u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA2_CH4, 46u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA2_CH5, 46u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH1, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH2, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH3, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH4, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH5, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH6, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH7, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA2_CH1, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA2_CH2, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA2_CH3, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA2_CH4, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA2_CH5, 47u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH1, 49u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH2, 49u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH3, 49u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH4, 49u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH5, 49u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH6, 49u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH7, 49u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA2_CH1, 49u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA2_CH2, 49u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA2_CH3, 49u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA2_CH4, 49u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA2_CH5, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH1, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH2, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH3, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH4, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH5, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH6, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH7, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA2_CH1, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA2_CH2, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA2_CH3, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA2_CH4, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA2_CH5, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH1, 27u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH2, 27u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH3, 27u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH4, 27u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH5, 27u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH6, 27u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH7, 27u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA2_CH1, 27u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA2_CH2, 27u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA2_CH3, 27u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA2_CH4, 27u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA2_CH5, 27u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH1, 28u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH2, 28u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH3, 28u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH4, 28u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH5, 28u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH6, 28u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH7, 28u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA2_CH1, 28u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA2_CH2, 28u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA2_CH3, 28u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA2_CH4, 28u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA2_CH5, 28u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH1, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH2, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH3, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH4, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH5, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH6, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH7, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA2_CH1, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA2_CH2, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA2_CH3, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA2_CH4, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA2_CH5, 29u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH1, 31u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH2, 31u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH3, 31u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH4, 31u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH5, 31u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH6, 31u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH7, 31u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA2_CH1, 31u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA2_CH2, 31u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA2_CH3, 31u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA2_CH4, 31u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA2_CH5, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH1, 32u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH2, 32u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH3, 32u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH4, 32u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH5, 32u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH6, 32u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH7, 32u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA2_CH1, 32u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA2_CH2, 32u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA2_CH3, 32u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA2_CH4, 32u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA2_CH5, 32u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH1, 33u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH2, 33u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH3, 33u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH4, 33u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH5, 33u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH6, 33u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH7, 33u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA2_CH1, 33u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA2_CH2, 33u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA2_CH3, 33u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA2_CH4, 33u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA2_CH5, 33u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH1, 34u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH2, 34u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH3, 34u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH4, 34u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH5, 34u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH6, 34u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH7, 34u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA2_CH1, 34u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA2_CH2, 34u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA2_CH3, 34u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA2_CH4, 34u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA2_CH5, 34u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH1, 35u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH2, 35u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH3, 35u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH4, 35u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH5, 35u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH6, 35u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH7, 35u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA2_CH1, 35u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA2_CH2, 35u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA2_CH3, 35u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA2_CH4, 35u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA2_CH5, 35u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH1, 37u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH2, 37u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH3, 37u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH4, 37u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH5, 37u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH6, 37u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH7, 37u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA2_CH1, 37u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA2_CH2, 37u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA2_CH3, 37u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA2_CH4, 37u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA2_CH5, 37u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH1, 68u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH2, 68u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH3, 68u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH4, 68u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH5, 68u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH6, 68u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH7, 68u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA2_CH1, 68u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA2_CH2, 68u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA2_CH3, 68u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA2_CH4, 68u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA2_CH5, 68u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH1, 69u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH2, 69u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH3, 69u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH4, 69u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH5, 69u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH6, 69u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH7, 69u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA2_CH1, 69u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA2_CH2, 69u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA2_CH3, 69u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA2_CH4, 69u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA2_CH5, 69u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH1, 70u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH2, 70u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH3, 70u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH4, 70u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH5, 70u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH6, 70u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH7, 70u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA2_CH1, 70u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA2_CH2, 70u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA2_CH3, 70u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA2_CH4, 70u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA2_CH5, 70u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM4, DMA1_CH1, 71u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM4, DMA1_CH2, 71u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM4, DMA1_CH3, 71u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM4, DMA1_CH4, 71u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM4, DMA1_CH5, 71u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM4, DMA1_CH6, 71u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM4, DMA1_CH7, 71u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM4, DMA2_CH1, 71u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM4, DMA2_CH2, 71u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM4, DMA2_CH3, 71u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM4, DMA2_CH4, 71u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM4, DMA2_CH5, 71u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH1, 73u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH2, 73u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH3, 73u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH4, 73u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH5, 73u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH6, 73u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH7, 73u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA2_CH1, 73u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA2_CH2, 73u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA2_CH3, 73u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA2_CH4, 73u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA2_CH5, 73u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH1, 38u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH2, 38u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH3, 38u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH4, 38u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH5, 38u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH6, 38u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH7, 38u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH1, 38u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH2, 38u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH3, 38u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH4, 38u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH5, 38u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH1, 39u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH2, 39u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH3, 39u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH4, 39u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH5, 39u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH6, 39u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH7, 39u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH1, 39u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH2, 39u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH3, 39u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH4, 39u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH5, 39u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD1, DMA1_CH1, 58u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD1, DMA1_CH2, 58u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD1, DMA1_CH3, 58u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD1, DMA1_CH4, 58u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD1, DMA1_CH5, 58u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD1, DMA1_CH6, 58u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD1, DMA1_CH7, 58u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD1, DMA2_CH1, 58u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD1, DMA2_CH2, 58u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD1, DMA2_CH3, 58u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD1, DMA2_CH4, 58u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD1, DMA2_CH5, 58u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD1, DMA1_CH1, 59u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD1, DMA1_CH2, 59u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD1, DMA1_CH3, 59u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD1, DMA1_CH4, 59u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD1, DMA1_CH5, 59u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD1, DMA1_CH6, 59u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD1, DMA1_CH7, 59u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD1, DMA2_CH1, 59u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD1, DMA2_CH2, 59u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD1, DMA2_CH3, 59u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD1, DMA2_CH4, 59u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD1, DMA2_CH5, 59u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD2, DMA1_CH1, 60u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD2, DMA1_CH2, 60u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD2, DMA1_CH3, 60u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD2, DMA1_CH4, 60u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD2, DMA1_CH5, 60u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD2, DMA1_CH6, 60u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD2, DMA1_CH7, 60u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD2, DMA2_CH1, 60u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD2, DMA2_CH2, 60u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD2, DMA2_CH3, 60u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD2, DMA2_CH4, 60u8, {});
dma_trait_impl!(crate::ucpd::RxDma, UCPD2, DMA2_CH5, 60u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD2, DMA1_CH1, 61u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD2, DMA1_CH2, 61u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD2, DMA1_CH3, 61u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD2, DMA1_CH4, 61u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD2, DMA1_CH5, 61u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD2, DMA1_CH6, 61u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD2, DMA1_CH7, 61u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD2, DMA2_CH1, 61u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD2, DMA2_CH2, 61u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD2, DMA2_CH3, 61u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD2, DMA2_CH4, 61u8, {});
dma_trait_impl!(crate::ucpd::TxDma, UCPD2, DMA2_CH5, 61u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH1, 50u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH2, 50u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH3, 50u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH4, 50u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH5, 50u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH6, 50u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH7, 50u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH1, 50u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH2, 50u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH3, 50u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH4, 50u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH5, 50u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH1, 51u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH2, 51u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH3, 51u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH4, 51u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH5, 51u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH6, 51u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH7, 51u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH1, 51u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH2, 51u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH3, 51u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH4, 51u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH5, 51u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH1, 52u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH2, 52u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH3, 52u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH4, 52u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH5, 52u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH6, 52u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH7, 52u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA2_CH1, 52u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA2_CH2, 52u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA2_CH3, 52u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA2_CH4, 52u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA2_CH5, 52u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH1, 53u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH2, 53u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH3, 53u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH4, 53u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH5, 53u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH6, 53u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH7, 53u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA2_CH1, 53u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA2_CH2, 53u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA2_CH3, 53u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA2_CH4, 53u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA2_CH5, 53u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH1, 54u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH2, 54u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH3, 54u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH4, 54u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH5, 54u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH6, 54u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH7, 54u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA2_CH1, 54u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA2_CH2, 54u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA2_CH3, 54u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA2_CH4, 54u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA2_CH5, 54u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH1, 55u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH2, 55u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH3, 55u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH4, 55u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH5, 55u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH6, 55u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH7, 55u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA2_CH1, 55u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA2_CH2, 55u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA2_CH3, 55u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA2_CH4, 55u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA2_CH5, 55u8, {});
dma_trait_impl!(crate::usart::RxDma, USART4, DMA1_CH1, 56u8, {});
dma_trait_impl!(crate::usart::RxDma, USART4, DMA1_CH2, 56u8, {});
dma_trait_impl!(crate::usart::RxDma, USART4, DMA1_CH3, 56u8, {});
dma_trait_impl!(crate::usart::RxDma, USART4, DMA1_CH4, 56u8, {});
dma_trait_impl!(crate::usart::RxDma, USART4, DMA1_CH5, 56u8, {});
dma_trait_impl!(crate::usart::RxDma, USART4, DMA1_CH6, 56u8, {});
dma_trait_impl!(crate::usart::RxDma, USART4, DMA1_CH7, 56u8, {});
dma_trait_impl!(crate::usart::RxDma, USART4, DMA2_CH1, 56u8, {});
dma_trait_impl!(crate::usart::RxDma, USART4, DMA2_CH2, 56u8, {});
dma_trait_impl!(crate::usart::RxDma, USART4, DMA2_CH3, 56u8, {});
dma_trait_impl!(crate::usart::RxDma, USART4, DMA2_CH4, 56u8, {});
dma_trait_impl!(crate::usart::RxDma, USART4, DMA2_CH5, 56u8, {});
dma_trait_impl!(crate::usart::TxDma, USART4, DMA1_CH1, 57u8, {});
dma_trait_impl!(crate::usart::TxDma, USART4, DMA1_CH2, 57u8, {});
dma_trait_impl!(crate::usart::TxDma, USART4, DMA1_CH3, 57u8, {});
dma_trait_impl!(crate::usart::TxDma, USART4, DMA1_CH4, 57u8, {});
dma_trait_impl!(crate::usart::TxDma, USART4, DMA1_CH5, 57u8, {});
dma_trait_impl!(crate::usart::TxDma, USART4, DMA1_CH6, 57u8, {});
dma_trait_impl!(crate::usart::TxDma, USART4, DMA1_CH7, 57u8, {});
dma_trait_impl!(crate::usart::TxDma, USART4, DMA2_CH1, 57u8, {});
dma_trait_impl!(crate::usart::TxDma, USART4, DMA2_CH2, 57u8, {});
dma_trait_impl!(crate::usart::TxDma, USART4, DMA2_CH3, 57u8, {});
dma_trait_impl!(crate::usart::TxDma, USART4, DMA2_CH4, 57u8, {});
dma_trait_impl!(crate::usart::TxDma, USART4, DMA2_CH5, 57u8, {});
dma_trait_impl!(crate::usart::RxDma, USART5, DMA1_CH1, 74u8, {});
dma_trait_impl!(crate::usart::RxDma, USART5, DMA1_CH2, 74u8, {});
dma_trait_impl!(crate::usart::RxDma, USART5, DMA1_CH3, 74u8, {});
dma_trait_impl!(crate::usart::RxDma, USART5, DMA1_CH4, 74u8, {});
dma_trait_impl!(crate::usart::RxDma, USART5, DMA1_CH5, 74u8, {});
dma_trait_impl!(crate::usart::RxDma, USART5, DMA1_CH6, 74u8, {});
dma_trait_impl!(crate::usart::RxDma, USART5, DMA1_CH7, 74u8, {});
dma_trait_impl!(crate::usart::RxDma, USART5, DMA2_CH1, 74u8, {});
dma_trait_impl!(crate::usart::RxDma, USART5, DMA2_CH2, 74u8, {});
dma_trait_impl!(crate::usart::RxDma, USART5, DMA2_CH3, 74u8, {});
dma_trait_impl!(crate::usart::RxDma, USART5, DMA2_CH4, 74u8, {});
dma_trait_impl!(crate::usart::RxDma, USART5, DMA2_CH5, 74u8, {});
dma_trait_impl!(crate::usart::TxDma, USART5, DMA1_CH1, 75u8, {});
dma_trait_impl!(crate::usart::TxDma, USART5, DMA1_CH2, 75u8, {});
dma_trait_impl!(crate::usart::TxDma, USART5, DMA1_CH3, 75u8, {});
dma_trait_impl!(crate::usart::TxDma, USART5, DMA1_CH4, 75u8, {});
dma_trait_impl!(crate::usart::TxDma, USART5, DMA1_CH5, 75u8, {});
dma_trait_impl!(crate::usart::TxDma, USART5, DMA1_CH6, 75u8, {});
dma_trait_impl!(crate::usart::TxDma, USART5, DMA1_CH7, 75u8, {});
dma_trait_impl!(crate::usart::TxDma, USART5, DMA2_CH1, 75u8, {});
dma_trait_impl!(crate::usart::TxDma, USART5, DMA2_CH2, 75u8, {});
dma_trait_impl!(crate::usart::TxDma, USART5, DMA2_CH3, 75u8, {});
dma_trait_impl!(crate::usart::TxDma, USART5, DMA2_CH4, 75u8, {});
dma_trait_impl!(crate::usart::TxDma, USART5, DMA2_CH5, 75u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH1, 76u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH2, 76u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH3, 76u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH4, 76u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH5, 76u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH6, 76u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH7, 76u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH1, 76u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH2, 76u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH3, 76u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH4, 76u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH5, 76u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH1, 77u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH2, 77u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH3, 77u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH4, 77u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH5, 77u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH6, 77u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH7, 77u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH1, 77u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH2, 77u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH3, 77u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH4, 77u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH5, 77u8, {});
impl core::ops::Div<crate::pac::rcc::vals::Hpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Hpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Hpre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Hpre::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Hpre::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Hpre::DIV16 => self * 1u32 / 16u32,
            crate::pac::rcc::vals::Hpre::DIV64 => self * 1u32 / 64u32,
            crate::pac::rcc::vals::Hpre::DIV128 => self * 1u32 / 128u32,
            crate::pac::rcc::vals::Hpre::DIV256 => self * 1u32 / 256u32,
            crate::pac::rcc::vals::Hpre::DIV512 => self * 1u32 / 512u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Hpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Hpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Hpre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV16 => self * 16u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV64 => self * 64u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV128 => self * 128u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV256 => self * 256u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV512 => self * 512u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Hsidiv> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Hsidiv) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Hsidiv::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Hsidiv::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Hsidiv::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Hsidiv::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Hsidiv::DIV16 => self * 1u32 / 16u32,
            crate::pac::rcc::vals::Hsidiv::DIV32 => self * 1u32 / 32u32,
            crate::pac::rcc::vals::Hsidiv::DIV64 => self * 1u32 / 64u32,
            crate::pac::rcc::vals::Hsidiv::DIV128 => self * 1u32 / 128u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Hsidiv> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Hsidiv) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Hsidiv::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Hsidiv::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Hsidiv::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Hsidiv::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Hsidiv::DIV16 => self * 16u32 / 1u32,
            crate::pac::rcc::vals::Hsidiv::DIV32 => self * 32u32 / 1u32,
            crate::pac::rcc::vals::Hsidiv::DIV64 => self * 64u32 / 1u32,
            crate::pac::rcc::vals::Hsidiv::DIV128 => self * 128u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Mcopre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Mcopre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Mcopre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Mcopre::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Mcopre::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Mcopre::DIV16 => self * 1u32 / 16u32,
            crate::pac::rcc::vals::Mcopre::DIV32 => self * 1u32 / 32u32,
            crate::pac::rcc::vals::Mcopre::DIV64 => self * 1u32 / 64u32,
            crate::pac::rcc::vals::Mcopre::DIV128 => self * 1u32 / 128u32,
            crate::pac::rcc::vals::Mcopre::DIV256 => self * 1u32 / 256u32,
            crate::pac::rcc::vals::Mcopre::DIV512 => self * 1u32 / 512u32,
            crate::pac::rcc::vals::Mcopre::DIV1024 => self * 1u32 / 1024u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Mcopre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Mcopre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Mcopre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV16 => self * 16u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV32 => self * 32u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV64 => self * 64u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV128 => self * 128u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV256 => self * 256u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV512 => self * 512u32 / 1u32,
            crate::pac::rcc::vals::Mcopre::DIV1024 => self * 1024u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllm> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllm) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllm::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Pllm::DIV3 => self * 1u32 / 3u32,
            crate::pac::rcc::vals::Pllm::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Pllm::DIV5 => self * 1u32 / 5u32,
            crate::pac::rcc::vals::Pllm::DIV6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Pllm::DIV7 => self * 1u32 / 7u32,
            crate::pac::rcc::vals::Pllm::DIV8 => self * 1u32 / 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllm> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllm) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllm::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV3 => self * 3u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV5 => self * 5u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV7 => self * 7u32 / 1u32,
            crate::pac::rcc::vals::Pllm::DIV8 => self * 8u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Plln> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Plln) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Plln::MUL8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Plln::MUL9 => self * 1u32 / 9u32,
            crate::pac::rcc::vals::Plln::MUL10 => self * 1u32 / 10u32,
            crate::pac::rcc::vals::Plln::MUL11 => self * 1u32 / 11u32,
            crate::pac::rcc::vals::Plln::MUL12 => self * 1u32 / 12u32,
            crate::pac::rcc::vals::Plln::MUL13 => self * 1u32 / 13u32,
            crate::pac::rcc::vals::Plln::MUL14 => self * 1u32 / 14u32,
            crate::pac::rcc::vals::Plln::MUL15 => self * 1u32 / 15u32,
            crate::pac::rcc::vals::Plln::MUL16 => self * 1u32 / 16u32,
            crate::pac::rcc::vals::Plln::MUL17 => self * 1u32 / 17u32,
            crate::pac::rcc::vals::Plln::MUL18 => self * 1u32 / 18u32,
            crate::pac::rcc::vals::Plln::MUL19 => self * 1u32 / 19u32,
            crate::pac::rcc::vals::Plln::MUL20 => self * 1u32 / 20u32,
            crate::pac::rcc::vals::Plln::MUL21 => self * 1u32 / 21u32,
            crate::pac::rcc::vals::Plln::MUL22 => self * 1u32 / 22u32,
            crate::pac::rcc::vals::Plln::MUL23 => self * 1u32 / 23u32,
            crate::pac::rcc::vals::Plln::MUL24 => self * 1u32 / 24u32,
            crate::pac::rcc::vals::Plln::MUL25 => self * 1u32 / 25u32,
            crate::pac::rcc::vals::Plln::MUL26 => self * 1u32 / 26u32,
            crate::pac::rcc::vals::Plln::MUL27 => self * 1u32 / 27u32,
            crate::pac::rcc::vals::Plln::MUL28 => self * 1u32 / 28u32,
            crate::pac::rcc::vals::Plln::MUL29 => self * 1u32 / 29u32,
            crate::pac::rcc::vals::Plln::MUL30 => self * 1u32 / 30u32,
            crate::pac::rcc::vals::Plln::MUL31 => self * 1u32 / 31u32,
            crate::pac::rcc::vals::Plln::MUL32 => self * 1u32 / 32u32,
            crate::pac::rcc::vals::Plln::MUL33 => self * 1u32 / 33u32,
            crate::pac::rcc::vals::Plln::MUL34 => self * 1u32 / 34u32,
            crate::pac::rcc::vals::Plln::MUL35 => self * 1u32 / 35u32,
            crate::pac::rcc::vals::Plln::MUL36 => self * 1u32 / 36u32,
            crate::pac::rcc::vals::Plln::MUL37 => self * 1u32 / 37u32,
            crate::pac::rcc::vals::Plln::MUL38 => self * 1u32 / 38u32,
            crate::pac::rcc::vals::Plln::MUL39 => self * 1u32 / 39u32,
            crate::pac::rcc::vals::Plln::MUL40 => self * 1u32 / 40u32,
            crate::pac::rcc::vals::Plln::MUL41 => self * 1u32 / 41u32,
            crate::pac::rcc::vals::Plln::MUL42 => self * 1u32 / 42u32,
            crate::pac::rcc::vals::Plln::MUL43 => self * 1u32 / 43u32,
            crate::pac::rcc::vals::Plln::MUL44 => self * 1u32 / 44u32,
            crate::pac::rcc::vals::Plln::MUL45 => self * 1u32 / 45u32,
            crate::pac::rcc::vals::Plln::MUL46 => self * 1u32 / 46u32,
            crate::pac::rcc::vals::Plln::MUL47 => self * 1u32 / 47u32,
            crate::pac::rcc::vals::Plln::MUL48 => self * 1u32 / 48u32,
            crate::pac::rcc::vals::Plln::MUL49 => self * 1u32 / 49u32,
            crate::pac::rcc::vals::Plln::MUL50 => self * 1u32 / 50u32,
            crate::pac::rcc::vals::Plln::MUL51 => self * 1u32 / 51u32,
            crate::pac::rcc::vals::Plln::MUL52 => self * 1u32 / 52u32,
            crate::pac::rcc::vals::Plln::MUL53 => self * 1u32 / 53u32,
            crate::pac::rcc::vals::Plln::MUL54 => self * 1u32 / 54u32,
            crate::pac::rcc::vals::Plln::MUL55 => self * 1u32 / 55u32,
            crate::pac::rcc::vals::Plln::MUL56 => self * 1u32 / 56u32,
            crate::pac::rcc::vals::Plln::MUL57 => self * 1u32 / 57u32,
            crate::pac::rcc::vals::Plln::MUL58 => self * 1u32 / 58u32,
            crate::pac::rcc::vals::Plln::MUL59 => self * 1u32 / 59u32,
            crate::pac::rcc::vals::Plln::MUL60 => self * 1u32 / 60u32,
            crate::pac::rcc::vals::Plln::MUL61 => self * 1u32 / 61u32,
            crate::pac::rcc::vals::Plln::MUL62 => self * 1u32 / 62u32,
            crate::pac::rcc::vals::Plln::MUL63 => self * 1u32 / 63u32,
            crate::pac::rcc::vals::Plln::MUL64 => self * 1u32 / 64u32,
            crate::pac::rcc::vals::Plln::MUL65 => self * 1u32 / 65u32,
            crate::pac::rcc::vals::Plln::MUL66 => self * 1u32 / 66u32,
            crate::pac::rcc::vals::Plln::MUL67 => self * 1u32 / 67u32,
            crate::pac::rcc::vals::Plln::MUL68 => self * 1u32 / 68u32,
            crate::pac::rcc::vals::Plln::MUL69 => self * 1u32 / 69u32,
            crate::pac::rcc::vals::Plln::MUL70 => self * 1u32 / 70u32,
            crate::pac::rcc::vals::Plln::MUL71 => self * 1u32 / 71u32,
            crate::pac::rcc::vals::Plln::MUL72 => self * 1u32 / 72u32,
            crate::pac::rcc::vals::Plln::MUL73 => self * 1u32 / 73u32,
            crate::pac::rcc::vals::Plln::MUL74 => self * 1u32 / 74u32,
            crate::pac::rcc::vals::Plln::MUL75 => self * 1u32 / 75u32,
            crate::pac::rcc::vals::Plln::MUL76 => self * 1u32 / 76u32,
            crate::pac::rcc::vals::Plln::MUL77 => self * 1u32 / 77u32,
            crate::pac::rcc::vals::Plln::MUL78 => self * 1u32 / 78u32,
            crate::pac::rcc::vals::Plln::MUL79 => self * 1u32 / 79u32,
            crate::pac::rcc::vals::Plln::MUL80 => self * 1u32 / 80u32,
            crate::pac::rcc::vals::Plln::MUL81 => self * 1u32 / 81u32,
            crate::pac::rcc::vals::Plln::MUL82 => self * 1u32 / 82u32,
            crate::pac::rcc::vals::Plln::MUL83 => self * 1u32 / 83u32,
            crate::pac::rcc::vals::Plln::MUL84 => self * 1u32 / 84u32,
            crate::pac::rcc::vals::Plln::MUL85 => self * 1u32 / 85u32,
            crate::pac::rcc::vals::Plln::MUL86 => self * 1u32 / 86u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Plln> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Plln) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Plln::MUL8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL9 => self * 9u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL10 => self * 10u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL11 => self * 11u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL12 => self * 12u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL13 => self * 13u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL14 => self * 14u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL15 => self * 15u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL16 => self * 16u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL17 => self * 17u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL18 => self * 18u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL19 => self * 19u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL20 => self * 20u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL21 => self * 21u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL22 => self * 22u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL23 => self * 23u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL24 => self * 24u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL25 => self * 25u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL26 => self * 26u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL27 => self * 27u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL28 => self * 28u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL29 => self * 29u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL30 => self * 30u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL31 => self * 31u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL32 => self * 32u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL33 => self * 33u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL34 => self * 34u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL35 => self * 35u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL36 => self * 36u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL37 => self * 37u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL38 => self * 38u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL39 => self * 39u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL40 => self * 40u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL41 => self * 41u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL42 => self * 42u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL43 => self * 43u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL44 => self * 44u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL45 => self * 45u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL46 => self * 46u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL47 => self * 47u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL48 => self * 48u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL49 => self * 49u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL50 => self * 50u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL51 => self * 51u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL52 => self * 52u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL53 => self * 53u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL54 => self * 54u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL55 => self * 55u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL56 => self * 56u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL57 => self * 57u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL58 => self * 58u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL59 => self * 59u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL60 => self * 60u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL61 => self * 61u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL62 => self * 62u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL63 => self * 63u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL64 => self * 64u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL65 => self * 65u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL66 => self * 66u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL67 => self * 67u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL68 => self * 68u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL69 => self * 69u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL70 => self * 70u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL71 => self * 71u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL72 => self * 72u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL73 => self * 73u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL74 => self * 74u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL75 => self * 75u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL76 => self * 76u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL77 => self * 77u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL78 => self * 78u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL79 => self * 79u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL80 => self * 80u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL81 => self * 81u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL82 => self * 82u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL83 => self * 83u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL84 => self * 84u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL85 => self * 85u32 / 1u32,
            crate::pac::rcc::vals::Plln::MUL86 => self * 86u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllp> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllp) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllp::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Pllp::DIV3 => self * 1u32 / 3u32,
            crate::pac::rcc::vals::Pllp::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Pllp::DIV5 => self * 1u32 / 5u32,
            crate::pac::rcc::vals::Pllp::DIV6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Pllp::DIV7 => self * 1u32 / 7u32,
            crate::pac::rcc::vals::Pllp::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Pllp::DIV9 => self * 1u32 / 9u32,
            crate::pac::rcc::vals::Pllp::DIV10 => self * 1u32 / 10u32,
            crate::pac::rcc::vals::Pllp::DIV11 => self * 1u32 / 11u32,
            crate::pac::rcc::vals::Pllp::DIV12 => self * 1u32 / 12u32,
            crate::pac::rcc::vals::Pllp::DIV13 => self * 1u32 / 13u32,
            crate::pac::rcc::vals::Pllp::DIV14 => self * 1u32 / 14u32,
            crate::pac::rcc::vals::Pllp::DIV15 => self * 1u32 / 15u32,
            crate::pac::rcc::vals::Pllp::DIV16 => self * 1u32 / 16u32,
            crate::pac::rcc::vals::Pllp::DIV17 => self * 1u32 / 17u32,
            crate::pac::rcc::vals::Pllp::DIV18 => self * 1u32 / 18u32,
            crate::pac::rcc::vals::Pllp::DIV19 => self * 1u32 / 19u32,
            crate::pac::rcc::vals::Pllp::DIV20 => self * 1u32 / 20u32,
            crate::pac::rcc::vals::Pllp::DIV21 => self * 1u32 / 21u32,
            crate::pac::rcc::vals::Pllp::DIV22 => self * 1u32 / 22u32,
            crate::pac::rcc::vals::Pllp::DIV23 => self * 1u32 / 23u32,
            crate::pac::rcc::vals::Pllp::DIV24 => self * 1u32 / 24u32,
            crate::pac::rcc::vals::Pllp::DIV25 => self * 1u32 / 25u32,
            crate::pac::rcc::vals::Pllp::DIV26 => self * 1u32 / 26u32,
            crate::pac::rcc::vals::Pllp::DIV27 => self * 1u32 / 27u32,
            crate::pac::rcc::vals::Pllp::DIV28 => self * 1u32 / 28u32,
            crate::pac::rcc::vals::Pllp::DIV29 => self * 1u32 / 29u32,
            crate::pac::rcc::vals::Pllp::DIV30 => self * 1u32 / 30u32,
            crate::pac::rcc::vals::Pllp::DIV31 => self * 1u32 / 31u32,
            crate::pac::rcc::vals::Pllp::DIV32 => self * 1u32 / 32u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllp> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllp) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllp::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV3 => self * 3u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV5 => self * 5u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV7 => self * 7u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV9 => self * 9u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV10 => self * 10u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV11 => self * 11u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV12 => self * 12u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV13 => self * 13u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV14 => self * 14u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV15 => self * 15u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV16 => self * 16u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV17 => self * 17u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV18 => self * 18u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV19 => self * 19u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV20 => self * 20u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV21 => self * 21u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV22 => self * 22u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV23 => self * 23u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV24 => self * 24u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV25 => self * 25u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV26 => self * 26u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV27 => self * 27u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV28 => self * 28u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV29 => self * 29u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV30 => self * 30u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV31 => self * 31u32 / 1u32,
            crate::pac::rcc::vals::Pllp::DIV32 => self * 32u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllq> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllq) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllq::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Pllq::DIV3 => self * 1u32 / 3u32,
            crate::pac::rcc::vals::Pllq::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Pllq::DIV5 => self * 1u32 / 5u32,
            crate::pac::rcc::vals::Pllq::DIV6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Pllq::DIV7 => self * 1u32 / 7u32,
            crate::pac::rcc::vals::Pllq::DIV8 => self * 1u32 / 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllq> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllq) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllq::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV3 => self * 3u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV5 => self * 5u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV7 => self * 7u32 / 1u32,
            crate::pac::rcc::vals::Pllq::DIV8 => self * 8u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllr> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllr) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllr::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Pllr::DIV3 => self * 1u32 / 3u32,
            crate::pac::rcc::vals::Pllr::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Pllr::DIV5 => self * 1u32 / 5u32,
            crate::pac::rcc::vals::Pllr::DIV6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Pllr::DIV7 => self * 1u32 / 7u32,
            crate::pac::rcc::vals::Pllr::DIV8 => self * 1u32 / 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllr> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllr) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllr::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Pllr::DIV3 => self * 3u32 / 1u32,
            crate::pac::rcc::vals::Pllr::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Pllr::DIV5 => self * 5u32 / 1u32,
            crate::pac::rcc::vals::Pllr::DIV6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Pllr::DIV7 => self * 7u32 / 1u32,
            crate::pac::rcc::vals::Pllr::DIV8 => self * 8u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Ppre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Ppre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Ppre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Ppre::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Ppre::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Ppre::DIV16 => self * 1u32 / 16u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Ppre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Ppre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Ppre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV16 => self * 16u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Rngdiv> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Rngdiv) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Rngdiv::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Rngdiv::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Rngdiv::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Rngdiv::DIV8 => self * 1u32 / 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Rngdiv> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Rngdiv) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Rngdiv::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Rngdiv::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Rngdiv::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Rngdiv::DIV8 => self * 8u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
#[allow(non_camel_case_types)]
pub mod peripheral_interrupts {
    pub mod ADC1 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC1_COMP;
    }
    pub mod ADC1_COMMON {}
    pub mod CEC {
        pub type GLOBAL = crate::interrupt::typelevel::CEC;
    }
    pub mod COMP1 {
        pub type WKUP = crate::interrupt::typelevel::ADC1_COMP;
    }
    pub mod COMP2 {
        pub type WKUP = crate::interrupt::typelevel::ADC1_COMP;
    }
    pub mod COMP3 {
        pub type WKUP = crate::interrupt::typelevel::ADC1_COMP;
    }
    pub mod CRC {}
    pub mod CRS {}
    pub mod DAC1 {
        pub type GLOBAL = crate::interrupt::typelevel::TIM6_DAC_LPTIM1;
    }
    pub mod DBGMCU {}
    pub mod DMA1 {
        pub type CH1 = crate::interrupt::typelevel::DMA1_CHANNEL1;
        pub type CH2 = crate::interrupt::typelevel::DMA1_CHANNEL2_3;
        pub type CH3 = crate::interrupt::typelevel::DMA1_CHANNEL2_3;
        pub type CH4 = crate::interrupt::typelevel::DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR;
        pub type CH5 = crate::interrupt::typelevel::DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR;
        pub type CH6 = crate::interrupt::typelevel::DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR;
        pub type CH7 = crate::interrupt::typelevel::DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR;
    }
    pub mod DMA2 {
        pub type CH1 = crate::interrupt::typelevel::DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR;
        pub type CH2 = crate::interrupt::typelevel::DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR;
        pub type CH3 = crate::interrupt::typelevel::DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR;
        pub type CH4 = crate::interrupt::typelevel::DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR;
        pub type CH5 = crate::interrupt::typelevel::DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR;
    }
    pub mod DMAMUX1 {}
    pub mod EXTI {
        pub type EXTI0 = crate::interrupt::typelevel::EXTI0_1;
        pub type EXTI1 = crate::interrupt::typelevel::EXTI0_1;
        pub type EXTI10 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI11 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI12 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI13 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI14 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI15 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI2 = crate::interrupt::typelevel::EXTI2_3;
        pub type EXTI3 = crate::interrupt::typelevel::EXTI2_3;
        pub type EXTI4 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI5 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI6 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI7 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI8 = crate::interrupt::typelevel::EXTI4_15;
        pub type EXTI9 = crate::interrupt::typelevel::EXTI4_15;
    }
    pub mod FDCAN1 {
        pub type IT0 = crate::interrupt::typelevel::TIM16_FDCAN_IT0;
        pub type IT1 = crate::interrupt::typelevel::TIM17_FDCAN_IT1;
    }
    pub mod FDCAN2 {
        pub type IT0 = crate::interrupt::typelevel::TIM16_FDCAN_IT0;
        pub type IT1 = crate::interrupt::typelevel::TIM17_FDCAN_IT1;
    }
    pub mod FDCANRAM1 {}
    pub mod FDCANRAM2 {}
    pub mod FLASH {
        pub type GLOBAL = crate::interrupt::typelevel::FLASH;
    }
    pub mod GPIOA {}
    pub mod GPIOB {}
    pub mod GPIOC {}
    pub mod GPIOD {}
    pub mod GPIOE {}
    pub mod GPIOF {}
    pub mod I2C1 {
        pub type ER = crate::interrupt::typelevel::I2C1;
        pub type EV = crate::interrupt::typelevel::I2C1;
    }
    pub mod I2C2 {
        pub type ER = crate::interrupt::typelevel::I2C2_3;
        pub type EV = crate::interrupt::typelevel::I2C2_3;
    }
    pub mod I2C3 {
        pub type ER = crate::interrupt::typelevel::I2C2_3;
        pub type EV = crate::interrupt::typelevel::I2C2_3;
    }
    pub mod IWDG {}
    pub mod LPTIM1 {
        pub type GLOBAL = crate::interrupt::typelevel::TIM6_DAC_LPTIM1;
    }
    pub mod LPTIM2 {
        pub type GLOBAL = crate::interrupt::typelevel::TIM7_LPTIM2;
    }
    pub mod LPUART1 {
        pub type GLOBAL = crate::interrupt::typelevel::USART3_4_5_6_LPUART1;
    }
    pub mod LPUART2 {
        pub type GLOBAL = crate::interrupt::typelevel::USART2_LPUART2;
    }
    pub mod PWR {}
    pub mod RCC {}
    pub mod RTC {
        pub type TAMP = crate::interrupt::typelevel::RTC_TAMP;
    }
    pub mod SPI1 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI1;
    }
    pub mod SPI2 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI2_3;
    }
    pub mod SPI3 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI2_3;
    }
    pub mod SYSCFG {}
    pub mod TAMP {}
    pub mod TIM1 {
        pub type BRK = crate::interrupt::typelevel::TIM1_BRK_UP_TRG_COM;
        pub type CC = crate::interrupt::typelevel::TIM1_CC;
        pub type COM = crate::interrupt::typelevel::TIM1_BRK_UP_TRG_COM;
        pub type TRG = crate::interrupt::typelevel::TIM1_BRK_UP_TRG_COM;
        pub type UP = crate::interrupt::typelevel::TIM1_BRK_UP_TRG_COM;
    }
    pub mod TIM14 {
        pub type BRK = crate::interrupt::typelevel::TIM14;
        pub type CC = crate::interrupt::typelevel::TIM14;
        pub type COM = crate::interrupt::typelevel::TIM14;
        pub type TRG = crate::interrupt::typelevel::TIM14;
        pub type UP = crate::interrupt::typelevel::TIM14;
    }
    pub mod TIM15 {
        pub type BRK = crate::interrupt::typelevel::TIM15;
        pub type CC = crate::interrupt::typelevel::TIM15;
        pub type COM = crate::interrupt::typelevel::TIM15;
        pub type TRG = crate::interrupt::typelevel::TIM15;
        pub type UP = crate::interrupt::typelevel::TIM15;
    }
    pub mod TIM16 {
        pub type BRK = crate::interrupt::typelevel::TIM16_FDCAN_IT0;
        pub type CC = crate::interrupt::typelevel::TIM16_FDCAN_IT0;
        pub type COM = crate::interrupt::typelevel::TIM16_FDCAN_IT0;
        pub type TRG = crate::interrupt::typelevel::TIM16_FDCAN_IT0;
        pub type UP = crate::interrupt::typelevel::TIM16_FDCAN_IT0;
    }
    pub mod TIM17 {
        pub type BRK = crate::interrupt::typelevel::TIM17_FDCAN_IT1;
        pub type CC = crate::interrupt::typelevel::TIM17_FDCAN_IT1;
        pub type COM = crate::interrupt::typelevel::TIM17_FDCAN_IT1;
        pub type TRG = crate::interrupt::typelevel::TIM17_FDCAN_IT1;
        pub type UP = crate::interrupt::typelevel::TIM17_FDCAN_IT1;
    }
    pub mod TIM2 {
        pub type BRK = crate::interrupt::typelevel::TIM2;
        pub type CC = crate::interrupt::typelevel::TIM2;
        pub type COM = crate::interrupt::typelevel::TIM2;
        pub type TRG = crate::interrupt::typelevel::TIM2;
        pub type UP = crate::interrupt::typelevel::TIM2;
    }
    pub mod TIM3 {
        pub type BRK = crate::interrupt::typelevel::TIM3_TIM4;
        pub type CC = crate::interrupt::typelevel::TIM3_TIM4;
        pub type COM = crate::interrupt::typelevel::TIM3_TIM4;
        pub type TRG = crate::interrupt::typelevel::TIM3_TIM4;
        pub type UP = crate::interrupt::typelevel::TIM3_TIM4;
    }
    pub mod TIM4 {
        pub type BRK = crate::interrupt::typelevel::TIM3_TIM4;
        pub type CC = crate::interrupt::typelevel::TIM3_TIM4;
        pub type COM = crate::interrupt::typelevel::TIM3_TIM4;
        pub type TRG = crate::interrupt::typelevel::TIM3_TIM4;
        pub type UP = crate::interrupt::typelevel::TIM3_TIM4;
    }
    pub mod TIM6 {
        pub type BRK = crate::interrupt::typelevel::TIM6_DAC_LPTIM1;
        pub type CC = crate::interrupt::typelevel::TIM6_DAC_LPTIM1;
        pub type COM = crate::interrupt::typelevel::TIM6_DAC_LPTIM1;
        pub type TRG = crate::interrupt::typelevel::TIM6_DAC_LPTIM1;
        pub type UP = crate::interrupt::typelevel::TIM6_DAC_LPTIM1;
    }
    pub mod TIM7 {
        pub type BRK = crate::interrupt::typelevel::TIM7_LPTIM2;
        pub type CC = crate::interrupt::typelevel::TIM7_LPTIM2;
        pub type COM = crate::interrupt::typelevel::TIM7_LPTIM2;
        pub type TRG = crate::interrupt::typelevel::TIM7_LPTIM2;
        pub type UP = crate::interrupt::typelevel::TIM7_LPTIM2;
    }
    pub mod UCPD1 {
        pub type GLOBAL = crate::interrupt::typelevel::USB_UCPD1_2;
    }
    pub mod UCPD2 {
        pub type GLOBAL = crate::interrupt::typelevel::USB_UCPD1_2;
    }
    pub mod UID {}
    pub mod USART1 {
        pub type GLOBAL = crate::interrupt::typelevel::USART1;
    }
    pub mod USART2 {
        pub type GLOBAL = crate::interrupt::typelevel::USART2_LPUART2;
    }
    pub mod USART3 {
        pub type GLOBAL = crate::interrupt::typelevel::USART3_4_5_6_LPUART1;
    }
    pub mod USART4 {
        pub type GLOBAL = crate::interrupt::typelevel::USART3_4_5_6_LPUART1;
    }
    pub mod USART5 {
        pub type GLOBAL = crate::interrupt::typelevel::USART3_4_5_6_LPUART1;
    }
    pub mod USART6 {
        pub type GLOBAL = crate::interrupt::typelevel::USART3_4_5_6_LPUART1;
    }
    pub mod USB {
        pub type HP = crate::interrupt::typelevel::USB_UCPD1_2;
        pub type LP = crate::interrupt::typelevel::USB_UCPD1_2;
        pub type WKUP = crate::interrupt::typelevel::USB_UCPD1_2;
    }
    pub mod USBRAM {}
    pub mod VREFBUF {}
    pub mod WWDG {
        pub type GLOBAL = crate::interrupt::typelevel::WWDG;
        pub type RST = crate::interrupt::typelevel::WWDG;
    }
}
dma_channel_impl!(DMA1_CH1, 0u8);
dma_channel_impl!(DMA1_CH2, 1u8);
dma_channel_impl!(DMA1_CH3, 2u8);
dma_channel_impl!(DMA1_CH4, 3u8);
dma_channel_impl!(DMA1_CH5, 4u8);
dma_channel_impl!(DMA1_CH6, 5u8);
dma_channel_impl!(DMA1_CH7, 6u8);
dma_channel_impl!(DMA2_CH1, 7u8);
dma_channel_impl!(DMA2_CH2, 8u8);
dma_channel_impl!(DMA2_CH3, 9u8);
dma_channel_impl!(DMA2_CH4, 10u8);
dma_channel_impl!(DMA2_CH5, 11u8);
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR() {
    <crate::peripherals::DMA1_CH4 as crate::dma::ChannelInterrupt>::on_irq();
    <crate::peripherals::DMA1_CH5 as crate::dma::ChannelInterrupt>::on_irq();
    <crate::peripherals::DMA1_CH6 as crate::dma::ChannelInterrupt>::on_irq();
    <crate::peripherals::DMA1_CH7 as crate::dma::ChannelInterrupt>::on_irq();
    <crate::peripherals::DMA2_CH1 as crate::dma::ChannelInterrupt>::on_irq();
    <crate::peripherals::DMA2_CH2 as crate::dma::ChannelInterrupt>::on_irq();
    <crate::peripherals::DMA2_CH3 as crate::dma::ChannelInterrupt>::on_irq();
    <crate::peripherals::DMA2_CH4 as crate::dma::ChannelInterrupt>::on_irq();
    <crate::peripherals::DMA2_CH5 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL1() {
    <crate::peripherals::DMA1_CH1 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL2_3() {
    <crate::peripherals::DMA1_CH2 as crate::dma::ChannelInterrupt>::on_irq();
    <crate::peripherals::DMA1_CH3 as crate::dma::ChannelInterrupt>::on_irq();
}
pub(crate) const DMA_CHANNELS: &[crate::dma::ChannelInfo] = &[
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 0usize,
        dmamux: crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 0usize,
        },
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 1usize,
        dmamux: crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 1usize,
        },
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 2usize,
        dmamux: crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 2usize,
        },
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 3usize,
        dmamux: crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 3usize,
        },
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 4usize,
        dmamux: crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 4usize,
        },
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 5usize,
        dmamux: crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 5usize,
        },
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 6usize,
        dmamux: crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 6usize,
        },
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 0usize,
        dmamux: crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 7usize,
        },
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 1usize,
        dmamux: crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 8usize,
        },
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 2usize,
        dmamux: crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 9usize,
        },
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 3usize,
        dmamux: crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 10usize,
        },
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA2),
        num: 4usize,
        dmamux: crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 11usize,
        },
    },
];
pub fn gpio_block(n: usize) -> crate::pac::gpio::Gpio {
    unsafe { crate::pac::gpio::Gpio::from_ptr((1342177280usize + 1024usize * n) as _) }
}
pub const FLASH_BASE: usize = 134217728usize;
pub const FLASH_SIZE: usize = 524288usize;
pub const WRITE_SIZE: usize = 8usize;
