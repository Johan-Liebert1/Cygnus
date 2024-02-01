-- A palindromic number reads the same both ways. The largest palindrome made from the product of two 3-digit numbers is 9009 = 91 * 99
-- Find the largest palindrome made from the product of two 3-digit numbers.


fun main() {
    def a: int = 100;
    def b: int = 100;

    def answer: int = 0;

    loop from 100 to 1000 {
        b = 100;

        loop from 100 to 1000 {
            def product: int = a * b;

            def original: int = product;
            def reversed: int = 0;

            -- just looping over the digits of the number
            -- since we don't have while loop
            loop from 1 to 1000 {
                def remainder: int = product % 10;
                reversed = reversed * 10 + remainder;
                product = product / 10;

                if product == 0 {
                    break;
                }
            }

            if original == reversed and original > answer {
                answer = original;
            }

            b = b + 1;
        }

        a = a + 1;
    }

    write(answer);
}

main()
