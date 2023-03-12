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
        let top_minus_one = self.stack[len - 2];
        let top = self.stack[len - 1];

        self.stack.pop();
        self.stack.pop();

        Some((top_minus_one, top))
    }

    fn two_operands_op<F>(&mut self, f: F)
        where F: FnOnce(f64, f64) -> Result<f64, ArithmeticErr>
    {
        match self.get_top_two_elem() {
            Some((top_minux_one, top)) =>
                match f(top_minux_one, top) {
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

            b'f' => state.print_stack(),

            b'+' => state.two_operands_op(|a, b| Ok(a + b)),
            b'-' => state.two_operands_op(|a, b| Ok(a - b)),
            b'*' => state.two_operands_op(|a, b| Ok(a * b)),
            b'/' => state.two_operands_op(|top_minus_one, top| {
                    if top == 0.0 {
                        Err(ArithmeticErr::DivideByZero)
                    } else { Ok(top_minus_one / top) }
                }),
            b'%' => state.two_operands_op(|top_minus_one, top| {
                    if top == 0.0 {
                        Err(ArithmeticErr::DivideByZero)
                    } else { Ok(top_minus_one % top) }
                }),

            b'~' => {
                // x y / x y %
                match state.get_top_two_elem() {
                    Some((top_minus_one, top)) => {
                        if top == 0.0 {
                            state.stack.push(top_minus_one);
                            state.stack.push(top);
                            state.print_error(Errors::A(ArithmeticErr::DivideByZero));
                        } else {
                            state.stack.push(top_minus_one / top);
                            state.stack.push(top_minus_one % top);
                        }
                    },
                    None => continue,
                }
            },

            b'^' => {
                state.two_operands_op(|top_minus_one, top| Ok(top_minus_one.powf(top)));
                todo!("handle case where top is negative and top-1 is 0");
            }

            b'v' => {
                match state.stack.pop() {
                    Some(num) => {
                        state.stack.push(num.sqrt());
                        todo!("handle case where num is negative");
                    },
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },

            b'_' => {
                match state.stack.pop() {
                    Some(num) => {
                        state.stack.push(num * -1.0);
                        todo!("handle case where - is followed by number i.e. -12.3");
                    },
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },

            b'b' => {
                match state.stack.pop() {
                    Some(top) => {
                        if top == 0.0 {
                            state.stack.push(top);
                        } else {
                            state.stack.push(top.abs());
                        }
                    },
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },

            b'$' => {
                match state.stack.pop() {
                    Some(top) => state.stack.push(top.trunc()),
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },

            b'G' => {
                match state.get_top_two_elem() {
                    Some((top_minus_one, top)) => {
                        if top_minus_one == top {
                            state.stack.push(1.0);
                        } else {
                            state.stack.push(0.0);
                        }
                    },
                    None => continue,
                }
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

            b'(' => {
                match state.get_top_two_elem() {
                    Some((top_minus_one, top)) => {
                        if top < top_minus_one {
                            state.stack.push(1.0);
                        } else {
                            state.stack.push(0.0);
                        }
                    },
                    None => continue,
                }
            },

            b'{' => {
                match state.get_top_two_elem() {
                    Some((top_minus_one, top)) => {
                        if top <= top_minus_one {
                            state.stack.push(1.0);
                        } else {
                            state.stack.push(0.0);
                        }
                    }
                    None => continue,
                }
            },

            b')' => {
                match state.get_top_two_elem() {
                    Some((top_minus_one, top)) => {
                        if top > top_minus_one {
                            state.stack.push(1.0);
                        } else {
                            state.stack.push(0.0);
                        }
                    }
                    None => continue,
                }
            },

            b'}' => {
                match state.get_top_two_elem() {
                    Some((top_minus_one, top)) => {
                        if top >= top_minus_one {
                            state.stack.push(1.0);
                        } else {
                            state.stack.push(0.0);
                        }
                    }
                    None => continue,
                }
            },

            b'M' => {
                match state.get_top_two_elem() {
                    Some((top_minus_one, top)) => {
                        if top_minus_one != 0.0 && top != 0.0 {
                            state.stack.push(1.0);
                        } else {
                            state.stack.push(0.0);
                        }
                    }
                    None => continue,
                }
            },

            b'm' => {
                match state.get_top_two_elem() {
                    Some((top_minus_one, top)) => {
                        if top_minus_one != 0.0 || top != 0.0 {
                            state.stack.push(1.0);
                        } else {
                            state.stack.push(0.0);
                        }
                    }
                    None => continue,
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
                match state.get_top_two_elem() {
                    Some((top_minus_one, top)) => {
                        state.stack.push(top);
                        state.stack.push(top_minus_one);
                    },
                    None => continue,
                }
            },

            b'R' => {
                match state.stack.pop() {
                    Some(_) => continue,
                    None => state.print_error(Errors::S(StackErr::FewElements)),
                }
            },

            b'z' => state.stack.push(state.stack.len() as f64),
            b'q' => std::process::exit(0),

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
