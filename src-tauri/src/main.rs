use std::process::Command;
use std::env;
use std::io;

fn main() {
    loop {
        // Affiche le prompt
        print!("> ");
        io::stdout().flush().unwrap();

        // Lit l'entrée de l'utilisateur
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Supprime les espaces au début et à la fin de l'entrée
        let input = input.trim();

        // Vérifie si l'utilisateur a entré une commande
        if input.is_empty() {
            continue;
        }

        // Sépare les arguments de la commande
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        // Exécute la commande appropriée
        match command {
            "cd" => {
                // Vérifie si un argument a été fourni
                if let Some(path) = args.next() {
                    // Change le répertoire de travail actuel
                    if let Err(e) = env::set_current_dir(path) {
                        eprintln!("{}", e);
                    }
                } else {
                    // Affiche le répertoire de travail actuel
                    if let Ok(path) = env::current_dir() {
                        println!("{}", path.display());
                    } else {
                        eprintln!("Impossible de récupérer le répertoire de travail actuel");
                    }
                }
            }
            "mkdir" => {
                // Vérifie si un argument a été fourni
                if let Some(path) = args.next() {
                    // Crée un nouveau répertoire
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
                // Vérifie si un argument a été fourni
                if let Some(path) = args.next() {
                    // Supprime un répertoire existant
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
                // Affiche le contenu du répertoire actuel (pour Windows)
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
                let dir = match args.get(0) {
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
            "cd" => {
                if let Some(directory) = args.get(0) {
                    if let Err(e) = env::set_current_dir(&directory) {
                        println!("cd: {}: {}", directory, e);
                    }
                } else {
                    println!("cd: missing directory operand");
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

            
            
            
            
            
            
            
            
            
            
            
