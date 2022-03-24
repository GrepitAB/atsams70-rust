use crate::{ehal, nb};

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


fn read_spi( regs: &SPIRegisterBlock) -> nb::Result<u8, Error> {
    if regs.spi_sr.read().txempty().bit_is_clear() {
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
