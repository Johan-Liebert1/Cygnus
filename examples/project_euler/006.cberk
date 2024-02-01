-- The sum of the squares of the first ten natural numbers is, 1^2 + 2^2 .. 10^2 = 385
-- The square of the sum of the first ten natural numbers is, ( 1 + 2 + ... + 10 ) ^ 2 = 3025

-- Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640
-- Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fun main() {
    def i: int = 1;
    def sum: int = 0;
    def square_sum: int = 0;

    loop from 1 to 101 {
        sum = sum + i;
        square_sum = square_sum + (i * i);
        
        i = i + 1;
    }

    write(sum * sum - square_sum)
}

main()
