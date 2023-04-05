use std::process::Command;
use std::env;
use std::io;



fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                if let Some(path) = args.next() {
                    if let Err(e) = env::set_current_dir(path) {
                        eprintln!("{}", e);
                    }
                } else {
                    if let Ok(path) = env::current_dir() {
                        println!("{}", path.display());
                    } else {
                        eprintln!("Impossible de récupérer le répertoire de travail actuel");
                    }
                }
            }
            "mkdir" => {
                if let Some(path) = args.next() {
                    let output = Command::new("mkdir")
                                     .arg(path)
                                     .output();
                    if let Err(e) = output {
                        eprintln!("{}", e);
                    }
                } else {
                    eprintln!("mkdir: missing operand");
                }
            }
            "rmdir" => {
                if let Some(path) = args.next() {
                    let output = Command::new("rmdir")
                                     .arg(path)
                                     .output();
                    if let Err(e) = output {
                        eprintln!("{}", e);
                    }
                } else {
                    eprintln!("rmdir: missing operand");
                }
            }
            "dir" => {
                let output = Command::new("cmd")
                                 .args(&["/C", "dir"])
                                 .output();
                if let Err(e) = output {
                    eprintln!("{}", e);
                } else {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
            }
            "touch" => {
                if let Some(filename) = args.get(0) {
                    let file = File::create(filename);
                    match file {
                        Ok(_) => println!("Created file {}", filename),
                        Err(e) => println!("Error creating file {}: {}", filename, e),
                    }
                } else {
                    println!("touch: missing file operand");
                }
            }
            "ls" => {
                let dir = match args.next() {
                    Some(dir) => Path::new(dir),
                    None => Path::new("."),
                };
                if let Ok(entries) = fs::read_dir(dir) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if let Some(name) = entry.file_name().to_str() {
                                println!("{}", name);
                            }
                        }
                    }
                }
            }
            "pass" => {
                if let Some(directory) = args.get(0) {
                    if let Err(e) = env::set_current_dir(&directory) {
                        println!("pass: {}: {}", directory, e);
                    }
                } else {
                    println!("pass: missing directory operand");
                }
            }
            "rmdir" => {
                if let Some(directory) = args.get(0) {
                    let path = Path::new(&directory);
                    if path.is_dir() {
                        if let Err(e) = fs::remove_dir(path) {
                            println!("rmdir: error removing directory {}: {}", directory, e);
                        } else {
                            println!("Removed directory {}", directory);
                        }
                    } else {
                        println!("rmdir: {} is not a directory", directory);
                    }
                } else {
                    println!("rmdir: missing directory operand");
                }
            }
           
            _ => {
                println!("{}: command not found", command);
            }
        }
    }
}

            
            
            
            
            
            
            
            
            
            
            
