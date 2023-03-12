struct ProgState {
    prog_name: String,
    stack: Vec<f64>,
}

enum Errors {
    A(ArithmeticErr),
    P(ParserErr),
    S(StackErr),
    Unimplemented { c: char },
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
            Errors::Unimplemented { c } =>
                eprintln!("{}: '{c}' feature is not implemented", self.prog_name),
        }
    }

    fn get_top_two_elem(&mut self) -> Option<(f64, f64)> {
        let len = self.stack.len();
        if len < 2 {
            self.print_error(Errors::S(StackErr::FewElements));
            return None;
        }
        let a = self.stack[len - 2];
        let b = self.stack[len - 1];

        self.stack.pop();
        self.stack.pop();

        Some((a, b))
    }

    fn two_operands_op<F>(&mut self, f: F)
        where F: FnOnce(f64, f64) -> Result<f64, ArithmeticErr>
    {
        match self.get_top_two_elem() {
            Some((a, b)) =>
                match f(a, b) {
                    Ok(x) => self.stack.push(x),
                    Err(_) => self.print_error(Errors::A(ArithmeticErr::DivideByZero)),
                }
            None => return,
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

            b'~' => {
                // x y / x y %
                let len = state.stack.len();
                if len < 2 {
                    state.print_error(Errors::S(StackErr::FewElements));
                    continue;
                }
                let x = state.stack[len - 2];
                let y = state.stack[len - 1];

                if y == 0.0 {
                    state.print_error(Errors::A(ArithmeticErr::DivideByZero));
                } else {
                    state.stack.pop();
                    state.stack.pop();
                    state.stack.push(x / y);
                    state.stack.push(x % y);
                }
                continue;
            },

            b'^' => state.two_operands_op(|a, b| Ok(a.powf(b))),
            b'v' => {
                match state.stack.pop() {
                    Some(num) => state.stack.push(num.sqrt()),
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },

            b'b' => {
                match state.stack.pop() {
                    Some(num) => {
                        if num == 0.0 {
                            state.stack.push(num);
                        } else {
                            state.stack.push(num.abs());
                        }
                    },
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },

            b'_' => {
                match state.stack.pop() {
                    Some(num) => state.stack.push(num * -1.0),
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },

            b'G' => {
                let len = state.stack.len();
                if len < 2 {
                    state.print_error(Errors::S(StackErr::FewElements));
                    continue;
                }
                let x = state.stack[len - 2];
                let y = state.stack[len - 1];
                state.stack.pop();
                state.stack.pop();

                if x == y {
                    state.stack.push(1.0);
                } else {
                    state.stack.push(0.0);
                }
                continue;
            },

            b'N' => {
                match state.stack.pop() {
                    Some(num) => {
                        if num == 0.0 {
                            state.stack.push(1.0);
                        } else {
                            state.stack.push(0.0);
                        }
                    },
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },

            b'z' => state.stack.push(state.stack.len() as f64),

            b'f' => state.print_stack(),
            b'p' => {
                match state.stack.last() {
                    Some(top) => println!("{top}"),
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },
            b'n' => {
                match state.stack.pop() {
                    Some(top) => println!("{top}"),
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },

            b'q' => std::process::exit(0),

            b'$' => {
                match state.stack.pop() {
                    Some(top) => state.stack.push(top.trunc()),
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },

            b'(' => {
                let len = state.stack.len();
                if len < 2 {
                    state.print_error(Errors::S(StackErr::FewElements));
                    continue;
                }
                let x = state.stack[len - 2];
                let y = state.stack[len - 1];
                state.stack.pop();
                state.stack.pop();

                if y < x {
                    state.stack.push(1.0);
                } else {
                    state.stack.push(0.0);
                }
            },

            b')' => {
                let len = state.stack.len();
                if len < 2 {
                    state.print_error(Errors::S(StackErr::FewElements));
                    continue;
                }
                let x = state.stack[len - 2];
                let y = state.stack[len - 1];
                state.stack.pop();
                state.stack.pop();

                if y > x {
                    state.stack.push(1.0);
                } else {
                    state.stack.push(0.0);
                }
            },

            b'{' => {
                let len = state.stack.len();
                if len < 2 {
                    state.print_error(Errors::S(StackErr::FewElements));
                    continue;
                }
                let x = state.stack[len - 2];
                let y = state.stack[len - 1];
                state.stack.pop();
                state.stack.pop();

                if y <= x {
                    state.stack.push(1.0);
                } else {
                    state.stack.push(0.0);
                }
            },

            b'}' => {
                let len = state.stack.len();
                if len < 2 {
                    state.print_error(Errors::S(StackErr::FewElements));
                    continue;
                }
                let x = state.stack[len - 2];
                let y = state.stack[len - 1];
                state.stack.pop();
                state.stack.pop();

                if y >= x {
                    state.stack.push(1.0);
                } else {
                    state.stack.push(0.0);
                }
            },

            b'M' => {
                let len = state.stack.len();
                if len < 2 {
                    state.print_error(Errors::S(StackErr::FewElements));
                    continue;
                }
                let x = state.stack[len - 2];
                let y = state.stack[len - 1];
                state.stack.pop();
                state.stack.pop();

                if x != 0.0 && y != 0.0 {
                    state.stack.push(1.0);
                } else {
                    state.stack.push(0.0);
                }
            },

            b'm' => {
                let len = state.stack.len();
                if len < 2 {
                    state.print_error(Errors::S(StackErr::FewElements));
                    continue;
                }
                let x = state.stack[len - 2];
                let y = state.stack[len - 1];
                state.stack.pop();
                state.stack.pop();

                if x != 0.0 || y != 0.0 {
                    state.stack.push(1.0);
                } else {
                    state.stack.push(0.0);
                }
            },

            b'c' => state.stack.clear(),

            b'd' => {
                match state.stack.last() {
                    Some(top) => state.stack.push(*top),
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },

            b'r' => {
                let len = state.stack.len();
                if len < 2 {
                    state.print_error(Errors::S(StackErr::FewElements));
                    continue;
                }
                let x = state.stack[len - 2];
                let y = state.stack[len - 1];
                state.stack.pop();
                state.stack.pop();
                state.stack.push(y);
                state.stack.push(x);
            },

            b'R' => {
                match state.stack.pop() {
                    Some(_) => continue,
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },

            b'P' | b'|' |        b'@' | b'H' | b'h' |
           b'\'' | b'"' |
            b's' | b'l' | b'S' | b'L' | b'Z' | b'X' =>
                state.print_error(Errors::Unimplemented { c: c as char }),

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
