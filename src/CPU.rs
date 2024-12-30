pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8, 0xFFFF]
}

fn main() {
    println!("");
}

impl CPU {
    
    pub fn new() -> Self {
        CPU{
            register_a: 0,                                                    //initialize the registers
            register_x: 0,
            register_y: 0,
            status: 0,                                                        //initialize the ccr
            program_counter: 0,                                               //initialize the program counter to point to memory addresses
        }
    }



    /*
        INSTRUCTIONS
    
    */
    fn lda(&mut self, value: u8){                                             //implementing the LDA instruction
        self.register_a = value;                                              
        self.change_zero_negative_flag(self.register_a);
    }

    fn tax(&mut self){                                                        //implementing the TAX instruction
        self.register_x = self.register_a;                                    //set the x reg to the a reg
        self.change_zero_negative_flag(self.register_x);                      //TAX affects the Z and N bits
    }
       
    fn inx(&mut self){                                                        //implementing the INX instruction
        self.register_x = self.register_x.wrapping_add(1);                    //increment x and use wrapping add for overflow case ie: FF -> 00
        self.change_zero_negative_flag(self.register_x);                      //inx affects the Z, and N bits
    }   
     
     
    /*
        STATUS REGISTER CHANGES
    
    */                                                                          //set the ccr bits, LDA affects Z, N (C Z I D B V N)
    fn change_zero_negative_flag(&mut self, result: u8){                      //generally if we are affecting the zero flag we are also affecting the negative flag so we can group them together
        if result == 0 {
            self.status = self.status | 0b0000_0010;                          //check to see if number is zero or not 
        }else{
            self.status = self.status & 0b1111_1101;
        }      

        if result & 0b1000_0000 != 0 {                                          //check to see if first bit is 1, meaning the number is negative if signed
            self.status = self.status | 0b1000_0000;
        }else{
            self.status = self.status & 0b0111_1111;
        }
    }



    /*
        INTERPRET OP CODE
    
    */
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
