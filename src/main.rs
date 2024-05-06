mod tokenizer;

use std::process::{exit, ExitCode};
use std::env;
use std::thread::spawn;
use tokenizer::{include::Token, tokenizer::Tokenizer};
use std::sync::mpsc::channel;

static OK: u8 = 0;
static ERROR: u8 = 1;


struct RequestParameters {
    request: String,
    json_file: String,
    pretty: bool,
    file_sql: String,
    ide: bool
}

impl RequestParameters {
    fn new() -> RequestParameters {
        RequestParameters{
            request: String::new(),
            pretty: false,
            json_file: String::new(),
            file_sql: String::new(),
            ide: false
        }
    }
}

/// Parameters:
///     -ide: Start the Iris ide
///     -j $file_name.json : export the result in the file_name.json file
///     -d "SQL REQUEST" : performs the query
///     -f $file_name.sql : execute the content of the .sql file.
fn main() -> ExitCode{
    let args: Vec<String> = env::args().collect();
    let mut req = RequestParameters::new();
    let mut iter = args.iter().skip(1);
    while let Some(elt) = iter.next() {
        match &elt as &str {
            "-j" => {
                if let Some(path) = iter.next() {
                    req.json_file = path.to_string();
                }else{
                    eprintln!("COMMAND LINE ERROR: You didn't precise the file path with the '-f' parameter.");
                    return ExitCode::from(ERROR)
                }
            }
            "-ide" => req.ide = true,
            "-f" => {
                if let Some(path) = iter.next() {
                    req.file_sql = path.to_string();
                }else{
                    eprintln!("COMMAND LINE ERROR: You didn't precise the file path with the '-f' parameter.");
                    return ExitCode::from(ERROR)
                }
            }
            "-d" => {
                if let Some(request) = iter.next() {
                   req.request = request.to_string()
                }
            }
            "-p" => req.pretty = true,
            _ => {
                eprintln!("COMMAND LINE ERROR: Unknow parameter: {}", elt);
                return ExitCode::from(ERROR)
            }
        }
    }

    let (sender, receiver) = channel::<Token>();
    let mut tokenizer = Tokenizer::new(sender);
    if !req.request.is_empty() && !req.file_sql.is_empty() {
        eprintln!("COMMAND LINE ERROR: You cannot at the same time load a file and a query, do it in two separate steps.");
        exit(1);
    }
    if !req.request.is_empty() {
        spawn(move ||
              tokenizer.tokenize_query(&req.request)
        );
    } else if !req.file_sql.is_empty() {
        spawn(move ||
              tokenizer.tokenize_file(&req.file_sql)
        );
    }
   
    let mut keep_compile: bool = true;
    while keep_compile {
        match receiver.recv(){
            Ok(token) => println!("New token : {}", token.content),
            Err(_) => keep_compile = false
        };
    }
    println!("Everything is ok");
    ExitCode::from(OK)
}


