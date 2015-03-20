#![allow(dead_code)]
use modules::dma::Readable;
use {adc, dma};

use libc::c_void;
use core::intrinsics::transmute;


#[derive(Copy)]
pub struct Adc {
    pub device: &'static adc::Adc
}

impl Readable for Adc {

    fn as_ptr(&self) -> *mut c_void {
        unsafe { transmute(&self.device.SINGLEDATA) }
    }

    fn inc_size(&self) -> dma::DataInc {
        dma::DataInc::IncNone
    }

    fn size(&self) -> dma::DataSize {
        dma::DataSize::Size1
    }
}
