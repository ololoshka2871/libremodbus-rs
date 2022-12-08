#![no_std]
#![feature(ptr_metadata)]

mod bindings;
mod port;

#[cfg(feature = "rtu")]
mod rtu;

#[cfg(feature = "rtu")]
pub use rtu::Rtu;

#[repr(u32)]
pub enum Parity {
    None = bindings::mb_port_ser_parity_enum_MB_PAR_NONE,
    Even = bindings::mb_port_ser_parity_enum_MB_PAR_EVEN,
    ODD = bindings::mb_port_ser_parity_enum_MB_PAR_ODD,
}

impl From<bindings::mb_port_ser_parity_enum> for Parity {
    fn from(parity: bindings::mb_port_ser_parity_enum) -> Self {
        match parity {
            bindings::mb_port_ser_parity_enum_MB_PAR_EVEN => crate::Parity::Even,
            bindings::mb_port_ser_parity_enum_MB_PAR_ODD => crate::Parity::ODD,
            bindings::mb_port_ser_parity_enum_MB_PAR_NONE => crate::Parity::None,
            _ => unreachable!(),
        }
    }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum MbError {
    MB_ENOREG = bindings::mb_err_enum_MB_ENOREG,
    MB_EINVAL = bindings::mb_err_enum_MB_EINVAL,
    MB_EPORTERR = bindings::mb_err_enum_MB_EPORTERR,
    MB_ENORES = bindings::mb_err_enum_MB_ENORES,
    MB_EIO = bindings::mb_err_enum_MB_EIO,
    MB_EILLSTATE = bindings::mb_err_enum_MB_EILLSTATE,
    MB_ETIMEDOUT = bindings::mb_err_enum_MB_ETIMEDOUT,
    MB_EILLFUNC = bindings::mb_err_enum_MB_EILLFUNC,
    MB_EBUSY = bindings::mb_err_enum_MB_EBUSY,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AccessMode {
    READ = bindings::mb_reg_mode_enum_MB_REG_READ,
    WRITE = bindings::mb_reg_mode_enum_MB_REG_WRITE,
}

impl From<bindings::mb_reg_mode_enum> for AccessMode {
    fn from(mode: bindings::mb_reg_mode_enum) -> Self {
        match mode {
            bindings::mb_reg_mode_enum_MB_REG_READ => AccessMode::READ,
            bindings::mb_reg_mode_enum_MB_REG_WRITE => AccessMode::WRITE,
            _ => unreachable!(),
        }
    }
}

pub trait MBInterface {
    fn enable(&mut self) -> bool;
    fn disable(&mut self);
    fn pool(&mut self);
}

pub trait MBTimerEvent {
    fn on_timer(&mut self) -> bool;
}

pub trait SerialEvent {
    fn on_rx(&mut self) -> bool;
    fn on_tx(&mut self) -> bool;
}

pub trait SerialInterface {
    fn configure(&mut self, boud: u32, data_bits: u8, parity: Parity) -> bool;
    fn close(&mut self);
    fn enable(&mut self, rx_enable: bool, tx_enable: bool);
    fn get_byte(&mut self) -> Option<u8>;
    fn put_byte(&mut self, data: u8) -> bool;
}

pub trait TimerInterface {
    fn set_timeout(&mut self, timeout: fugit_timer::Duration<u32, 1, 1_000_000>) -> bool;
    fn close(&mut self);

    fn start(&mut self);
    fn stop(&mut self);
    fn wait(&mut self);
}

#[allow(unused_variables)]
pub trait DataInterface {
    fn read_inputs(
        &mut self,
        reg_buff: &mut [u8],
        reg_addr: u16,
        reg_num: u16,
    ) -> Result<(), MbError> {
        Err(MbError::MB_ENOREG)
    }
    fn rw_holdings(
        &mut self,
        reg_buff: &mut [u8],
        reg_addr: u16,
        reg_num: u16,
        mode: AccessMode,
    ) -> Result<(), MbError> {
        Err(MbError::MB_ENOREG)
    }
    fn rw_coils(
        &mut self,
        reg_buff: &mut [u8],
        reg_addr: u16,
        reg_num: u16,
        mode: AccessMode,
    ) -> Result<(), MbError> {
        Err(MbError::MB_ENOREG)
    }
    fn read_discretes(
        &mut self,
        reg_buff: &mut [u8],
        reg_addr: u16,
        reg_num: u16,
    ) -> Result<(), MbError> {
        Err(MbError::MB_ENOREG)
    }
}

use core::ptr::Pointee;

pub(crate) static mut PORT: Option<(*mut (), <dyn SerialInterface as Pointee>::Metadata)> = None;
pub(crate) static mut TIMER: Option<(*mut (), <dyn TimerInterface as Pointee>::Metadata)> = None;
pub(crate) static mut DATA: Option<(*mut (), <dyn DataInterface as Pointee>::Metadata)> = None;
