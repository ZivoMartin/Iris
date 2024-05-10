use crate::interpreteur::include::*;
use super::from_where_req::FromWhereReq;

pub struct SelectReq {
    table_name: String,
    asked_cols: Vec<String>,
    nb_kw: u8,
    from_where: Box<dyn Request>
}

impl Request for SelectReq {

    fn new() -> BoxedReq {
        Box::from(SelectReq {
            table_name: String::new(),
            asked_cols: Vec::new(),
            nb_kw: 0,
            from_where: FromWhereReq::new()
        })
    }

    fn end(&mut self, database: &mut Database) -> ConsumeResult {
        self.from_where.end(database)?;
        database.test_column_existance(&self.table_name, &self.asked_cols)?;
        self.table_name.clear();
        self.asked_cols.clear();
        self.nb_kw = 0;
        Ok(())
    }
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        match token.token_type {
            TokenType::Ident => self.new_ident(database, token),
            TokenType::Keyword => self.new_keyword(database, token),
            _ => self.from_where.consume(database, token)
        }
    }
    
}


impl SelectReq {

    fn new_ident(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        if self.nb_kw == 0 {
            self.asked_cols.push(token.content);
        } else if self.nb_kw == 1 {
            self.table_name = token.content.clone();
            self.from_where.consume(database, token)?;
        } else {
            self.from_where.consume(database, token)?;
        }
        Ok(())
    }

    fn new_keyword(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        if self.nb_kw != 0 {
            self.from_where.consume(database, token)?;
        }
        self.nb_kw += 1;
        Ok(())
    }
     
}
