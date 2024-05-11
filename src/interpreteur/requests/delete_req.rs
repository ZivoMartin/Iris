use crate::interpreteur::include::*;
use super::from_where_req::FromWhereReq;

pub struct DeleteReq {
    from_where: FromWhereReq,
    delete_stack: Stack<usize>
}

impl BrowserReq for DeleteReq {

    fn browse_action(&mut self, _line: &mut Map::<String, JsonValue>, line_number: usize) {
        self.delete_stack.push(line_number)
    }
    
    fn get_expr(&mut self) -> &mut ExpressionEvaluator {
        self.from_where.get_where_expr()
    }
}

impl Request for DeleteReq {

    fn new() -> BoxedReq {
        Box::from(DeleteReq {
            from_where: FromWhereReq::pure_new(),
            delete_stack: Stack::new()
        })
    }

    fn end(&mut self, database: &mut Database) -> ConsumeResult {
        self.from_where.push_last_string();
        let table = database.get_table_mut(self.from_where.table_name());
        table.browse(self);
        table.drop_lines(&mut self.delete_stack);
        self.from_where.end(database)
    }
    
    fn consume(&mut self, database: &mut Database, token: Token) -> ConsumeResult {
        self.from_where.consume(database, token)
    }
    
}

