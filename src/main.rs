struct ProgState {
    prog_name: String,
    stack: Vec<i64>,
    curr_num: Option<i64>,
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
            curr_num: Option::None
        }
    }

    fn print_stack(&self) {
        for i in self.stack.iter().rev() {
            println!("{i}");
        }
    }

    fn two_operands_op<F>(&mut self, f: F)
        where F: FnOnce(i64, i64) -> Result<i64, ArithmeticErr>
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
    for c in s.bytes() {
        if c >= b'0' && c <= b'9' {
            if state.curr_num.is_none() {
                state.curr_num = Some(0);
            }

            let num = match (c as char).to_string().parse::<i64>() {
                Ok(num) => Some(num),
                Err(_) => {
                    eprintln!("Cannot parse number");
                    None
                },
            };

            state.curr_num = Some(
                state.curr_num.as_ref().unwrap() * 10 + num.unwrap()
            );

            continue;
        };

        if let Some(num) = state.curr_num {
            state.stack.push(num);
            state.curr_num = None;
        }

        match c {
            b'\n' | b'\t' | b'\r' | b' ' => state.curr_num = None,

            // b'A' ..= b'F' => println!("Hex Num: {c}"),

            b'+' => state.two_operands_op(|a, b| Ok(a + b)),
            b'-' => state.two_operands_op(|a, b| Ok(a - b)),
            b'*' => state.two_operands_op(|a, b| Ok(a * b)),
            b'/' => state.two_operands_op(|a, b| {
                    if b == 0 {
                        Err(ArithmeticErr::DivideByZero)
                    } else { Ok(a / b) }
                }),
            b'%' => state.two_operands_op(|a, b| {
                    if b == 0 {
                        Err(ArithmeticErr::DivideByZero)
                    } else { Ok(a % b) }
                }),

            b'^' => state.two_operands_op(|a, b| Ok(a.pow(b as u32))),

            b'.' => eprintln!("floating point numbers not supported"),

            b'~' | b'_' => {
                match state.stack.pop() {
                    Some(num) => state.stack.push(num * -1),
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
