use std::{cell::RefCell, error::Error, rc::Rc};

use ctru::{prelude::*, services::am::Am};

use crate::{
    c2d::C2D,
    platform::{enable_hight_performance_for_new_3ds, is_new_3ds, setup_log_redirect},
};

pub struct Resource {
    pub soc: Soc,
    pub hid: RefCell<Hid>,
    pub c2d: Rc<C2D>,
    pub apt: Apt,
    _am: Am,
}

impl Resource {
    pub fn new(enable_log_redirect: bool) -> Result<Rc<Self>, Box<dyn Error>> {
        // enable high performance for new 3ds
        if is_new_3ds() {
            enable_hight_performance_for_new_3ds();
        }
        // enable socket for network
        let mut soc = Soc::new()?;
        // set log redirect
        if enable_log_redirect {
            setup_log_redirect(&mut soc);
        }
        // am init
        let _am = Am::new()?;
        // applet init
        let apt = Apt::new()?;
        // hid init
        let hid = Hid::new()?;
        // c2d init
        let c2d = Rc::new(C2D::new()?);

        Ok(Rc::new(Self {
            soc,
            hid: RefCell::new(hid),
            c2d,
            apt,
            _am,
        }))
    }

    pub fn main_loop(&self) -> bool {
        self.apt.main_loop()
    }
}
