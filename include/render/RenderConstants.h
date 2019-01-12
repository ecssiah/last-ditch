#ifndef RENDER_CONSTANTS_H
#define RENDER_CONSTANTS_H

#include "../utility/Types.h"
#include "../map/MapConstants.h"

constexpr i32 FPS{30};
constexpr f32 FRAME_TIME{1.0 / FPS};

constexpr i32 SCREEN_SIZE_X{1024};
constexpr i32 SCREEN_SIZE_Y{768};
constexpr i32 HALF_SCREEN_SIZE_X{SCREEN_SIZE_X / 2};
constexpr i32 HALF_SCREEN_SIZE_Y{SCREEN_SIZE_Y / 2};
constexpr i32 VIEW_X{HALF_SCREEN_SIZE_X / TILE_SIZE + 1};
constexpr i32 VIEW_Y{HALF_SCREEN_SIZE_Y / TILE_SIZE + 1};

constexpr f32 ASPECT_RATIO{(f32)(SCREEN_SIZE_X / SCREEN_SIZE_Y)};

#endif
