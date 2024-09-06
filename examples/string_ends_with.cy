include "./include/std.cy"

fun main() {
    def string: str = "index.html";
    def subs: str = "html";
    write(string_ends_with(string as *char, strlen(&string), subs as *char, strlen(&subs)))

    string = "first second third";
    subs = "third";
    write(string_ends_with(string as *char, strlen(&string), subs as *char, strlen(&subs)))

    string = "hello";
    subs = "ell";
    write(string_ends_with(string as *char, strlen(&string), subs as *char, strlen(&subs)))

    string = "wowo";
    subs = "wow";
    write(string_ends_with(string as *char, strlen(&string), subs as *char, strlen(&subs)))

    string = "mississippi";
    subs = "ippi";
    write(string_ends_with(string as *char, strlen(&string), subs as *char, strlen(&subs)))
}

main()
