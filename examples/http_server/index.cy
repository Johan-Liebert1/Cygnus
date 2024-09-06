include "../include/std.cy"

struct sockaddr_in {
    sa_prefix: int16,
    sin_port: int16,
    s_addr: int32,
    pad: int,
};

mem clientaddr 1024
mem read_data 4096

mem serveraddr_mem 16
mem clientaddr_mem 16

mem file_len 32

mem req_method 32
mem req_path 256
mem file_to_read 256

const PRINT_REQ: int = 1;
const SPACE_ASCII: int8 = 32;
const NEW_LINE_ASCII: int8 = 10;
const NULL_BYTE: int8 = 0;

-- GET / HTTP/1.1
-- Host: localhost:5000
-- User-Agent: curl/8.5.0
-- Accept: */*
fun parse_http_request(connfd: int, req: *int, read_bytes: int) {
    if PRINT_REQ {
        syscall(WRITE_SYSCALL, STDOUT, req, read_bytes);
    }

    def dot_html: str = ".html";

    def http_ok: str = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 2\r\n\r\nOK";
    def http_404: str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    def http_500: str = "HTTP/1.1 500 Internal Server Error\r\n\r\n";

    def http_index_html: str = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: ";
    def http_index_html_len: int = strlen(&http_index_html);

    def header_body_seperator: str = "\r\n\r\n";
    def header_body_seperator_len: int = strlen(&header_body_seperator);

    def index_html_file_dir_path: str = "/home/pragyan/Rust/lang/examples/http_server";

    -- we'll only parse the method and path for now
    def idx: int = 0;

    -- parse the method
    def method_ends_at_idx: int = 0;
    loop {
        def character: *char = req + idx;

        if *character == SPACE_ASCII or *character == NULL_BYTE {
            method_ends_at_idx = idx - 1;
            break;
        }

        idx += 1;
    }

    -- consume the space character
    idx += 1;

    def path_starts_at_idx: int = idx;
    def path_ends_at_idx: int = -1;

    -- parse path
    loop {
        def character1: *char = req + idx;

        if *character1 == SPACE_ASCII or *character1 == NULL_BYTE {
            path_ends_at_idx = idx - 1;
            break;
        }

        idx += 1;
    }

    def path_as_char: *char = req + path_starts_at_idx;
    def path_len: int = path_ends_at_idx - path_starts_at_idx + 1;

    if string_ends_with(path_as_char, path_len, dot_html as *char, strlen(&dot_html)) == 0 {
        def write_ret: int = syscall(WRITE_SYSCALL, connfd, http_404 as *char, strlen(&http_404));

        write("Writing http_404 to connfd returned: ");
        print_int(write_ret)
        write("Client asked for path: ")
        syscall(WRITE_SYSCALL, STDOUT, path_as_char, path_len)
        write("\n");

        syscall(CLOSE_SYSCALL, connfd);
        return;
    }

    def final_file_abs_path: int = str_concat(
        index_html_file_dir_path as *char, 
        strlen(&index_html_file_dir_path), 
        path_as_char, 
        path_ends_at_idx - path_starts_at_idx + 1,
        file_to_read
    );
    
    syscall(WRITE_SYSCALL, STDOUT, path_as_char, path_ends_at_idx - path_starts_at_idx + 1)
    write("\n")
    syscall(WRITE_SYSCALL, STDOUT, file_to_read, final_file_abs_path)

    def file_read_bytes: int = read_file_into_memory(read_data, 4096, file_to_read as *char);

    if file_read_bytes < 0 {
        syscall(WRITE_SYSCALL, connfd, http_500 as *char, strlen(&http_500));
        write("read_file_into_memory returned: ")
        print_int(file_read_bytes)
    } else {
        write("Read ", file_read_bytes, " bytes from file ", file_to_read, "\n")

        def write_ret: int = syscall(WRITE_SYSCALL, connfd, http_index_html as *char, http_index_html_len);
        write("Writing to connfd returned: ");
        print_int(write_ret)

        def num_written: int = write_int_into_mem(file_len, file_read_bytes);
        write_ret = syscall(WRITE_SYSCALL, connfd, file_len, num_written);

        write_ret = syscall(WRITE_SYSCALL, connfd, header_body_seperator as *char, header_body_seperator_len);
        write("Writing header_body_seperator to connfd returned: ");
        print_int(write_ret)

        write_ret = syscall(WRITE_SYSCALL, connfd, read_data, file_read_bytes);
        write("Writing to connfd returned: ");
        print_int(write_ret)

    }

    syscall(CLOSE_SYSCALL, connfd);
}

fun main() {
    def sockfd: int = syscall(SOCKET_SYSCALL, AF_INET, SOCK_STREAM, 0);
    write("SOCKET_SYSCALL return: ");
    print_int(sockfd);

    if sockfd < 0 {
        exit(1);
    }

    def sa_prefix: *int16 = serveraddr_mem;
    def sin_port: *int16 = serveraddr_mem + 2;
    def s_addr: *int = serveraddr_mem + 4;

    *sa_prefix = AF_INET;
    *sin_port = PORT;
    *s_addr = S_ADDR;

    def bind_ret: int = syscall(BIND_SYSCALL, sockfd, serveraddr_mem, 16);
    write("BIND_SYSCALL return: ");
    print_int(bind_ret);
    if bind_ret < 0 {
        exit(1);
    }

    def listener: int = syscall(LISTEN_SYSCALL, sockfd, 10);
    write("LISTEN_SYSCALL return: ");
    print_int(listener);
    if listener < 0 {
        exit(1);
    }

    loop {
        def connfd: int = syscall(ACCEPT_SYSCALL, sockfd, 0, 0);
        write("ACCEPT_SYSCALL return: ");
        print_int(connfd);
        
        if connfd < 0 {
            exit(1);
        }

        def read_bytes: int = syscall(READ_SYSCALL, connfd, read_data, 4096);

        parse_http_request(connfd, read_data, read_bytes);
    }
}

main()
