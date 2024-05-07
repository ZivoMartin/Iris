use super::include::*;

pub struct InsertReq;

impl Request for InsertReq {

    fn new() -> BoxedReq {
        Box::from(InsertReq)
    }

    fn end(&mut self, database: &mut Database) {
        database.reset_database();
    }
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        match token.token_type {
            _ => self.panic_bad_token(token, "drop")
        }
        Ok(())
    }
    
}


impl InsertReq {

    

    
}
