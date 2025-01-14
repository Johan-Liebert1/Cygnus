fun fib_recursive(n: int) -> int {
    if n <= 0 {
        return 0
    }

    if n == 1 {
        return 1
    }

    return fib_recursive(n - 1) + fib_recursive(n - 2)
}

fun main() {
    write(fib_recursive(10))
    write(fib_recursive(11))
}

main()
