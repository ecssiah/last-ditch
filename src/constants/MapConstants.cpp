#include "MapConstants.h"

const I32 TILE_SIZE{64};
const I32 TILES_PER_LAYER{64}; 
const I32 TILESET_WIDTH{25};
const I32 TILESET_HEIGHT{15};
const I32 NUM_FLOORS{6};
const I32 OUTER_PATH{3};
const I32 CENTRAL_PATH{6};

const I32 MAP_SEED{123456};

std::unordered_map<std::string, TileInfo> TileData;
