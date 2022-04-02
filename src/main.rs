use ecc::strtol;

use std::env;
use std::process::exit;

enum TokenType {
    Num, // Number literal
}

// Token Type
#[derive(Default, Debug)]
struct Token {
    typ: i32, // Token type
    val: i32, // Number literal
    // input: String, // Token string for error reporting
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
                // input: _org.clone(),
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
                // input: _org.clone(),
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

fn fail(tokens: &Vec<Token>, i: usize) {
    eprintln!("unexpected character; {:?}", tokens[i]);
    exit(1);
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません: {:?}", args);
        exit(1);
    }

    let tokens = tokenize(args.nth(1).unwrap());

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    // Verify that the given expression starts with a number,
    // and then emit the first `mov` instruction.
    if tokens[0].typ != TokenType::Num as i32 {
        fail(&tokens, 0);
    }
    println!("\tmov rax, {}", tokens[0].val);

    // Emit assembly as we consume the sequence of
    // `+ <number>` or `- <number>`.
    let mut i = 1;
    while i < tokens.len() {
        if tokens[i].typ == '+' as i32 {
            i += 1;
            if tokens[i].typ != TokenType::Num as i32 {
                fail(&tokens, i);
            }
            println!("\tadd rax, {}", tokens[i].val);
            i += 1;
            continue;
        }

        if tokens[i].typ == '-' as i32 {
            i += 1;
            if tokens[i].typ != TokenType::Num as i32 {
                fail(&tokens, i);
            }
            println!("\tsub rax, {}", tokens[i].val);
            i += 1;
            continue;
        }

        fail(&tokens, i);
    }

    println!("\tret");
    return;
}
