use std::env;
use std::fs;

#[derive(Debug)]
enum Token {
    SYM(char),
    NUM(i64),
    LP,
    RP,
}

fn check_parens(tokens: &[Token]) {
    let mut balance = 0;
    for token in tokens {
        match token {
            Token::LP => balance += 1,
            Token::RP => balance -= 1,
            _ => {}
        }
    }
    assert_eq!(balance, 0);
}

fn tokenize(src: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = src.trim_start().chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::LP),
            ')' => tokens.push(Token::RP),
            _ if c.is_numeric() => {
                let mut token_string = String::new();
                token_string.push(c);
                while let Some(p) = chars.peek() {
                    if !p.is_numeric() {
                        break;
                    }
                    token_string.push(*p);
                    chars.next();
                }
                tokens.push(Token::NUM(token_string.parse().unwrap()));
            }
            '+' | '-' | '/' | '*' | '%' | 'r' => tokens.push(Token::SYM(c)),
            _ if c.is_whitespace() => {}
            _ => panic!(
                "Invalid token chars found starting at {:?}",
                chars.collect::<String>()
            ),
        }
    }
    tokens
}

#[derive(Debug)]
enum Opr {
    SUM,
    SUB,
    DIV,
    MUL,
    MOD,
    SRT,
}

#[derive(Debug)]
enum Ast {
    NUM(i64),
    OPR(Opr, Vec<Ast>),
}

fn parse(tokens: &mut &[Token]) -> Ast {
    let [first, rest @ ..] = tokens else{panic!("Empty expr")};
    {
        match first {
            Token::LP => {
                let [Token::SYM(s), rest2 @ ..] = rest else{
                    panic!("Expected operator after '(' {:?}", rest);
                };
                let op = match s {
                    '+' => Opr::SUM,
                    '-' => Opr::SUB,
                    '/' => Opr::DIV,
                    '*' => Opr::MUL,
                    '%' => Opr::MOD,
                    'r' => Opr::SRT,
                    _ => panic!("unreachable"),
                };
                *tokens = rest2;
                let mut args = Vec::<Ast>::new();
                while !tokens.is_empty() {
                    if let [Token::RP, rest3 @ ..] = tokens {
                        *tokens = rest3;
                        break;
                    }
                    args.push(parse(tokens));
                }
                return Ast::OPR(op, args);
            }
            Token::NUM(i) => {
                *tokens = rest;
                return Ast::NUM(*i);
            }
            _ => panic!("ASDFASDFsd"),
        }
    }
}

fn eval(ast: Ast) -> i64 {
    match ast {
        Ast::NUM(n) => n,
        Ast::OPR(Opr::SRT, mut arg) => (eval(arg.swap_remove(0)) as f64).sqrt() as i64,
        Ast::OPR(op, args) => {
            use std::collections::VecDeque;
            let mut arg_deq = VecDeque::from(args);
            let Some( n) = arg_deq.pop_front() else{panic!("aaaaaa")};
            let rest = arg_deq.into_iter().map(eval);
            match op {
                Opr::SUM => rest.fold(eval(n), |i, r|i+r),
                Opr::SUB => rest.fold(eval(n), |i, r|i-r),
                Opr::DIV => rest.fold(eval(n), |i, r|i/r),
                Opr::MUL => rest.fold(eval(n), |i, r|i*r),
                Opr::MOD => rest.fold(eval(n), |i, r|i%r),
                _ => unreachable!(),
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <Input File>", args[0]);
        return;
    }
    let src = fs::read_to_string(&args[1]).expect(&format!("Could not read file \"{}\"", args[1]));

    println!("src -\n{}", src);

    let tokens = tokenize(src);
    println!("tokens -\n{:#?}", tokens);
    check_parens(&tokens);
    let ast = parse(&mut tokens.as_slice());
    println!("ast -\n{:#?}", ast);
    println!("result: {:?}", eval(ast));
}
