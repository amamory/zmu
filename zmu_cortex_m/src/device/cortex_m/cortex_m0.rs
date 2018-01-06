use bus::busmatrix::BusMatrix;
use bus::internal::InternalBus;
use bus::ahblite::AHBLite;
use core::Core;
use core::instruction::Instruction;
use core::register::Reg;
use memory::flash::FlashMemory;
use memory::ram::RAM;
use semihosting::SemihostingResponse;
use semihosting::SemihostingCommand;

pub fn cortex_m0_simulate<F, G>(code: &[u8], mut trace_func: F, mut semihost_func: G) -> u64
where
    F: FnMut(u64, u32, &Instruction),
    G: FnMut(&SemihostingCommand) -> SemihostingResponse,
{
    let mut flash_memory = FlashMemory::new(0, 32768);
    let mut ram_memory = RAM::new(0x2000_0000, 128 * 1024);

    flash_memory.load(code);

    let mut internal_bus = InternalBus::new();
    let mut ahb = AHBLite::new(&mut flash_memory, &mut ram_memory);
    let mut bus = BusMatrix::new(&mut internal_bus, &mut ahb);

    let mut core = Core::new(&mut bus);
    let mut count = 0;
    core.reset();

    let mut instruction_cache = Vec::new();
    // pre-cache the decoded instructions

    {
        let mut pc = 0;
        core.set_r(&Reg::PC, pc);

        while pc < (code.len() as u32 / 2) {
            let thumb = core.fetch();
            let instruction = core.decode(&thumb);
            instruction_cache.push(instruction);
            pc += 2;
            core.set_r(&Reg::PC, pc);
        }
    }

    core.reset();
    while core.running {
        let pc = core.get_pc();
        let instruction = &instruction_cache[(pc >> 1) as usize];

        core.step(
            instruction,
            |semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                semihost_func(semihost_cmd)
            },
        );
        trace_func(count, pc, instruction);
        count += 1;
    }
    count
}
