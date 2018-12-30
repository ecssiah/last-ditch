#ifndef MAP_CONSTANTS_H
#define MAP_CONSTANTS_H

#include <string>
#include <unordered_map>

#include "../Types.h"
#include "../components/info/TileInfo.h"

extern const u8 RIGHT;
extern const u8 UP;
extern const u8 LEFT;
extern const u8 DOWN;

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

