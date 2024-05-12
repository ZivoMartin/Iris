pub use crate::tokenizer::include::{Token, TokenType, Flag};
pub use std::process::exit;
pub use super::expression_evaluator::ExpressionEvaluator;
pub use super::string_builder::StringBuilder;
pub use std::collections::HashMap;
pub type ConsumeResult = Result<(), String>;
pub use super::stack::Stack;
use std::fs::{
    File,
    OpenOptions,
    remove_file
};
use std::io::{
    Write,
    Seek,
    Read
};

use std::path::{
    Path,
};

use std::fmt;

pub use serde_json::{
    json,
    Value as JsonValue,
    Map,
    Number
};

use crate::get_iris_path;

pub static ALL_INDICATOR: &str = "*";

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Type {
    Int,
    String,
    Bool
}

impl Copy for Type {}

impl Clone for Type {

    fn clone(&self) -> Type {
        *self
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_uppercase())
    }
}


pub fn from_string_to_type(s: String) -> Type {
    match &s as &str {
        "INT" => Type::Int,
        "STRING" => Type::String,
        "BOOL" => Type::Bool,
        _ => panic!("Type {s} doesn't exist")
            
    }
}

#[derive(Debug)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct Value {
    number: i64,
    string: String
}

#[allow(dead_code)]
impl Value {

    pub fn new_by_val(val: i64) -> Value {
        Value {
            number: val,
            string: format!("{val}")
        }
    }

    pub fn new_by_string(s: &mut StringBuilder, hash: bool) -> Value {
        let val = if hash { s.hash() } else { 0 };
        Value {
            number: val,
            string: s.extract()
        }
    }

    pub fn from_json(data: &JsonValue) -> Value {
        match data {
            JsonValue::String(string) => Value::new_by_pure_string(string.clone(), true),
            JsonValue::Number(number) => Value::new_by_val(number.as_i64().expect("Failed to convert the serde_json Number to i64")),
            _ => panic!("Unexpected serde_json value for a column: {data:?}")
        }
    }

    pub fn new_by_pure_string(s: String, hash: bool) -> Value {
        Value::new_by_string(&mut StringBuilder::from_string(s), hash)
    }

    pub fn val(&self) -> i64 {
        self.number
    }

    pub fn string(&self) -> &String {
        &self.string
    }

    pub fn string_mut(&mut self) -> &mut String {
        &mut self.string
    }
    
}

fn extract_string_from_json(json_value: &JsonValue) -> String {
     match json_value {
         JsonValue::String(string) => string.to_string(),
         _ => panic!("Failed to catch a string for the column name")
     }
}

fn extract_vec_from_json(json_value: &JsonValue) -> Vec::<JsonValue> {
     match json_value {
         JsonValue::Array(arr) => arr.clone(),
         _ => panic!("Failed to catch a vector for the column name")
     }
}

fn extract_map_from_json(json_value: &mut JsonValue) -> &mut Map::<String, JsonValue> {
     match json_value {
         JsonValue::Object(map) => map,
         _ => panic!("Failed to catch a vector for the column name")
     }
}

#[derive(Debug)]
pub struct Column {
    name: String,
    type_col: Type,
    default_value: Option<Value>,
    flag: bool
}

impl Column {

    pub fn new_empty() -> Column {
        Column {
            name: String::new(),
            type_col: Type::Int,
            default_value: None,
            flag: false
        }
    }

    fn load(json_data: &JsonValue) -> Column {
        let mut column = Column::new_empty();
        column.set_name(extract_string_from_json(&json_data["name"]));
        column.set_type(from_string_to_type(extract_string_from_json(&json_data["type_col"])));
        column.load_default_value(extract_string_from_json(&json_data["default_value"]));
        column
    }

    fn load_default_value(&mut self, default_value: String) {
        if !default_value.is_empty() {
            match self.type_col {
                Type::String => self.default_value = Some(Value::new_by_pure_string(default_value, true)),
                _ => self.set_default_value(default_value.parse::<i64>().expect("Failed to parse the default value."))
            }
        }
    }
    
    pub fn set_type(&mut self, t: Type) {
        self.type_col = t
    }
    
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn set_default_value(&mut self, val: i64) {
        self.default_value = Some(Value::new_by_val(val))
    }

    pub fn get_type(&self) -> Type {
        self.type_col
    }
    
    pub fn set_value_by_string(&mut self, s: &mut StringBuilder) {
        self.default_value = Some(Value::new_by_string(s, false));
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_empty(&self) -> bool {
        self.name().is_empty()
    }

    pub fn flag(&self) -> bool {
        self.flag
    }

    pub fn active_flag(&mut self) {
        self.flag = true
    }

    pub fn disable_flag(&mut self) {
        self.flag = false
    }

    pub fn default_value(&self) -> &Value {
        self.default_value.as_ref().expect(&format!("The column {} doesn't have a default value.", self.name()))
    }
    
    pub fn has_default_value(&self) -> bool {
        self.default_value.is_some()
    }

    fn get_datas(&self) -> JsonValue {
        json!({
            "name": self.name(),
            "type_col": self.type_col.to_string(),
            "default_value": if self.default_value.is_some() { self.default_value.as_ref().unwrap().string() } else { "" }
        })
    }

  
}

fn open_file(file_path: &str) -> File {
    OpenOptions::new()
        .append(true)
        .read(true)
        .open(file_path)
        .expect(&format!("failed to open the file {}", file_path))
}

#[derive(Debug)]
pub struct Table {
    name: String,
    columns: HashMap<String, Column>,
    p_key: String,
    table_file: Option<File>,
    lines: Vec<JsonValue>
}

impl Table {
    

    /// Créer une nouvelle table et la renvoie sans la charger dans la database.
    pub fn new() -> Table {
        Table {
            name: String::new(),
            columns: HashMap::new(),
            p_key: String::new(),
            table_file: None,
            lines: Vec::new()
        }
    }

    fn load(json_data: &JsonValue) -> Table {
        let mut table = Table::new();
        table.set_name(extract_string_from_json(&json_data["name"]));
        table.set_pkey(extract_string_from_json(&json_data["p_key"]));
        match &json_data["columns"] {
            JsonValue::Array(columns) =>  {
                for c in columns {
                    table.add_column(Column::load(c));
                }
            },
            _ => panic!("Failed to catch the columns as an array")
        }
        table
    }

    pub fn save(&mut self) {
        let path = self.get_table_file_path();
        File::create(Path::new(&path)).expect(&format!("Failed to create the file of the table {}", self.name()));
        self.table_file = Some(open_file(&path));
        self.table_file().write_all("[]".as_bytes()).expect("Failed to write");
        self.lines = Vec::new();
    }

    pub fn drop(&mut self) {
        remove_file(&self.get_table_file_path()).expect(&format!("Failed to remove the file of the table {}", self.name))
    }

    pub fn insert(&mut self, asked_cols: &Vec<String>, values: &Vec<Value>) {
        let mut map = Map::<String, JsonValue>::new();
        for (col, val) in asked_cols.iter().zip(values.iter()) {
            map.insert(col.clone(), if self.get_column(col).get_type() == Type::String { JsonValue::String(val.string().clone()) } else { JsonValue::Number(Number::from(val.val())) });
        }
        self.lines.push(JsonValue::Object(map));
        self.actualise_table_file();
    }

    pub fn actualise_table_file(&mut self) {
        let lines = self.lines.clone();
        self.table_file().set_len(0).expect("Failed to reset the data file len");
        self.table_file()
            .write_all(JsonValue::Array(lines).to_string().trim().as_bytes())
            .expect("write failed");
    }
    
    pub fn get_table_file_path(&self) -> String {
        get_iris_path() + self.name()
    }

    pub fn table_file(&mut self) -> &mut File {
        self.table_file.as_mut().expect(&format!("Failed to unwrap the file of the table {}", self.name))
    }
    
    pub fn set_pkey(&mut self, p_key: String) {
        self.p_key = p_key;
    }
    
    /// Set the name of the table
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn has_name(&self) -> bool {
        !self.name().is_empty()
    }

    /// Add a column in the given table
    pub fn add_column(&mut self, column: Column) {
        self.columns.insert(column.name().clone(), column);
    }

    /// Indicate if the given column exists
    pub fn column_exists(&self, name: &String) -> bool {
        self.columns.contains_key(name)
    }

    pub fn drop_lines(&mut self, stack_line_number: &mut Stack<usize>) {
        while !stack_line_number.is_empty() {
            self.lines.remove(stack_line_number.pop().unwrap());
        }
        self.actualise_table_file();
    }
    
    pub fn get_column(&self, name: &String) -> &Column {
        self.columns.get(name).expect(&format!("ERROR: Column {name} doesn't exists in the table {}", self.name))
    }

    
    pub fn get_column_mut(&mut self, name: &String) -> &mut Column {
        self.columns.get_mut(name).expect(&format!("ERROR: Column {name} doesn't exists in the table {}", self.name))
    }

    pub fn column_exists_without_flag(&self, name: &String) -> bool {
        if let Some(c) = self.columns.get(name) {
            !c.flag()
        } else  {
            false
        }
    }

    pub fn active_column_flag(&mut self, name: &String) {
        self.get_column_mut(name).active_flag()
    }
    
    pub fn reset_all_flags(&mut self) {
        for (_, c) in self.columns.iter_mut() {
            c.disable_flag()
        }
    }
    
    /// Simply returns the name of the given table
    pub fn name(&self) -> &String {
       &self.name
    }

    pub fn get_cols(&self) -> &HashMap<String, Column> {
        &self.columns
    }

    pub fn p_key(&self) -> &String {
        &self.p_key
    }
    
    fn get_datas(&self) -> JsonValue {
        json!({
            "name": self.name(),
            "p_key": self.p_key(),
            "columns": self.columns.iter().map(|(_, c)| {
                c.get_datas()
            }).collect::<Vec<_>>()
        })
    }

    pub fn browse(&mut self, browser: &mut dyn BrowserReq) {
        for (i, line) in self.lines.iter_mut().enumerate() {
            let map = extract_map_from_json(line);
            if browser.get_expr().compute(map, false) != 0 {
                browser.browse_action(map, i);
            }
        }
    }
}


pub struct Database {
    tables: HashMap<String, Table>,
    data_file: File,
    json_table_data: Vec<JsonValue>
}


impl Database {

    pub fn new_empty() -> Database {
        Database {
            tables: HashMap::new(),
            data_file: Database::load_data_file(),
            json_table_data: Vec::new()
        }
    }

    fn load_data_file() -> File {
        let iris_path = get_iris_path();
        let mut create = false;
        let data_path = iris_path + "/tables.json";
        let path = Path::new(&data_path);
        if !path.exists() {
            create = true;
            File::create(path).expect("Failed to create the data file");
        }
        let mut file = open_file(&data_path);
        if create {
            file.write_all("[]".as_bytes()).expect("write failed");
        }
        file
    }

    
    /// Load the database (only the tables data not each lines..) and return it
    pub fn load() -> Database {
        let mut res = Database::new_empty();
        res.load_table_vec_from_file();
        let mut map = HashMap::new();
        for table in res.json_table_data.iter() {
            let table = Table::load(table); 
            map.insert(table.name().clone(), table);
        }
        res.tables = map;
        res
    }


    fn load_table_vec_from_file(&mut self)  {
        let data_file_content = self.get_data_file_content();
        let datas: JsonValue = serde_json::from_str(if data_file_content.is_empty() { "[]" } else  { &data_file_content }).expect("Failed to extract json data file.");
        self.json_table_data = extract_vec_from_json(&datas);
    }
    
    fn get_data_file_content(&mut self) -> String {
        self.data_file.seek(std::io::SeekFrom::Start(0)).expect("Failed to seek during the reading of data file");
        let mut result = String::new();
        self.data_file.read_to_string(&mut result).expect("Failed to read the data file");
        result
    }
    
    /// Add a table in the database, in the database of the program and in the one of the system
    pub fn add_table(&mut self, mut table: Table) {
        table.save();
        self.json_table_data.push(table.get_datas());
        self.actualise_data_file();
        self.insert_table(table);
    }

    fn actualise_data_file(&mut self) {
        self.data_file.set_len(0).expect("Failed to reset the data file len");
        self.data_file
            .write_all(JsonValue::Array(self.json_table_data.clone()).to_string().trim().as_bytes())
            .expect("write failed");
    }

    fn insert_table(&mut self, table: Table) {
        self.tables.insert(table.name().clone(), table);
    }

    /// Indicate if the given table exists
    pub fn table_exists(&self, name: &str) -> bool {
        self.tables.contains_key(name)
    }


    /// Delete the table in the database of the program and in the one of the system
    pub fn delete_table(&mut self, name: &String) {
        self.tables.get_mut(name).expect(&format!("Drop error: The table {} doesn't exists", name)).drop();
        self.tables.remove(name);
        let mut i = 0;
        for t in self.json_table_data.iter() {
            if extract_string_from_json(&t["name"]) == *name {
                break;
            }
            i += 1;
        }
        self.json_table_data.remove(i);
        self.actualise_data_file();
    }

    pub fn reset_database(&mut self) {
        self.tables.clear();
        self.data_file.set_len(0).expect("failed to reset the data file");
        self.load_table_vec_from_file();
    }

    pub fn get_table(&self, name: &String) -> &Table {
        self.tables.get(name).expect(&format!("ERROR: The table {name} doesn't exists."))
    }

    pub fn get_table_mut(&mut self, name: &String) -> &mut Table {
        self.tables.get_mut(name).expect(&format!("ERROR: The table {name} doesn't exists."))
    }

    
    pub fn test_column_existance(&self, table_name: &String, cols: &Vec<String>) -> ConsumeResult {
        let table = self.get_table(&table_name);
        for c in cols.iter() {
            if !table.column_exists(c) {
                return Err(format!("Error during the selection on {}: the column {} doesn't exists.", table_name, c))
            }
        }
        Ok(())
    }
    
}

pub type BoxedReq = Box<dyn Request>;

pub  trait Request {
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult;

    fn new() -> BoxedReq where Self: Sized;

    fn end(&mut self, database: &mut Database) -> ConsumeResult;
    
    fn panic_bad_token(&self, token: Token, name: &str) {
        eprintln!("Tried to conusme an unexpected token in {name}: {type_token:?}: {content}", type_token=token.token_type, content=token.content);
        exit(1);
    }
    
}

pub trait BrowserReq {

    fn browse_action(&mut self, line: &mut Map::<String, JsonValue>, i: usize);

    fn get_expr(&mut self) -> &mut ExpressionEvaluator;
    
}
