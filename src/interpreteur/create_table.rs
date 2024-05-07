use super::include::*;

/// Handle the CREATE TABLE request
pub struct CreateReq {
    table: Table,
    current_col: Option<Column>,
    pkey_exists: bool,
    expr: ExpressionEvaluator
}


impl Request for CreateReq {

    fn new() -> BoxedReq {
        Box::from(CreateReq {
            table: Table::new(),
            current_col: Some(Column::new_empty()),
            pkey_exists: false,
            expr: ExpressionEvaluator::new()
        })
    }

    fn end(mut self, database: &mut Database) -> BoxedReq where Self: Sized {
        self.push_col();
        database.add_table(self.table);
        CreateReq::new()
    }
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        match token.token_type {
            TokenType::Ident => self.new_ident(database, token.content)?,
            TokenType::Keyword => self.new_keyword(token.content)?,
            TokenType::Type => self.new_type(token.content),
            TokenType::Operator => self.expr.new_operator(token.content),
            TokenType::Number => self.expr.new_number(token.content),
            _ => self.panic_bad_token(token, "create")
        }
        Ok(())
    }
    
}

impl CreateReq {

    fn table(&self) -> &Table {
        &self.table
    }
    
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }

    
    fn col(&self) -> &Column {
        self.current_col.as_ref().unwrap()
    }
    
    fn col_mut(&mut self) -> &mut Column {
        self.current_col.as_mut().unwrap()
    }

    fn extract_col(&mut self) -> Column {
        let mut result = self.current_col.take().unwrap();
        if !self.expr.is_empty() {
            result.set_default_value(self.expr.compute());
        }
        self.current_col = Some(Column::new_empty());
        result
    }
    
    pub fn new_ident(&mut self, database: &Database, name: String) -> ConsumeResult {
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

    pub fn push_col(&mut self) {
         if !self.col().is_empty() {
             let col = self.extract_col();
             self.table_mut().add_column(col);
         }
    }
    
    pub fn pkey_exists(&self) -> bool {
        self.pkey_exists
    }

    fn def_pkey(&mut self) -> ConsumeResult {
        if self.pkey_exists() {
            Err(format!("You defined a primary key twice for the table {}", self.table().name()))
        } else {
            self.col_mut().set_as_p_key();
            self.pkey_exists = true;
            Ok(())
        }
    }
    
    pub fn new_keyword(&mut self, kw: String) -> ConsumeResult {
        match &kw as &str {
            "PRIMARY" => self.def_pkey()?,
            _ => panic!("Unknow keyword: {kw}")
        }
        Ok(())
    }

    pub fn new_type(&mut self, type_string: String) {
        self.col_mut().set_type(from_string_to_type(type_string))
    }
    
}

