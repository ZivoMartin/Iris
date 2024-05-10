use crate::interpreteur::include::*;
use super::from_where_req::FromWhereReq;

pub struct DeleteReq {
    from_where: Box<dyn Request>
}

impl Request for DeleteReq {

    fn new() -> BoxedReq {
        Box::from(DeleteReq {
            from_where: FromWhereReq::new()
        })
    }

    fn end(&mut self, database: &mut Database) -> ConsumeResult {
        self.from_where.end(database)
    }
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        self.from_where.consume(database, token)
    }
    
}
