fun main() {
    def a: int8 = 97; 
    def inc: *char = &a;

    def b: int = 40;
    def c: *int32 = &b;

    write("a = ", a)
    write("inc = ", inc)
    write("b = ", b)
    write("c = ", c)
}

main()
