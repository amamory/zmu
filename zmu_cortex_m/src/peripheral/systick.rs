//!
//! Cortex System Tick Simulation
//!

use crate::core::exception::Exception;
use crate::core::exception::ExceptionHandling;
use crate::Processor;

///
/// Register API for ```SysTick``` peripheral
///
pub trait SysTick {
    ///
    /// write to SYST_RVR, reload value register
    ///
    fn write_syst_rvr(&mut self, value: u32);

    ///
    /// write to current value register
    ///
    fn write_syst_cvr(&mut self, _value: u32);

    ///
    /// write to control and status register
    ///
    fn write_syst_csr(&mut self, value: u32);

    ///
    /// Read control and status register
    ///
    fn read_syst_csr(&self) -> u32;

    ///
    /// Read reload value register
    ///
    fn read_syst_rvr(&self) -> u32;

    ///
    /// Read current value register
    ///
    fn read_syst_cvr(&self) -> u32;

    ///
    /// Read calibration register value
    ///
    fn read_syst_calib(&self) -> u32;

    ///
    /// Step systick one clock cycle forward
    ///
    fn syst_step(&mut self);
}

const SYST_ENABLE: u32 = 1;
const SYST_TICKINT: u32 = 1 << 1;
const SYST_COUNTFLAG: u32 = 1 << 16;

impl SysTick for Processor {
    fn write_syst_rvr(&mut self, value: u32) {
        self.syst_rvr = value & 0x00ff_ffff;
    }

    fn write_syst_cvr(&mut self, _value: u32) {
        self.syst_cvr = 0;
        self.syst_csr &= SYST_COUNTFLAG ^ 0xffff_ffff;
    }

    fn write_syst_csr(&mut self, value: u32) {
        // is it an activation?
        if (self.syst_csr & SYST_ENABLE == 0) && (value & SYST_ENABLE == SYST_ENABLE) {
            // reload value -> counter value
            self.syst_cvr = self.syst_rvr & 0x00ff_ffff;
        }

        self.syst_csr &= 0b_111 ^ 0xffff_ffff;
        self.syst_csr |= value & 0b_111;
    }

    fn read_syst_csr(&self) -> u32 {
        self.syst_csr
    }

    fn read_syst_rvr(&self) -> u32 {
        self.syst_rvr
    }

    fn read_syst_cvr(&self) -> u32 {
        self.syst_cvr
    }

    fn read_syst_calib(&self) -> u32 {
        0
    }

    #[inline(always)]
    fn syst_step(&mut self) {
        if (self.syst_csr & SYST_ENABLE) == SYST_ENABLE {
            self.syst_cvr = self.syst_cvr.saturating_sub(1);
            self.syst_cvr &= 0x00ff_ffff;

            // reach 0?
            if self.syst_cvr == 0 {
                // reload -> to counter value
                self.syst_cvr = self.syst_rvr & 0x00ff_ffff;
                self.syst_csr |= SYST_COUNTFLAG;
                if (self.syst_csr & SYST_TICKINT) == SYST_TICKINT {
                    self.set_exception_pending(Exception::SysTick);
                }
            }
        }
    }
}
