fun main() {
    def current: int[25] = [
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0
    ]; 

    def next: int[25] = [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0
    ]; 

    def i: int = 0;
    def j: int = 0;

    def rows: int = 5;
    def cols: int = 5;

    loop from 0 to 5 {
        i = 0
        j = 0

        loop from 0 to rows {
            j = 0
            loop from 0 to cols {
                if current[i * rows + j] == 0 {
                    write(" . ")
                } else {
                    write(" # ")
                }

                j = j + 1
            }

            write("\n")

            i = i + 1
        }

        write("\n")

        i = 0
        j = 0

        loop from 0 to rows {
            j = 0

            loop from 0 to cols {
                def index: int = i * rows + j;

                def alive_neighbors: int = 0;

                def top_idx: int = index - cols;
                def right_idx: int = index + 1;
                def left_idx: int = index - 1;
                def down_idx: int = index + cols;

                -- top
                if top_idx >= 0 {
                    if current[top_idx] == 1 {
                        alive_neighbors = alive_neighbors + 1
                    }

                    if right_idx < rows * cols {
                        if current[top_idx + 1] == 1 {
                            alive_neighbors = alive_neighbors + 1
                        }
                    }

                    if left_idx >= 0 {
                        if current[top_idx - 1] == 1 {
                            alive_neighbors = alive_neighbors + 1
                        }
                    }
                }

                -- down
                if down_idx < rows * cols {
                    if current[down_idx] == 1 {
                        alive_neighbors = alive_neighbors + 1
                    }

                    if right_idx < rows * cols {
                        if current[down_idx + 1] == 1 {
                            alive_neighbors = alive_neighbors + 1
                        }
                    }

                    if left_idx >= 0 {
                        if current[down_idx - 1] == 1 {
                            alive_neighbors = alive_neighbors + 1
                        }
                    }
                }

                if right_idx < rows * cols {
                    if current[right_idx] == 1 { alive_neighbors = alive_neighbors + 1 }
                }

                if left_idx >= 0 {
                    if current[left_idx] == 1 { alive_neighbors = alive_neighbors + 1 }
                }

                if current[index] == 1 {
                    if (alive_neighbors == 2 or alive_neighbors == 3) {
                        next[index] = 1;
                    } else {
                        next[index] = 0;
                    }
                } else {
                    if alive_neighbors == 3 {
                        next[index] = 1;
                    } else {
                        next[index] = 0;
                    }
                }

                j = j + 1
            }

            i = i + 1
        }

        i = 0
        j = 0

        loop from 0 to rows {
            j = 0
            loop from 0 to cols {
                def index: int = i * rows + j;

                current[index] = next[index]

                j = j + 1
            }

            i = i + 1
        }
    }
}

main()
