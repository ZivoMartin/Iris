use crate::interpreteur::include::*;

/// Handle the CREATE TABLE request
pub struct CreateReq {
    table: Option<Table>,
    current_col: Option<Column>,
    pkey_exists: bool,
    expr: ExpressionEvaluator,
    string_builder: StringBuilder
}


impl Request for CreateReq {

    fn new() -> BoxedReq {
        Box::from(CreateReq {
            table: Some(Table::new()),
            current_col: Some(Column::new_empty()),
            pkey_exists: false,
            expr: ExpressionEvaluator::new(),
            string_builder: StringBuilder::new()
        })
    }

    fn end(&mut self, database: &mut Database) -> ConsumeResult{
        if !self.pkey_exists {
            return Err(format!("Error during the creation of the table {}, you didn' indicate a primary key", self.table().name()))
        }
        self.push_col();
        database.add_table(self.table.take().expect("Create: Failed to unwrap the final table during the end method"));
        self.table = Some(Table::new());
        self.pkey_exists = false;
        Ok(())
    }
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        match token.token_type {
            TokenType::Ident => self.new_ident(database, token.content)?,
            TokenType::Keyword => self.new_keyword(token.content)?,
            TokenType::Type => self.new_type(token.content),
            TokenType::Operator => self.expr.new_operator(token.content),
            TokenType::Number => self.expr.new_number(token.content),
            TokenType::Symbol => self.new_char(token.content),
            _ => self.panic_bad_token(token, "create")
        }
        Ok(())
    }
    
}

impl CreateReq {

    fn table(&self) -> &Table {
        self.table.as_ref().expect("Create: Failed to unwrap the table when calling table method")
    }
    
    fn table_mut(&mut self) -> &mut Table {
        self.table.as_mut().expect("Create: Failed to unwrap the table when calling table_mut method")
    }

    
    fn col(&self) -> &Column {
        self.current_col.as_ref().expect("Create: Failed to unwrap the column when calling col method")
    }
    
    fn col_mut(&mut self) -> &mut Column {
        self.current_col.as_mut().expect("Create: Failed to unwrap the column when calling col_mut method")
    }

    fn extract_col(&mut self) -> Column {
        let mut result = self.current_col.take().unwrap();
        if !self.expr.is_empty() {
            result.set_default_value(self.expr.compute(Map::new(), true)); 
        } else if !self.string_builder.is_empty() {
            result.set_value_by_string(&mut self.string_builder)
        }
        self.current_col = Some(Column::new_empty());
        result
    }
    
    fn new_ident(&mut self, database: &Database, name: String) -> ConsumeResult {
        if !self.table().has_name() {
            if database.table_exists(&name) {
                return Err(format!("The table {name} already exists."))
            }
            self.table_mut().set_name(name);
        } else {
            self.push_col();
            if self.table().column_exists(&name) {
                return Err(format!("You declared the column {name} twice for the table {}", self.table().name()))
            }
            self.col_mut().set_name(name)
        }
        Ok(())
    }

    fn push_col(&mut self) {
         if !self.col().is_empty() {
             let col = self.extract_col();
             self.table_mut().add_column(col);
         }
    }
    
    fn pkey_exists(&self) -> bool {
        self.pkey_exists
    }

    fn def_pkey(&mut self, p_key: String) -> ConsumeResult {
        if self.pkey_exists() {
            Err(format!("You defined a primary key twice for the table {}", self.table().name()))
        } else {
            self.table_mut().set_pkey(p_key);
            self.pkey_exists = true;
            Ok(())
        }
    }
    
    fn new_keyword(&mut self, kw: String) -> ConsumeResult {
        match &kw as &str {
            "PRIMARY" => self.def_pkey(self.col().name().clone())?,
            _ => panic!("Unknow keyword: {kw}")
        }
        Ok(())
    }

    fn new_type(&mut self, type_string: String) {
        self.col_mut().set_type(from_string_to_type(type_string))
    }

    fn new_char(&mut self, c: String) {
        self.string_builder.new_char(c)
    }
    
}

