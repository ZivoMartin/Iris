

pub trait Tool {

    fn new() -> Box<dyn Tool> where Self: Sized;

    fn end(&mut self) -> Result<(TokenType, String), String>;
    
    fn new_token(&mut self, token: Token) -> Result<String, String>;

}



fn build_constructor_map() -> HashMap<TokenType, fn(&mut ProgManager) -> Box<dyn Tool>> {
    let mut res = HashMap::<TokenType, fn() -> Box<dyn Tool>>::new();
    res
}

pub struct Program {
    line_number: usize,
    tools_stack: Stack<Box<dyn Tool>>,
    constructor_map: HashMap<TokenType, fn(pm: &mut ProgManager) -> Box<dyn Tool>>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            line_number: 1,
            tools_stack: Stack::new(),
            constructor_map: build_constructor_map(),
        }
    }

    pub fn tokenize(&mut self, token: Token) -> Result<(), String> {
        println!("{token:?}");
        match token.token_type {
            TokenType::BackLine => self.new_line(),
            TokenType::ERROR => return Err(self.error_msg(token.content)),
            TokenType::End => self.end_group()?,
            TokenType::EndProgram => self.push_script(&self.end_prog(), SCRIPTF), 
            TokenType::New => self.new_group(token.flag),
            _ => {
                match self.tools_stack.val_mut().unwrap().new_token(token, &mut self.memory) {
                    Ok(asm) => self.push_script(&asm, SCRIPTF),
                    Err(e) => return Err(self.error_msg(e))
                }
            }
        };
        Ok(())
    }

    #[inline]
    pub fn new_group(&mut self, type_token: TokenType) {
        self.tools_stack.push((self.constructor_map.get(&type_token).unwrap())(&mut self.memory));
    }

    fn new_line(&mut self) {
        self.new_line += 1
    }

    fn line_number(&self) {
        self.line_number
    }
    
    pub fn end_group(&mut self) -> Result<(), String>{
        let (token_to_raise, end_txt) = self.tools_stack.pop().unwrap()
                                            .end().unwrap_or_else(|e| {
                                                println!("{}", self.error_msg(e));
                                                exit(1);
                                            });
        if !self.tools_stack.is_empty() {
            self.tokenize(Token::empty(token_to_raise))?;
        };
        Ok(())
    }


    

    fn error_msg(&self, msg: String) -> String {
        format!("{}: {}", self.line_number(), msg)
    }

}
