struct hi {
    b: int,
    c: int,
    d: str
}

struct hello {
    a: int
}

fun main() {
    def a: hi = hi {
        c: 2,
        b: 69,
        d: "hello\n"
    };

    write(a.d)
    write(a.c)
    write(a.b)
}

main()
