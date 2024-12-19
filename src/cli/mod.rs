use clap::{arg, command, ArgMatches};

pub fn matches() -> ArgMatches {
    command!() // requires `cargo` feature
        .arg(
            arg!(
                -d --data <VALUE> "Input JSON file"
            )
            .required(true),
        )
        .arg(
            arg!(
                -i --input <VALUE> "Markdown template file"
            )
            .required(true),
        )
        .arg(
            arg!(
                -o --output <VALUE> "Output file"
            )
            .required(false),
        )
        .get_matches()
}
