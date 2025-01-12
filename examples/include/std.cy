def READ_SYSCALL: int = 0;
def WRITE_SYSCALL: int = 1;
def OPEN_SYSCALL: int = 2;
def CLOSE_SYSCALL: int = 3;
def MMAP_SYSCALL: int = 9;
def SOCKET_SYSCALL: int = 41;
def ACCEPT_SYSCALL: int = 43;
def BIND_SYSCALL: int = 49;
def LISTEN_SYSCALL: int = 50;

def AF_INET: int = 2;
def S_ADDR: int32 = 16777343; -- htonl(127.0.0.1)
def PORT: int16 = 34835; -- htons(5000)
def PAD: int = 0;

def SOCK_STREAM: int = 1;

def STDOUT: int = 1;
def STDIN: int = 0;
def STDERR: int = 2;

fun print_int(a: int) {
    def n: int = 0;

    if a >> 63 == 1 {
        n = not a - 1
        write("-", n)
    } else {
        n = a
        write(n)
    }
}

fun memset(ptr: *int, value: int, size: int) {
    def i: int = 0;

    loop from 0 to value {
        def thing: *int = ptr + i;
        *thing = value;

        i += 8;
    }
}

fun strlen_cstr(string: *char) -> int {
    def i: int = 0;

    loop {
        if *(string + i) == 0 {
            break;
        }

        i += 1;
    }

    return i;
}

fun strlen(string: *str) -> int {
    def len: *str = string + 8;
    def length: int = *(len as *int);
    return length;
}

fun read_file_into_memory(memory: *int, mem_size: int, abs_file_path: *char) -> int {
    -- (syscall number, file_name, readonly flag)
    -- open syscall
    def fd: int = syscall(OPEN_SYSCALL, abs_file_path, 0, 0);

    if fd < 0 {
        write("read_file_into_memory open syscall failed for file: ")
        print_int(fd)
        return -1;
    }

    -- read syscall
    def read_bytes: int = syscall(READ_SYSCALL, fd, memory, mem_size);

    syscall(CLOSE_SYSCALL, fd)

    return read_bytes;
}

-- returns the amount of bytes written into memory
fun write_int_into_mem(memory: *int, number: int) -> int {
    def zero_ascii: int8 = 48;
    def n: int = number;

    def number_len: int = 0;
    
    loop {
        n = n / 10
        number_len += 1

        if n == 0 {
            break;
        }
    }

    def idx: int = number_len - 1;
    n = number

    loop {
        def c: int8 = n % 10;

        def idx_into_mem: *int8 = memory + idx;
        *idx_into_mem = c + zero_ascii;

        n = n / 10
        idx -= 1

        if n == 0 {
            break;
        }
    }

    return number_len;
}

fun write_str_into_mem(memory: *char, string: *char, len: int) -> int {
    loop from 0 to len with i {
        def memo: *char = memory + i;
        *memo = *(string + i);
    }

    return len
}

-- @returns 0 if string does not end with substr else returns the index in the string where substr starts
fun string_ends_with(string: *char, string_len: int, substr: *char, substr_len: int) -> int {
    if substr_len > string_len {
        return 0;
    }

    def idx_into_str: int = string_len - 1;
    def idx_into_substr: int = substr_len - 1;

    loop {
        def str_char: *char = string + idx_into_str;
        def substr_char: *char = substr + idx_into_substr;

        if *str_char != *substr_char {
            return 0;
        }

        idx_into_str -= 1;
        idx_into_substr -= 1;

        if idx_into_substr < 0 {
            break;
        }
    }

    return idx_into_str + 1;
}

-- @returns true if string starts with substr
fun string_starts_with(string: *char, string_len: int, substr: *char, substr_len: int) -> int {
    if substr_len > string_len {
        return 0;
    }

    def idx_into_str: int = 0;
    def idx_into_substr: int = 0;

    loop {
        def str_char: *char = string + idx_into_str;
        def substr_char: *char = substr + idx_into_substr;

        if *str_char != *substr_char {
            return 0;
        }

        idx_into_str += 1;
        idx_into_substr += 1;

        if idx_into_substr >= substr_len - 1 {
            break;
        }
    }

    return 1;
}

fun string_eq(string: *char, string_len: int, string2: *char, string2_len: int) -> int {
    if string_len != string2_len {
        return 0;
    }

    def idx_into_str: int = 0;
    def idx_into_substr: int = 0;

    loop {
        def str_char: *char = string + idx_into_str;
        def substr_char: *char = string2 + idx_into_substr;

        if *str_char != *substr_char {
            return 0;
        }

        idx_into_str += 1;
        idx_into_substr += 1;

        if idx_into_substr >= string_len - 1 {
            break;
        }
    }

    return 1;
}

fun str_concat(string: *char, string_len: int, string2: *char, string2_len: int, new_str: *char) -> int {
    def idx_into_new_str: int = 0;

    loop from 0 to string_len with i {
        def new_str_idx: *char = new_str + idx_into_new_str;
        def old_char: *char = string + i;

        *new_str_idx = *old_char;

        idx_into_new_str += 1;
    }

    loop from 0 to string2_len with i {
        def new_str_idx: *char = new_str + idx_into_new_str;
        def old_char: *char = string2 + i;

        *new_str_idx = *old_char;

        idx_into_new_str += 1;
    }

    return idx_into_new_str;
}
