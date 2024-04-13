## Programming Language called CBerk

### Todo, to do, tu du, tu du, tu du, tu du, tu duuuuuu, tu du du

- [x] Write tests

- [x] Command line args for the compiler, to turn on/off various features
- [x] Implement all binary ops
- [x] Add ==, != and other comparison ops
- [x] Add bitwise operations
    - [x] shl, shr
    - [x] or, and, xor
- [x] `return` and `break` statements
- [x] Functions in Compilation mode
- [x] Variables in Compilation mode
- [x] Local Variables
    - [x] Stack variables for functions
    - [x] Variable Shadowing
    - [x] Upper stack variable access
- [x] Variable scoping
- [x] Implement `+=` and `-=`
- [x] Pointer deref
- [x] Pointer to pointer to pointer ...
- [x] Random access to memory
- [x] Function returns
- [x] Function arguments and their type checking
- [x] Proper type checking
- [x] Type Casting
- [x] Array
- [x] Structures
- [x] File Includes

- [ ] Have a single function for both variable and binary op asm generation

- [ ] Floating point numbers
- [ ] Enums - Use `offset` and `reset` kinda like Go's `iota`
- [ ] Make sure nothing is left on the stack when we break out of a loop
- [ ] Command line arguments
- [ ] Ability to allocate and deallocate memory
- [ ] Macros

# Grammar

    PROGRAM                  -> STATEMENT[]
    STATEMENT                -> VARIABLE_DECLARATION | CONDITIONAL_STATEMENT | COMPARISON_EXPRESSION | LPAREN COMPARISON_EXPRESSION RPAREN | LOOP | FUNCTION_CALL | FUNCTION_DEF
    MEMORY_BLOCK             -> mem VAR_NAME (size in bytes)
    FUNCTION_DEF             -> fun VAR_NAME LPAREN (VAR_NAME : VAR_TYPE)* RPAREN (-> VarType)* LCURLY (STATEMENT[] - FUNCTION_DEF) RCURLY
    FUNCTION_CALL            -> VAR_NAME LPAREN (COMPARISON_EXPRESSION)* RPAREN
    LOOP                     -> loop from LPAREN* EXPRESSION to EXPRESSION (step EXPRESSION)* RPAREN* (with VAR_NAME)* LCURLY STATEMENT[] RCURLY
    CONDITIONAL_STATEMENT    -> if LPAREN* LOGICAL_EXPRESSION RPAREN* LCURLY STATEMENT[]* RCURLY ELSE_STATEMENT*
    ASSIGNMENT_STATEMENT     -> VAR_NAME (= | += | -=) (COMPARISON_EXPRESSION)*
    ELSE_STATEMENT           -> else LCURLY STATEMENT[]* RCURLY
    VARIABLE_DECLARATION     -> def VAR_NAME: (*)* VAR_TYPE (= LOGICAL_EXPRESSION)*
    VAR_TYPE                 -> int | float
    LOGICAL_EXPRESSION       -> (not)* COMPARISON_EXPRESSION ((and | or) COMPARISON_EXPRESSION)*
    COMPARISON_EXPRESSION    -> EXPRESSION ((> | < | >= | <= | == | !=) EXPRESSION)*
    EXPRESSION               -> TERM (( + | - ) TERM)*                      # for precedence as term will be calculated first
    TERM                     -> FACTOR (( * | /  | << | >> | % ) FACTOR)*
    COMMENT                  -> -- (ANY)*
    FACTOR                   -> (*|&)* INTEGER | FLOAT | VARIABLE (as type)* | STRING_LITERAL | LPAREN EXPRESSION RPAREN | FUNCTION_CALL
    VAR_NAME                 -> any valid identifier
    LPAREN                   -> (
    RPAREN                   -> )
    LCURLY                   -> {
    RCURLY                   -> }

# Handling strings

def hi: str = "hello"

stack top
1008 5
1016 ptr to "hello"

---

def hi: *char = C"hello"

stack top
1008 ptr to "hello"

---

def hi: str = "hello"
def addr: *str = &hi;

stack top
1000 1016
1008 5
1016 ptr to "hello"

---
string deref is allowed

def hi: str = "hello"

write(*hi) -> 'h'
