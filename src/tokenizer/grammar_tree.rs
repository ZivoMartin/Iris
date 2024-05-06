use std::collections::HashMap;
use super::include::{Node, TokenType, AFFECT_OPERATOR};
use super::tokenizer::{
    push_token,
    push_group,
    end_group,
    push_once,
    push_ending_group,
    push_ending_once,
    push_ending_token,
    push_token_and_end,
};

pub fn build_grammar_tree() -> HashMap<TokenType, Node> {
    let mut group_map = HashMap::new();
    group_map.insert(
        TokenType::Request,
        Node::leaf(TokenType::Request)
    );
    group_map
}


