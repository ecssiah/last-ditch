#include "MapGenerator.h"

#include <cstdlib>
#include <ctime>
#include <algorithm>
#include <boost/algorithm/string.hpp>

#include "../constants/MapConstants.h"
#include "../components/map/Room.h"

using namespace std;

MapGenerator::MapGenerator(Map& map)
  : map_{map}
  , rooms_{(size_t)NUM_FLOORS, vector<Room>()}
  , blocked_rooms_{(size_t)NUM_FLOORS, vector<Room>()}
  , num_rooms_{60}
  , expansion_iterations_{20000}
  , show_grid_{false}
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
    finish_rooms(floor);
  }
}


void MapGenerator::layout_main_floor(unsigned floor)
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

      if (show_grid_) {
        set_tile("overlay", x, y, floor, "selection"); 
      }
    }
  }
}


void MapGenerator::seed_rooms(unsigned floor)
{
  for (auto i{0}; i < num_rooms_; i++) {
    bool collision{true};
    string floor_type, wall_type;

    if (floor + 1 > 2 * NUM_FLOORS / 3) {
      floor_type = "bright_light_concrete";
      wall_type = "wall3";
    } else if (floor + 1 > NUM_FLOORS / 3) {
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
      test_room.l = 3 + rand() % (TILES_PER_LAYER - 8);
      test_room.t = 3 + rand() % (TILES_PER_LAYER - 8);
      test_room.r = test_room.l + 2;
      test_room.b = test_room.t + 2;

      collision = room_collision(floor, test_room);
    }

    rooms_[floor].push_back(test_room);
  }
}


void MapGenerator::expand_rooms(unsigned floor)
{
  /* Randomize room expansion */
  /* srand(time(nullptr)); */

  for (auto i{0}; i < expansion_iterations_; i++) {
    bool found{false};
    vector<int> dirs{0, 1, 2, 3}; 

    Room& room{rooms_[floor][rand() % rooms_[floor].size()]}; 

    while (!found && dirs.size() > 0) {
      int choice{dirs[(int)(rand() % dirs.size())]};

      switch (choice) {
        case 0: room.l--; break;
        case 1: room.r++; break;
        case 2: room.t--; break;
        case 3: room.b++; break;
      }

      if (room_collision(floor, room)) {
        dirs.erase(remove(dirs.begin(), dirs.end(), choice), dirs.end());

        switch (choice) {
          case 0: room.l++; break;
          case 1: room.r--; break;
          case 2: room.t++; break;
          case 3: room.b--; break;
        }
      } else {
        found = true;
      }
    }
  }   
}


void MapGenerator::build_rooms(unsigned floor)
{
  for (const auto& room : rooms_[floor]) {
    for (auto x{room.l}; x <= room.r; x++) {
      for (auto y{room.t}; y <= room.b; y++) {
        set_tile("floor", x, y, floor, room.floor_type);
      }
    }

    for (auto x{room.l}; x <= room.r; x++) {
      set_tile("wall", x, room.t, floor, room.wall_type + "-str"); 
      set_tile("wall", x, room.b, floor, room.wall_type + "-str");
      set_solid(x, room.t, floor, true);
      set_solid(x, room.b, floor, true);
    }

    for (auto y{room.t + 1}; y <= room.b - 1; y++) {
      set_tile("wall", room.l, y, floor, room.wall_type + "-str", 90); 
      set_tile("wall", room.r, y, floor, room.wall_type + "-str", 90);
      set_solid(room.l, y, floor, true);
      set_solid(room.r, y, floor, true);
    }
  } 

  cout << "Floor " << floor + 1 << " rooms built" << endl;
}


void MapGenerator::finish_rooms(unsigned floor)
{
  place_doors(floor);
  integrate_walls(floor);
}


void MapGenerator::integrate_walls(unsigned floor)
{
  for (auto x{3}; x < TILES_PER_LAYER - 3; x++) {
    for (auto y{3}; y < TILES_PER_LAYER - 3; y++) {
      Tile& tile{map_.floors[floor].layers["wall"].tiles[x][y]};

      if (tile.category == "wall") {
        Tile& utile{map_.floors[floor].layers["wall"].tiles[x][y - 1]};
        Tile& dtile{map_.floors[floor].layers["wall"].tiles[x][y + 1]};
        Tile& ltile{map_.floors[floor].layers["wall"].tiles[x - 1][y]};
        Tile& rtile{map_.floors[floor].layers["wall"].tiles[x + 1][y]};

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


bool MapGenerator::check_clearance(
  const string& category, 
  unsigned x, unsigned y, unsigned floor, unsigned direction
) {
  auto& tiles{map_.floors[floor].layers["wall"].tiles};

  unsigned place;
  unsigned dx1, dx2, dx3;
  unsigned dy1, dy2, dy3;

  if (direction == 0) {
    dx1 = 0; dy1 = -1;
    dx2 = -1; dy2 = 0;
    dx3 = 1; dy3 = 0;
  } else if (direction == 1) {
    dx1 = 1; dy1 = 0;
    dx2 = 0; dy2 = -1;
    dx3 = 0; dy3 = 1;
  } else if (direction == 2) {
    dx1 = 0; dy1 = 1;
    dx2 = 1; dy2 = 0;
    dx3 = -1; dy3 = 0;
  } else if (direction == 3) {
    dx1 = -1; dy1 = 0;
    dx2 = 0; dy2 = 1;
    dx3 = 0; dy3 = -1;
  }

  auto place_free{tiles[x + dx1][y + dy1].type == ""};
  auto clear_left{tiles[x + dx2][y + dy2].category != "door"};
  auto clear_right{tiles[x + dx3][y + dy3].category != "door"};

  return place_free && clear_left && clear_right;
}


void MapGenerator::place_doors(unsigned floor)
{
  for (auto& room : rooms_[floor]) {
    unsigned count{0};
    bool found{false};

    while (!found && count++ < 40) {
      auto choice{rand() % 4}; 
      string door_type{"door1-cls"};

      if (choice == 0) {
        auto place{(rand() % (room.r - room.l - 1)) + room.l + 1};

        if (check_clearance("door", place, room.t, floor, choice)) {
          found = true;
          set_tile("wall", place, room.t, floor, door_type);
          set_solid(place, room.t, floor, true);
        }
      } else if (choice == 1) {
        auto place{(rand() % (room.b - room.t - 1)) + room.t + 1};

        if (check_clearance("door", room.r, place, floor, choice)) {
          found = true;
          set_tile("wall", room.r, place, floor, door_type, 90);
          set_solid(room.r, place, floor, true);
        }
      } else if (choice == 2) {
        auto place{(rand() % (room.r - room.l - 1)) + room.l + 1};
        
        if (check_clearance("door", place, room.b, floor, choice)) {
          found = true;
          set_tile("wall", place, room.b, floor, door_type);
          set_solid(place, room.b, floor, true);
        }
      } else if (choice == 3) {
        auto place{(rand() % (room.b - room.t - 1)) + room.t + 1};

        if (check_clearance("door", room.l, place, floor, choice)) {
          found = true;
          set_tile("wall", room.l, place, floor, door_type, 90);
          set_solid(room.l, place, floor, true);
        }
      }
    }
  }

  cout << "Floor " << floor + 1 << " doors placed" << endl;
}


bool MapGenerator::intersects(
  const Room& r1, unsigned l, unsigned r, unsigned t, unsigned b
) {
  auto lr_check{r1.l < r && r1.r > l};
  auto tb_check{r1.t < b && r1.b > t};

  return lr_check && tb_check ? true : false;
}


bool MapGenerator::intersects(const Room& r1, const Room& r2)
{
  return intersects(r1, r2.l, r2.r, r2.t, r2.b);
}


bool MapGenerator::room_collision(unsigned floor, const Room& test_room) 
{
  for (const auto& room : blocked_rooms_[floor]) 
    if (intersects(room, test_room)) return true;

  for (const auto& room : rooms_[floor]) 
    if (room != test_room && intersects(room, test_room)) return true;

  return false;
}


void MapGenerator::define_blocked_rooms(unsigned floor)
{
  blocked_rooms_[floor].push_back({
    0, 3, 0, TILES_PER_LAYER - 1
  });  
  blocked_rooms_[floor].push_back({
    TILES_PER_LAYER - 4, TILES_PER_LAYER - 1, 0, TILES_PER_LAYER - 1
  });  
  blocked_rooms_[floor].push_back({
    0, TILES_PER_LAYER - 1, 0, 3
  });  
  blocked_rooms_[floor].push_back({
    0, TILES_PER_LAYER - 1, TILES_PER_LAYER - 4, TILES_PER_LAYER - 1
  });  
  blocked_rooms_[floor].push_back({
    TILES_PER_LAYER / 2 - 4, TILES_PER_LAYER / 2 + 3, 0, TILES_PER_LAYER - 1
  });
  blocked_rooms_[floor].push_back({
    0, TILES_PER_LAYER - 1, TILES_PER_LAYER / 2 - 4, TILES_PER_LAYER / 2 + 3
  });
}


void MapGenerator::set_tile(
  const string& layer, 
  int x, int y, int floor, 
  const string& full_type, float rotation, SDL_RendererFlip flip
) {
  Tile& tile{map_.floors[floor].layers[layer].tiles[x][y]};

  if (TileData.find(full_type) != TileData.end()) {
    vector<string> type_vector; 
    boost::split(type_vector, full_type, boost::is_any_of("-"));

    tile.active = true;
    tile.type = type_vector[0];
    tile.subtype = type_vector.size() > 1 ? type_vector[1] : "";
    tile.category = TileData[full_type].category;
    tile.rotation = rotation;
    tile.flip = flip;

    tile.src.x = TileData[full_type].uv.x * TILE_SIZE;  
    tile.src.y = TileData[full_type].uv.y * TILE_SIZE;
  } else {
    cerr << "Tile(" << x << "," << y << ") has invalid type: "; 
    cerr << full_type << endl; 
  }
}


void MapGenerator::set_solid(int x, int y, int floor, bool solid)
{
  Tile& tile{map_.floors[floor].layers["wall"].tiles[x][y]};
  tile.solid = solid;
}

