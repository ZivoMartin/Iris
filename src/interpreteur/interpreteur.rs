use super::{
    include::*,
    create_req::CreateReq,
    drop_req::DropReq,
    reset_req::ResetReq,
    insert_req::InsertReq
};


pub struct Interpreteur {
    request_treaters: Vec<Box<dyn Request>>,
    keyword_link: HashMap<String, usize>,
    current_treater: usize,
    database: Database,
    request_in_treatment: bool
}

impl Interpreteur {

    pub fn new() -> Interpreteur{
        Interpreteur {
            request_treaters: Interpreteur::build_treaters(),
            keyword_link: Interpreteur::build_keyword_link(),
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
            self.current_treater = *self.keyword_link.get(&token.content).expect(&format!("Interpreteur: Unknow main keyword: {}", token.content));
            self.request_in_treatment = true;
        }
        Ok(())
    }
    
    fn end_request(&mut self) {
        self.request_treaters[self.current_treater].end(&mut self.database);
        self.request_in_treatment = false;
    }
    
    fn build_treaters() -> Vec<Box<dyn Request>> {
        vec!(CreateReq::new(), DropReq::new(), ResetReq::new(), InsertReq::new())
    }

    fn build_keyword_link() -> HashMap::<String, usize> {
        let mut res = HashMap::<String, usize>::new();
        for (i, kw) in Vec::from(["CREATE", "DROP", "RESET", "INSERT"]).iter().enumerate() {
            res.insert(String::from(*kw), i);
        }
        res
    }
    
}

