pub use crate::tokenizer::include::{Token, TokenType, Flag};
pub use std::process::exit;
pub use super::expression_evaluator::ExpressionEvaluator;
pub use super::string_builder::StringBuilder;
pub use std::collections::HashMap;
pub type ConsumeResult = Result<(), String>;

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

pub fn from_string_to_type(s: String) -> Type {
    match &s as &str {
        "INT" => Type::Int,
        "STRING" => Type::String,
        "BOOL" => Type::Bool,
        _ => panic!("Type {s} doesn't exist")
            
    }
}

#[derive(Debug)]
pub struct Value {
    number: i64,
    string: String
}

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

    fn val(&self) -> i64 {
        self.number
    }

    fn string(&self) -> &String {
        &self.string
    }

    fn string_mut(&mut self) -> &mut String {
        &mut self.string
    }
    
}

#[derive(Debug)]
pub struct Column {
    name: String,
    type_col: Type,
    is_p_key: bool,
    default_value: Option<Value>,

    flag: bool
}

impl Column {

    pub fn new_empty() -> Column {
        Column {
            name: String::new(),
            type_col: Type::Int,
            is_p_key: false,
            default_value: None,
            flag: false
        }
    }

    pub fn set_type(&mut self, t: Type) {
        self.type_col = t
    }
    
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn set_as_p_key(&mut self) {
        self.is_p_key = true;
    }

    pub fn set_default_value(&mut self, val: i64) {
        println!("{val}");
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

    pub fn has_default_value(&self) -> bool {
        self.default_value.is_some()
    }

    
}

#[derive(Debug)]
pub struct Table {
    name: String,
    columns: HashMap<String, Column>
}

impl Table {
    

    /// Créer un nouvelle table et la renvoie sans la charger dans la database.
    pub fn new() -> Table {
        Table {
            name: String::new(),
            columns: HashMap::new()
        }
    }

    /// Set the name of the table
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn has_name(&self) -> bool {
        !self.name().is_empty()
    }
    
    /// Load the table from the database and return it
    pub fn load_table() -> Table {
        todo!("Load the table from the database");
    }

    /// Add a column in the given table
    pub fn add_column(&mut self, column: Column) {
        self.columns.insert(column.name().clone(), column);
    }

    /// Indicate if the given column exists
    pub fn column_exists(&self, name: &String) -> bool {
        self.columns.contains_key(name)
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
}


pub struct Database {
    tables: HashMap<String, Table>
}


impl Database {

    pub fn new_empty() -> Database {
        Database {
            tables: HashMap::new()
        }
    }

    /// Load the database (only the tables data not each lines..) and return it
    pub fn load() -> Database {
        todo!()
    }
    
    /// Add a table in the database, in the database of the program and in the one of the system
    pub fn add_table(&mut self, table: Table) {
        self.tables.insert(table.name().clone(), table);
        // TODO: Save the table in the database
    }

    /// Indicate if the given table exists
    pub fn table_exists(&self, name: &str) -> bool {
        self.tables.contains_key(name)
    }


    /// Delete the table in the database of the program and in the one of the system
    pub fn delete_table(&mut self, name: &String) {
        self.tables.remove(name);
        // TODO: Supprimer la table du système
    }

    pub fn reset_database(&mut self) {
        self.tables.clear();
        // TODO: Supprimer toutes les tables du system
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
