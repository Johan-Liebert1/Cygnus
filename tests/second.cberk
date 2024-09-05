type mah_function = def (int) -> int;

fun foo(a: int) -> int {
    write("from function foo a = ", a)
    return a + 5
}

fun bar2(g: mah_function, b: float) {
    write("from function bar2 b = ", b)
}

fun bar(f: mah_function, a: int) {
    def first_ret: int = f(200)
    write("first_ret = ", first_ret)

    def second_ret: int = f(a)
    write("second_ret = ", second_ret)

    bar2(f, 3.14)
}

fun main() {
    bar(&foo, 1);
    bar(&foo, 5);
}

main()
