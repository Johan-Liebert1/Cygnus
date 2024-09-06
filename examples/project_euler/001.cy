fun main() {
    def sum: int = 0;
    def i: int = 0;

    loop from 1 to 1001 {
        if i % 3 == 0 or i % 5 == 0 {
            sum = sum + i;
        }

        i = i + 1;
    }

    write(sum)
}

main()
