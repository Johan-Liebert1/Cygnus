#include <SDL2/SDL.h>
#include <SDL2/SDL_video.h>
#include <stdint.h>
#include <stdio.h>

void print_double_as_int(double d) { printf("%ld\n", *(uint64_t *)&d); }

void double_test() {
    double pi = 2.33;
    printf("%ld\n", *(uint64_t *)&pi);
    printf("%f\n", pi);
}

void double_add(double d, double e) {
    double sum = d + e;
    printf("%f\n", sum);
    printf("%ld\n", *(uint64_t *)&sum);
}

void double_sub(double d, double e) {
    double sum = d - e;
    printf("%f\n", sum);
    printf("%ld\n", *(uint64_t *)&sum);
}

void double_mul(double d, double e) {
    double sum = d * e;
    printf("%f\n", sum);
    printf("%ld\n", *(uint64_t *)&sum);
}

void double_div(double d, double e) {
    double sum = d / e;
    printf("%f\n", sum);
    printf("%ld\n", *(uint64_t *)&sum);
}

int main() {
    SDL_Window *window =
        SDL_CreateWindow("Chess", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 800, 800, SDL_WINDOW_SHOWN | SDL_WINDOW_ALLOW_HIGHDPI);

    return 0;
}
