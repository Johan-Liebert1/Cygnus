-- def READ_SYSCALL: int = 0;
-- def WRITE_SYSCALL: int = 1;
-- def OPEN_SYSCALL: int = 2;

include "../examples/include/std.cy"

def size: int = 1024
mem file size

fun main() {
    def i: int = 399;

    write_int_into_mem(file, i)

    syscall(WRITE_SYSCALL, STDOUT, file, 1024)
}

main()
