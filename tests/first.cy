const NULL_BYTE: int8 = 0;

mem thingy 128

fun main() {
    -- def NULL_BYTE: int8 = 0;

    *thingy = 1;

    loop from 0 to 10 with i {
        def current_char: *char = thingy + i;

        write("current_char = ", *current_char)

        if *current_char == NULL_BYTE {
            write("ch == NULL_BYTE\n")
            break;
        } else {
            write("ch IS NOT NULL_BYTE\n")
        }
    }
}

main()
