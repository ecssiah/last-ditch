#include "../../include/map/MapGenerator.h"

#include <ctime>
#include <cstdlib>
#include <algorithm>
#include <boost/algorithm/string.hpp>

#include "../../include/map/MapConstants.h"
#include "../../include/map/Room.h"

using namespace std;

MapGenerator::MapGenerator(Map& map)
  : map_{map}
  , rooms_{(u16)NUM_FLOORS, vector<Room>()}
  , blocked_rooms_{(u16)NUM_FLOORS, vector<Room>()}
  , num_rooms_{60}
  , expansion_iterations_{20000}
  , show_grid_{false}
  , randomize_rooms_{false}
{
  srand(MAP_SEED);
}


void MapGenerator::generate_map()
{
  for (auto floor{0}; floor < NUM_FLOORS; floor++) {
    define_blocked_rooms(floor);
    layout_main_floor(floor);
    seed_rooms(floor);
    expand_rooms(floor);
    build_rooms(floor);
    place_doors(floor);
    integrate_walls(floor);
  }
}


void MapGenerator::layout_main_floor(i32 floor)
{
  string floor_type;
  if (floor + 1 > 2 * NUM_FLOORS / 3) {
    floor_type = "bright_dark_concrete";
  } else if (floor + 1 > NUM_FLOORS / 3) {
    floor_type = "smooth_dark_concrete"; 
  } else {
    floor_type = "dark_concrete";
  }

  for (auto x{0}; x < TILES_PER_LAYER; x++) { 
    for (auto y{0}; y < TILES_PER_LAYER; y++) {
      set_tile("floor", x, y, floor, floor_type);

      if (show_grid_) set_tile("overlay", x, y, floor, "selection");
    }
  }
}


void MapGenerator::seed_rooms(i32 floor)
{
  for (auto i{0}; i < num_rooms_; i++) {
    bool collision{true};
    string floor_type, wall_type;

    if (floor + 1 > 2 * NUM_FLOORS / 3) {
      floor_type = "bright_light_concrete";
      wall_type = "wall3";
    } else if (floor + 1 > 1 * NUM_FLOORS / 3) {
      floor_type = "smooth_light_concrete";
      wall_type = "wall2";
    } else {
      floor_type = "light_concrete";
      wall_type = "wall1";
    }

    Room test_room;
    test_room.floor_type = floor_type;
    test_room.wall_type = wall_type;

    while (collision) {
      test_room.rect.x = rand() % (TILES_PER_LAYER - 1);
      test_room.rect.y = rand() % (TILES_PER_LAYER - 1);
      test_room.rect.w = 2;
      test_room.rect.h = 2;

      collision = room_collision(floor, test_room);
    }

    rooms_[floor].push_back(test_room);
  }
}


void MapGenerator::expand_rooms(i32 floor)
{
  if (randomize_rooms_) srand(time(nullptr));

  for (auto i{0}; i < expansion_iterations_; i++) {
    bool found{false};
    vector<Dirs> dirs; 

    Room& room{rooms_[floor][rand() % rooms_[floor].size()]}; 

    while (!found && dirs.size() < 4) {
      const Dirs dir{static_cast<Dirs>(rand() % 4)};

      switch (dir) {
      case UP:    room.rect.y--; break;
      case DOWN:  room.rect.h++; break;
      case LEFT:  room.rect.x--; break;
      case RIGHT: room.rect.w++; break;
      }

      if (room_collision(floor, room)) {
        dirs.push_back(dir);

        switch (dir) {
        case UP:    room.rect.y++; break;
        case DOWN:  room.rect.h--; break;
        case LEFT:  room.rect.x++; break;
        case RIGHT: room.rect.w--; break;
        }
      } else {
        found = true;
      }
    }
  }  
}


void MapGenerator::build_rooms(i32 floor)
{
  for (const auto& room : rooms_[floor]) {
    for (auto x{room.l()}; x <= room.r(); x++) {
      for (auto y{room.t()}; y <= room.b(); y++) {
        set_tile("floor", x, y, floor, room.floor_type);
      }
    }

    for (auto x{room.l()}; x <= room.r(); x++) {
      set_solid(x, room.t(), floor, true);
      set_solid(x, room.b(), floor, true);
      set_tile("wall", x, room.t(), floor, room.wall_type + "-str"); 
      set_tile("wall", x, room.b(), floor, room.wall_type + "-str");
    }

    for (auto y{room.t() + 1}; y <= room.b(); y++) {
      set_solid(room.l(), y, floor, true);
      set_solid(room.r(), y, floor, true);
      set_tile("wall", room.l(), y, floor, room.wall_type + "-str", 90); 
      set_tile("wall", room.r(), y, floor, room.wall_type + "-str", 90);
    }
  }

  cout << "Floor " << floor + 1 << " rooms built" << endl;
}


void MapGenerator::integrate_walls(i32 floor)
{
  for (auto x{OUTER_PATH}; x < TILES_PER_LAYER - OUTER_PATH; x++) {
    for (auto y{OUTER_PATH}; y < TILES_PER_LAYER - OUTER_PATH; y++) {
      const auto& tiles{map_.floors[floor].layers["wall"].tiles};
      const Tile& tile{tiles[x][y]};

      if (tile.category == "wall") {
        const Tile& utile{tiles[x][y - 1]};
        const Tile& dtile{tiles[x][y + 1]};
        const Tile& ltile{tiles[x - 1][y]};
        const Tile& rtile{tiles[x + 1][y]};

        bool umatch{tile.type == utile.type};
        bool rmatch{tile.type == rtile.type};
        bool dmatch{tile.type == dtile.type};
        bool lmatch{tile.type == ltile.type};

        if (umatch && lmatch && dmatch && rmatch) {
          set_tile("wall", x, y, floor, tile.type + "-int");
        } else if (umatch && rmatch && dmatch) {
          set_tile("wall", x, y, floor, tile.type + "-tee");
        } else if (rmatch && dmatch && lmatch) {
          set_tile("wall", x, y, floor, tile.type + "-tee", 90);
        } else if (dmatch && lmatch && umatch) {
          set_tile("wall", x, y, floor, tile.type + "-tee", 180);
        } else if (lmatch && umatch && rmatch) {
          set_tile("wall", x, y, floor, tile.type + "-tee", 270);
        } else if (umatch && rmatch) {
          set_tile("wall", x, y, floor, tile.type + "-cor");
        } else if (rmatch && dmatch) {
          set_tile("wall", x, y, floor, tile.type + "-cor", 90);
        } else if (dmatch && lmatch) {
          set_tile("wall", x, y, floor, tile.type + "-cor", 180);
        } else if (lmatch && umatch) {
          set_tile("wall", x, y, floor, tile.type + "-cor", 270);
        } else if (lmatch && rmatch) {
          set_tile("wall", x, y, floor, tile.type + "-str");
        } else if (umatch && dmatch) {
          set_tile("wall", x, y, floor, tile.type + "-str", 90);
        } else if (umatch) {
          set_tile("wall", x, y, floor, tile.type + "-end");
        } else if (rmatch) {
          set_tile("wall", x, y, floor, tile.type + "-end", 90);
        } else if (dmatch) {
          set_tile("wall", x, y, floor, tile.type + "-end", 180);
        } else if (lmatch) {
          set_tile("wall", x, y, floor, tile.type + "-end", 270);
        } else {
          set_tile("wall", x, y, floor, tile.type + "-one");
        }
      }
    }
  }

  cout << "Floor " << floor + 1 << " rooms integrated" << endl;
}


bool MapGenerator::has_clearance(
  const string& category, i32 x, i32 y, i32 floor, Dirs dir
) {
  i8 dx1, dy1;
  i8 dx2, dy2;
  i8 dx3, dy3;

  if (dir == UP) {
    dx1 =  0; dy1 = -1;
    dx2 = -1; dy2 =  0;
    dx3 =  1; dy3 =  0;
  } else if (dir == RIGHT) {
    dx1 =  1; dy1 =  0;
    dx2 =  0; dy2 = -1;
    dx3 =  0; dy3 =  1;
  } else if (dir == DOWN) {
    dx1 =  0; dy1 =  1;
    dx2 =  1; dy2 =  0;
    dx3 = -1; dy3 =  0;
  } else if (dir == LEFT) {
    dx1 = -1; dy1 =  0;
    dx2 =  0; dy2 =  1;
    dx3 =  0; dy3 = -1;
  }

  const auto& tiles{map_.floors[floor].layers["wall"].tiles};

  const auto place_free{tiles[x + dx1][y + dy1].type == ""};
  const auto clear_left{tiles[x + dx2][y + dy2].category != "door"};
  const auto clear_right{tiles[x + dx3][y + dy3].category != "door"};

  return place_free && clear_left && clear_right;
}


void MapGenerator::place_doors(i32 floor)
{
  for (auto& room : rooms_[floor]) {
    u8 count{0};
    bool found{false};

    while (!found && count++ < 40) {
      string door_type{"door1-cls"};
      const Dirs dir{static_cast<Dirs>(rand() % 4)}; 

      const auto ud_range{room.w() - 1};
      const auto lr_range{room.h() - 1};

      if (dir == UP) {
        const auto place{room.l() + 1 + rand() % ud_range};

        if (has_clearance("door", place, room.t(), floor, dir)) {
          found = true;
          set_tile("wall", place, room.t(), floor, door_type);
          set_solid(place, room.t(), floor, true);
        }
      } else if (dir == RIGHT) {
        const auto place{room.t() + 1 + rand() % lr_range};

        if (has_clearance("door", room.r(), place, floor, dir)) {
          found = true;
          set_tile("wall", room.r(), place, floor, door_type, 90);
          set_solid(room.l(), place, floor, true);
        }
      } else if (dir == DOWN) {
        const auto place{room.l() + 1 + rand() % ud_range};
        
        if (has_clearance("door", place, room.b(), floor, dir)) {
          found = true;
          set_tile("wall", place, room.b(), floor, door_type);
          set_solid(place, room.b(), floor, true);
        }
      } else if (dir == LEFT) {
        const auto place{room.t() + 1 + rand() % lr_range};

        if (has_clearance("door", room.l(), place, floor, dir)) {
          found = true;
          set_tile("wall", room.l(), place, floor, door_type, 90);
          set_solid(room.l(), place, floor, true);
        }
      }
    }
  }

  cout << "Floor " << floor + 1 << " doors placed" << endl;
}


bool MapGenerator::room_collision(i32 floor, const Room& test_room) 
{
  for (const auto& room : blocked_rooms_[floor]) 
    if (SDL_HasIntersection(&room.rect, &test_room.rect)) return true;

  for (const auto& room : rooms_[floor]) {
    const auto intersection{SDL_HasIntersection(&room.rect, &test_room.rect)};
    if (&room.rect != &test_room.rect && intersection) return true;
  }

  return false;
}


void MapGenerator::define_blocked_rooms(i32 floor)
{
  // left edge
  blocked_rooms_[floor].push_back({
    0, 0, OUTER_PATH, TILES_PER_LAYER - 1
  });
  // right edge
  blocked_rooms_[floor].push_back({
    TILES_PER_LAYER - OUTER_PATH - 1, 0, OUTER_PATH, TILES_PER_LAYER - 1
  });  
  // top edge 
  blocked_rooms_[floor].push_back({
    0, 0, TILES_PER_LAYER - 1, OUTER_PATH
  });
  // bottom edge
  blocked_rooms_[floor].push_back({
    0, TILES_PER_LAYER - OUTER_PATH - 1, TILES_PER_LAYER - 1, OUTER_PATH
  });
  // middle horizontal
  blocked_rooms_[floor].push_back({
    0, TILES_PER_LAYER / 2 - OUTER_PATH - 1, 
    TILES_PER_LAYER - 1, CENTRAL_PATH + 1
  });
  // middle vertical
  blocked_rooms_[floor].push_back({
    TILES_PER_LAYER / 2 - OUTER_PATH - 1, 0, 
    CENTRAL_PATH + 1, TILES_PER_LAYER - 1
  });
}


void MapGenerator::set_tile(
  const string& layer, i32 x, i32 y, i32 floor, const string& type, 
  f32 rotation, SDL_RendererFlip flip
) {
  Tile& tile{map_.floors[floor].layers[layer].tiles[x][y]};

  if (TileData.find(type) != TileData.end()) {
    vector<string> type_vector; 
    boost::split(type_vector, type, boost::is_any_of("-"));

    tile.active = true;
    tile.type = type_vector[0];
    tile.subtype = type_vector.size() <= 1 ? "" : type_vector[1];
    tile.category = TileData[type].category;
    tile.rotation = rotation;
    tile.flip = flip;

    tile.src.x = TileData[type].uv.x * TILE_SIZE;  
    tile.src.y = TileData[type].uv.y * TILE_SIZE;
  } else {
    cerr << "Tile(" << x << "," << y << ") has invalid type: "; 
    cerr << type << endl; 
  }
}


void MapGenerator::set_solid(i32 x, i32 y, i32 floor, bool solid)
{
  Tile& tile{map_.floors[floor].layers["wall"].tiles[x][y]};
  tile.solid = solid;
}
