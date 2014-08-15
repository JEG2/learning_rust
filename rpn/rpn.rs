use std::fmt;
use std::os;

struct Stack {
    numbers: Vec<f64>
}
impl Stack {
    fn new() -> Stack {
        Stack{numbers: vec![]}
    }

    fn is_empty(&self) -> bool {
        self.numbers.is_empty()
    }

    fn push(&mut self, number: f64) {
        self.numbers.push(number);
    }

    fn result(&self) -> f64 {
        *self.numbers.last().expect("Stack empty.")
    }

    fn add(&mut self)      { self._do_binary_operation(|l, r| l + r); }
    fn subtract(&mut self) { self._do_binary_operation(|l, r| l - r); }
    fn multiply(&mut self) { self._do_binary_operation(|l, r| l * r); }
    fn divide(&mut self)   { self._do_binary_operation(|l, r| l / r); }

    fn _do_binary_operation(&mut self, operation: |f64, f64| -> f64) {
        let r = self.numbers.pop().expect("Stack underflow.");
        let l = self.numbers.pop().expect("Stack underflow.");
        self.numbers.push(operation(l, r));
    }
}
impl fmt::Show for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let mut i = self.numbers.len();
        for number in self.numbers.iter() {
            i -= 1;
            s = s.add(&format!("{}: {}\n", i, number));
        }
        s.pop_char();
        write!(f, "{}", s)
    }
}

struct Tokenizer<'a> {
    tokens: Vec<&'a str>,
    i:      uint
}
impl<'a> Tokenizer<'a> {
    fn new(expression: &str) -> Tokenizer {
        Tokenizer{
          tokens: expression.split(|c: char| c.is_whitespace()).collect(),
          i:      0
        }
    }

    fn has_next_token(&self) -> bool {
        self.i < self.tokens.len()
    }

    fn next_token(&mut self) -> &str {
        if !self.has_next_token() { fail!("Tokens exhausted.") }

        let token = self.tokens[self.i];
        self.i   += 1;
        token
    }
}

struct RPNCalculator<'a> {
    stack:  Stack,
    tokens: Tokenizer<'a>
}
impl<'a> RPNCalculator<'a> {
    fn new(stack: Stack, tokens: Tokenizer) -> RPNCalculator {
        RPNCalculator{stack: stack, tokens: tokens}
    }

    fn calculate(&mut self) -> f64 {
        while self.tokens.has_next_token() {
            let token = self.tokens.next_token();
            if !self.stack.is_empty() {
                println!("{}", self.stack);
            }
            println!("T: {}\n", token);
            match token {
                "+" => { self.stack.add(); }
                "-" => { self.stack.subtract(); }
                "*" => { self.stack.multiply(); }
                "/" => { self.stack.divide(); }
                n   => { self.stack.push(from_str(n).expect("Not a number.")); }
            }
        }
        if !self.stack.is_empty() {
            println!("{}\n", self.stack);
        }
        self.stack.result()
    }
}

fn main() {
    let     expression = os::args();
    let     stack      = Stack::new();
    let     tokenizer  = Tokenizer::new(expression[1].as_slice());
    let mut calculator = RPNCalculator::new(stack, tokenizer);
    println!("{}", calculator.calculate());
}
