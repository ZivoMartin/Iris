use super::{
    include::*,
    create_table::CreateReq
};


pub struct Interpreteur {
    request_treaters: Vec<Box<dyn Request>>,
    current_treater: usize,
    database: Database,
    request_in_treatment: bool
}

impl Interpreteur {

    pub fn new() -> Interpreteur{
        Interpreteur {
            request_treaters: Interpreteur::build_treaters(),
            current_treater: 0,
            database: Database::new_empty(),
            request_in_treatment: false
        }
    }

    pub fn new_token(&mut self, token: Token) -> ConsumeResult {
        match token.token_type {
            TokenType::BackLine => Ok(()),
            TokenType::End => Ok(self.end_request()),
            _ => self.consume_token(token)
        }

    }

    fn consume_token(&mut self, token: Token) -> ConsumeResult {
        if self.request_in_treatment {
            self.request_treaters[self.current_treater].consume(&mut self.database, token)?;
        } else {
            self.request_in_treatment = true;
        }
        Ok(())
    }
    
    fn end_request(&mut self) {
        self.request_treaters[self.current_treater] = self.request_treaters[self.current_treater].end(&mut self.database);
        self.request_in_treatment = false;
    }
    
    fn build_treaters() -> Vec<Box<dyn Request>> {
        vec!(CreateReq::new())
    }
    
}
