use crate::clock::Clock;
use crate::cpu::cpu::Cpu;
use crate::io::joypad::Joypad;
use crate::memory::mmu::Mmu;

pub const CPU_CLOCK_HZ: usize = 4194304;
pub const FPS: f32 = 60.0;

pub struct Emulation {
    clock: Clock,
}

impl Emulation {
    pub fn new() -> Emulation {
        Emulation {
            clock: Clock::new(CPU_CLOCK_HZ, FPS),
        }
    }

    /// This method will cycle the emulator and sleep afterwards for an amount of time
    /// Execute in a loop
    pub fn cycle(&mut self, cpu: &mut Cpu, mmu: &mut Mmu, joypad: &Joypad) {
        let mut counter = 0;
        let breakpoint = 10;

        let memory_addresses: [u16; 31] = [
            0xFF05, 0xFF06, 0xFF07, 0xFF10, 0xFF11, 0xFF12, 0xFF14, 0xFF16, 0xFF17, 0xFF19, 0xFF1A,
            0xFF1B, 0xFF1C, 0xFF1E, 0xFF20, 0xFF21, 0xFF22, 0xFF23, 0xFF24, 0xFF25, 0xFF26, 0xFF40,
            0xFF42, 0xFF43, 0xFF45, 0xFF47, 0xFF48, 0xFF49, 0xFF4A, 0xFF4B, 0xFFFF,
        ];
        for mem in memory_addresses.iter() {
            println!("[{:04X}] = {:02X}", mem, mmu.read(*mem));
        }

        while self.clock.clock_cycles_passed_frame <= self.clock.clock_cycles_per_frame
            && counter < breakpoint
        {
            println!("{}", cpu.registers);
            let last_cycle = cpu.step(mmu);
            mmu.step(joypad, last_cycle);
            self.clock.cycle(last_cycle);
            counter += 1;
        }

        panic!("Breakpoint hit.");

        self.clock.reset();
    }
}
