use crate::opcodes;
use std::collections::HashMap;

bitflags! {
    pub struct CpuFlags: u8 {
        const CARRY                 = 0b00000001;
        const ZERO                  = 0b00000010;
        const INTERRUPT_DISABLE     = 0b00000100;
        const DECIMAL_MODE          = 0b00001000;       //unused for NES
        const BREAK                 = 0b00010000;
        const BREAK2                = 0b00100000;       //unused
        const OVERFLOW              = 0b01000000;
        const NEGTAIVE              = 0b10000000;
    }
}

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xfd;
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: CpuFlags,
    pub program_counter: u16,
    pub stack_pointer: u8,
    memory: [u8; 0xFFFF]
}
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressMode {
    Immeditate,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    IndirectX,
    IndirectY,
    NoneAddress,
}

fn main() {
    println!("");
}

trait Mem {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {                                    //this is to read address space of 8bits
        self.memory[addr as usize]
    }
    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}


impl CPU {   
    pub fn new() -> Self {
        CPU{
            register_a: 0,                                                    //initialize the registers
            register_x: 0,
            register_y: 0,
            stack_pointer: STACK_RESET,                                                        //initialize the ccr
            program_counter: 0,                                               //initialize the program counter to point to memory addresses
            status: CpuFlags::from_bits_truncate(0b100100),
            memory: [0; 0xFFFF]
        }
    }


    fn get_operand_address(&self, mode: &AddressMode) -> u16{

        match mode {
            AddressMode::Immeditate => self.program_counter,                                    //For immeditate addressing we load in a value into a register (ie LDX #$01 loads $01 into X reg)

            AddressMode::ZeroPage => self.mem_read(self.program_counter) as u16,          //For zero page addressing mode we load in the value at an address into a register (ie LDX $01 loads the value at address $01 into X reg)

            AddressMode::Absolute => self.mem_read_u16(self.program_counter) as u16,       //For Absolute addressing mode we store an value at an entire 16bit memory location (ie STA $1234 stores the value in A at $1234)

            //in zero page only first page of addresses are allowed (first 256 bytes have 3 cpu cycle retrieve time rather than 4-7)
            //For zero page a zero page address is given and then the value of reg x is added to it 
            AddressMode::ZeroPageX => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;        //wrapping add is used if sum is larger than single byte
                addr
            }

            AddressMode::ZeroPageY => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }

            //Absolute version of zero page, uses full memory location rather than just zero page
            AddressMode::AbsoluteX => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }

            AddressMode::AbsoluteY => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }

            //Indirect uses absolute address to look up another address, ie first address gives least sig byte of address and following gives most sig byte
            AddressMode::IndirectX => {
                let base = self.mem_read(self.program_counter);
                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }

            AddressMode::IndirectY => {
                let base = self.mem_read(self.program_counter);
                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }

            AddressMode::NoneAddress => {
                panic!("Mode {:?} is not suppoeted", mode);
            }
        }

    }

    /*
    
        MEMORY COMMANDS
    
    */
    pub fn load_and_run(&mut self, program: Vec<u8>){
        self.load(program);
        self.reset();
        self.run()
    }

    pub fn load(&mut self, program: Vec<u8>){
        let end_address = 0x8000 + program.len();                        //This is to load the ROM into memory. In the NES System the program ROM 
        self.memory[0x8000 .. end_address].copy_from_slice(&program[..]);  //program ROM Starts at address of 0x8000 and should end at however long the ROM is
        self.program_counter = 0x8000;                                         //copy_from_slice will copy the program from the program file into memory
        self.mem_write_u16(0xFFFC, 0x8000);
    }                                                                          //side note: a slice is essentially a block of memory represented as a pointer and length of said block

    pub fn reset(&mut self) {
        self.register_a = 0;                                                    //initialize the registers
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = STACK_RESET;
        self.status = CpuFlags::from_bits_truncate(0b100100);                  //Default state of CPU Flags                                                        //initialize the ccr
        self.program_counter = self.mem_read_u16(0xFFFC);                 
    }
    

    /*
        INSTRUCTIONS
        Instructions are all from 6502 chip (http://www.6502.org/tutorials/6502opcodes.html)
    
    */
    fn lda(&mut self, mode: &AddressMode){                                             //implementing the LDA instruction
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;                                              
        self.change_zero_negative_flag(self.register_a);
    }

    fn tax(&mut self){                                                        //implementing the TAX instruction
        self.register_x = self.register_a;                                    //set the x reg to the a reg
        self.change_zero_negative_flag(self.register_x);              //TAX affects the Z and N bits
    }
       
    fn inx(&mut self){                                                        //implementing the INX instruction
        self.register_x = self.register_x.wrapping_add(1);                    //increment x and use wrapping add for overflow case ie: FF -> 00
        self.change_zero_negative_flag(self.register_x);              //inx affects the Z, and N bits
    }   
     
     
    /*
        STATUS REGISTER CHANGES
    
    */                                                                        //set the ccr bits, LDA affects Z, N (C Z I D B V N)
    fn change_zero_negative_flag(&mut self, result: u8){                      //generally if we are affecting the zero flag we are also affecting the negative flag so we can group them together
        if result == 0 {
            self.status.insert(CpuFlags::ZERO);                         //check to see if number is zero or not 
        }
        else{
            self.status.remove(CpuFlags::ZERO);
        }      

        if result & 0b1000_0000 != 0 {                                          //check to see if first bit is 1, meaning the number is negative if signed
            self.status.insert(CpuFlags::NEGTAIVE);
        }
        else{
            self.status.remove(CpuFlags::NEGTAIVE);
        }
    }



    /*
        INTERPRET OP CODE
    
    */
    pub fn run(&mut self){
        let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;

        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let program_counter_state = self.program_counter;

            let opcode = opcodes.get(&code).expect(&format!("OpCode {:x} is not recognized", code));
            
            match code {
                //ADC, Add with Carry
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => {
                    self.adc()
                }

                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => {
                    self.lda(&opcode.mode);
                }
                _ => todo!(),

            }
        }
    }
    pub fn interpret(&mut self, program: Vec<u8>){
        self.program_counter = 0;        
        loop{

            let opscode = program[self.program_counter as usize];               //gets the op code from where the program counter is pointing in memory
            self.program_counter += 1;                                          //increase the program counter

            match opscode{
                0x00 => {                                                       //0x00 is the op code for BRK  
                    return;
                }

                0xA9 => {                                                       //0xA9 is the op code for the LDA instruction
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.lda(param);                    
                }
                
                0xAA => {                                                       //0xAA is the op code for TAX
                    self.tax();
                }

                0xE8 => {                                                       //0xE8 is the op code for INX
                    self.inx();
                }
                
                _ => todo!(),
            }
        }
    
}   
}



/*
    TEST CASES

*/
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_lda() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_lda_ccr() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
  
        assert_eq!(cpu.register_x, 0xc1)
    }
 
     #[test]
     fn test_inx_overflow() {
         let mut cpu = CPU::new();
         cpu.register_x = 0xff;
         cpu.interpret(vec![0xe8, 0xe8, 0x00]);
 
         assert_eq!(cpu.register_x, 1)
     }
}
