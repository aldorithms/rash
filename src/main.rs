fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    loop {
        println!("[{}@{}]{}\n$",std::env::var("USERNAME")?, std::env::var("COMPUTERNAME")?, std::env::current_dir()?.display());
        let command: Vec<&str> = input_macro::input!().split_whitespace().collect();
        let matches = rash_cmd().try_get_matches_from(vec!["rash"])?;
    }
}

fn rash_cmd() -> clap::Command {
    clap::Command::new("rash")
        .subcommand(
            clap::Command::new("Change-Directory")
                .alias("chdir").alias("cd")
                .args(&[
                    clap::arg!(path: <PATH>).value_parser(clap::value_parser!(std::path::PathBuf)),
                ]),

    
        )

}