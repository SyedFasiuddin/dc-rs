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

fn tokenize_line(s: &str, state: &mut ProgState) /* -> Vec<u8>  */{
    // let v = Vec::new();
    for c in s.bytes() {
        match c {
            b'\n' | b'\t' | b'\r' | b' ' => {
                println!("Do nothing");
                state.curr_num = None;
            }

            b'0' ..=b'9' => {
                println!("Parsing number: {c}");
                let num = match (c as char).to_string().parse::<i64>() {
                    Ok(v) => Some(v),
                    Err(_) => {
                        eprintln!("Cannot parse number");
                        None
                    },
                };

                if let Some(num) = num {
                    if state.curr_num.is_none() {
                        state.stack.push(num);
                        state.curr_num = Some(0);
                        continue;
                    }
                    if let Some(mut v) = state.stack.pop() {
                        v *= 10;
                        v += num;
                        state.stack.push(v);
                    }
                }
            },

            // b'A' ..= b'F' => println!("Hex Num: {c}"),

            b'+' => {
                let len = state.stack.len();
                if len < 2 {
                    eprintln!("Stack empty");
                    continue;
                }
                let a = state.stack[len - 1];
                let b = state.stack[len - 2];
                state.stack.pop();
                state.stack.pop();
                state.stack.push(a+b);
            }

            b'-' => println!("Sub"),
            b'*' => println!("Mul"),
            b'/' => println!("Div"),
            b'%' => println!("Rem"),
            b'^' => println!("Pow"),

            b'f' => state.print_stack(),
            _ => continue,
        }
    }
    // v
}

// fn loop_over_input(r: &mut impl Read) {
//     tokenize_line(s);
// }

fn main() {

    // loop_over_input(&mut std::io::stdin().lock());

    let args: Vec<String> = std::env::args().collect();
    let mut line_buf: String;
    let mut state = ProgState::new(&args[0][..]);

    loop {
        line_buf = "".to_string();
        match std::io::stdin().read_line(&mut line_buf) {
            Ok(_) => tokenize_line(&line_buf[..], &mut state),
            Err(e) => {
                println!("Err: {e}");
                // Vec::new()
            }
        };
    }

}
