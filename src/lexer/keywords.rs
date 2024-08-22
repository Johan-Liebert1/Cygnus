pub const FUNCTION_DEFINE: &str = "fun";
pub const VAR_DEFINE: &str = "def";
pub const CONST_VAR_DEFINE: &str = "const";

pub const IF_STATEMENT: &str = "if";
pub const ELSE_STATEMENT: &str = "else";
pub const ELIF_STATEMENT: &str = "elif";

pub const LOGICAL_OR: &str = "or";
pub const LOGICAL_AND: &str = "and";
pub const LOGICAL_NOT: &str = "not";

pub const LOOP: &str = "loop";
pub const USING: &str = "using";
pub const FROM: &str = "from";
pub const TO: &str = "to";
pub const STEP: &str = "step";
pub const RETURN: &str = "return";
pub const BREAK: &str = "break";
pub const WITH: &str = "with";
pub const AS: &str = "as";

pub const MEM: &str = "mem";
pub const STRUCT: &str = "struct";

pub const INCLUDE: &str = "include";
pub const TYPE_DEF: &str = "type";

pub const KEYWORDS: [&str; 22] = [
    VAR_DEFINE,
    CONST_VAR_DEFINE,
    IF_STATEMENT,
    ELSE_STATEMENT,
    ELIF_STATEMENT,
    LOGICAL_OR,
    LOGICAL_AND,
    LOGICAL_NOT,
    LOOP,
    USING,
    FROM,
    TO,
    STEP,
    FUNCTION_DEFINE,
    RETURN,
    BREAK,
    WITH,
    MEM,
    AS,
    STRUCT,
    INCLUDE,
    TYPE_DEF,
];

// Predefined functions

/// prints anything in args to stdout
pub const FUNC_WRITE: &str = "write";
pub const FUNC_EXIT: &str = "exit";
pub const FUNC_STRLEN: &str = "strlen";
pub const FUNC_SYSCALL: &str = "syscall";
