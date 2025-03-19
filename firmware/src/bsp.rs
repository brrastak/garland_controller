
pub use stm32f1xx_hal as hal;
pub use hal:: {
        prelude::*,
        rcc::*,
        gpio::*,
        spi::*,
        pac::Peripherals,
        adc::*,
    };


pub struct Board {

    pub clocks: Clocks,
    pub led: ErasedPin<Output>,
    pub spi: Spi<hal::pac::SPI1, Spi1Remap, (NoSck, NoMiso, Pin<'B', 5, Alternate>), u8>,
    pub adc: AdcWrapper,
}

impl Board {

    pub fn new(p: Peripherals) -> Self {

        let mut flash = p.FLASH.constrain();
        let rcc = p.RCC.constrain();
        let mut afio = p.AFIO.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .freeze(&mut flash.acr);

        let mut gpioa = p.GPIOA.split();
        let mut gpiob = p.GPIOB.split();

        Board {

            clocks,
            led: gpiob.pb15.into_push_pull_output(&mut gpiob.crh).erase(),
            spi: Spi::spi1(
                p.SPI1,
                (NoSck, NoMiso, gpiob.pb5.into_alternate_push_pull(&mut gpiob.crl)),
                &mut afio.mapr,
                Mode {
                    polarity: Polarity::IdleLow,
                    phase: Phase::CaptureOnFirstTransition,
                },
                3.MHz(),
                clocks
            ),
            adc: AdcWrapper {
                adc_instance: Adc::adc1(p.ADC1, clocks),
                adc_pin: gpioa.pa0.into_analog(&mut gpioa.crl)
            }
            
        }
    }
}

pub struct AdcWrapper {
    adc_instance: Adc<hal::pac::ADC1>,
    adc_pin: Pin<'A', 0, Analog>,
}

impl AdcWrapper {
    
    pub fn read(&mut self) -> u16 {
        self.adc_instance.read(&mut self.adc_pin).unwrap()
    }
}
