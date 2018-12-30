#include "MapConstants.h"

const u8 RIGHT{0};
const u8 UP{1};
const u8 LEFT{2};
const u8 DOWN{3};

const i32 TILE_SIZE{64};
const i32 TILES_PER_LAYER{64}; 
const i32 TILESET_WIDTH{25};
const i32 TILESET_HEIGHT{15};
const i32 NUM_FLOORS{6};
const i32 OUTER_PATH{3};
const i32 CENTRAL_PATH{6};

const i32 MAP_SEED{123456};

std::unordered_map<std::string, TileInfo> TileData;
