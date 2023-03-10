struct ProgState {
    prog_name: String,
    stack: Vec<f64>,
}

#[derive(Debug)]
enum ArithmeticErr {
    DivideByZero,
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

    fn two_operands_op<F>(&mut self, f: F)
        where F: FnOnce(f64, f64) -> Result<f64, ArithmeticErr>
    {
        let len = self.stack.len();
        if len < 2 {
            eprintln!("{} stack empty", self.prog_name);
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
            Err(ArithmeticErr::DivideByZero) => eprintln!("{} divide by zero", self.prog_name),
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
            match number.parse::<f64>() {
                Ok(parsed_float) => {
                    state.stack.push(parsed_float);
                    number = String::new();
                }
                Err(e) => eprintln!("Cannot parse number: {e}"),
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
                    None => eprintln!("stack is empty"),
                }
            },

            b'f' => state.print_stack(),

            b'q' => std::process::exit(0),
            _ => continue,
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
