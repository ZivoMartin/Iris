mod tokenizer;
mod interpreteur;
mod c_extention;
use interpreteur::interpreteur::Interpreteur;
use std::process::exit;
use std::thread::spawn;
use tokenizer::{include::{TokenType, TokenizerMessage}, tokenizer::Tokenizer};
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::fs::{
    File,
    OpenOptions,
    create_dir
};
use std::io::Read;
use std::path::Path;

pub fn begin(mut args: Vec<String>) {
    pre_init_database();
    let mut actions = Vec::<Box<dyn RequestParameter>>::new();
    let mut iter = args.iter_mut().skip(1);
    while let Some(elt) = iter.next() {
        match &elt as &str {
            "-j" => todo!("Enregistrer le fichier de sortie"),
            "-run" => actions.push(Run::new()),
            "-f" => {
                let mut path = iter.next();
                if path.is_some() {
                    actions.push(OneFile::new(path.take().unwrap().to_string()))
                }else{
                    error_catched("You didn't precise the file path with the '-f' parameter.");
                }
            }
            "-d" => {
                let mut query = iter.next();
                if query.is_some() {
                   actions.push(OneQuery::new(query.take().unwrap().to_string()))
                } else {
                    error_catched("You didn't precise the query with the '-d' parameter.");
                }
            }
            _ => error_catched(&format!("Unknow parameter: {}", elt))
        }
    }
    let (sender, receiver) = channel::<TokenizerMessage>();
    let mut interp = Interpreteur::new();
    let mut tokenizer = Tokenizer::new(sender);
    for act in actions.iter_mut() {
        tokenizer = act.execute(tokenizer, &mut interp, &receiver);
    }
}

fn pre_init_database() {
    let iris_path = get_iris_path();
    if !Path::new(&iris_path).is_dir() {
        create_dir(&iris_path).expect("Failed to create database directory");
    }
    if !Path::new(&(iris_path.clone() + ENTRY_FILE)).exists() {
        File::create(iris_path + ENTRY_FILE).expect("Failed to create entry file");
    }
}


fn execute(interp: &mut Interpreteur, receiver: &Receiver<TokenizerMessage>) -> Result<Tokenizer, String> {
    let mut tokenizer: Option<Tokenizer> = None;
    while tokenizer.is_none() {
        match receiver.recv().expect("Something went wrong") {
            TokenizerMessage::Token(token) =>
                if token.token_type == TokenType::ERROR {
                    return Err(token.content)
                } else if let Err(e) = interp.new_token(token) {
                    return Err(e)
                }
            TokenizerMessage::Tokenizer(the_tokenizer) => tokenizer = Some(the_tokenizer)
        }
    }
    Ok(tokenizer.take().expect("Failed to catch the tokenizer throught the threads."))
}

fn error_catched(err: &str) {
    println!("{err}");
    exit(1)
}

pub fn get_iris_path() -> String {
    homedir::get_my_home().unwrap().unwrap().into_os_string().into_string().unwrap() + "/.iris/"
}




trait RequestParameter {

    fn execute(&mut self, tokenizer: Tokenizer, interp: &mut Interpreteur, receiver: &Receiver<TokenizerMessage>) -> Tokenizer;
    
}

struct OneQuery {
    query: String
}

impl OneQuery {

    fn new(query: String) -> Box<dyn RequestParameter> {
        Box::from(OneQuery { query })
    }
       
}

impl RequestParameter for OneQuery {

    fn execute(&mut self, tokenizer: Tokenizer, interp: &mut Interpreteur, receiver: &Receiver<TokenizerMessage>) -> Tokenizer {
        let query = self.query.clone();
        spawn(move ||
              tokenizer.tokenize_query(query)
        );
        match execute(interp, receiver) {
            Ok(tokenizer) => {
                println!("The execution of the query {} has been a success.", self.query);
                return tokenizer
            }
            Err(e) => error_catched(&e)
        }
        panic!("Impossible case");
    }
}


struct OneFile {
    path: String
}

impl OneFile {

    fn new(path: String) -> Box<dyn RequestParameter> {
        Box::from(OneFile { path })
    }
    
}

impl RequestParameter for OneFile {

    fn execute(&mut self, tokenizer: Tokenizer, interp: &mut Interpreteur, receiver: &Receiver<TokenizerMessage>) -> Tokenizer {
        let path = self.path.clone();
        spawn(move ||
              tokenizer.tokenize_file(path)
        );
        match execute(interp, receiver) {
            Ok(tokenizer) => {
                println!("The execution of the file {} has been a success.", self.path);
                return tokenizer;
            }
            Err(e) => error_catched(&e)
        };
        panic!("Impossible case");
    }
}

static ENTRY_FILE: &str = "entry_file.sql";

struct Run {
    entry_file: File
}

impl Run {
    fn new() -> Box<dyn RequestParameter> {
        Box::from(Run {
            entry_file: OpenOptions::new().read(true).open(get_iris_path() + ENTRY_FILE).expect("Failed to open entry file from runner")
        })
    }

    fn new_request(&mut self) -> bool {
        let mut content = String::new();
        self.entry_file.read_to_string(&mut content).expect("Failed to read the content of the entry file");
        !content.is_empty()
    }
}

impl RequestParameter for Run {

    fn execute(&mut self, mut tokenizer: Tokenizer, interp: &mut Interpreteur, receiver: &Receiver<TokenizerMessage>) -> Tokenizer {
        let mut executer = OneFile::new(get_iris_path() + ENTRY_FILE);
        loop {
            println!("running...");
            if self.new_request() {
                tokenizer = executer.execute(tokenizer, interp, receiver)
            }
        }
    }
}
