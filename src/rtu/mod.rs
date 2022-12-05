use crate::bindings::{
    mb_err_enum_MB_ENOERR, mb_init_rtu, mb_inst_struct, mb_port_base_struct,
    mb_port_ser_parity_enum_MB_PAR_NONE, mb_rtu_tr_struct,
};

pub struct Rtu<SERIAL, TIM> {
    port: SERIAL,
    timer: TIM,

    inst: mb_inst_struct,
    transport: mb_rtu_tr_struct,
    port_obj: mb_port_base_struct,
}

impl<SERIAL, TIM> Rtu<SERIAL, TIM>
where
    SERIAL: embedded_hal::serial::Read<u8> + embedded_hal::serial::Write<u8>,
    TIM: fugit_timer::Timer<1_000_000>,
{
    pub fn init(slave_addr: u8, port: SERIAL, boudrate: u32, timer: TIM) -> Self {
        let mut res = unsafe {
            Self {
                port,
                timer,
                inst: core::mem::MaybeUninit::zeroed().assume_init(),
                transport: core::mem::MaybeUninit::zeroed().assume_init(),
                port_obj: core::mem::MaybeUninit::zeroed().assume_init(),
            }
        };
        let err = unsafe {
            mb_init_rtu(
                &mut res.inst,
                &mut res.transport,
                slave_addr,
                &mut res.port_obj,
                boudrate,
                mb_port_ser_parity_enum_MB_PAR_NONE,
            )
        };

        assert!(err == mb_err_enum_MB_ENOERR);

        res
    }
}

impl<SERIAL, TIM> super::StaticDispatcher for Rtu<SERIAL, TIM>
where
    SERIAL: embedded_hal::serial::Read<u8> + embedded_hal::serial::Write<u8>,
    TIM: fugit_timer::Timer<1_000_000>,
{
}

impl<SERIAL, TIM> super::MBInterface for Rtu<SERIAL, TIM>
where
    SERIAL: embedded_hal::serial::Read<u8> + embedded_hal::serial::Write<u8>,
{
    fn enable(&mut self) {
        todo!()
    }

    fn disable(&mut self) {
        todo!()
    }

    fn pool(&mut self) {
        todo!()
    }
}

impl<SERIAL, TIM> super::SerialEvent for Rtu<SERIAL, TIM>
where
    SERIAL: embedded_hal::serial::Read<u8> + embedded_hal::serial::Write<u8>,
{
    fn on_rx(&mut self) {
        todo!()
    }

    fn on_tx(&mut self) {
        todo!()
    }
}

impl<SERIAL, TIM> super::MBTimerEvent for Rtu<SERIAL, TIM>
where
    TIM: fugit_timer::Timer<1_000_000>,
{
    fn on_timer(&mut self) {
        todo!()
    }
}
