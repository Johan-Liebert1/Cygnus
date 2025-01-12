-- Count the number of occurances of all alphabets in a file

mem file 4096

-- int * num alphabets
mem lower_char_occurances 8 * 26

-- int * num alphabets
mem upper_char_occurances 8 * 26

fun main() {
    def lower_a: int8 = 97;
    def lower_z: int8 = lower_a + 26 - 1;

    def upper_a: int8 = 65;
    def upper_z: int8 = lower_a + 26 - 1;

    def file_name: str = "../examples/test_files/read_a_file\0";

    -- (syscall number, file_name, readonly flag)
    -- open syscall
    def fd: int = syscall(2, file_name as *char, 0, 0);

    write(fd)

    -- read syscall
    def read_bytes: int = syscall(0, fd, file, 4096);

    -- write
    syscall(1, 1, file, read_bytes);

    def i: int = 0;

    loop from 0 to read_bytes {
        -- Kinda scuffed but eh it is what it is
        def thing: *int = file + i;

        if *(thing as *char) >= lower_a and *(thing as *char) <= lower_z {
            -- write("after the first if\n")
            def inc: int8 =  *(thing as *char) - lower_a;

            def addr_to_update: *int = lower_char_occurances + (inc * 8) as int;

            -- FIXME: Can't do *addr_to_update = *addr_to_update + 1;
            *addr_to_update = *(addr_to_update as *int) + 1;
        }

        i += 1
    }

    i = lower_a;

    def j: int = 0;

    loop from 0 to 26 {
        write(i, *(lower_char_occurances + j * 8));

        j += 1;
        i += 1;
    }

}

main()
