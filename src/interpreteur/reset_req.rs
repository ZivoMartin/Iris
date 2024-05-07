use super::include::*;

pub struct ResetReq;

impl Request for ResetReq {

    fn new() -> BoxedReq {
        Box::from(ResetReq)
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


