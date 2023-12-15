use super::asm::ASM;

pub enum ConditionalJumpTo {
    Else,
    Elif,
}

impl ASM {
    pub fn gen_if(jump_to: ConditionalJumpTo) {}

    /// The label names for all elifs will be of the same format, i.e. <elif label name>_<elif_number>
    /// so that we can easily jump around
    pub fn gen_elif(label_name: String, elif_number: i16, jump_to: ConditionalJumpTo) {}

    /// The label name for else will be unique
    pub fn gen_else(label_name: String) {}
}
