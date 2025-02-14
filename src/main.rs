use serde_json::Value;
use std::fs;

mod cli;
mod common;
mod otd;

fn main() {
    let matches = cli::matches();

    let data = matches.get_one::<String>("data").unwrap();

    let template = matches.get_one::<String>("input").unwrap();

    let openapi = read_json(data);
    let contents = fs::read_to_string(template).unwrap();

    let rows: Vec<&str> = contents.lines().map(|line| line).collect();
    let otds = otd::otd::Otd::parse(&rows);

    let funcmanage = otd::func::FuncManage {};
    let mut exec = otd::exec::Exec::new(otds, openapi, Box::new(funcmanage));
    exec.run();
}

fn read_json(filepath: &str) -> Value {
    let file_content = fs::read_to_string(filepath).expect("error: reading file field");
    serde_json::from_str(&file_content).expect("error: serializing to json field")
}
