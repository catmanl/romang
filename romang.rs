use std::fs::File;
use std::env;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Check if there is a file
    rm::fail_if(env::args().len() < 2, "romang: missing file argument");

    // Set up
    let first = format!("{}", env::args().nth(1).unwrap());
    rm::fail_if(rm::get_ext(&first) != "rm", "romang: not a romang file. Required extension: .rm");
    let mut file_program = File::open(first)?;
    let mut program = String::new();
    file_program.read_to_string(&mut program)?;

    // Parse
    rm::parse(&program);

    Ok(())
}

mod rm {
    use std::path::Path;
    use std::ffi::OsStr;
    fn init_vec(size: usize) -> Vec<u8> {
        let mut zero_vec: Vec<u8> = Vec::with_capacity(size);
        for _i in 0..size {
            zero_vec.push(0);
        }
        zero_vec
    }

    pub fn fail_if(expr: bool, msg: &str) {
        if expr {
            println!("{}", msg);
            std::process::exit(1);
        }
    }

    pub fn get_ext(f: &String) -> String {
        Path::new(f).extension().and_then(OsStr::to_str).unwrap().to_string()
    }

    pub fn parse(program: &String) {
        let size: usize = 30000;
        let mut ptrs = init_vec(size);
        let mut index = 0;

        if index < size {
            for c in program.bytes() {
                let ch = c as char;
                match ch {
                    'i' | 'I' => ptrs[index] += 1,
                    'v' | 'V' => ptrs[index] += 5,
                    'x' | 'X' => ptrs[index] += 10,
                    'l' | 'L' => ptrs[index] += 50,
                    'c' | 'C' => ptrs[index] += 100,
                    '.' => print!("{}", ptrs[index] as char),
                    '>' => index += 1,
                    _ => (), // for the rest of cases just do nothing
                }
            }
            println!();
        } else {
            println!("romang: overflow");
            std::process::exit(1);
        }
    }
}

