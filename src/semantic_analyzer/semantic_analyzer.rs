pub type CallStack = Vec<ActivationRecord>;

pub enum ActivationRecordType {
    Function,
    IfElse,
    Loop,
}

pub struct ActivationRecord {
    pub name: String,
}

pub struct SemanticAnalyzer {
    pub call_stack: CallStack,
}
