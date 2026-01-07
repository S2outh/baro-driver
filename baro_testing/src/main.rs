#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use hscmrnn030pa::driver::*;
use defmt::*; 
use embassy_executor::{Spawner};
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_stm32::{
    i2c::{Config as I2cconfig},
    time::Hertz,
};
use {defmt_rtt as _, panic_probe as _};



bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut cfg = I2cconfig::default();
    cfg.frequency = Hertz(100_000);
    cfg.sda_pullup = true;
    cfg.scl_pullup = true;

    //scl = pb8, sda = pb9

    info!("Hello World!");
    let mut baro = Baro::new(p.I2C1, p.PB8, p.PB9, Irqs, p.DMA1_CH1, p.DMA1_CH2, cfg);

    loop {
        let result = baro.read_out().await;
        match result {
            Ok(raw) => {
                let temp = raw.baro_temp_convert();
                let pressure = raw.baro_pressure_convert_pa();

                info!("Temp: {}, Pressure: {}", temp, pressure);
            }
            Err(e) => {
                error!("ERROR: {:?}", e);
            }
        }
    }
}
