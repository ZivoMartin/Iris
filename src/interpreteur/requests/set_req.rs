use crate::interpreteur::include::*;
use super::from_where_req::FromWhereReq;

struct Update {
    column: String,
    operator: String,
    expr: ExpressionEvaluator,
    string_builder: StringBuilder
}

impl Update {

    fn new() -> Update {
        Update {
            column: String::new(),
            operator: String::new(),
            expr: ExpressionEvaluator::new(),
            string_builder: StringBuilder::new()
        }
    }

    fn has_operator(&self) -> bool {
        !self.operator.is_empty()
    }

    fn test_string_exp_compability(&self) -> ConsumeResult {
        if !self.string_builder.is_empty() && !self.expr.is_empty() {
            return Err("Error during a set request: Iris doesn't support yet the string operations".to_string())
        }
        Ok(())
    }
    
    fn new_operator(&mut self, op: String)  {
        if self.has_operator() {
            self.expr.new_operator(op);
        } else {
            self.operator = op;
        }
        
    }

    fn new_name(&mut self, name: String) -> ConsumeResult {
        if self.has_operator() {
            self.expr.new_field(name);
            self.test_string_exp_compability()?;
        } else {
            self.column = name;
        }
        Ok(())
    }

    fn new_number(&mut self, n: String) -> ConsumeResult {
        self.test_string_exp_compability()?;        
        self.expr.new_number(n);
        Ok(())
    }

    fn new_char(&mut self, c: String) -> ConsumeResult {
        self.string_builder.new_char(c);
        self.test_string_exp_compability()
    }
}

pub struct SetReq {
    redirect: bool,
    aff_vec: Vec<Update>,
    from_where: FromWhereReq
}

impl Request for SetReq {

    fn new() -> BoxedReq {
        Box::from(SetReq {
            redirect: false,
            aff_vec: vec!(Update::new()),
            from_where: FromWhereReq::pure_new()
        })
    }

    fn end(&mut self, database: &mut Database) -> ConsumeResult {
        self.from_where.push_last_string();
        let table = database.get_table_mut(self.from_where.table_name());
        table.browse(self);
        table.actualise_table_file();
        self.from_where.end(database)?;
        self.redirect = false;
        self.aff_vec.clear();
        Ok(())
    }
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        if self.redirect {
            self.from_where.consume(database, token)?;
        } else {
            let last = self.get_last_update();
            match token.token_type {
                TokenType::Keyword => self.new_keyword(),
                TokenType::Symbol => self.new_char(token.content, token.flag)?,
                TokenType::Operator => last.new_operator(token.content),
                TokenType::Number => last.new_number(token.content)?,
                TokenType::Ident => last.new_name(token.content)?,
                _ => self.panic_bad_token(token, "set")
            }
        }
        Ok(())
    }
    
}

impl SetReq {

    fn new_keyword(&mut self) {
        self.redirect = true;
    }

    fn new_char(&mut self, c: String, flag: Flag) -> ConsumeResult {
        if flag == Flag::Comma {
            self.aff_vec.push(Update::new())
        } else {
            self.get_last_update().new_char(c)?;
        }
        Ok(())
    }

    fn get_last_update(&mut self) -> &mut Update {
        self.aff_vec.last_mut().expect("SetReq: aff_vec empty..")
    }
}

impl BrowserReq for SetReq {

    fn browse_action(&mut self, line: &mut Map::<String, JsonValue>, _line_number: usize) {
        for aff in self.aff_vec.iter_mut() {
            line[&aff.column] = aff.expr.compute(line, false).into();
        }
    }

    fn get_expr(&mut self) -> &mut ExpressionEvaluator {
        self.from_where.get_where_expr()
    }
    
}
