#include "../../include/render/RenderConstants.h"

#include "../../include/map/MapConstants.h"

const i32 SCREEN_SIZE_X{1024};
const i32 SCREEN_SIZE_Y{768};
const i32 HALF_SCREEN_SIZE_X{SCREEN_SIZE_X / 2};
const i32 HALF_SCREEN_SIZE_Y{SCREEN_SIZE_Y / 2};
const i32 VIEW_X{HALF_SCREEN_SIZE_X / TILE_SIZE + 1};
const i32 VIEW_Y{HALF_SCREEN_SIZE_Y / TILE_SIZE + 1};

const f32 ASPECT_RATIO{(f32)(SCREEN_SIZE_X / SCREEN_SIZE_Y)};