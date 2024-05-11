use super::stack::Stack;
use std::collections::HashMap;
use super::include::{JsonValue, Value, Map};

type Number = i64;
type Operation = fn(Number, Number) -> Number;

pub struct ExpressionEvaluator {
    op_stack: Stack<String>,
    pf_exp: Vec<ExpTokenType>,
    operator_priority: HashMap<String, u8>,
    op_map: HashMap<String, Operation>,
}

#[derive(Debug)]
enum ExpTokenType {
    Operator(Operation),
    Number(Number),
    Field(String)
}

impl ExpressionEvaluator {

    pub fn new() -> ExpressionEvaluator {
        ExpressionEvaluator {
            op_stack: Stack::new(),
            pf_exp: Vec::new(),
            operator_priority: ExpressionEvaluator::build_prio_map(),
            op_map: ExpressionEvaluator::build_op_map(),
        }
    }

    pub fn clear(&mut self) {
        self.pf_exp.clear();
        self.op_stack.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.pf_exp.is_empty()
    }

    pub fn new_operator(&mut self, content: String) {
        if "()".contains(&content) {
            self.new_parenthesis(content)
        } else {
            while !self.op_stack.is_empty() && 
                self.op_stack.val().unwrap() != "(" && 
                self.get_priority(self.top_op()) >= self.get_priority(&content) {
                    self.push_op_val();
                }
            self.op_stack.push(content);
        }
    }

    fn top_op(&self) -> &String {
        self.op_stack.val().as_ref().unwrap()
    }
    
    pub fn new_number(&mut self, number: String) {
        self.pf_exp.push(ExpTokenType::Number(str::parse::<Number>(&number).unwrap()));
    }

    pub fn new_direct_number(&mut self, number: Number) {
        self.pf_exp.push(ExpTokenType::Number(number));
    }


    pub fn new_field(&mut self, field_name: String) {
        self.pf_exp.push(ExpTokenType::Field(field_name));
    }

    /// ( -> Push it on the op stack
    /// ) -> pop the operators til we pop an opening bracket
    pub fn new_parenthesis(&mut self, par: String) {
        match &par as &str {
            "(" => self.op_stack.push(par),
            ")" => {
                while self.top_op() != "(" {
                    self.push_op_val();
                }
                self.op_stack.pop();
            }
            _ => panic!("Unknow parenthesis: {par}")
        } 
    }

    fn get_priority(&self, op: &String) -> u8 {
        *self.operator_priority.get(op).unwrap_or_else(
            || panic!("This operator doesn't have priority yet: {op}")
        )
    }

    fn push_op_val(&mut self) {
        let op_string = &self.op_stack.pop().expect("Operator stack empty");
        let operation: Operation = *self.op_map.get(op_string).expect("Operator doesn't exists");
        self.pf_exp.push(ExpTokenType::Operator(operation));
    }


    pub fn compute(&mut self, fields: &Map<String, JsonValue>, clear: bool) -> Number {
        while !self.op_stack.is_empty() {
            self.push_op_val();
        }
        let mut number_stack = Stack::<Number>::new();
        for t in self.pf_exp.iter() {
            match t {
                ExpTokenType::Operator(operation) => self.op_found(&mut number_stack, *operation),
                ExpTokenType::Number(number) => self.number_found(&mut number_stack, *number),
                ExpTokenType::Field(field) => self.number_found(&mut number_stack, Value::from_json(&fields[field]).val())
            }
        }
        if clear {
            self.pf_exp.clear();
        }
        number_stack.pop().unwrap()
    }

    fn op_found(&self, number_stack: &mut Stack<Number>, operation: Operation) {
        let n2 = number_stack.pop().unwrap();
        let n1 = number_stack.pop().unwrap();
        self.number_found(number_stack, operation(n1, n2));
    }

    fn number_found(&self, number_stack: &mut Stack<Number>, number: Number) {
        number_stack.push(number)
    }

    
    fn build_prio_map() -> HashMap<String, u8>{
        let mut res = HashMap::<String, u8>::new();
        for op in vec!["%", "*", "/"].iter() {
            res.insert(String::from(*op), 4);
        }
        for op in vec!("<", "<=", ">", ">=", "==", "!=", "||", "&&").iter() {
            res.insert(String::from(*op), 2);
        }
        res.insert(String::from("+"), 3);
        res.insert(String::from("-"), 3);
        res.insert(String::from(")"), 4);
        res.insert(String::from("("), 5);
        res
    }

    fn build_op_map() -> HashMap<String, Operation> {
        let mut res = HashMap::<String, Operation>::new();
        res.insert(String::from("%"), |n1, n2| n1 % n2);
        res.insert(String::from("*"), |n1, n2| n1 * n2);
        res.insert(String::from("+"), |n1, n2| n1 + n2);
        res.insert(String::from("-"), |n1, n2| n1 - n2);
        res.insert(String::from("<"), |n1, n2| (n1 < n2) as Number);
        res.insert(String::from("<="), |n1, n2| (n1 <= n2) as Number);
        res.insert(String::from(">"), |n1, n2| (n1 > n2) as Number);
        res.insert(String::from(">="), |n1, n2| (n1 >= n2) as Number);
        res.insert(String::from("=="), |n1, n2| (n1 == n2) as Number);
        res.insert(String::from("!="), |n1, n2| (n1 != n2) as Number);
        res.insert(String::from("||"), |n1, n2| ((n1 != 0) || (n2 != 0)) as Number);
        res.insert(String::from("&&"), |n1, n2| ((n1 != 0) && (n2 != 0)) as Number);
        res
    }
    


    
}
