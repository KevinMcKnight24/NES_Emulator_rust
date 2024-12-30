


pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
}

impel CPU {
    pub fn new() -> Self {
        CPU{
            register_a: 0,                                                    //initialize the registers
            register_x: 0,
            register_y: 0,
            status: 0,                                                        //initialize the ccr
            program_counter: 0,                                               //initialize the program counter to point to memory addresses
        }
    }

    fn lda(&mut self, value: u8){                                             //implementing the LDA instruction
        self.register_a = value;                                              
        self.change_zero_negative_flag(self.register_a);
    }

    fn tax(){                                                                 //implementing the TAX instruction
        self.register_x = self.register_a;                                    //set the x reg to the a reg
        self.change_zero_negative_flag(self.register_x);                      //TAX affects the Z and N bits
    }
                                                                              //set the ccr bits, LDA affects Z, N (C Z I D B V N)
    fn change_zero_negative_flag(&mut self, result: u8){                      //generally if we are affecting the zero flag we are also affecting the negative flag so we can group them together
        if result == 0{
            self.status = self.status | 0b0000_0010;                          //check to see if number is zero or not 
        }else{
            self.status = self.status & 0b1111_1101;
        }      

        if result & 0b1000_0000 != 0{                                          //check to see if first bit is 1, meaning the number is negative if signed
            self.status = self.status | 0b0000_0001;
        }else{
            self.status = self.status & 0b1111_1110;
        }
    }




    pub fn interpret(&mut self, program: Vec<u8>){
        self.program_counter = 0;                                               //set program counter to 0
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;

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
                
                0xAA => {
                    self.tax();
                }

            }
        }
    
}   
}

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
}
