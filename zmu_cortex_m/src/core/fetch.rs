//!
//! Fetching instructions for execution
//!
//!
use crate::bus::Bus;
use crate::core::fault::Fault;
use crate::core::thumb::ThumbCode;
use crate::decoder::is_thumb32;
use crate::Processor;

///
/// Fetching instructions
pub trait Fetch {
    /// Fetch instruction from current PC (Program Counter) position,
    /// decoding the possible thumb32 variant
    fn fetch(&self, pc: u32) -> Result<ThumbCode, Fault>;
}

impl Fetch for Processor {
    // Fetch next Thumb2-coded instruction from current
    // PC location. Depending on instruction type, fetches
    // one or two half-words.
    fn fetch(&self, pc: u32) -> Result<ThumbCode, Fault> {
        let hw = self.read16(pc)?;

        if is_thumb32(hw) {
            let hw2 = self.read16(pc + 2)?;
            Ok(ThumbCode::Thumb32 {
                opcode: (u32::from(hw) << 16) + u32::from(hw2),
            })
        } else {
            Ok(ThumbCode::Thumb16 { opcode: hw })
        }
    }
}
