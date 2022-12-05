#![no_std]

mod bindings;
mod port;

#[cfg(feature = "rtu")]
mod rtu;
#[cfg(feature = "rtu")]
pub use rtu::Rtu;

pub trait MBInterface {
    fn enable(&mut self);
    fn disable(&mut self);
    fn pool(&mut self);
}

pub trait MBTimerEvent {
    fn on_timer(&mut self);
}

pub trait SerialEvent {
    fn on_rx(&mut self);
    fn on_tx(&mut self);
}

pub(crate) trait StaticDispatcher {}

extern "C" {
    fn get_context() -> *mut dyn StaticDispatcher;
    fn set_context(ctx: *const dyn StaticDispatcher);
}
