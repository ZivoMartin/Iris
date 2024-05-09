use crate::interpreteur::include::*;


pub struct SelectReq {
    table_name: String,
    asked_cols: Vec<String>,
    expr: ExpressionEvaluator,
    string_builder: StringBuilder,
    kw_state: u8,
}

impl Request for SelectReq {

    fn new() -> BoxedReq {
        Box::from(SelectReq {
            table_name: String::new(),
            asked_cols: Vec::new(),
            expr: ExpressionEvaluator::new(),
            kw_state: 0,
            string_builder: StringBuilder::new()
        })
    }

    fn end(&mut self, database: &mut Database) -> ConsumeResult {
        if !self.string_builder.is_empty() {
            self.push_string()
        }
        if !database.table_exists(&self.table_name) {
            return Err(format!("Error during a selection: table {} don't exists.", self.table_name))
        }
        self.test_column_existance(database, &self.asked_cols)?;
        self.test_column_existance(database, self.expr.fields())?;
        self.table_name.clear();
        self.asked_cols.clear();
        self.kw_state = 0;
        Ok(())
    }
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        match token.token_type {
            TokenType::Ident => self.new_ident(token.content, database),
            TokenType::Operator => self.new_operator(token.content),
            TokenType::Number => self.expr.new_number(token.content),
            TokenType::Symbol => self.new_char(token.content),
            TokenType::Keyword => self.new_keyword(),
            _ => self.panic_bad_token(token, "select")
        }
        Ok(())
    }
    
}


impl SelectReq {

    fn new_ident(&mut self, name: String, database: &mut Database) {
        match self.kw_state {
            0 => self.asked_cols.push(name),
            1 => self.table_name = name,
            2 => self.expr.new_field(name),
            _ => panic!("Unriechable")
        }
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

    fn new_keyword(&mut self) {
        self.kw_state += 1;
    }

    fn push_string(&mut self) {
        self.expr.new_direct_number(self.string_builder.hash());
        self.string_builder.extract();
    }

    fn test_column_existance(&self, database: &Database, cols: &Vec<String>) -> ConsumeResult {
        let table = database.get_table(&self.table_name);
        for c in cols.iter() {
            if !table.column_exists(c) {
                return Err(format!("Error during the selection on {}: the column {} doesn't exists.", self.table_name, c))
            }
        }
        Ok(())
    }
}
