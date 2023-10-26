use super::{abstract_syntax_tree::AST, comparison_exp::ComparisonExp, expression::Expression};

struct IfStatement {
    condition: ComparisonExp,
    block: Vec<Expression>,
}

struct ConditionalStatement {
    if_statement: Box<IfStatement>,
    elif_ladder: Option<IfStatement>,
    else_statement: Option<Vec<Expression>>,
}

impl AST for ConditionalStatement {
    fn visit(&self) -> super::abstract_syntax_tree::VisitResult {
        todo!()
    }

    fn get_token(&self) -> &crate::lexer::lexer::Token {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}
