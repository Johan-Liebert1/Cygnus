fun another(depth: int) {
    if depth == 5 {
        return
    }

    loop from 1 to 10 with i {
        another(depth + 1)

        if i == 5 {
            return
        }

        write(i);
    }
}

fun main() {
    another(0)
}

main()
