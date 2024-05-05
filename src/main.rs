//=====================================================================================================================
// My first small rust program! 
// It is a simple shell with only a few builtins, I have made a few of these.
// One in C and another in Zig(Still need to add [*:0] strings, found a way to do it but lost interest in the program)
//=====================================================================================================================
use std::{
    env::{current_dir, set_current_dir}, fs::{create_dir, create_dir_all}, io::{self, Write}, path::Path, process::Command, rc::Rc
};
use colored::*;
fn main() {
        let mut args: Vec<String> = vec![];
        // start loop
        println!("{} {}", "Welcome to", "Rush!".blue().italic().bold());
        loop {
        println!("{}", current_dir().unwrap().display());
        print!("{} ", "$>".purple().bold());
        io::stdout().flush().expect("Flush failed!"); // flush the stdout so the print above appears
        args.clear();   // clear out args vector
        // read in line
        let mut buffer = String::new(); 
            io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line!");
        // create iterator, split by whitespace
        let buffer_split = shell_words::split(&buffer).expect("Failed to split buffer!");
        // push into vector
        for part in buffer_split {
            args.push(part.into());
        }
        //======================================
        //========Start matching builtins=======
        //======================================
        if args.last().is_none() { // check if the vector is empty
            println!(" ");
        } else {
        // vector is not empty, match builtin
        match &args[0] as &str {
            "pwd" => {
                let pwd = current_dir().unwrap();
                println!("{}", pwd.display());
            }
            "cd" => {
                let path = Path::new(&args[1]);
                set_current_dir(&path).unwrap_or_else(|err| println!("{} {}", "Error:".red(), Rc::new(err)));
            }
            "hostname" => {
                println!("{:?}", hostname::get().unwrap());
            }
            "mkdir" => {
                if args[1] == "-p" {
                    create_dir_all(&args[2]).unwrap_or_else(|e| println!("{} {}", "Error Creating Directory:".red(),e));
                } else {
                    create_dir(&args[1]).unwrap_or_else(|e| println!("{} {}", "Error Creating Directory:".red(),e));
                }
            }
            "help" => help_msg(),

            "exit" => return,


        //======================================
        //============End builtins=============+
        //======================================
            _ => {
                    let output = Command::new(&args[0])
                    .args(&args[1..])
                    .spawn();
                    // match for error handling!
                    match output {
                    Ok(mut output) => {let _ = output.wait();},
                    Err(e) => eprintln!("{} {}", "Error:".red(),e),
                       }
             }
           }
            println!(" ");
        }
   }
}

fn help_msg() {
    println!("{} is a simple shell with not very many features", "Rush".blue().bold().italic());
    println!("The best help I can give is either look at the README or at the source code");
    println!("The only builtin that accepts any flags is {}, which will accept {} to make any parent directories", "mkdir".purple().bold(), "-p".bold().purple());
    println!("Have fun! And please try and break it!");
}
