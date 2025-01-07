fun get_len() -> int {
    return 5
}

fun main() {
    def thingy: str = "hello";
    syscall(1, 1, thingy as *char, get_len())
}

main()
