use genpayvm::{VM, Instruction}; 

fn main() {
    let program = vec![
        Instruction::Load(0, 10),
        Instruction::Load(1, 20),
        Instruction::Add(0, 1),
        Instruction::Print(0),
        Instruction::Halt,
    ];
    let mut vm_pay = VM::new(program);
    vm_pay.run();
}
