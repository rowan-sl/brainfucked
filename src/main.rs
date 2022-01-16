use std::{io::{self, Read, Write}, path::PathBuf, fs::File};
use clap::Parser;

struct Interpreter<const MS: usize> {
    prog: Vec<char>,
    mem: [u8; MS],
    dp: usize,
    ip: usize,
    stdin: io::Stdin,
    stdout: io::Stdout,
}

impl<const MS: usize> Interpreter<MS> {
    pub fn new(prog: String) -> Self {
        Self {
            prog: prog.chars().filter(
                |ch| {
                    match ch {
                        '>' => true,
                        '<' => true,
                        '+' => true,
                        '-' => true,
                        '.' => true,
                        ',' => true,
                        ']' => true,
                        '[' => true,
                        _ => false,
                    }
                }
            ).collect::<Vec<char>>(),
            mem: [0; MS],
            dp: 0,
            ip: 0,
            stdin: io::stdin(),
            stdout: io::stdout(),
        }
    }

    pub fn cycle<const EOF: u8>(&mut self) -> bool {
        if self.ip == self.prog.len() {
            return  true;//done
        } else {
            let ins = self.prog[self.ip];
            // dbg!("running instruction {}", ins);
            match ins {
                '>' => {
                    self.dp += 1;
                }
                '<' => {
                    self.dp -= 1;
                }
                '+' => {
                    self.mem[self.dp] += 1;
                }
                '-' => {
                    self.mem[self.dp] -= 1;
                }
                '.' => {
                    self.stdout.write_all(&[self.mem[self.dp]]).unwrap();
                }
                ',' => {
                    let mut input: [u8; 1] = [0; 1];
                    if let Ok(_) = self.stdin.read_exact(&mut input) {
                        self.mem[self.dp] = input[0];
                    } else {
                        self.mem[self.dp] = EOF;
                    }
                }
                '[' => {
                    if self.mem[self.dp] == 0 {
                        loop {
                            self.ip += 1;
                            if self.prog[self.ip] == ']' {
                                break;
                            }
                        }
                    }
                }
                ']' => {
                    if self.mem[self.dp] != 0 {
                        loop {
                            self.ip -= 1;
                            if self.prog[self.ip] == '[' {
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
            self.ip += 1;
            // dbg!("instruction ptr at {}", self.ip);
            false
        }
    }
}


#[derive(Parser)]
#[clap(about = "Simple brainfuck interpreter", author = "Rowan S-L <rowan@fawkes.io>")]
struct Args {
    #[clap(long, short, help = "program to run")]
    prog: PathBuf
}


fn main() {
    let a = Args::parse();
    let mut program = String::new();
    let f = File::open(a.prog);
    match f {
        Ok(mut file) => {
            match file.read_to_string(&mut program) {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Failed to read file!");
                    return;
                }
            }
        }
        Err(_) => {
            eprintln!("Failed to open file");
            return;
        }
    }
    let mut interp: Interpreter<30_000> = Interpreter::new(program);
    while !interp.cycle::<0>() {}
}
