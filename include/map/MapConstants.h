#ifndef MAP_CONSTANTS_H
#define MAP_CONSTANTS_H

#include <string>
#include <unordered_map>

#include "../utility/Types.h"
#include "TileInfo.h"

enum Dir { RIGHT, UP, LEFT, DOWN };
enum Section { LOW, MID, TOP };

constexpr i32 TILE_SIZE{64};
constexpr i32 TILES_PER_LAYER{64}; 
constexpr i32 TILESET_WIDTH{25};
constexpr i32 TILESET_HEIGHT{15};
constexpr i32 NUM_FLOORS{6};
constexpr i32 OUTER_PATH{3};
constexpr i32 CENTRAL_PATH{6};
constexpr i32 MAP_SEED{123456};

#endif
