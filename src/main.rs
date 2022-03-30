use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません: {:?}", args);
        process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("\tmov rax, {}", args[1].parse::<i32>().unwrap());
    println!("\tret");
}
