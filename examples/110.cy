mem current 100 * 8
mem next 100 * 8

fun main() {
    def iterations: int = 100;

    def fourty_seven: *int = current + (iterations * 8 - 8 * 3);
    def fourty_eight: *int = current + (iterations * 8 - 8 * 2);
    def fourty_nine: *int = current +  (iterations * 8 - 8);

    *fourty_seven = 1;
    *fourty_eight = 1;
    *fourty_nine = 0;

    def first: *int = current;
    def second: *int = current;
    def third: *int = current;

    def i: int = 0;

    loop from 0 to iterations {
        i = 0;

        loop from 0 to iterations + 1 {
            first = current + i * 8;
            second = current + i * 8 + 8;
            third = current + i * 8 + 16;

            def next_second: *int = next + i * 8 + 8;

            -- 0, 1, 1, 1, 0, 1, 1, 0
            if *first == 0 and *second == 0 and *third == 0   { *next_second = 0 }
            elif *first == 0 and *second == 0 and *third == 1 { *next_second = 1 }
            elif *first == 0 and *second == 1 and *third == 0 { *next_second = 1 }
            elif *first == 0 and *second == 1 and *third == 1 { *next_second = 1 }
            elif *first == 1 and *second == 0 and *third == 0 { *next_second = 0 }
            elif *first == 1 and *second == 0 and *third == 1 { *next_second = 1 }
            elif *first == 1 and *second == 1 and *third == 0 { *next_second = 1 }
            elif *first == 1 and *second == 1 and *third == 1 { *next_second = 0 }

            i = i + 1
        }

        def j: int = 0;

        loop from 0 to iterations {
            if *(next + j) == 0 {
                write(" ");
            } else {
                write("*");
            }
            
            j = j + 8
        }

        write("\n");

        j = 0;

        loop from 0 to iterations {
            def idx_into_current: *int = current + j;
            def idx_into_next: *int = next + j;
            
            *idx_into_current = *idx_into_next;
            
            j = j + 8
        }
    }
}

main()
