enum Instruction {
    Add,
    Sub,
    Mult,
    Div,
}

struct Vm {
    stack: Vec<i32>,
}

impl Vm {
    fn new() -> Vm {
        Vm { stack: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    // The pop method should return an Option<i32> to handle the case when the stack is empty.
    pub fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    fn add(&mut self) {
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        self.push(a + b);
    }

    fn subtract(&mut self) {
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        self.push(a - b);
    }

    fn multiply(&mut self) {
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        self.push(a * b);
    }

    fn divide(&mut self) {
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        self.push(a / b);
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add => self.add(),
            Instruction::Sub => self.subtract(),
            Instruction::Mult => self.multiply(),
            Instruction::Div => self.divide(),
            _ => panic!("Instruction not defined"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_a_new_vm() {
        let vm = Vm::new();
        assert_eq!(vm.stack.len(), 0);
    }

    #[test]
    fn should_push_new_item_to_stack() {
        let mut vm = Vm::new();
        vm.push(22);
        assert_eq!(vm.stack.len(), 1);
    }

    #[test]
    fn should_pop_item_from_stack() {
        let mut vm = Vm::new();
        vm.push(22);
        vm.pop();
        assert_eq!(vm.stack.len(), 0);
    }

    #[test]
    fn should_add_two_numbers() {
        let mut vm = Vm::new();
        vm.push(10);
        vm.push(10);
        vm.execute(Instruction::Add);
        assert_eq!(vm.pop().unwrap(), 20);
    }

    #[test]
    fn should_subtract_two_numbers() {
        let mut vm = Vm::new();
        vm.push(10);
        vm.push(10);
        vm.execute(Instruction::Sub);
        assert_eq!(vm.pop().unwrap(), 0);
    }
}
