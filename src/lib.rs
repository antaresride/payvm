pub enum Instruction{
    Load(usize, i32),
    Add(usize, usize),
    Print(usize),
    Halt,
}
pub struct VM {
    registers: [i32; 4],
    pc: usize,
    program: Vec<Instruction>,
}
impl VM {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            registers: [0; 4],
            pc: 0,
            program,
        }
    }
   pub fn run(&mut self) {
        loop {
            if self.pc >= self.program.len() {
                break;
            }
        }
    }
}
