#include "RenderConstants.h"

#include "MapConstants.h"

const int SCREEN_SIZE_X{1024};
const int SCREEN_SIZE_Y{768};
const int HALF_SCREEN_SIZE_X{SCREEN_SIZE_X / 2};
const int HALF_SCREEN_SIZE_Y{SCREEN_SIZE_Y / 2};
const int VIEW_X{HALF_SCREEN_SIZE_X / TILE_SIZE + 1};
const int VIEW_Y{HALF_SCREEN_SIZE_Y / TILE_SIZE + 1};

const float ASPECT_RATIO{(float)SCREEN_SIZE_X / (float)SCREEN_SIZE_Y};
