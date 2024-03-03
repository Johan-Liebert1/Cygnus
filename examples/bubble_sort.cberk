fun main() {
    def array: int[15] = [12, 8, 9, 13, 5, 4, 3, 1, 11, 6, 2, 14, 15, 7, 10];

    def i: int = 0;
    def j: int = 0;

    loop from 0 to 15 {
        j = 0
        loop from 0 to 14 {
            if array[j] > array[j + 1] {
                def temp: int = array[j];
                array[j] = array[j + 1];
                array[j + 1] = temp;
            }

            j = j + 1
        }

        i = i + 1
    }

    i = 0

    loop from 0 to 15 {
        write(array[i])
        i = i + 1
    }
}

main() 
