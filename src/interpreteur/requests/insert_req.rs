use crate::interpreteur::include::*;

pub struct InsertReq {
    table_name: String,
    asked_cols: Vec<String>,
    values: Vec<Value>,
    expr: ExpressionEvaluator,
    string_builder: StringBuilder
}

impl Request for InsertReq {

    fn new() -> BoxedReq {
        Box::from(InsertReq {
            table_name: String::new(),
            asked_cols: Vec::new(),
            values: Vec::new(),
            expr: ExpressionEvaluator::new(),
            string_builder: StringBuilder::new()
        })
    }

    fn end(&mut self, database: &mut Database) -> ConsumeResult {
        self.save_value(database)?;
        if self.values.len() != self.asked_cols.len() {
            return Err(format!("Error during the insertion of the table {}, the number of value is less than the number of columns.", self.table_name))
        }
        let table = database.get_table_mut(&self.table_name);
        for (_, c) in table.get_cols().iter() {
            if !c.has_default_value() && !c.flag() {
                return Err(format!("Error during the insertion of the table {}, the column {} doesn't have a default value and you didn't indicate his value.", self.table_name, c.name()))
            } else if !c.flag() {
                self.asked_cols.push(c.name().clone());
                self.values.push(c.default_value().clone())
            }
        }
        table.insert(&self.asked_cols, &self.values);
        database.get_table_mut(&self.table_name).reset_all_flags();
        self.table_name.clear();
        self.asked_cols.clear();
        self.values.clear();
        Ok(())
    }
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        match token.token_type {
            TokenType::Ident => self.new_ident(token.content, database)?,
            TokenType::Operator => self.expr.new_operator(token.content),
            TokenType::Number => self.expr.new_number(token.content),
            TokenType::Symbol => self.new_char(token.content, token.flag, database)?,
            _ => self.panic_bad_token(token, "drop")
        }
        Ok(())
    }
    
}


impl InsertReq {

    fn new_ident(&mut self, name: String, database: &mut Database) -> ConsumeResult {
        if self.table_name.is_empty() {
            self.set_table_name(name, database)
        } else {
            self.new_col(name, database)
        }
    }

    fn new_col(&mut self, col_name: String, database: &mut Database) -> ConsumeResult {
        let table = database.get_table_mut(&self.table_name);
        if !table.column_exists_without_flag(&col_name) {
            return if !table.column_exists(&col_name) {
                Err(format!("Error during an insertion, the column {col_name} doesn't exists in the table {}.", self.table_name))            
            } else {
                Err(format!("Error during an insertion, you asked for the column {col_name} twice."))            
            }
        } 
        table.active_column_flag(&col_name);
        self.asked_cols.push(col_name);
        Ok(())
    }

    fn set_table_name(&mut self, table_name: String, database: &Database) -> ConsumeResult {
        if !database.table_exists(&table_name) {
            return Err(format!("Error during an insertion, the table {table_name} doesn't exists"))
        }
        self.table_name = table_name;
        Ok(())
    }

    fn new_char(&mut self, c: String, flag: Flag, database: &Database) -> ConsumeResult {
        if flag == Flag::Comma {
            self.save_value(database)?;
        } else {
            self.string_builder.new_char(c);
        }
        Ok(())
    }

    fn save_value(&mut self, database: &Database) -> ConsumeResult {
        if self.asked_cols.len() < self.values.len() {
            return Err(format!("Error during insert request in the table {}, you put more values than column.", self.table_name))
        }
        let column = database.get_table(&self.table_name).get_column(&self.asked_cols[self.values.len()]);
        if self.string_builder.is_empty() {
            if column.get_type() == Type::String {
                return Err(format!("Error during insert request in the table {}, a string was expected for the column {}.", self.table_name, column.name()))
            }
            self.values.push(Value::new_by_val(self.expr.compute(vec!(), true)));
        } else {
            if column.get_type() != Type::String {
                return Err(format!("Error during insert request in the table {}, the column {} doesn't have the String type.", self.table_name, column.name()))
            }
            self.values.push(Value::new_by_string(&mut self.string_builder, false))
        }
        Ok(())
    }
    
}
