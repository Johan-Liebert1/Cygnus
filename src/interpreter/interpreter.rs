use crate::lexer::types::VarType;
use crate::semantic_analyzer::semantic_analyzer::CallStack;
use crate::trace;
use crate::types::ASTNode;

use core::panic;
use std::io::prelude::*;
use std::{cell::RefCell, collections::HashMap, fs::File, rc::Rc};

use crate::lexer::tokens::VariableEnum;
use crate::{asm::asm::ASM, ast::abstract_syntax_tree::VisitResult, lexer::tokens::Number};

// This is a struct because
// This will always panic on semantic_visit of a `return` node as semantic_visit needs a mutable reference
// and this is called by semantic_visit of function_def which holds the mutable ref
#[derive(Debug)]
pub struct FunctionHashMapValue {
    /// The function body AST
    pub func: ASTNode,
    /// We store return type here for recursive function calls 
    /// Since we will get already borrowed error for recursive calls on semantic visit
    /// We need to store the return type separately
    pub return_type: VarType,
    pub is_extern_func: bool,
}

pub type Variables = HashMap<String, VariableEnum>;
pub type Functions = HashMap<String, FunctionHashMapValue>;

pub struct Interpreter {
    ast: ASTNode,
    pub variables: Variables,
    pub functions: Rc<RefCell<Functions>>,
    pub asm: ASM,
}

impl Interpreter {
    pub fn new(ast: ASTNode, functions: Rc<RefCell<Functions>>) -> Self {
        Self {
            ast,
            variables: HashMap::from([
                ("argc".into(), VariableEnum::Number(Number::Integer(0))),
                ("argv".into(), VariableEnum::String("".into())),
            ]),
            functions,
            asm: ASM::default(),
        }
    }

    fn write_nasm(&self) -> Result<(), std::io::Error> {
        let file_name = "generated/output.asm";

        println!("pwd {:?}", std::env::current_dir());

        let mut file = File::create(&file_name);

        match file {
            Ok(ref mut file) => {
                // write includes
                file.write(self.asm.include.join("\n\t").as_bytes())?;
                file.write(b"\n\n")?;

                // write .bss section
                file.write(b"section .bss\n\t")?;
                file.write_all(self.asm.bss.join("\n\t").as_bytes())?;
                file.write(b"\n\n")?;

                if self.asm.data.len() > 0 {
                    // write .data section
                    file.write(b"section .data\n\t")?;
                    file.write_all(self.asm.data.join("\n\t").as_bytes())?;
                    file.write(b"\n\n")?;
                }

                // write .text section
                file.write(b"section .text\n\t")?;
                file.write_all(self.asm.text.join("\n\t").as_bytes())?;
                file.write(b"\n\n")?;

                for label in &self.asm.labels {
                    let mut file_bytes = format!("{}:\n\t", &label.name);
                    file_bytes += &label.code.join("\n\t");

                    file.write_all(file_bytes.as_bytes())?;

                    if label.name == "_start" {
                        file.write(b"\n\texit 0")?;
                    }

                    file.write(b"\n\n")?;
                }
            }

            Err(e) => {
                println!("Failed to open file `{file_name}` in interpreter: {}", e);
            }
        }

        return Ok(());
    }

    pub fn interpret(&mut self, call_stack: &mut CallStack) -> VisitResult {
        return self
            .ast
            .borrow()
            .visit(&mut self.variables, Rc::clone(&self.functions), call_stack);
    }

    pub fn compile(&mut self, call_stack: &mut CallStack) {
        self.ast.borrow().visit_com(
            &mut self.variables,
            Rc::clone(&self.functions),
            &mut self.asm,
            call_stack,
        );

        if self.asm.get_used_registers().len() > 0 {
            trace!("Used Registers: {:#?}", self.asm.get_used_registers());
            // trace!("Labels: {:#?}", self.asm.labels);
            panic!("Found unused registers");
        }

        let _ = self.write_nasm();
    }
}
