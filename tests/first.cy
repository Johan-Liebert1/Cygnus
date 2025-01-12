mem req 1024

fun main() {
    def thing: *int8 = req;
    def path_starts_at_idx: int = 8;
    def path_as_char: *char = thing + path_starts_at_idx;
}

main()
