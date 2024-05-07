 pub use crate::tokenizer::include::{Token, TokenType};
pub use std::process::exit;
pub use super::expression_evaluator::ExpressionEvaluator;
pub use std::collections::HashMap;
pub type ConsumeResult = Result<(), String>;


#[derive(Debug)]
pub enum Type {
    INT,
    STRING,
    BOOL
}

pub fn from_string_to_type(s: String) -> Type {
    match &s as &str {
        "INT" => Type::INT,
        "STRING" => Type::STRING,
        "BOOL" => Type::BOOL,
        _ => panic!("Type {s} doesn't exist")
            
    }
}

#[derive(Debug)]
pub struct Column {
    name: String,
    type_col: Type,
    is_p_key: bool,
    default_value: Option<i64>
}

impl Column {
    pub fn new(name: String, type_col: Type, is_p_key: bool, default_value: Option<i64>) -> Column {
        Column {
            name,
            type_col,
            is_p_key,
            default_value
        }
    }

    pub fn new_empty() -> Column {
        Column {
            name: String::new(),
            type_col: Type::INT,
            is_p_key: false,
            default_value: None
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
        self.default_value = Some(val)
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_empty(&self) -> bool {
        self.name().is_empty()
    }
}

#[derive(Debug)]
pub struct Table {
    name: String,
    columns: Vec<Column>
}

impl Table {
    

    /// Créer un nouvelle table et la renvoie sans la charger dans la database.
    pub fn new() -> Table {
        Table {
            name: String::new(),
            columns: Vec::new()
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
        self.columns.push(column);
    }

    /// Indicate if the given column exists
    pub fn column_exists(&self, name: &str) -> bool {
        for c in self.columns.iter() {
            if c.name() == name {
                return true
            }
        }
        false
    }

    /// Simply returns the name of the given table
    pub fn name(&self) -> &str {
       &self.name
    }
}


pub struct Database {
    tables: Vec<Table>
}


impl Database {

    pub fn new_empty() -> Database {
        Database {
            tables: Vec::new()
        }
    }

    /// Load the database (only the tables data not each lines..) and return it
    pub fn load() -> Database {
        todo!()
    }
    
    /// Add a table in the database, in the database of the program and in the one of the system
    pub fn add_table(&mut self, table: Table) {
        self.tables.push(table);
        // TODO: Save the table in the database
    }

    /// Indicate if the given table exists
    pub fn table_exists(&self, name: &str) -> bool {
        for t in self.tables.iter() {
            if t.name() == name {
                return true
            }
        }
        false
    }


    /// Delete the table in the database of the program and in the one of the system
    pub fn delete_table(&mut self, name: &String) {
        let mut to_delete: Option<usize> = None;
        for (i, t) in self.tables.iter().enumerate() {
            if t.name() == name {
                to_delete = Some(i);
                break;
            }
        }
        if let Some(i) = to_delete {
            self.tables.remove(i);
        }
        // TODO: Supprimer la table du système
    }

    pub fn reset_database(&mut self) {
        self.tables = Vec::new();
        // TODO: Supprimer toutes les tables du system
    }
    
}

pub type BoxedReq = Box<dyn Request>;

pub  trait Request {
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult;

    fn new() -> BoxedReq where Self: Sized;

    fn end(&mut self, database: &mut Database);
    
    fn panic_bad_token(&self, token: Token, name: &str) {
        eprintln!("Tried to conusme an unexpected token in {name}: {type_token:?}: {content}", type_token=token.token_type, content=token.content);
        exit(1);
    }
    
}
