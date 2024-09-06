fun main() {
    def a: int = 1;

    loop from 1 to 5 {
        if 5 <= 8 {
            write("hello\n")

            loop from 1 to 4 {
                if 3 != 3 {
                    write("lol here\n")
                } else {
                    write("lol not here\n")
                }
            }

        } elif 6 == 2 {
            write("hi\n")
        } else {
            write("bye\n")
        }
    }

    loop from 1 to 3 {
        if 4 < 2 {
            write("4 < 2\n")
        } else {
            write("4 > 2\n")
        }
    }

    loop from 1 to 11 step 2 {
        write("loop with step of 2\n")
    }
}

main()
