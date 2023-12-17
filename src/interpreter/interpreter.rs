use std::io::prelude::*;
use std::{cell::RefCell, collections::HashMap, fs::File, rc::Rc};

use crate::{
    asm::asm::ASM,
    ast::abstract_syntax_tree::{VisitResult, AST},
    lexer::tokens::Number,
};

pub type Variables = HashMap<String, Number>;
pub type Functions = HashMap<String, Rc<Box<dyn AST>>>;

pub struct Interpreter {
    ast: Rc<Box<dyn AST>>,
    pub variables: Variables,
    pub functions: Rc<RefCell<Functions>>,
    pub asm: ASM,
}

impl Interpreter {
    pub fn new(ast: Rc<Box<dyn AST>>, functions: Rc<RefCell<Functions>>) -> Self {
        Self {
            ast,
            variables: HashMap::new(),
            functions,
            asm: ASM::default(),
        }
    }

    fn write_nasm(&self) -> Result<(), std::io::Error> {
        let mut file = File::create("generated/output.asm");

        match file {
            Ok(ref mut file) => {
                // write includes
                file.write(self.asm.include.join("\n\t").as_bytes())?;
                file.write(b"\n\n")?;

                // write .bss section
                file.write(b"section .bss\n")?;
                file.write_all(self.asm.bss.join("\n\t").as_bytes())?;
                file.write(b"\n\n")?;

                if self.asm.data.len() > 0 {
                    // write .data section
                    file.write(b"section .data\n")?;
                    file.write_all(self.asm.data.join("\n\t").as_bytes())?;
                    file.write(b"\n\n")?;
                }

                // write .text section
                file.write(b"section .text\n")?;
                file.write_all(self.asm.text.join("\n\t").as_bytes())?;
                file.write(b"\n\n")?;

                for label in &self.asm.labels {
                    let mut file_bytes = format!("{}:\n", &label.name);
                    file_bytes += &label.code.join("\n\t");

                    file.write_all(file_bytes.as_bytes())?;

                    if label.name == "_start" {
                        file.write(b"\n\texit 0")?;
                    }

                    file.write(b"\n\n")?;
                }
            }

            Err(e) => {
                println!("Failed to open file: {}", e);
            }
        }

        return Ok(());
    }

    pub fn interpret(&mut self) -> VisitResult {
        return self
            .ast
            .visit(&mut self.variables, Rc::clone(&self.functions));
    }

    pub fn compile(&mut self) {
        self.ast.visit_com(
            &mut self.variables,
            Rc::clone(&self.functions),
            &mut self.asm,
        );

        self.write_nasm();
    }
}
