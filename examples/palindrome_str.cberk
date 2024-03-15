fun is_palindrome(length: int, string: *char) -> int {
    def i: int = 0;

    loop from 0 to ((length / 2) + 1) {
        def sub: int = length - i - 1;

        def eq: int = *(string + i) == *(string + sub);

        if eq != 1 {
            return 0
        }

        i = i + 1
    }

    return 1
}

fun main() {
    def rush: str = "rush";

    if is_palindrome(4, rush as *char) == 1 {
        write(rush, " is a palindrome\n")
    } else {
        write(rush, " is NOT a palindrome\n")
    }

    def racecar: str = "racecar";
    if is_palindrome(7, racecar as *char) == 1 {
        write(racecar, " is a palindrome\n")
    } else {
        write(racecar, " is NOT a palindrome\n")
    }

    def mississippi: str = "mississippi";
    if is_palindrome(11, mississippi as *char) == 1 {
        write(mississippi, " is a palindrome\n")
    } else {
        write(mississippi, " is NOT a palindrome\n")
    }

    def kayak: str = "kayak";
    if is_palindrome(5, kayak as *char) == 1 {
        write(kayak, " is a palindrome\n")
    } else {
        write(kayak, " is NOT a palindrome\n")
    }

    def deified: str = "deified";
    if is_palindrome(7, deified as *char) == 1 {
        write(deified, " is a palindrome\n")
    } else {
        write(deified, " is NOT a palindrome\n")
    }

    def deed: str = "deed";
    if is_palindrome(4, deed as *char) == 1 {
        write(deed, " is a palindrome\n")
    } else {
        write(deed, " is NOT a palindrome\n")
    }

    def repaper: str = "repaper";
    if is_palindrome(7, repaper as *char) == 1 {
        write(repaper, " is a palindrome\n")
    } else {
        write(repaper, " is NOT a palindrome\n")
    }
}

main()
