use crate::interpreteur::include::*;
use super::from_where_req::FromWhereReq;

pub struct DeleteReq {
    from_where: FromWhereReq
}

impl BrowserReq for DeleteReq {

    fn browse_action(&mut self) {
        println!("Delete")
    }

    
    fn get_expr(&mut self) -> &mut ExpressionEvaluator {
        self.from_where.get_where_expr()
    }
}

impl Request for DeleteReq {

    fn new() -> BoxedReq {
        Box::from(DeleteReq {
            from_where: FromWhereReq::pure_new()
        })
    }

    fn end(&mut self, database: &mut Database) -> ConsumeResult {
        self.from_where.push_last_string();
        database.get_table_mut(self.from_where.table_name()).browse(self);
        self.from_where.end(database)
    }
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        self.from_where.consume(database, token)
    }
    
}

