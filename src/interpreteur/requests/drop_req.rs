use crate::interpreteur::include::*;

pub struct DropReq;

impl Request for DropReq {

    fn new() -> BoxedReq {
        Box::from(DropReq)
    }

    fn end(&mut self, _database: &mut Database) -> ConsumeResult {
        Ok(())
    }
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        match token.token_type {
            TokenType::Ident => self.new_ident(database, token.content)?,
            _ => self.panic_bad_token(token, "drop")
        }
        Ok(())
    }
    
}


impl DropReq {

    fn new_ident(&self, database: &mut Database, name: String) -> ConsumeResult {
        if !database.table_exists(&name) {
            return Err(format!("Error during drop request: The table {name} doesn't exists"))
        }
        database.delete_table(&name);
        Ok(())
    }
    
}


