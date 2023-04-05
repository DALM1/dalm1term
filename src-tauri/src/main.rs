use std::process::{Command, Stdio};
use std::io::{self, Write};

fn main() {
    loop {
        // print the prompt
        print!("> ");
        io::stdout().flush().unwrap();

        // read the user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // parse the command and arguments
        let input = input.trim();
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        // execute the command
        match command {
            "cd" => {
                let new_dir = args.peekable()
                    .peek()
                    .map_or("/", |x| *x);
                let root = std::path::Path::new("/");
                let display = root.display();
                let mut path = std::path::PathBuf::from(&new_dir);

                if path.is_relative() {
                    path = std::env::current_dir().unwrap().join(path);
                }

                if let Err(e) = std::env::set_current_dir(&path) {
                    eprintln!("couldn't cd to {}: {}", display, e);
                }
            },
            "ls" => {
                let output = Command::new("ls")
                    .output()
                    .expect("failed to execute process");
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            },
            "mkdir" => {
                let dir_name = args.peekable()
                    .peek()
                    .expect("no directory name specified");
                let result = std::fs::create_dir(dir_name);
                if let Err(e) = result {
                    eprintln!("couldn't create directory {}: {}", dir_name, e);
                }
            },
            "touch" => {
                let file_name = args.peekable()
                    .peek()
                    .expect("no file name specified");
                let result = std::fs::File::create(file_name);
                if let Err(e) = result {
                    eprintln!("couldn't create file {}: {}", file_name, e);
                }
            },
            command => {
                let child = Command::new(command)
                    .args(args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn();

                match child {
                    Ok(mut child) => {
                        child.wait().unwrap();
                    },
                    Err(e) => {
                        eprintln!("failed to execute process: {}", e);
                    }
                }
            }
        }
    }
}
