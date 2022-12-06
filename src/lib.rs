#![no_std]
#![feature(ptr_metadata)]

mod bindings;
mod port;

#[cfg(feature = "rtu")]
mod rtu;

#[cfg(feature = "rtu")]
pub use rtu::Rtu;

pub enum Parity {
    None,
    Even,
    ODD,
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

use core::ptr::Pointee;

pub(crate) static mut PORT: Option<(*mut (), <dyn SerialInterface as Pointee>::Metadata)> = None;
pub(crate) static mut TIMER: Option<(*mut (), <dyn TimerInterface as Pointee>::Metadata)> = None;
