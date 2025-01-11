mem memory 128

fun main() {
    *memory = 97;

    def lower_a: int8 = 97;

    def thing: *int = memory;
    def inc: int8 =  *(memory as *int) - 97;

    def addr_to_update: *int = memory + inc * 8;

    *addr_to_update = 14;
}

main()
