use crate::bindings::{
    mb_close, mb_disable, mb_enable, mb_err_enum_MB_ENOERR, mb_init_rtu, mb_inst_struct, mb_poll,
    mb_port_base_struct, mb_port_ser_parity_enum, mb_rtu_rcv_fsm, mb_rtu_snd_fsm,
    mb_rtu_snd_state_enum_MB_RTU_TX_STATE_IDLE, mb_rtu_tmr_35_expired, mb_rtu_tr_struct,
};

pub struct Rtu {
    inst: mb_inst_struct,
    transport: mb_rtu_tr_struct,
    port_obj: mb_port_base_struct,

    slave_addr: u8,
    boudrate: u32,
    parity: crate::Parity,
}

unsafe impl Send for Rtu {}

impl Rtu {
    pub fn init(
        slave_addr: u8,
        port: *mut dyn super::SerialInterface,
        boudrate: u32,
        parity: crate::Parity,
        timer: *mut dyn super::TimerInterface,
        data: *mut dyn super::DataInterface,
    ) -> Self {
        unsafe {
            super::PORT.replace(port.to_raw_parts());
            super::TIMER.replace(timer.to_raw_parts());
            super::DATA.replace(data.to_raw_parts());
        }

        unsafe {
            Self {
                inst: core::mem::MaybeUninit::zeroed().assume_init(),
                transport: core::mem::MaybeUninit::zeroed().assume_init(),
                port_obj: core::mem::MaybeUninit::zeroed().assume_init(),

                slave_addr,
                boudrate,
                parity,
            }
        }
    }
}

impl super::MBInterface for Rtu {
    fn enable(&mut self) -> bool {
        unsafe {
            if mb_init_rtu(
                &mut self.inst,
                &mut self.transport,
                self.slave_addr,
                &mut self.port_obj,
                self.boudrate,
                self.parity as mb_port_ser_parity_enum,
            ) != mb_err_enum_MB_ENOERR
            {
                return false;
            }

            mb_enable(&mut self.inst) == mb_err_enum_MB_ENOERR
        }
    }

    fn disable(&mut self) {
        unsafe {
            assert!(mb_disable(&mut self.inst) == mb_err_enum_MB_ENOERR);
        }
    }

    fn pool(&mut self) {
        unsafe {
            assert!(mb_poll(&mut self.inst) == mb_err_enum_MB_ENOERR);
        }
    }

    fn reconfigure(
        &mut self,
        _cs: &cortex_m::interrupt::CriticalSection,
        addr: u8,
        boud: u32,
        parity: crate::Parity,
    ) -> bool {
        self.slave_addr = addr;
        self.boudrate = boud;
        self.parity = parity;

        unsafe {
            if mb_disable(&mut self.inst) != mb_err_enum_MB_ENOERR {
                return false;
            }

            if mb_close(&mut self.inst) != mb_err_enum_MB_ENOERR {
                return false;
            }

            if mb_init_rtu(
                &mut self.inst,
                &mut self.transport,
                self.slave_addr,
                &mut self.port_obj,
                self.boudrate,
                self.parity as mb_port_ser_parity_enum,
            ) != mb_err_enum_MB_ENOERR {
                return false;
            }

            mb_enable(&mut self.inst) == mb_err_enum_MB_ENOERR
        }
    }
}

impl super::SerialEvent for Rtu {
    fn on_rx(&mut self) -> bool {
        unsafe { mb_rtu_rcv_fsm(&mut self.transport) != 0 }
    }

    fn on_tx(&mut self) -> bool {
        unsafe { mb_rtu_snd_fsm(&mut self.transport) != 0 }
    }
}

impl super::MBTimerEvent for Rtu {
    fn on_timer(&mut self) -> bool {
        unsafe { mb_rtu_tmr_35_expired(&mut self.transport) != 0 }
    }
}

impl super::REDEControl for Rtu {
    fn is_tx_finished(&self) -> bool {
        (self.transport.snd_state & 0xff) == mb_rtu_snd_state_enum_MB_RTU_TX_STATE_IDLE
    }

    fn deassert_re_de(&mut self) {
        crate::port::port_serial::get_serial_interface().deassert_re_de();
    }
}
