pub const FUNCTION_DEFINE: &str = "fun";
pub const VAR_DEFINE: &str = "def";

pub const IF_STATEMENT: &str = "if";
pub const ELSE_STATEMENT: &str = "else";
pub const ELIF_STATEMENT: &str = "elif";

pub const LOOP: &str = "loop";
pub const USING: &str = "using";
pub const FROM: &str = "from";
pub const TO: &str = "to";
pub const STEP: &str = "step";

pub const KEYWORDS: [&str; 10] = [
    VAR_DEFINE,
    IF_STATEMENT,
    ELSE_STATEMENT,
    ELIF_STATEMENT,
    LOOP,
    USING,
    FROM,
    TO,
    STEP,
    FUNCTION_DEFINE,
];

// Predefined functions

/// prints anything in args to stdout
pub const FUNC_WRITE: &str = "write";
pub const FUNC_EXIT: &str = "exit";
pub const FUNC_STRLEN: &str = "strlen";

// types
pub const TYPE_INT: &str = "int";
pub const TYPE_FLOAT: &str = "float";

pub const TYPES: [&str; 2] = [TYPE_INT, TYPE_FLOAT];
