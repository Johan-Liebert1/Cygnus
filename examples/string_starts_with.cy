include "./include/std.cy"

fun main() {
    def string: str = "index.html";
    def subs: str = "html";
    write(string_starts_with(string as *char, strlen(&string), subs as *char, strlen(&subs)))

    string = "first second third";
    subs = "first sec";
    write(string_starts_with(string as *char, strlen(&string), subs as *char, strlen(&subs)))

    string = "index.html";
    subs = "index.html but there's more";
    write(string_starts_with(string as *char, strlen(&string), subs as *char, strlen(&subs)))

    string = "spaceship";
    subs = "space";
    write(string_starts_with(string as *char, strlen(&string), subs as *char, strlen(&subs)))

    string = "television";
    subs = "elev";
    write(string_starts_with(string as *char, strlen(&string), subs as *char, strlen(&subs)))
}

main()
