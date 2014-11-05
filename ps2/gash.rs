//
// gash.rs
//
// Starting code for PS2
// Running on Rust 0.9
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu, David Evans
// Version 0.4
//
use std::io::Command;
use std::io::fs::PathExtensions;
use std::io::process::ProcessOutput;
use std::io;
use std::os;

mod history;


fn cd(args: &[&str]) {
    let path = Path::new(args[0]);

    if !path.exists() {
        println!("no such file or directory: {}", &args[0]);
    }

    else if path.is_file() {
        println!("not a directory: {}", &args[0]);
    }

    else {
        os::change_dir(&path);
    }
}

fn exec(program: &str, args: &[&str]) -> ProcessOutput {
    let output = match Command::new(program).args(args).output() {
        Ok(output) => output,
        Err(e) => fail!("failed to execute process: {}", e),
    };

    output
}

fn which(program: &str) -> bool {
    match Command::new("which").arg(program).output() {
        Ok(output) => output.status.success(),
        Err(e) => fail!("failed to execute process: {}", e),
    }
}

fn main() {
    let mut history = history::History::new(".gash_history");
    let mut stdin = io::stdin();

    loop {
        print!("gash > ");
        let line = stdin.read_line().unwrap();
        let cmd = line.as_slice().slice(0, line.len() - 1);
        history.write(cmd);

        match cmd.as_slice() {
            "" => continue,
            "exit" => break,
            _ => {},
        }

        let v: Vec<&str> = cmd.split(' ').collect();
        let program = v[0];
        let args = v.tail();

        match program {
            "cd" => { cd(args); continue },
            "history" => { history.read(); continue },
            _ => {},
        }

        match which(program) {
            false => { println!("program not found: {}", program); continue },
            true => {},
        }

        let output = exec(program, args);
        print!("{}", String::from_utf8_lossy(output.output.as_slice()));
    }
}
