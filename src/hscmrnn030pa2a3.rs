use defmt::*; 
use embassy_stm32::{
    Peri,
    gpio::{Flex, Speed},
    i2c::{
        Config as I2cconfig, Error, ErrorInterruptHandler, EventInterruptHandler, I2c,
        Instance as I2cInstance, Master, RxDma, SclPin, SdaPin, TxDma,
    },
    interrupt::typelevel::Binding,
    mode::Async,
};
use embassy_time::Timer; 
use {defmt_rtt as _, panic_probe as _};

const ADDR_OUTPUT: u8 = 0x28;

pub trait Irqs<'d, T: I2cInstance> = Binding<T::EventInterrupt, EventInterruptHandler<T>>
    + Binding<T::ErrorInterrupt, ErrorInterruptHandler<T>>
    + 'd;

#[derive(Copy, Clone, defmt::Format)]
pub enum MyChild {
    NotInitialized,
    I2cError(Error),
}

#[derive(Copy, Clone, defmt::Format)]
pub struct Raw {
    pub status: u8,            // 2 bit
    pub bridge_data: u16,      //14 bit
    pub temperature_data: u16, //14 bit
}

impl Raw {
    pub fn baro_temp_convert(&self) -> f32 {
        (self.temperature_data as f32 / 2047.0) * 200.0 - 50.0 // Temprature = 11 Bit -> 2^11 = 2048
        // Temp Range -50...150 C -> 150-(-50) = 200
        // Offset -50, so raw_temp = 0 -> -50 C
    }

    pub fn baro_pressure_convert_pa(&self) -> f32 {
        const OUT_MIN: f32 = 1638.0; //10% of 2^14
        const OUT_MAX: f32 = 14745.0; //90% of 2^14
        const PRESSURE_RANGE_PA: f32 = 206_842.0; //30 psi in PA

        let c = (self.bridge_data as f32).clamp(OUT_MIN, OUT_MAX);

        (c - OUT_MIN) * PRESSURE_RANGE_PA / (OUT_MAX - OUT_MIN)
    }
}

pub struct Baro<'d, T, SCL, SDA, B, TX, RX>
where
    T: I2cInstance,
    SCL: SclPin<T>,
    SDA: SdaPin<T>,
    B: Irqs<'d, T>,
    TX: TxDma<T>,
    RX: RxDma<T>,
{
    i2c_peri: Option<Peri<'d, T>>,
    scl: Option<Peri<'d, SCL>>,
    sda: Option<Peri<'d, SDA>>,
    irq: Option<B>,
    tx_dma: Option<Peri<'d, TX>>,
    rx_dma: Option<Peri<'d, RX>>,
    cfg: I2cconfig,
    err_cnt: u8,
    i2c: Option<I2c<'d, Async, Master>>,
}

impl<'d, T, SCL, SDA, B, TX, RX> Baro<'d, T, SCL, SDA, B, TX, RX>
where
    T: I2cInstance,
    SCL: SclPin<T>,
    SDA: SdaPin<T>,
    B: Irqs<'d, T>,
    TX: TxDma<T>,
    RX: RxDma<T>,
{
    pub fn new(
        i2c_peri: Peri<'d, T>,
        scl: Peri<'d, SCL>,
        sda: Peri<'d, SDA>,
        irq: B,
        tx_dma: Peri<'d, TX>,
        rx_dma: Peri<'d, RX>,
        cfg: I2cconfig,
    ) -> Self {
        //let mut i2c = I2c::new(i2c_peri, scl, sda, irq, tx_dma, rx_dma, cfg);
        let mut baro_driver = Self {
            i2c_peri: Some(i2c_peri),
            scl: Some(scl),
            sda: Some(sda),
            irq: Some(irq),
            tx_dma: Some(tx_dma),
            rx_dma: Some(rx_dma),
            cfg,
            err_cnt: 0,
            i2c: None,
        };
        baro_driver.i2c = Some(I2c::new(
            baro_driver.i2c_peri.take().unwrap(),
            baro_driver.scl.take().unwrap(),
            baro_driver.sda.take().unwrap(),
            baro_driver.irq.take().unwrap(),
            baro_driver.tx_dma.take().unwrap(),
            baro_driver.rx_dma.take().unwrap(),
            baro_driver.cfg,
        ));
        baro_driver
    }

    pub async fn read_out(&mut self) -> Result<Raw, MyChild> {
        let mut raw_data_arr = [0u8; 4];
        match self
            .i2c
            .as_mut()
            .ok_or(MyChild::NotInitialized)?
            .read(ADDR_OUTPUT, &mut raw_data_arr)
            .await
        {
            Ok(()) => {
                self.err_cnt = 0;
                Ok(Raw {
                    status: (raw_data_arr[0] & 0xC0) >> 6,
                    bridge_data: (((raw_data_arr[0] & 0x3F) as u16) << 8)
                        | (raw_data_arr[1] as u16),
                    temperature_data: (((raw_data_arr[2] as u16) << 8)
                        | ((raw_data_arr[3] & 0xE0) as u16))
                        >> 5,
                })
            }
            Err(e) => {
                if self.err_cnt > 5 {
                    self.i2c = None;
                    let scl = self.scl.take().unwrap();
                    let sda = self.sda.take().unwrap();

                    self.manual_bus_recovery(scl, sda).await;
                    self.i2c = Some(I2c::new(
                        self.i2c_peri.take().unwrap(),
                        self.scl.take().unwrap(),
                        self.sda.take().unwrap(),
                        self.irq.take().unwrap(),
                        self.tx_dma.take().unwrap(),
                        self.rx_dma.take().unwrap(),
                        self.cfg,
                    ));
                }
                self.err_cnt += 1;
                error!("I2c Error: {:?}", e);
                Err(MyChild::I2cError(e))
            }
        }
    }

    pub async fn manual_bus_recovery(&mut self, scl_pin: Peri<'d, SCL>, sda_pin: Peri<'d, SDA>) {
        let mut scl = Flex::new(scl_pin);
        let mut sda = Flex::new(sda_pin);

        scl.set_as_output(Speed::Medium);
        scl.set_high();
        sda.set_as_output(Speed::Medium);
        sda.set_high();
        for _ in 0..9 {
            scl.set_low();
            Timer::after_micros(10).await;
            scl.set_high();
            Timer::after_micros(10).await;
        }

        // Stop-Condition
        scl.set_low();
        Timer::after_micros(10).await;
        sda.set_as_output(Speed::Medium);
        sda.set_low();
        Timer::after_micros(10).await;
        scl.set_high();
        Timer::after_micros(10).await;
        sda.set_high();
        Timer::after_micros(10).await;
    }
}
