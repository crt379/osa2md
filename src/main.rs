use serde_json::Value;
use std::fs;

mod otd;

fn main() {
    let openapi = read_json("Northwind-V3.openapi3.json");

    let template = vec![
        "$go(paths, paths);",
        "$for(paths, path{/Orders}, m_o):",
        "# $get(path);",
        "",
        "$for(m_o, method{!parameters}, o):",
        "## $get(method);",
        "",
        "### 描述",
        "",
        "$get(o.summary);",
        "",
        "$tryget(o.description);",
        "$;",
        "$;",
    ];

    let otds = otd::otd::Otd::parse(&template);
    println!("{:?}", otds);

    let mut stack = otd::stack::Stack::new();
    stack.push_ref(openapi);
    for otd in otds.iter() {
        otd.run(&mut stack);
    }
}

fn read_json(filepath: &str) -> Value {
    let file_content = fs::read_to_string(filepath).expect("LogRocket: error reading file");
    serde_json::from_str(&file_content).expect("LogRocket: error serializing to JSON")
}

