-- include "../examples/include/std.cberk"

type renderer = int;
type window = int;

struct SDL_Rect {
    x: int32,
    y: int32,
    w: int32,
    h: int32,
};

extern fun SDL_Init(flags: int) -> int;
extern fun SDL_CreateWindow(windowName: *char, x: int32, y: int32, width: int32, height: int32, flags: int32) -> *window;
extern fun SDL_PollEvent(event: *char) -> int;
extern fun SDL_GetTicks() -> int;
extern fun SDL_GetError() -> *char;

extern fun SDL_Quit();

extern fun SDL_CreateRenderer(window: *window, index: int32, flags: int32) -> *renderer;
extern fun SDL_SetRenderDrawColor(renderer: *renderer, r: int, g: int, b: int, a: int) -> int;
extern fun SDL_RenderFillRect(renderer: *renderer, dst_rect: *SDL_Rect) -> int;
extern fun SDL_RenderPresent(renderer: *renderer);
extern fun SDL_DestroyRenderer(renderer: *renderer);
extern fun SDL_RenderClear(renderer: *renderer) -> int;

-- The SDL_Event
mem event 256

def SDL_QUIT: int32 = 256;
def SDL_KEYDOWN: int32 = 0x300

def SDL_ESC: int32 = 27;
def SDLK_a: int32 = 97;
def SDLK_b: int32 = 98;
def SDLK_c: int32 = 99;
def SDLK_d: int32 = 100;
def SDLK_e: int32 = 101;
def SDLK_f: int32 = 102;
def SDLK_g: int32 = 103;
def SDLK_h: int32 = 104;
def SDLK_i: int32 = 105;
def SDLK_j: int32 = 106;
def SDLK_k: int32 = 107;
def SDLK_l: int32 = 108;
def SDLK_m: int32 = 109;
def SDLK_n: int32 = 110;
def SDLK_o: int32 = 111;
def SDLK_p: int32 = 112;
def SDLK_q: int32 = 113;
def SDLK_r: int32 = 114;
def SDLK_s: int32 = 115;
def SDLK_t: int32 = 116;
def SDLK_u: int32 = 117;
def SDLK_v: int32 = 118;
def SDLK_w: int32 = 119;
def SDLK_x: int32 = 120;
def SDLK_y: int32 = 121;
def SDLK_z: int32 = 122;

def SDL_RENDERER_SOFTWARE: int32 = 0x00000001;      -- < The renderer is a software fallback */
def SDL_RENDERER_ACCELERATED: int32 = 0x00000002;   -- < The renderer uses hardware
def SDL_RENDERER_PRESENTVSYNC: int32 = 0x00000004;  -- < Present is synchronized
def SDL_RENDERER_TARGETTEXTURE: int32 = 0x00000008;  -- < The renderer supports

def RECT_SPEED: int32 = 10;

fun gol() {
    def array: int[50];
}

fun main() {
    def sdl_init: int = SDL_Init(32);
    write("sdl_init = ", sdl_init)

    def name: str = "hello\0";
    def window: *int = SDL_CreateWindow(name as *char, 10, 10, 800, 600, 8196);

    write("window = ", window);

    def NULL: int = 0;

    if window == NULL as *int {
        def error: *char = SDL_GetError();
        write(error)
        return;
    }

    def renderer: *renderer = SDL_CreateRenderer(window, 0, SDL_RENDERER_ACCELERATED);
    write("renderer = ", renderer);

    def FPS: int = 60;
    def quit: int = 0;

    def a: int = SDL_GetTicks();
    def b: int = SDL_GetTicks();

    def delta: int = 0;

    def ret: int = 0;

    def rect: SDL_Rect = SDL_Rect {
        x: 100 + 45,
        y: 100,
        w: 50,
        h: 50,
    };

    loop {
        if quit == 1 {
            break;
        }

        a = SDL_GetTicks();
        delta += (a - b);

        if (delta < FPS) {
            continue
        }

        ret = SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
        ret = SDL_RenderClear(renderer);

        ret = SDL_PollEvent(event);

        if ret != 0 {
            if *(event as *int32) == SDL_QUIT {
                quit = 1;
            } elif *(event as *int32) == SDL_KEYDOWN {
                def keysym: *int32 = event + 4 * 5;

                if *keysym == SDL_ESC or *keysym == SDLK_q {
                    quit = 1;
                } elif *keysym == SDLK_j {
                    rect.y = rect.y - RECT_SPEED
                } elif *keysym == SDLK_k {
                    rect.y = rect.y + RECT_SPEED
                }  elif *keysym == SDLK_h {
                    rect.x = rect.x - RECT_SPEED
                } elif *keysym == SDLK_l {
                    rect.x = rect.x + RECT_SPEED
                }
            }
        }

        ret = SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255);

        ret = SDL_RenderFillRect(renderer, &rect);

        SDL_RenderPresent(renderer);

        b = SDL_GetTicks();
    }

    SDL_DestroyRenderer(renderer);
    SDL_Quit();
}

main()
