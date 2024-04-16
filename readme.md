## Programming Language called CBerk

### Implemented features

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
- [x] Negative Numbers
- [x] HTTP Server

- [ ] Fix structs
- [ ] Fix type casting for pointers

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

# (Extremely) Simple HTTP server

```lua
include "../include/std.cberk"

def AF_INET: int16 = 2;
def S_ADDR: int32 = 16777343; -- htonl(127.0.0.1)
def PORT: int16 = 34835; -- htons(5000)
def PAD: int = 0;

def SOCK_STREAM: int = 1;

def SOCKET_SYSCALL: int = 41;
def READ_SYSCALL: int = 0;
def WRITE_SYSCALL: int = 1;
def OPEN_SYSCALL: int = 2;
def CLOSE_SYSCALL: int = 3;
def ACCEPT_SYSCALL: int = 43;
def BIND_SYSCALL: int = 49;
def LISTEN_SYSCALL: int = 50;

def STDOUT: int = 1;

struct sockaddr_in {
    sa_prefix: int16,
    sin_port: int16,
    s_addr: int32,
    pad: int,
};

mem clientaddr 1024
mem read_data 4096

mem serveraddr_mem 16
mem clientaddr_mem 16

fun main() {
    def sockfd: int = syscall(SOCKET_SYSCALL, AF_INET, SOCK_STREAM, 0);
    write("SOCKET_SYSCALL return: ");
    print_int(sockfd);

    if sockfd < 0 {
        exit(1);
    }

    def sa_prefix: *int16 = serveraddr_mem;
    def sin_port: *int16 = serveraddr_mem + 2;
    def s_addr: *int = serveraddr_mem + 4;

    *sa_prefix = AF_INET;
    *sin_port = PORT;
    *s_addr = S_ADDR;

    def len: int32 = 0;

    def http_ok: str = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 2\r\n\r\nOK";

    def bind_ret: int = syscall(BIND_SYSCALL, sockfd, serveraddr_mem, 16);
    write("BIND_SYSCALL return: ");
    print_int(bind_ret);
    write(bind_ret);

    if bind_ret < 0 {
        exit(1);
    }

    def listener: int = syscall(LISTEN_SYSCALL, sockfd, 10);
    write("LISTEN_SYSCALL return: ");
    print_int(listener);
    if listener < 0 {
        exit(1);
    }

    loop {
        def connfd: int = syscall(ACCEPT_SYSCALL, sockfd, 0, 0);
        write("ACCEPT_SYSCALL return: ");
        print_int(connfd);
        
        if connfd < 0 {
            exit(1);
        }

        def read_bytes: int = syscall(READ_SYSCALL, connfd, read_data, 4096);
        syscall(WRITE_SYSCALL, STDOUT, read_data, 4096);

        def write_ret: int = syscall(WRITE_SYSCALL, connfd, http_ok as *char, 74);
        write("Writing to connfd returned: ", write_ret);

        syscall(CLOSE_SYSCALL, connfd);
    }
}

main()
```
