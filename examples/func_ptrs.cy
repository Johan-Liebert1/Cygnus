type mah_function = def (int) -> int;

fun foo(a: int) -> int {
    write("from function foo a = ", a)

    return -1
}

fun bar(f: mah_function, a: int) {
    f(200)
    f(a)
}

fun main() {
    bar(&foo, 1);
    bar(&foo, 5);
}

main()
