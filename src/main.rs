use ecc::strtol;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません: {:?}", args);
        return;
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let p = args[1].clone();
    let (n, mut p) = strtol(&p);
    println!("\tmov rax, {}", n.unwrap());

    while let Some(c) = p.chars().nth(0) {
        let s = p.split_off(1);

        if c == '+' {
            let (n, remaining) = strtol(&s);
            p = remaining;
            println!("\tadd rax, {}", n.unwrap());
            continue;
        }

        if c == '-' {
            let (n, remaining) = strtol(&s);
            p = remaining;
            println!("\tsub rax, {}", n.unwrap());
            continue;
        }

        eprintln!("unexpected character: {}", p);
        return;
    }

    println!("\tret");
    return;
}
