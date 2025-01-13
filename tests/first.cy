include "../examples/include/std.cy"

fun main() {
    def hello: str = "hello";
    syscall(1, 1, hello as *char, strlen(&hello))
}

main()
