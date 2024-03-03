mem current 25 * 8 -- 5 x 5 grid
mem next 25 * 8 -- 5 x 5 grid

-- If the cell is alive, then it stays alive if it has either 2 or 3 live neighbors
-- If the cell is dead, then it springs to life only in the case that it has 3 live neighbors

-- *             *
-- * -> * * * -> *
-- *             *
fun handle_next(i: int) {
    def into_current: *int = current + i * 8;
    def into_next: *int = next + i * 8;

    *into_next = 0;

    def top: *int = into_next;
    def top_left: *int = into_next;
    def top_right: *int = into_next;

    def down: *int = into_next;
    def down_left: *int = into_next;
    def down_right: *int = into_next;

    def left: *int = into_next;
    def right: *int = into_next;

    if i >= 5 {
        top = current + (i - 5) * 8;

        def top_left_index: int = i - 5 - 1;
        def top_right_index: int = i - 5 + 1;

        if top_left_index >= 0 and  top_left_index < 25 {
            top_left = current + top_left_index * 8;
        }

        if top_right_index >= 0 and top_right_index < 25 {
            top_right = current + top_right_index * 8;
        }
    }

    if i <= (25 - 5) {
        down = current + (i + 5) * 8;

        def down_left_index: int = i + 5 - 1;
        def down_right_index: int = i + 5 + 1;

        if down_left_index >= 0 and down_left_index < 25  {
            down_left = current + down_left_index * 8;
        }

        if down_right_index >= 0 and down_right_index < 25 {
            down_right = current + down_right_index * 8;
        }
    }

    if i - 1 >= 0 {
        left = current + (i - 1) * 8;
    }
    
    if i + 1 < 25 {
        right = current + (i + 1) * 8;
    }

    def alive_neighbors: int = *top + *top_left + *top_right + *down + *down_left + *down_right + *right + *left;

    if *into_current == 1 {
        if (alive_neighbors == 2 or alive_neighbors == 3) {
            *into_next = 1;
        } else {
            *into_next = 0;
        }
    } else {
        if alive_neighbors == 3 {
            *into_next = 1;
        } else {
            *into_next = 0;
        }
    }
}

fun main() {
    def thing: *int = current + 8;
    def thing2: *int = current + (5 + 1) * 8;
    def thing3: *int = current + (2 * 5 + 1) * 8;

    *thing = 1;
    *thing2 = 1;
    *thing3 = 1;

    def iterations: int = 4;

    def i: int = 0;

    loop from 0 to iterations {
        -- printing bit
        loop from 0 to 25 {
            def thing: *int = current + i * 8;

            if *thing == 1 {
                write(" # ")
            } else {
                write(" . ")
            }

            i = i + 1

            if i % 5 == 0 {
                write("\n")
            }
        }

        write("\n\n")

        i = 0;
        -- Calculate the next state
        loop from 0 to 25 {
            handle_next(i)
            i = i + 1
        }

        i = 0;
        loop from 0 to 25 {
            def into_current: *int = current + i * 8;
            def into_next: *int = next + i * 8;

            *into_current = *into_next

            i = i + 1
        }
    }
}

main()
