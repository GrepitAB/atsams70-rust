use crate::{ehal, nb};
use ehal::spi;
use ehal::spi::FullDuplex;
use nb::block;
use cortex_m;

use crate::target_device::SPI0;
#[cfg(any(
        feature = "sams70n19b",
        feature = "sams70n20b",
        feature = "sams70n21b",
        feature = "sams70q19b",
        feature = "sams70q20b",
        feature = "sams70q21b",
        feature = "same70n19b",
        feature = "same70n20b",
        feature = "same70n21b",
        feature = "same70q19b",
        feature = "same70q20b",
        feature = "same70q21b",
))]
use crate::target_device::{QSPI, USART0, USART1, USART2};
#[cfg(any(
        feature = "sams70q19b",
        feature = "sams70q20b",
        feature = "sams70q21b",
        feature = "same70q19b",
        feature = "same70q20b",
        feature = "same70q21b",
))]
use crate::target_device::SPI1;

use crate::target_device::spi0::RegisterBlock as SPIRegisterBlock;
#[cfg(any(
        feature = "sams70n19b",
        feature = "sams70n20b",
        feature = "sams70n21b",
        feature = "sams70q19b",
        feature = "sams70q20b",
        feature = "sams70q21b",
        feature = "same70n19b",
        feature = "same70n20b",
        feature = "same70n21b",
        feature = "same70q19b",
        feature = "same70q20b",
        feature = "same70q21b",
))]
use crate::target_device::usart0::RegisterBlock as USARTRegisterBlock;

pub struct Spi<P> {
    peripheral: P,
}

pub type Spi0 = Spi<SPI0>;
#[cfg(any(
        feature = "sams70q19b",
        feature = "sams70q20b",
        feature = "sams70q21b",
        feature = "same70q19b",
        feature = "same70q20b",
        feature = "same70q21b",
))]
pub type Spi1 = Spi<SPI1>;
#[cfg(any(
        feature = "sams70n19b",
        feature = "sams70n20b",
        feature = "sams70n21b",
        feature = "sams70q19b",
        feature = "sams70q20b",
        feature = "sams70q21b",
        feature = "same70n19b",
        feature = "same70n20b",
        feature = "same70n21b",
        feature = "same70q19b",
        feature = "same70q20b",
        feature = "same70q21b",
))]
pub type Qspi = Spi<QSPI>;
#[cfg(any(
        feature = "sams70n19b",
        feature = "sams70n20b",
        feature = "sams70n21b",
        feature = "sams70q19b",
        feature = "sams70q20b",
        feature = "sams70q21b",
        feature = "same70n19b",
        feature = "same70n20b",
        feature = "same70n21b",
        feature = "same70q19b",
        feature = "same70q20b",
        feature = "same70q21b",
))]
pub type Usart0Spi = Spi<USART0>;
#[cfg(any(
        feature = "sams70n19b",
        feature = "sams70n20b",
        feature = "sams70n21b",
        feature = "sams70q19b",
        feature = "sams70q20b",
        feature = "sams70q21b",
        feature = "same70n19b",
        feature = "same70n20b",
        feature = "same70n21b",
        feature = "same70q19b",
        feature = "same70q20b",
        feature = "same70q21b",
))]
pub type Usart1Spi = Spi<USART1>;
#[cfg(any(
        feature = "sams70n19b",
        feature = "sams70n20b",
        feature = "sams70n21b",
        feature = "sams70q19b",
        feature = "sams70q20b",
        feature = "sams70q21b",
        feature = "same70n19b",
        feature = "same70n20b",
        feature = "same70n21b",
        feature = "same70q19b",
        feature = "same70q20b",
        feature = "same70q21b",
))]
pub type Usart2Spi = Spi<USART2>;


#[derive(Debug)]
pub enum Error {

}

impl ehal::spi::FullDuplex<u8> for Spi<SPI0> {
    type Error = Error;
    
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        read_spi(&*self.peripheral)
    }

    fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        send_spi(&*self.peripheral, word)
    }


}

impl ehal::spi::FullDuplex<u8> for Spi<USART1> {
    type Error = Error;
    
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        read_usart(&*self.peripheral)
    }

    fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        send_usart(&*self.peripheral, word)
    }


}

// impl ehal::blocking::spi::transfer::Default<u8> for Spi<SPI0> {}
// impl ehal::blocking::spi::transfer::Default<u8> for Spi<USART1> {}
impl ehal::blocking::spi::write::Default<u8> for Spi<SPI0> {}

impl ehal::blocking::spi::Write<u8> for Spi<USART1> {

    type Error = Error;

    fn write(&mut self, words: &[u8]) -> Result<(), Error> {
        self.set_cs_low();
        // Delay because 1.75us between CS low and SCLK high is needed for mc33664
        cortex_m::asm::delay(10);
        words.iter().for_each( |word| {
            block!(self.send(word.clone())).unwrap();
            block!(self.read()).unwrap();
        });

        self.set_cs_high();

        Ok(())
    }
}

impl ehal::blocking::spi::Transfer<u8> for Spi<SPI0> {

    type Error = Error;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Error> {
        match self.peripheral.spi_mr.read().pcs().bits() {
            _ => ()
        }
        self.peripheral.spi_csr[1].modify( |_,w| w.csaat().set_bit());


        let l = words.len();
        words.iter_mut().take(l-1).for_each( |word| {
            block!(self.send(word.clone())).unwrap();
            *word = block!(self.read()).unwrap();
        });

        // One byte has to be saved until after csaat is reset, or CS will never go high
        self.peripheral.spi_csr[1].modify(|_,w| w.csaat().clear_bit());
        if let Some(word) = words.iter_mut().last() {
            block!(self.send(word.clone()))?;
            *word = block!(self.read())?;
        }

        Ok(words)
    }

}

impl ehal::blocking::spi::Transfer<u8> for Spi<USART1> {

    type Error = Error;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Error> {
        self.set_cs_low();
        words.iter_mut().for_each( |word| {
            block!(self.send(word.clone())).unwrap();
            *word = block!(self.read()).unwrap();
        });

        self.set_cs_high();

        Ok(words)
    }
}

impl Spi<SPI0> {

    pub fn new(spi: SPI0) -> Self {
        Spi {
            peripheral: spi,
        }
    }

    pub fn set_spi_mode( &mut self, mode: spi::Mode) {
        set_mode_spi(&*self.peripheral, mode);
    }

    pub fn spi_mode( mut self, mode: spi::Mode) -> Self {
        self.set_spi_mode(mode);
        self
    }

}

impl Spi<USART1> {
    pub fn new(spi: USART1) -> Self {
        spi.us_mr_spi_mode().modify(|_,w| w.usart_mode().spi_master());
        Spi {
            peripheral: spi,
        }
    }
    
    pub fn set_spi_mode( &mut self, mode: spi::Mode) {
        set_mode_usart(&*self.peripheral, mode);
    }

    pub fn spi_mode( mut self, mode: spi::Mode) -> Self {
        self.set_spi_mode(mode);
        self
    }

    #[inline]
    pub fn set_cs_low(&mut self) {
        self.peripheral.us_cr_spi_mode().write(|w| w.fcs().set_bit());
    }

    #[inline]
    pub fn set_cs_high(&mut self) {
        self.peripheral.us_cr_spi_mode().write(|w| w.rcs().set_bit());
    }

    // These two functions should not touch the interrupts
    #[inline]
    pub fn client_mode(&mut self) {
        self.peripheral.us_mr_spi_mode().modify(|_,w| w.usart_mode().spi_slave());
        self.peripheral.us_ier_spi_mode().write(|w| w.rxrdy().set_bit());
        // self.peripheral.us_cr_spi_mode().write(|w| w.rstrx().set_bit().rsttx().set_bit());
    }

    #[inline]
    pub fn host_mode(&mut self) {
        self.peripheral.us_mr_spi_mode().modify(|_,w| w.usart_mode().spi_master());
        self.peripheral.us_idr_spi_mode().write(|w| w.rxrdy().set_bit());
        // self.peripheral.us_cr_spi_mode().write(|w| w.rstrx().set_bit().rsttx().set_bit());
    }

}

fn read_spi( regs: &SPIRegisterBlock) -> nb::Result<u8, Error> {
    if regs.spi_sr.read().rdrf().bit_is_clear() {
        Err(nb::Error::WouldBlock)
    } else {
        Ok(regs.spi_rdr.read().rd().bits() as u8)
    }
}

fn send_spi( regs: &SPIRegisterBlock, word: u8) -> nb::Result<(), Error> {
    if regs.spi_sr.read().tdre().bit_is_clear() {
        Err(nb::Error::WouldBlock)
    } else {
        regs.spi_tdr.write(|w| unsafe{w.td().bits(word as u16)});
        Ok(())
    }
}

fn set_mode_spi( regs: &SPIRegisterBlock, mode: spi::Mode) {
    match mode {
        spi::MODE_0 => {
            regs.spi_csr[0].modify(|_,w| w.cpol().idle_low().ncpha().valid_trailing_edge());
        }
        spi::MODE_1 => {
            regs.spi_csr[0].modify(|_,w| w.cpol().idle_low().ncpha().valid_leading_edge());
        }
        spi::MODE_2 => {
            regs.spi_csr[0].modify(|_,w| w.cpol().idle_high().ncpha().valid_trailing_edge());
        }
        spi::MODE_3 => {
            regs.spi_csr[0].modify(|_,w| w.cpol().idle_high().ncpha().valid_leading_edge());
        }
    }
}

fn read_usart( regs: &USARTRegisterBlock) -> nb::Result<u8, Error> {
    if regs.us_csr_spi_mode().read().rxrdy().bit_is_clear() {
        Err(nb::Error::WouldBlock)
    } else {
        Ok(regs.us_rhr.read().rxchr().bits() as u8)
    }
}

fn send_usart( regs: &USARTRegisterBlock, word: u8) -> nb::Result<(), Error> {
    if regs.us_csr_spi_mode().read().txrdy().bit_is_clear() {
        Err(nb::Error::WouldBlock)
    } else {
        regs.us_thr.write(|w| unsafe {w.txchr().bits(word as u16)});
        Ok(())
    }
}
fn set_mode_usart( regs: &USARTRegisterBlock, mode: spi::Mode) {
    match mode {
        spi::MODE_0 => {
            regs.us_mr_spi_mode().modify(|_,w| w.cpol().clear_bit().cpha().set_bit());
        }
        spi::MODE_1 => {
            regs.us_mr_spi_mode().modify(|_,w| w.cpol().clear_bit().cpha().clear_bit());
        }
        spi::MODE_2 => {
            regs.us_mr_spi_mode().modify(|_,w| w.cpol().set_bit().cpha().set_bit());
        }
        spi::MODE_3 => {
            regs.us_mr_spi_mode().modify(|_,w| w.cpol().set_bit().cpha().clear_bit());
        }
    }
}

