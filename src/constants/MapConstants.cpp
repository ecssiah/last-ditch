#include "MapConstants.h"

const int TILE_SIZE{64};
const int TILES_PER_LAYER{64}; 
const int TILESET_WIDTH{25};
const int TILESET_HEIGHT{15};
const int NUM_FLOORS{6};

const long MAP_SEED{123456};

std::unordered_map<std::string, TileInfo> TileData;
