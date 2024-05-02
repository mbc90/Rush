//=====================================================================================================================
// My first rust program!
// It is a simple shell with only a few builtins, I have made a few of these.
// One in C and another in Zig(Still need to add [*:0] strings, found a way to do it but lost interest in the program)
//=====================================================================================================================
use std::{
    env::{current_dir, set_current_dir}, fs::{create_dir, create_dir_all}, io::{self, Write}, path::Path, process::Command
};
fn main() {
        let mut args: Vec<String> = vec![];
        // start loop
        loop {
        print!("~$ ");
        io::stdout().flush().expect("Flush failed!"); // flush the stdout so the print above appears
        args.clear();   // clear out args vector
        // read in line
        let mut buffer = String::new(); 
            io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line!");
        // create iterator, split by whitespace
        let itr = buffer.trim().split_whitespace();
        // push into vector
        for part in itr {
            args.push(part.into());
        }
        //======================================
        //========Start matching builtins=======
        //======================================
        match &args[0] as &str {
            "pwd" => {
                let pwd = current_dir().unwrap();
                println!("{}", pwd.display());
            }
            "cd" => {
                let path = Path::new(&args[1]);
                set_current_dir(&path).unwrap_or_else(|err| println!("Error: {}", err));
            }
            "hostname" => {
                println!("{:?}", hostname::get().unwrap());
            }
            "mkdir" => {
                if args[1] == "-p" {
                    create_dir_all(&args[2]).unwrap_or_else(|e| println!("Error Creating Directory: {}", e));
                } else {
                    create_dir(&args[1]).unwrap_or_else(|e| println!("Error creating Directory: {}", e));
                }
            }
            "exit" => return,

        //======================================
        //============End builtins=============+
        //======================================
            _ => {
                    let  output = Command::new(&args[0])
                    .args(&args[1..])
                    .spawn();
                    // match for error handling!
                    match output {
                    Ok(mut output) => {let _ = output.wait();},
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }

}
