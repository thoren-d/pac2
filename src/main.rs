use pac2;

use pac2::PacfileBuilder;

#[derive(Debug)]
struct Config {
    sort: bool,
    name: String,
    files: Vec<String>,
    help: bool,
    stdin: bool,
}

impl Config {
    fn from_args(args: impl Iterator<Item = String>) -> Config {
        let mut res = Config {
            sort: true,
            name: String::new(),
            files: Vec::new(),
            help: false,
            stdin: false,
        };

        for arg in args {
            if arg.starts_with("--") {
                if arg == "--no-sort" {
                    res.sort = false;
                } else if arg == "--help" {
                    res.help = true;
                } else if arg == "--stdin" {
                    res.stdin = true;
                }
            } else if res.name.is_empty() {
                res.name = arg.to_owned();
            } else {
                res.files.push(arg);
            }
        }

        res
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::from_args(std::env::args().skip(1));

    if config.help {
        println!("Usage: pac2 [--no-sort] [--help] [--stdin] <pacfile name> <files...>");
        return Ok(());
    }

    if config.stdin {
        loop {
            let mut line = String::new();
            match std::io::stdin().read_line(&mut line) {
                Ok(0) => {
                    println!("{}", line);
                    break;
                }
                Ok(_) => {
                    let line = line.trim();
                    if !line.is_empty() {
                        config.files.push(line.to_owned());
                    }
                }
                Err(error) => {
                    return Err(error.into());
                }
            }
        }
    }

    println!("Config: {:?}", config);

    let mut builder = PacfileBuilder::new();
    for file in config.files {
        builder.add(file.clone(), file);
    }

    if config.sort {
        builder.sort();
    }

    builder.build(std::path::Path::new(&config.name).with_extension("p2i"))?;

    Ok(())
}
