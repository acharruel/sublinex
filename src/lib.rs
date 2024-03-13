pub struct Config {
    input_file: String,
    output_file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let input_file = args[1].clone();
        let output_file = args[2].clone();
        Ok(Config {
            input_file,
            output_file,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Input file: {}", config.input_file);
    println!("Output file: {}", config.output_file);
    Ok(())
}
