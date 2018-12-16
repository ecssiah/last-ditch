#include "MapConstants.h"

const unsigned TILE_SIZE{64};
const unsigned TILES_PER_LAYER{64}; 
const unsigned TILESET_WIDTH{25};
const unsigned TILESET_HEIGHT{15};

const long MAP_SEED{123456};

const unsigned NUM_FLOORS{4};

std::unordered_map<std::string, TileInfo> TileData;
