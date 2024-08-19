#include <stdint.h>
#include <stdio.h>

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
    double_add(6.28, 3.14);
    double_sub(6.28, 3.14);
    double_mul(6.28, 3.14);
    double_div(6.28, 3.14);
    return 0;
}
