#include <stdint.h>
#include <stdio.h>

int main() {
    double pi = 3.1415;
    printf("%lb\n", *(uint64_t*)&pi);

    printf("%f\n", pi);
}
