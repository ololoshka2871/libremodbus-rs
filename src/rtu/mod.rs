use crate::bindings::{
    mb_disable, mb_enable, mb_err_enum_MB_ENOERR, mb_init_rtu, mb_inst_struct, mb_poll,
    mb_port_base_struct, mb_port_ser_parity_enum_MB_PAR_NONE, mb_rtu_rcv_fsm, mb_rtu_snd_fsm,
    mb_rtu_tmr_35_expired, mb_rtu_tr_struct,
};

pub struct Rtu {
    inst: mb_inst_struct,
    transport: mb_rtu_tr_struct,
    port_obj: mb_port_base_struct,
}

unsafe impl Send for Rtu {}

impl Rtu {
    pub fn init(
        slave_addr: u8,
        port: *mut dyn super::SerialInterface,
        boudrate: u32,
        timer: *mut dyn super::TimerInterface,
    ) -> Self {
        let mut res = unsafe {
            Self {
                inst: core::mem::MaybeUninit::zeroed().assume_init(),
                transport: core::mem::MaybeUninit::zeroed().assume_init(),
                port_obj: core::mem::MaybeUninit::zeroed().assume_init(),
            }
        };

        unsafe {
            super::PORT.replace(port.to_raw_parts());
            super::TIMER.replace(timer.to_raw_parts());
        }

        //let p = (&res as &dyn super::StaticDispatcher) as *const _;
        //res.inst.transport = &mut res as *mut Self as *mut crate::bindings::mb_trans_base_struct;
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

impl super::MBInterface for Rtu {
    fn enable(&mut self) -> bool {
        unsafe { mb_enable(&mut self.inst) == mb_err_enum_MB_ENOERR }
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
