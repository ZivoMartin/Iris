use crate::interpreteur::include::*;


pub struct FromWhereReq {
    table_name: String,
    expr: ExpressionEvaluator,
    string_builder: StringBuilder,
    where_passed: bool,
}

impl Request for FromWhereReq {

    fn new() -> BoxedReq {
        Box::from(FromWhereReq::pure_new())
    }

    fn end(&mut self, database: &mut Database) -> ConsumeResult {
        self.table_name.clear();
        self.where_passed = false;
        Ok(())
    }
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        match token.token_type {
            TokenType::Ident => self.new_ident(token.content, database)?,
            TokenType::Operator => self.new_operator(token.content),
            TokenType::Number => self.expr.new_number(token.content),
            TokenType::Symbol => self.new_char(token.content),
            TokenType::Keyword => self.new_keyword(token.content),
            _ => self.panic_bad_token(token, "from where")
        }
        Ok(())
    }
    
}


impl FromWhereReq {

    pub fn pure_new() -> FromWhereReq {
        FromWhereReq {
            table_name: String::new(),
            expr: ExpressionEvaluator::new(),
            where_passed: false,
            string_builder: StringBuilder::new()
        }
    }

    fn new_ident(&mut self, name: String, database: &Database) -> ConsumeResult {
       if !self.where_passed {
           self.table_name = name;
           self.where_passed = true;
           if !database.table_exists(&self.table_name) {
               return Err(format!("Error: table {} don't exists.", self.table_name))
           }
       } else {
           if !database.get_table(self.table_name()).column_exists(&name) {
               return Err(format!("The column {} doesn't exists for the table {}", name, self.table_name()))
           }
           self.expr.new_field(name)
       }
        Ok(())
    }

    fn new_operator(&mut self, op: String) {
        if !self.string_builder.is_empty() {
            self.push_string();
        }
        self.expr.new_operator(op);
    }
    
    fn new_char(&mut self, c: String) {
        self.string_builder.new_char(c);
    }

    fn new_keyword(&mut self, keyword: String) {
        if keyword == "WHERE" {
             self.where_passed = true
        }
    }

    fn push_string(&mut self) {
        self.expr.new_direct_number(self.string_builder.hash());
        self.string_builder.extract();
    }

    pub fn table_name(&self) -> &String {
        &self.table_name
    }

    pub fn get_where_expr(&mut self) -> &mut ExpressionEvaluator {
        &mut self.expr
    }

    pub fn push_last_string(&mut self) {
        if !self.string_builder.is_empty() {
            self.push_string()
        }
    }
    
}
