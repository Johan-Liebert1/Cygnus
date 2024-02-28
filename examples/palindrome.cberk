fun is_palindrome(a: int) {
    def backup_a: int = a;
    def reversed: int = 0;

    loop from 0 to 40000 {
        def last: int = a % 10;

        reversed = reversed * 10 + last;
        
        a = a / 10

        if a == 0 {
            break
        }
    }

    if reversed == backup_a {
        write(reversed, "is a palindrome\n")
    } else {
        write(reversed, "is NOT a palindrome\n")
    }
}

fun main() {
    is_palindrome(10022001)
    is_palindrome(12227)
    is_palindrome(10011001)
    is_palindrome(76874)
    is_palindrome(27212)
    is_palindrome(64868)
    is_palindrome(10033001)
    is_palindrome(65675)
    is_palindrome(10055001)
    is_palindrome(10044001)
}

main() 
