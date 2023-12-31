#[derive(Debug)]
pub struct CallStackRecord {
    above_node: Box<CallStackRecord>
}

#[derive(Debug)]
pub struct SemanticAnalyzer {
    call_stack: Vec<CallStackRecord>,
}
