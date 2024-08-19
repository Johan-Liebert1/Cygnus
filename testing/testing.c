#include <stdint.h>
#include <stdio.h>

int main() {
    double pi = 2.33;
    printf("%ld\n", *(uint64_t*)&pi);

    printf("%f\n", pi);
}
