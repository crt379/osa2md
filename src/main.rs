use serde_json::Value;
use std::fs;

mod cli;
mod otd;
mod common;

fn main() {
    let matches = cli::matches();

    let data = matches.get_one::<String>("data").unwrap();

    let template = matches.get_one::<String>("input").unwrap();

    let openapi = read_json(data);
    let contents = fs::read_to_string(template).unwrap();

    let rows: Vec<&str> = contents.lines().map(|line| line).collect();
    // rows.clone().into_iter().for_each(|row| println!("{}", row));

    let otds = otd::otd::Otd::parse(&rows);
    // println!("{:?}", otds);

    let mut stack = otd::stack::Stack::new();
    stack.push_val(openapi);

    let funcmanage = otd::func::OtdFuncManage;

    for otd in otds.iter() {
        otd.run(&mut stack, &funcmanage);
    }
}

fn read_json(filepath: &str) -> Value {
    let file_content = fs::read_to_string(filepath).expect("error: reading file field");
    serde_json::from_str(&file_content).expect("error: serializing to json field")
}
