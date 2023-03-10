use crate::http_request::HttpData;
use std::env;

fn help_arg() {
    let body = r#"{"id": 1, "title": "foo", "body": "bar", "userId": 1}"#;
    println!(
        r#"
██████  ██    ██ ███████ ███████ 
██   ██ ██    ██ ██      ██    ██
██████  ██    ██ ███████ ██    ██
██   ██ ██    ██      ██ ███████  
██   ██  ██████  ███████ ██   

Usage:  
 -u, --url                  HTTP url direction (--url https://yourapi.dev)
 -m, --method               HTTP methods: GET, POST, PUT, PATCH, DELETE (--method GET | POST | PUT | PATCH | DELETE)
 -b, --body                 Body: http body (--body '{}') 
 -h, --header               Header: (--header 'Content-Type: application/json') 
 -H, --help                 Get help for commands
 -b, --version              Version information
        "#,
        body
    );
}

fn checking_arguments(args: &Vec<String>) -> bool {
    let mut counter = 0;
    let valid_flags = vec![
        "--url", "-u", "--method", "-m", "--body", "-b", "--header", "-h",
    ];
    let mut method_get = false;
    for arg in args {
        if arg == "--help" || arg == "-H" || arg == "--version" || arg == "-v" {
            return true;
        }
        if arg == "GET" || arg == "get" || arg == "DELETE" || arg == "delete" {
            method_get = true;
        }
        for flag in &valid_flags {
            if arg == flag {
                counter += 1;
            }
        }
    }
    if method_get {
        return counter >= 2;
    }
    if !(counter >= valid_flags.len() / 2) {
        println!(
            r#"
RUSP: Missing some of these flags: --header --url --body --method
RUSP ADVICE: try 'rusp --help'
        "#
        );
        return false;
    }
    return true;
}

pub fn args() {
    let args: Vec<String> = env::args().collect();
    if !checking_arguments(&args) {
        return;
    }
    let mut http_data = HttpData {
        header: vec![],
        path: vec![],
        body: String::from(""),
        method: String::from(""),
        url: String::from(""),
    };
    let mut index = 0;
    for arg in &args {
        match arg.as_str() {
            "--url" | "-u" => {
                http_data.url = args.get(index + 1).unwrap().to_string();
            }
            "--method" | "-m" => {
                http_data.method = args.get(index + 1).unwrap().to_string();
            }
            "--body" | "-b" => {
                http_data.body = args.get(index + 1).unwrap().to_string();
            }
            "--header" | "-h" => http_data
                .header
                .push(args.get(index + 1).unwrap().to_string()),
            "--path" | "-p" => http_data
                .path
                .push(args.get(index + 1).unwrap().to_string()),
            "--help" | "-H" => help_arg(),
            "--version" | "-v" => println!("RUSP version 0.1.0 Beta"),
            _ => {}
        }
        index += 1;
    }
    http_data.http_request();
}
