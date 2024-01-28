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
- [ ] Make sure nothing is left on the stack when we break out of a loop
- [ ] Variable scoping
- [ ] Pointer deref
- [ ] Command link arguments
- [ ] Proper type checking
- [ ] Random access to memory
- [ ] Ability to allocate and deallocate memory
- [ ] Macros

# Grammar

    PROGRAM                  -> STATEMENT[]
    STATEMENT                -> VARIABLE_DECLARATION | CONDITIONAL_STATEMENT | COMPARISON_EXPRESSION | LPAREN COMPARISON_EXPRESSION RPAREN | LOOP | FUNCTION_CALL | FUNCTION_DEF
    FUNCTION_DEF             -> fun VAR_NAME LPAREN (VAR_NAME : VAR_TYPE)* RPAREN LCURLY (STATEMENT[] - FUNCTION_DEF) RCURLY
    FUNCTION_CALL            -> VAR_NAME LPAREN (COMPARISON_EXPRESSION)* RPAREN
    LOOP                     -> loop from LPAREN* EXPRESSION to EXPRESSION (step EXPRESSION)* RPAREN* (with VAR_NAME)* LCURLY STATEMENT[] RCURLY
    CONDITIONAL_STATEMENT    -> if LPAREN* LOGICAL_EXPRESSION RPAREN* LCURLY STATEMENT[]* RCURLY ELSE_STATEMENT*
    ASSIGNMENT_STATEMENT     -> VAR_NAME = (COMPARISON_EXPRESSION)*
    ELSE_STATEMENT           -> else LCURLY STATEMENT[]* RCURLY
    VARIABLE_DECLARATION     -> def VAR_NAME: VAR_TYPE (= LOGICAL_EXPRESSION)*
    VAR_TYPE                 -> int | float
    LOGICAL_EXPRESSION       -> COMPARISON_EXPRESSION ((and | or) COMPARISON_EXPRESSION)*
    COMPARISON_EXPRESSION    -> EXPRESSION ((> | < | >= | <= | == | !=) EXPRESSION)*
    EXPRESSION               -> TERM (( + | - ) TERM)*                      # for precedence as term will be calculated first
    TERM                     -> FACTOR (( * | /  | << | >> | % ) FACTOR)*
    COMMENT                  -> -- (ANY)*
    FACTOR                   -> INTEGER | FLOAT | VARIABLE | STRING_LITERAL | LPAREN EXPRESSION RPAREN
    VAR_NAME                 -> any valid identifier
    LPAREN                   -> (
    RPAREN                   -> )
    LCURLY                   -> {
    RCURLY                   -> }
