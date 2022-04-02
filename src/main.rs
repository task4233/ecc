use ecc::strtol;

use std::env;
use std::process::exit;

const REGS: [&str; 8] = ["rdi", "rsi", "r10", "r11", "r12", "r13", "r14", "r15"];
static mut CUR: usize = 0;

// Node type
enum NodeType {
    Num, // Number
}

// Node type
#[derive(Default, Debug, Clone)]
struct Node {
    typ: i32, // Node type
    lhs: Option<Box<Node>>, // Left-hand-side node
    rhs: Option<Box<Node>>, // Right-hand-side node
    val: i32, // Number literal
}

impl Node {
    // new is a factory method for Node
    fn new(op: i32, lhs: Box<Node>, rhs: Box<Node>) -> Self {
        Self {
            typ: op,
            lhs: Some(lhs),
            rhs: Some(rhs),
            ..Default::default()
        }
    }

    // new_num is a setter method for Node
    fn new_num(val: i32) -> Self {
        Self {
            typ: NodeType::Num as i32,
            val: val,
            ..Default::default()
        }
    }

    fn number(tokens: &Vec<Token>, pos: usize) -> Self {
        if tokens[pos].typ == TokenType::Num as i32 {
            let val = tokens[pos].val;
            return Self::new_num(val);
        }
        panic!("number expected, but got {}", tokens[pos].input);
    }

    // expr parses an expression
    // expr := num ( '+' num | '-' num )*
    // expr := lhs ( '+' rhs | '-' rhs )*
    pub fn expr(tokens: Vec<Token>) -> Self {
        let mut pos = 0;
        let mut lhs = Self::number(&tokens, pos);
        pos += 1;
        if tokens.len() == pos {
            return lhs;
        }

        loop {
            if tokens.len() == pos {
                break;
            }

            let op = tokens[pos].typ;
            if op != '+' as i32 && op != '-' as i32 {
                println!("Break op: {}", op);
                break;
            }
            pos += 1;
            lhs = Self::new(op, Box::new(lhs), Box::new(Self::number(&tokens, pos)));
            pos += 1;
        }

        if tokens.len() != pos {
            panic!("stray token: {}", tokens[pos].input);
        }
        return lhs;
    }

    // Code generator
    fn gen(self) -> String {
        // Verify that the given expression starts with a number,
        // and then emit the first `mov` instruction.
        if self.typ == NodeType::Num as i32 {
            let reg: &str;
            unsafe {
                if CUR > REGS.len() {
                    panic!("register exhausted");
                }
                reg = REGS[CUR];
                CUR += 1;
            }
            println!("\tmov {}, {}", reg, self.val);
            return reg.into();
        }

        let dst = self.lhs.unwrap().gen();
        let src = self.rhs.unwrap().gen();
        match self.typ as u8 as char {
            '+' => {
                println!("\tadd {}, {}", dst, src);
                return dst;
            }
            '-' => {
                println!("\tsub {}, {}", dst, src);
                return dst;
            }
            _ => panic!("unexpected operator: {}", self.typ)
        }
    }
}

enum TokenType {
    Num, // Number literal
}

// Token type
#[derive(Default, Debug)]
struct Token {
    typ: i32, // Token type
    val: i32, // Number literal
    input: String, // Token string for error reporting
}

fn tokenize(mut p: String) -> Vec<Token> {
    // Tokenized input is stored to this vec.
    let mut tokens: Vec<Token> = vec![];

    let _org = p.clone();
    while let Some(c) = p.chars().nth(0) {
        // Skip whitespace
        if c.is_whitespace() {
            p = p.split_off(1); // p++
            continue
        }

        // + or -
        if c == '+' || c == '-' {
            let token = Token {
                typ: c as i32,
                input: _org.clone(),
                ..Default::default()
            };
            p = p.split_off(1); // p++
            tokens.push(token);
            continue
        }

        // Number
        if c.is_ascii_digit() {
            let (n, remaining) = strtol(&p);
            p = remaining;
            let token = Token {
                typ: TokenType::Num as i32,
                input: _org.clone(),
                val: n.unwrap() as i32,
            };
            tokens.push(token);
            continue;
        }

        eprintln!("unexpected character: {}", p);
        exit(1);
    }

    return tokens;
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません: {:?}", args);
        exit(1);
    }

    let tokens = tokenize(args.nth(1).unwrap());
    let node = Node::expr(tokens);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    // Generate code while descending the parse tree.
    println!("\tmov rax, {}", node.gen());
    println!("\tret");
    return;
}
