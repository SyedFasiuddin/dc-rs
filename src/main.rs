struct ProgState {
    prog_name: String,
    stack: Vec<f64>,
}

enum Errors {
    A(ArithmeticErr),
    P(ParserErr),
    S(StackErr),
}

enum ArithmeticErr {
    DivideByZero,
}

enum ParserErr {
    BadCharacter { c: char},
    FloatParse,
}

enum StackErr {
    FewElements,
}

impl ProgState {
    fn new(s: &str) -> ProgState {
        ProgState {
            prog_name: s.to_string(),
            stack: Vec::new(),
        }
    }

    fn print_stack(&self) {
        for i in self.stack.iter().rev() {
            println!("{i}");
        }
    }

    fn print_error(&self, e: Errors) {
        match e {
            Errors::A(ArithmeticErr::DivideByZero) =>
                eprintln!("{}: Arithmetic error: divide by zero", self.prog_name),
            Errors::P(ParserErr::FloatParse) =>
                eprintln!("{}: Parser error: cannot parse floating point number", self.prog_name),
            Errors::P(ParserErr::BadCharacter { c }) =>
                eprintln!("{}: Parser error: bad character '{c}'", self.prog_name),
            Errors::S(StackErr::FewElements) =>
                eprintln!("{}: Runtime error: stack has too few elements", self.prog_name),
        }
    }

    fn two_operands_op<F>(&mut self, f: F)
        where F: FnOnce(f64, f64) -> Result<f64, ArithmeticErr>
    {
        let len = self.stack.len();
        if len < 2 {
            self.print_error(Errors::S(StackErr::FewElements));
            return;
        }
        let a = self.stack[len - 2];
        let b = self.stack[len - 1];

        match f(a, b) {
            Ok(x) => {
                self.stack.pop();
                self.stack.pop();
                self.stack.push(x);
            },
            Err(_) => self.print_error(Errors::A(ArithmeticErr::DivideByZero)),
        }
    }
}

fn tokenize_line(s: &str, state: &mut ProgState) {
    let mut number = String::new();
    let mut have_number_to_parse = false;

    for c in s.bytes() {
        if c >= b'0' && c <= b'9' || c == b'.' {
            number += &(c as char).to_string();
            have_number_to_parse = true;
            continue;
        }

        if have_number_to_parse {
            if c == b'e' || c == b'E' {
                number += &(c as char).to_string();
                continue;
            }

            if c == b'-' || c == b'+' {
                match number.pop() {
                    Some(x) => {
                        if x == 'e' || x == 'E' {
                            number.push(x);
                            number.push(c as char);
                            continue;
                        }
                        number.push(x)
                    },
                    None => (),
                }
            }

            match number.parse::<f64>() {
                Ok(parsed_float) => {
                    state.stack.push(parsed_float);
                    number = String::new();
                }
                Err(_) => state.print_error(Errors::P(ParserErr::FloatParse)),
            };
            have_number_to_parse = false;
        }

        match c {
            b'\n' | b'\t' | b'\r' | b' ' => continue,

            // b'A' ..= b'F' => println!("Hex Num: {c}"),

            b'+' => state.two_operands_op(|a, b| Ok(a + b)),
            b'-' => state.two_operands_op(|a, b| Ok(a - b)),
            b'*' => state.two_operands_op(|a, b| Ok(a * b)),
            b'/' => state.two_operands_op(|a, b| {
                    if b == 0.0 {
                        Err(ArithmeticErr::DivideByZero)
                    } else { Ok(a / b) }
                }),
            b'%' => state.two_operands_op(|a, b| {
                    if b == 0.0 {
                        Err(ArithmeticErr::DivideByZero)
                    } else { Ok(a % b) }
                }),

            b'^' => state.two_operands_op(|a, b| Ok(a.powf(b))),

            b'~' | b'_' => {
                match state.stack.pop() {
                    Some(num) => state.stack.push(num * -1.0),
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },

            b'f' => state.print_stack(),

            b'q' => std::process::exit(0),
            _ => state.print_error(Errors::P(ParserErr::BadCharacter { c: c as char })),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut line_buf: String;
    let mut state = ProgState::new(&args[0][..]);

    loop {
        line_buf = "".to_string();
        match std::io::stdin().read_line(&mut line_buf) {
            Ok(_) => tokenize_line(&line_buf[..], &mut state),
            Err(e) => {
                println!("Err: {e}");
            }
        };
    }
}
