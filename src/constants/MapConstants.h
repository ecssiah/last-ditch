#ifndef MAP_CONSTANTS_H
#define MAP_CONSTANTS_H

#include <string>
#include <unordered_map>

#include "../components/TileInfo.h"

extern const unsigned TILE_SIZE;
extern const unsigned TILES_PER_LAYER;
extern const unsigned TILESET_WIDTH;
extern const unsigned TILESET_HEIGHT;

extern const long MAP_SEED;

extern const unsigned NUM_FLOORS;

extern std::unordered_map<std::string, TileInfo> TileData;

#endif // MAP_CONSTANTS_H

