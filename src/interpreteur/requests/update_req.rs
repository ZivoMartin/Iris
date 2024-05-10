use crate::interpreteur::include::*;
use super::from_where_req::FromWhereReq;

struct Update {
    column: String,
    operator: String,
    expr: ExpressionEvaluator
}

pub struct UpdateReq {
    aff_vec: Vec<Update>,
    from_where: Box<dyn Request>
}

impl Request for UpdateReq {

    fn new() -> BoxedReq {
        Box::from(UpdateReq {
            aff_vec: Vec::new(),
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
