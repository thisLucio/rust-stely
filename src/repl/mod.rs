use std;
use std::io;
use std::num::ParseIntError;
use std::io::Write;

use nom::types::CompleteStr;

use crate::vm::VM;
use crate::assembler::program_parsers::{Program, program};

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {

    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![]
        }
    }


    pub fn run(&mut self) {
        println!("Hey it's Stely! Come on, give me some bits ;)");

        loop {
            let mut buffer = String::new();

            let stdin = io::stdin();
            print!(">>>> ");

            io::stdout().flush().expect("STELY_ERROR::UNABLE TO FLUSH STDOUT");
            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("Stely VM says Goodbye!");
                    std::process::exit(0);
                },
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                },
                ".program" => {
                    println!("Listing instructions currently in VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing");

                },
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing")
                },
                _ => {
                    let program = match program(buffer.into()) {
                        // Rusts pattern matching is pretty powerful an can even be nested
                        Ok((_,  program)) => program,
                        Err(_) => {
                            println!("Unable to parse input");
                            continue;
                        }
                    };
                    // The `program` is `pub` anyways so you can just `append` to the `Vec`
                    self.vm.program.append(&mut program.to_bytes());

                }
            }

        }
    }

    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError>{
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results:  Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
}
