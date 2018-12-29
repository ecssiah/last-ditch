#ifndef MAP_CONSTANTS_H
#define MAP_CONSTANTS_H

#include <string>
#include <unordered_map>

#include "../Types.h"
#include "../components/info/TileInfo.h"

extern const I32 TILE_SIZE;
extern const I32 TILES_PER_LAYER;
extern const I32 TILESET_WIDTH;
extern const I32 TILESET_HEIGHT;
extern const I32 OUTER_PATH;
extern const I32 CENTRAL_PATH;

extern const I32 MAP_SEED;

extern const I32 NUM_FLOORS;

extern std::unordered_map<std::string, TileInfo> TileData;

#endif

