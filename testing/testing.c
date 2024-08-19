#include <stdint.h>
#include <stdio.h>

void print_double_as_int(double d) {
    printf("%ld\n", *(uint64_t *)&d);
}

void double_test() {
    double pi = 2.33;
    printf("%ld\n", *(uint64_t *)&pi);
    printf("%f\n", pi);
}

void double_add(double d, double e) {
    double sum = d + e;
    printf("%f\n", sum);
    printf("%ld\n", *(uint64_t*)&sum);
}

void double_sub(double d, double e) {
    double sum = d - e;
    printf("%f\n", sum);
    printf("%ld\n", *(uint64_t*)&sum);
}

void double_mul(double d, double e) {
    double sum = d * e;
    printf("%f\n", sum);
    printf("%ld\n", *(uint64_t*)&sum);
}

void double_div(double d, double e) {
    double sum = d / e;
    printf("%f\n", sum);
    printf("%ld\n", *(uint64_t*)&sum);
}

int main() {
    double_add(3.14, 3.14);
    print_double_as_int(3.14);
    return 0;
}
