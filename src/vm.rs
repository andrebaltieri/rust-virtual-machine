const STACK_SIZE: usize = 100;

struct Stack {
    stack: [i32; STACK_SIZE],
    top: isize, // Index of the top of the stack
}

impl Stack {
    fn init(&mut self) {
        self.top = -1; // Initialize the stack top as -1 (indicating an empty stack)
    }

    fn push(&mut self, value: i32) {
        if self.top < (STACK_SIZE - 1) as isize {
            self.top += 1;
            self.stack[self.top as usize] = value;
        } else {
            println!("Stack overflow");
        }
    }

    fn pop(&mut self) -> i32 {
        if self.top >= 0 {
            let value = self.stack[self.top as usize];
            self.top -= 1;
            value
        } else {
            println!("Stack underflow");
            -1 // Arbitrary value to indicate an error
        }
    }
}

fn main() {
    let mut s = Stack {
        stack: [0; STACK_SIZE],
        top: 0,
    };

    // Example usage
    s.push(5);
    s.push(10);
    s.push(7);

    println!("Popping values:");
    println!("{}", s.pop());
    println!("{}", s.pop());
    println!("{}", s.pop());
}
