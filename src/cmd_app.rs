use std::process::Command;
use std::env;
use crate::bf_2_asm;

fn assemble(asm_name: &str, o_name: &str) {
    let _ = Command::new("yasm").args(["-f", "elf64", "-o", o_name, asm_name]).output();
}

fn link(exec_name: &str, o_name: &str) {
    let _ = Command::new("ld").args(["-o", exec_name, o_name]).output();
}

fn compile(bf_name: &str, asm_name: &str) -> u8 {
    bf_2_asm::run(bf_name, asm_name)
}

fn clean(asm_name: &str, o_name: &str, not_clean_asm: bool) {
    let _ = Command::new("rm").arg(o_name).output();
    if !not_clean_asm {
        let _ = Command::new("rm").arg(asm_name).output();
    }
}

fn run(exec_name: &str) {
    let _ = Command::new(exec_name).output();
}

fn version() {
    println!("Version 1.0.0");
}

fn print_compile_error(error: u8) {
    println!("{}", match error {
        2 => "Error loading file bf file",
        _ => "Error while compiling"
    });
}

fn help() {
    println!("Compiler for the brainfuck language.
Syntax: bf [-h|o|t|...] file

Options:
    -h          help, shows this message
    -o          selects output file
    -t          conserve temporary files
    -s          just creates the asm file
    
    --asm-name  name of the assembly file generated
    --obj-name  name of the object file generated
    --version   shows the version of the program
    --run       run the compiled version
    ")
}

pub fn cmd() {
    let mut exec_file = "brainfucked";
    let mut o_file = "brainfucking.o";
    let mut asm_file = "brainfucking.asm";
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        if arg == "-o" || arg == "--output" {
            exec_file = &args[i+1];
        }
        if arg == "--asm-name" {
            asm_file = &args[i+1]
        }
        if arg == "--obj-name" {
            o_file = &args[i+1]
        }
    }
    if &args[1] == "--version" {
        //if args.len() > 2 {}
        version();
    }
    else if &args[1] == "--help" || &args[1] == "-h" {
        help();
    }
    else if args.contains(&String::from("-s")) {
        match compile(&args[args.len()-1], &asm_file) {
            1 => {},
            error => {
                print_compile_error(error);
                return
            }
        };
    }
    else {
        match compile(&args[args.len()-1], &asm_file) {
            1 => {},
            error => {
                print_compile_error(error);
                return
            }
        };
        assemble(&asm_file, &o_file);
        link(&exec_file, &o_file);
        if !args.contains(&String::from("-t")) {
            clean(asm_file, o_file, args.contains(&String::from("-s")));
        }
        if args.contains(&String::from("--run")) {
            run(exec_file);
        }
    }
    
}