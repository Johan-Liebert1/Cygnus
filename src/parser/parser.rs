use crate::{
    ast::{abstract_syntax_tree::AST, typedef::Typedef, void::Void},
    helpers::{self, compiler_error, unexpected_token},
    lexer::{
        keywords::{CONST_VAR_DEFINE, CONTINUE, EXTERN, INCLUDE, MEM, STRUCT, TYPE_DEF},
        tokens::{Number, Operations},
        types::VarType,
    },
    trace,
    types::ASTNode,
};

use core::panic;
use std::{cell::RefCell, collections::HashMap, fs, path::Path, process::exit, rc::Rc};

use crate::{
    ast::{
        jump::{Jump, JumpType},
        program::Program,
    },
    interpreter::interpreter::Functions,
    lexer::{
        keywords::{BREAK, ELIF_STATEMENT, ELSE_STATEMENT, FUNCTION_DEFINE, IF_STATEMENT, LOOP, RETURN, VAR_DEFINE},
        lexer::{Lexer, Token},
        tokens::{Bracket, TokenEnum},
    },
};

pub type ParserFunctions = Rc<RefCell<Functions>>;

#[derive(Debug)]
pub struct UserDefinedType {
    pub name: String,
    pub type_: VarType,
}

#[derive(Debug)]
pub struct Parser {
    pub lexer: Rc<RefCell<Box<Lexer>>>,
    pub bracket_stack: Vec<Token>,
    pub functions: ParserFunctions,

    // how deeply nested are we inside loops
    pub inside_loop_depth: usize,
    // how deeply nested are we inside function bodies
    pub inside_function_depth: usize,
    // how deeply nested are we inside conditionals
    pub inside_if_else_depth: usize,

    pub num_loops: usize,
    pub inside_current_loop_number: i32,

    pub num_strings: usize,
    pub num_floats: usize,
    pub times_dereferenced: usize,

    pub current_function_being_parsed: Option<String>,

    pub user_defined_types: Vec<UserDefinedType>,

    pub type_aliases: Vec<Typedef>,
}

impl Parser {
    pub fn new(file: Vec<u8>, file_name: String) -> Self {
        let lexer = Lexer::new(file, file_name);

        Self {
            lexer: Rc::new(RefCell::new(Box::new(lexer))),
            bracket_stack: vec![],
            functions: Rc::new(RefCell::new(HashMap::new())),

            inside_loop_depth: 0,
            inside_function_depth: 0,
            inside_if_else_depth: 0,

            num_loops: 0,
            inside_current_loop_number: -1,

            num_strings: 0,
            num_floats: 0,

            times_dereferenced: 0,

            current_function_being_parsed: None,
            user_defined_types: vec![],
            type_aliases: vec![],
        }
    }

    /// Validates the current token with expected token and consumes the token
    /// panics if current token is not the same as expected token
    pub fn validate_token(&mut self, token_expected: TokenEnum) -> Token {
        let token = self.get_next_token();

        if token.token != token_expected {
            helpers::unexpected_token(&token, Some(&token_expected));
            exit(1);
        }

        return token;
    }

    /// Validates the current token with expected token and consumes the token
    /// panics if current token is not the same as expected token
    pub fn validate_any_token(&mut self, tokens_expected: Vec<TokenEnum>) -> TokenEnum {
        let token = self.get_next_token();

        let mut validated_token = None;

        for token_ in &tokens_expected {
            if *token_ == token.token {
                validated_token = Some(token_);
                break;
            }
        }

        match validated_token {
            Some(token) => token.clone(),
            None => {
                panic!("Expected {:?}, got {:?}", tokens_expected, token)
            }
        }
    }

    /// STATEMENT -> VARIABLE_DECLARATION | CONDITIONAL_STATEMENT | COMPARISON_EXPRESSION | LPAREN COMPARISON_EXPRESSION RPAREN
    pub fn parse_statements(&mut self) -> ASTNode {
        let current_token = self.peek_next_token();

        // println!("parse_statements current_token {:#?}", current_token);

        match &current_token.token {
            TokenEnum::Keyword(keyword) => {
                self.get_next_token();

                match keyword as &str {
                    VAR_DEFINE => self.parse_declaration_statement(false),
                    TYPE_DEF => {
                        self.parse_typedef();
                        Rc::new(RefCell::new(Box::new(Void)))
                    }

                    CONST_VAR_DEFINE => self.parse_declaration_statement(true),

                    IF_STATEMENT => self.parse_conditional_statement(),

                    LOOP => self.parse_loop(),

                    FUNCTION_DEFINE => {
                        if self.inside_function_depth != 0 {
                            // don't allow function in function definitions
                            compiler_error("Defining function inside functions is not allowed", &current_token);
                        }

                        self.parse_function_definition(Rc::clone(&self.functions), false)
                    }

                    EXTERN => {
                        // as parse_function_definition expectes 'fun' to be already consumed
                        self.validate_token(TokenEnum::Keyword(FUNCTION_DEFINE.into()));

                        self.parse_function_definition(Rc::clone(&self.functions), true)
                    }

                    BREAK => {
                        if self.inside_loop_depth == 0 || self.inside_current_loop_number == -1 {
                            compiler_error("Found `break` outside of a loop", &current_token);
                        }

                        Rc::new(RefCell::new(Box::new(Jump::new(
                            JumpType::Break,
                            self.inside_current_loop_number as usize,
                            None,
                            None,
                            current_token.clone(),
                        ))))
                    }

                    CONTINUE => {
                        if self.inside_loop_depth == 0 || self.inside_current_loop_number == -1 {
                            compiler_error("Found `continue` outside of a loop", &current_token);
                        }

                        Rc::new(RefCell::new(Box::new(Jump::new(
                            JumpType::Continue,
                            self.inside_current_loop_number as usize,
                            None,
                            None,
                            current_token.clone(),
                        ))))
                    }

                    RETURN => self.parse_return_statement(&current_token),

                    MEM => self.parse_memory_alloc(),

                    STRUCT => {
                        self.parse_struct_definition();

                        Rc::new(RefCell::new(Box::new(Void)))
                    }

                    INCLUDE => {
                        if self.inside_loop_depth != 0 || self.inside_function_depth != 0 {
                            compiler_error("`include` can only be used at the beginning of a file", &current_token)
                        }

                        let included_file_tok = self.peek_next_token();

                        let mut file_path = String::new();

                        if let TokenEnum::StringLiteral(fp) = included_file_tok.token {
                            self.get_next_token();
                            file_path = fp;
                        } else {
                            unexpected_token(&included_file_tok, Some(&TokenEnum::StringLiteral("".into())));
                        }

                        let current_lexer = self.lexer.clone();

                        let borrow = self.lexer.borrow();

                        // Dereferencing the Rc, which gives us the String, and then borrowing the String
                        let path = Path::new(&*borrow.file_name);

                        let file_path = path
                            .parent()
                            .unwrap_or_else(|| Path::new(""))
                            .join(Path::new(&file_path.strip_prefix("./").unwrap_or_else(|| &file_path)));

                        // Dropping the borrow here so that we can have a new reference to the
                        // Lexer down below
                        // We cannot have another mutable or immutable borrow while this exists
                        drop(borrow);

                        // Create a new lexer for the new file
                        let file_contents = fs::read(file_path.clone()).unwrap();
                        let new_file_lexer = Lexer::new(file_contents, file_path.to_str().unwrap().into());
                        let new_lexer = Rc::new(RefCell::new(Box::new(new_file_lexer)));

                        // New reference to the lexer
                        self.lexer = new_lexer.clone();

                        let ast = self.parse_program();

                        self.lexer = current_lexer;

                        ast
                    }

                    ELSE_STATEMENT => {
                        compiler_error("Found 'else' without an 'if' {:?}", &current_token);
                        exit(1);
                    }

                    ELIF_STATEMENT => {
                        compiler_error("Found 'elif' without an 'if' {:?}", &current_token);
                        exit(1);
                    }

                    _ => {
                        compiler_error(format!("Keyword '{}' not recognised", keyword), &current_token);
                        exit(1);
                    }
                }
            }

            // FIXME: This cannot be any bracket, example { is not correct
            TokenEnum::Number(..) | TokenEnum::Bracket(..) => self.parse_logical_expression(),

            TokenEnum::Variable(var) => {
                // 2nd token here as we haven't consumed the `var` token
                let nth_token = self.peek_nth_token(2);

                // println!("parse_statements variable nth_token {:#?}", current_token);

                match nth_token.token {
                    TokenEnum::Bracket(b) => {
                        match b {
                            Bracket::LParen => {
                                // function invocation
                                self.get_next_token();
                                self.parse_function_call(var.to_string(), false)
                            }

                            Bracket::LSquare => {
                                // array index assignment
                                // array[7] = 43
                                let var_token = self.get_next_token();

                                let next_token = self.get_next_token();

                                let array_access_index = self.parse_logical_expression();

                                self.validate_token(TokenEnum::Bracket(Bracket::RSquare));

                                self.parse_assignment_statement(var_token, var.to_string(), 0, Some(array_access_index))
                            }

                            _ => unexpected_token(&current_token, None),
                        }
                    }

                    TokenEnum::Equals | TokenEnum::MinusEquals | TokenEnum::PlusEquals | TokenEnum::Dot => {
                        // variable assignment
                        let var_token = self.get_next_token();
                        self.parse_assignment_statement(var_token, var.to_string(), 0, None)
                    }

                    e => {
                        panic!("Expected `)` or `=` after {}, got {:?}", var, e)
                    }
                }
            }

            // could be something like *a = 23 or *(a + 1) = 34
            TokenEnum::Op(op) => match op {
                Operations::Multiply => {
                    let mut times_dereferenced = 0;

                    while let TokenEnum::Op(Operations::Multiply) = self.peek_next_token().token {
                        self.get_next_token();
                        times_dereferenced += 1;
                    }

                    let token = self.get_next_token();

                    if let TokenEnum::Variable(ref var_name) = &token.token {
                        self.parse_assignment_statement(token.clone(), var_name.into(), times_dereferenced, None)
                    } else {
                        unexpected_token(&token, Some(&TokenEnum::Variable("".into())));
                        exit(1);
                    }
                }

                Operations::Plus => todo!(),
                Operations::Minus => todo!(),
                Operations::Divide => todo!(),
                Operations::ShiftLeft => todo!(),
                Operations::ShiftRight => todo!(),
                Operations::Modulo => todo!(),
            },

            TokenEnum::LogicalOp(op) => {
                helpers::unexpected_token(&current_token, None);
                exit(1);
            }

            TokenEnum::StringLiteral(_) => todo!(),

            TokenEnum::Equals => todo!(),
            TokenEnum::PlusEquals => todo!(),
            TokenEnum::MinusEquals => todo!(),
            TokenEnum::Ampersand => todo!(),
            TokenEnum::Comparator(_) => todo!(),
            TokenEnum::Bool(_) => todo!(),
            TokenEnum::Type(_) => todo!(),
            TokenEnum::Colon => todo!(),
            TokenEnum::Comma => todo!(),
            TokenEnum::SemiColon => todo!(),
            TokenEnum::FunctionReturnIndicator => todo!(),
            TokenEnum::Comment => todo!(),
            TokenEnum::Dot => todo!(),

            TokenEnum::Unknown(..) => {
                panic!("Unknown token: {:?}", &current_token);
            }

            TokenEnum::EOF => {
                unreachable!("Reached EOF");
            }
        }
    }

    pub fn parse_program(&mut self) -> ASTNode {
        let mut statements: Vec<ASTNode> = vec![];

        loop {
            let current_token = self.peek_next_token();

            match &current_token.token {
                TokenEnum::EOF => {
                    break;
                }

                TokenEnum::SemiColon => {
                    self.get_next_token();
                    continue;
                }

                TokenEnum::Bracket(b) => match b {
                    Bracket::RCurly => {
                        if self.inside_function_depth > 0 || self.inside_loop_depth > 0 || self.inside_if_else_depth > 0
                        {
                            return Rc::new(RefCell::new(Box::new(Program::new(statements))));
                        } else {
                            statements.push(self.parse_statements())
                        }
                    }

                    _ => statements.push(self.parse_statements()),
                },

                TokenEnum::Comment => continue,

                _ => {
                    statements.push(self.parse_statements());
                }
            }
        }

        return Rc::new(RefCell::new(Box::new(Program::new(statements))));
    }
}
