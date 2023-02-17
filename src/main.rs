use std::{
    collections::HashMap,
    io::{self, Write},
};

fn main() {
    repl();
}

fn repl() {
    let mut env: Env = Env::new();
    loop {
        let result: Result<(), RithpErr> = read(&mut env).and_then(eval).and_then(prnt);

        match result {
            Ok(()) => {}
            Err(e) => e.notify(),
        }
    }
}

fn prnt(text: String) -> Result<(), RithpErr> {
    Ok(())
}

fn eval(env_tok: (&mut Env, Vec<String>)) -> Result<String, RithpErr> {
    let mut result = String::new();

    Ok(result)
}

fn read(envr: &mut Env) -> Result<(&mut Env, Vec<String>), RithpErr> {
    prompt(">>> ");
    let tokens: Vec<String> = scan()
        .replace('(', " ( ")
        .replace(')', " ) ")
        .split(' ')
        .map(|s: &str| s.to_string())
        .collect();

    let mut nest_counter: i32 = 0;
    for token in tokens.iter() {
        match token.as_str() {
            "(" => nest_counter += 1,
            ")" => nest_counter -= 1,
            _ => continue,
        }

        if nest_counter < 0 {
            return Err(RithpErr::UnexpectedParen);
        }
    }

    if nest_counter > 0 {
        return Err(RithpErr::UnexpectedEOF);
    }

    Ok((envr, tokens))
}

fn prompt(text: &str) {
    print!("{text}");
    io::stdout().flush().expect("Error flushing STDOUT");
}

fn scan() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN");
    input
}

#[derive(Debug)]
enum Expr {
    Atom(Atom),
    List(Vec<Expr>),
}

#[derive(Debug)]
enum Atom {
    Symbol(String),
    Number(Number),
}

#[derive(Debug)]
enum Number {
    Int(i32),
    Float(f64),
}

struct Env {
    lut: HashMap<String, fn(Vec<Expr>) -> Result<Expr, RithpErr>>,
}

impl Env {
    fn new() -> Env {
        Env {
            lut: HashMap::from([
                //("define".to_string(), Op::define())
                ("+".to_string(), Op::add()),
            ]),
        }
    }
}

enum Op {}

impl Op {
    fn add() -> fn(Vec<Expr>) -> Result<Expr, RithpErr> {
        |vexpr: Vec<Expr>| -> Result<Expr, RithpErr> {
            let mut sum: f64 = 0.0;

            let mut iflag: bool = true;
            for expr in vexpr {
                match expr {
                    Expr::Atom(Atom::Number(Number::Int(n))) => sum += n as f64,
                    Expr::Atom(Atom::Number(Number::Float(n))) => {
                        iflag = false;
                        sum += n
                    }
                    _ => return Err(RithpErr::OpAdd(expr)),
                }
            }

            if iflag {
                Ok(Expr::Atom(Atom::Number(Number::Int(sum as i32))))
            } else {
                Ok(Expr::Atom(Atom::Number(Number::Float(sum))))
            }
        }
    }
}

enum RithpErr {
    UnexpectedParen,
    UnexpectedEOF,
    OpAdd(Expr),
}

impl RithpErr {
    fn notify(self) {
        println!("Unable to evaluate input:");
        match self {
            Self::UnexpectedParen => println!("\tUnexpected ')'"),
            Self::UnexpectedEOF => println!("\tUnexpected EOF"),
            Self::OpAdd(expr) => {
                println!(
                    "\tExpected: {:?}\n\tGot: {:?}",
                    stringify!(Expr::Atom(Atom::Number)),
                    expr
                )
            }
        }
    }
}
