#ifndef MAP_CONSTANTS_H
#define MAP_CONSTANTS_H

#include <string>
#include <unordered_map>

#include "../Types.h"
#include "../components/info/TileInfo.h"

enum Dirs { RIGHT = 0, UP = 1, LEFT = 2, DOWN = 3 };

extern const i32 TILE_SIZE;
extern const i32 TILES_PER_LAYER;
extern const i32 TILESET_WIDTH;
extern const i32 TILESET_HEIGHT;
extern const i32 OUTER_PATH;
extern const i32 CENTRAL_PATH;
extern const i32 MAP_SEED;
extern const i32 NUM_FLOORS;

extern std::unordered_map<std::string, TileInfo> TileData;

#endif

