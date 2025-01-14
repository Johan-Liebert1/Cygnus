fun fib_recursive(n: int) -> int {
    if n <= 0 {
        return 0
    }

    if n == 1 {
        return 1
    }

    return fib_recursive(n - 1) + fib_recursive(n - 2)
}

fun one() -> int {
    return 1
}

fun two() -> int {
    return 2
}

fun main() {
    def three: int = one() + two()

    write("three = ", three)
}

main()
