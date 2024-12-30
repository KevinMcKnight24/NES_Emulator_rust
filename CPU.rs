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
            register_a: 0,                                                     //initialize the registers
            register_x: 0,
            register_y: 0,
            status: 0,                                                         //initialize the ccr
            program_counter: 0,                                                //initialize the program counter to point to memory addresses
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
                0xA9 => {
                    let param = program[]
                }
            }
        }
    }

}
