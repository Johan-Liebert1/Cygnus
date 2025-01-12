mem lower_char_occurances 8 * 26

fun main() {
    -- def inc: int8 =  *(thing as *char) - lower_a;

    def inc: int8 = 3;
    def addr_to_update: *int = lower_char_occurances + (inc * 8) as int;

    write("lower_char_occurances = ", lower_char_occurances)
    write("addr_to_update = ", addr_to_update)
}

main()
