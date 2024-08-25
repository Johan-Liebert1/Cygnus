#include <SDL2/SDL.h>
#include <SDL2/SDL_video.h>
#include <stdint.h>

int main() {
    SDL_Init(SDL_INIT_VIDEO);

    SDL_Window *window =
        SDL_CreateWindow("Chess", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 800, 800, SDL_WINDOW_SHOWN | SDL_WINDOW_ALLOW_HIGHDPI);

    return 0;
}
