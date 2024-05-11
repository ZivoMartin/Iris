use crate::interpreteur::include::*;
use super::from_where_req::FromWhereReq;

pub struct SelectReq {
    table_name: String,
    asked_cols: Vec<String>,
    redirect: bool,
    from_where: FromWhereReq
}

impl Request for SelectReq {

    fn new() -> BoxedReq {
        Box::from(SelectReq {
            table_name: String::new(),
            asked_cols: Vec::new(),
            redirect: false,
            from_where: FromWhereReq::pure_new()
        })
    }

    fn end(&mut self, database: &mut Database) -> ConsumeResult {
        self.from_where.push_last_string();
        database.get_table_mut(self.from_where.table_name()).browse(self);
        if self.asked_cols.contains(&ALL_INDICATOR.to_string()) {
            self.fill_asked_cols(database);
        } else {
            database.test_column_existance(self.from_where.table_name(), &self.asked_cols)?;
        }
        self.from_where.end(database)?;
        self.table_name.clear();
        self.asked_cols.clear();
        self.redirect = false;
        Ok(())
    }
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        if self.redirect {
            self.from_where.consume(database, token)?;
        } else {
            match token.token_type {
                TokenType::Ident | TokenType::Symbol => self.asked_cols.push(token.content),
                TokenType::Keyword => self.redirect = true,
                _ => self.panic_bad_token(token, "select")
            }
        }
        Ok(())
    }
    
}

impl SelectReq {

    fn fill_asked_cols(&mut self, database: &Database) {
        self.asked_cols = database.get_table(self.from_where.table_name()).get_cols().keys().map(|c| c.clone()).collect::<_>();
    }
    
}

impl BrowserReq for SelectReq {

    fn browse_action(&mut self) {
        println!("Select");
    }

    fn get_expr(&mut self) -> &mut ExpressionEvaluator {
        self.from_where.get_where_expr()
    }
    
}
