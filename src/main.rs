mod vm;

use vm::{Instruction, Vm};

fn main() {
    let mut vm = Vm::new();

    vm.push(10);
    vm.push(5);

    vm.execute(Instruction::Add);
    println!("Resultado final: {}", vm.pop().expect("Erro ao remover item do Stack"));
}
