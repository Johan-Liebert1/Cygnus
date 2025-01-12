-- def READ_SYSCALL: int = 0;
-- def WRITE_SYSCALL: int = 1;
-- def OPEN_SYSCALL: int = 2;

include "../examples/include/std.cy"

mem file 1024

fun main() {
    def i: int = 399;

    write_int_into_mem(file, i)

    syscall(WRITE_SYSCALL, STDOUT, file, 1024)
}

main()
