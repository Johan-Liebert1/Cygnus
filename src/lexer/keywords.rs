pub const VAR_DEFINE: &str = "def";

pub const IF_STATEMENT: &str = "if";
pub const ELSE_STATEMENT: &str = "else";
pub const ELIF_STATEMENT: &str = "elif";

pub const LOOP: &str = "loop";
pub const USING: &str = "using";
pub const FROM: &str = "from";
pub const TO: &str = "to";
pub const STEP: &str = "step";

pub const KEYWORDS: [&str; 9] = [
    VAR_DEFINE,
    IF_STATEMENT,
    ELSE_STATEMENT,
    ELIF_STATEMENT,
    LOOP,
    USING,
    FROM,
    TO,
    STEP,
];

// Predefined functions

/// prints anything in args to stdout
pub const FUNC_OUTPUT: &str = "output";

// types

pub const TYPE_INT: &str = "int";
pub const TYPE_FLOAT: &str = "float";

pub const TYPES: [&str; 2] = [TYPE_INT, TYPE_FLOAT];
