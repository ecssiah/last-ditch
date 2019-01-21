#include "MapSystem.h"

#include <iostream>
#include <boost/algorithm/string.hpp>

#include "../utility/Logging.h"
#include "../constants/MapConstants.h"
#include "../constants/RenderConstants.h"

using namespace std;

MapSystem::MapSystem(Input& input, Camera& camera, Map& map, Log& log)
  : input_{input}
  , camera_{camera}
  , map_{map}
  , log_{log}
{
}


void 
MapSystem::init()
{
  cout << "MapSystem initializing" << endl;

  srand(MAP_SEED);

  generate_map();
}


void 
MapSystem::update()
{
  if (map_.floor_changed) {
    map_.floor_changed = false;
  }

  if (input_.lclick) {
    input_.lclick = false;
    calculate_selected_tile();
  }

  update_floor();
}


void 
MapSystem::generate_map()
{
  for (auto floor{1}; floor <= NUM_FLOORS; floor++) {
    define_blocks(floor);
    layout_main_floor(floor);
    seed_rooms(floor);
    expand_rooms(floor);
    build_rooms(floor);
    place_doors(floor);
    integrate_walls(floor);
  }
}


void 
MapSystem::update_floor()
{
  if (input_.descend) {
    input_.descend = false;

    if (map_.cur_floor > 1) {
      map_.floor_changed = true;
      map_.cur_floor--;
    }
  }

  if (input_.ascend) {
    input_.ascend = false;

    if (map_.cur_floor < NUM_FLOORS) {
      map_.floor_changed = true;
      map_.cur_floor++; 
    }
  } 
}


void 
MapSystem::define_blocks(i32 floor)
{
  // left edge
  map_.blocks[floor].push_back({
    0, 0, OUTER_PATH, TILES_PER_LAYER - 1
  });
  // right edge
  map_.blocks[floor].push_back({
    TILES_PER_LAYER - OUTER_PATH - 1, 0, OUTER_PATH, TILES_PER_LAYER - 1
  });  
  // top edge 
  map_.blocks[floor].push_back({
    0, 0, TILES_PER_LAYER - 1, OUTER_PATH
  });
  // bottom edge
  map_.blocks[floor].push_back({
    0, TILES_PER_LAYER - OUTER_PATH - 1, TILES_PER_LAYER - 1, OUTER_PATH
  });
  // middle horizontal
  map_.blocks[floor].push_back({
    0, TILES_PER_LAYER / 2 - CENTRAL_PATH / 2 - 1, 
    TILES_PER_LAYER - 1, CENTRAL_PATH + 1
  });
  // middle vertical
  map_.blocks[floor].push_back({
    TILES_PER_LAYER / 2 - CENTRAL_PATH / 2 - 1, 0, 
    CENTRAL_PATH + 1, TILES_PER_LAYER - 1
  });
}


void 
MapSystem::layout_main_floor(i32 floor)
{
  string floor_type;

  switch (get_section(floor)) {
  case 1: floor_type = "dark_concrete"; break;
  case 2: floor_type = "smooth_dark_concrete"; break;
  case 3: floor_type = "bright_dark_concrete"; break;
  }

  for (auto x{0}; x < TILES_PER_LAYER; x++) {
    for (auto y{0}; y < TILES_PER_LAYER; y++) {
      set_tile("flr", x, y, floor, floor_type);
    }
  }
}


void 
MapSystem::seed_rooms(i32 floor)
{
  for (auto i{0}; i < NUM_ROOMS; i++) {
    Room test_room;
    bool collision{true};

    switch (get_section(floor)) {
    case 1:
      test_room.wall_type = "wall1"; 
      test_room.floor_type = "light_concrete"; 
      break;
    case 2:
      test_room.wall_type = "wall2"; 
      test_room.floor_type = "smooth_light_concrete"; 
      break;
    case 3:
      test_room.wall_type = "wall3"; 
      test_room.floor_type = "bright_light_concrete"; 
      break;
    }

    do {
      test_room.rect.x = rand() % (TILES_PER_LAYER - 1);
      test_room.rect.y = rand() % (TILES_PER_LAYER - 1);
      test_room.rect.w = 2;
      test_room.rect.h = 2;
    } 
    while (room_collision(floor, test_room));

    map_.rooms[floor].push_back(test_room);
  }
}


void 
MapSystem::expand_rooms(i32 floor)
{
  if (RANDOM_ROOMS) srand(time(nullptr));

  for (auto i{0}; i < EXPANSION_ITERATIONS; i++) {
    bool found{false};
    vector<Dir> dirs{Dir::UP, Dir::DOWN, Dir::LEFT, Dir::RIGHT}; 

    u64 random_room_index{rand() % map_.rooms[floor].size()};

    Room& room{map_.rooms[floor][random_room_index]}; 

    while (!found && !dirs.empty()) {
      const Dir dir{rand_dir()};
      dirs.erase(remove(dirs.begin(), dirs.end(), dir), dirs.end());

      switch (dir) {
      case Dir::UP:    room.rect.y--; break;
      case Dir::DOWN:  room.rect.h++; break;
      case Dir::LEFT:  room.rect.x--; break;
      case Dir::RIGHT: room.rect.w++; break;
      }

      if (room_collision(floor, room)) {
        switch (dir) {
        case Dir::UP:    room.rect.y++; break;
        case Dir::DOWN:  room.rect.h--; break;
        case Dir::LEFT:  room.rect.x++; break;
        case Dir::RIGHT: room.rect.w--; break;
        }
      } else {
        found = true;
      }
    }
  }  
}


void 
MapSystem::build_rooms(i32 floor)
{
  for (const auto& room : map_.rooms[floor]) {
    for (auto x{room.l()}; x <= room.r(); x++) {
      for (auto y{room.t()}; y <= room.b(); y++) {
        set_tile("flr", x, y, floor, room.floor_type);
      }
    }

    for (auto x{room.l()}; x <= room.r(); x++) {
      set_solid(x, room.t(), floor);
      set_solid(x, room.b(), floor);
      set_tile("wal", x, room.t(), floor, room.wall_type + "-str"); 
      set_tile("wal", x, room.b(), floor, room.wall_type + "-str");
    }

    for (auto y{room.t() + 1}; y <= room.b(); y++) {
      set_solid(room.l(), y, floor);
      set_solid(room.r(), y, floor);
      set_tile("wal", room.l(), y, floor, room.wall_type + "-str", 90); 
      set_tile("wal", room.r(), y, floor, room.wall_type + "-str", 90);
    }
  }

  cout << " Floor " << floor << " rooms built" << endl;
}


void 
MapSystem::integrate_walls(i32 floor)
{
  const auto& tiles{map_.floors[floor].layers["wal"].tiles};

  for (auto x{OUTER_PATH}; x < TILES_PER_LAYER - OUTER_PATH; x++) {
    for (auto y{OUTER_PATH}; y < TILES_PER_LAYER - OUTER_PATH; y++) {
      const Tile& tile{tiles[x][y]};

      if (tile.category == "walls") {
        const Tile& utile{tiles[x + 0][y - 1]};
        const Tile& dtile{tiles[x + 0][y + 1]};
        const Tile& ltile{tiles[x - 1][y + 0]};
        const Tile& rtile{tiles[x + 1][y + 0]};

        const bool umatch{tile.type == utile.type};
        const bool rmatch{tile.type == rtile.type};
        const bool dmatch{tile.type == dtile.type};
        const bool lmatch{tile.type == ltile.type};

        if (umatch && lmatch && dmatch && rmatch) {
          set_tile("wal", x, y, floor, tile.type + "-int");
        } else if (umatch && rmatch && dmatch) {
          set_tile("wal", x, y, floor, tile.type + "-tee");
        } else if (rmatch && dmatch && lmatch) {
          set_tile("wal", x, y, floor, tile.type + "-tee", 90);
        } else if (dmatch && lmatch && umatch) {
          set_tile("wal", x, y, floor, tile.type + "-tee", 180);
        } else if (lmatch && umatch && rmatch) {
          set_tile("wal", x, y, floor, tile.type + "-tee", 270);
        } else if (umatch && rmatch) {
          set_tile("wal", x, y, floor, tile.type + "-cor");
        } else if (rmatch && dmatch) {
          set_tile("wal", x, y, floor, tile.type + "-cor", 90);
        } else if (dmatch && lmatch) {
          set_tile("wal", x, y, floor, tile.type + "-cor", 180);
        } else if (lmatch && umatch) {
          set_tile("wal", x, y, floor, tile.type + "-cor", 270);
        } else if (lmatch && rmatch) {
          set_tile("wal", x, y, floor, tile.type + "-str");
        } else if (umatch && dmatch) {
          set_tile("wal", x, y, floor, tile.type + "-str", 90);
        } else if (umatch) {
          set_tile("wal", x, y, floor, tile.type + "-end");
        } else if (rmatch) {
          set_tile("wal", x, y, floor, tile.type + "-end", 90);
        } else if (dmatch) {
          set_tile("wal", x, y, floor, tile.type + "-end", 180);
        } else if (lmatch) {
          set_tile("wal", x, y, floor, tile.type + "-end", 270);
        } else {
          set_tile("wal", x, y, floor, tile.type + "-one");
        }
      }
    }
  }

  cout << " Floor " << floor << " rooms integrated" << endl;
}


void 
MapSystem::place_doors(i32 floor)
{
  for (auto& room : map_.rooms[floor]) {
    u8 count{0};
    bool found{false};

    while (!found && count++ < 40) {
      const auto horz_range{room.w() - 1};
      const auto horz_start{room.l() + 1};
      const auto vert_range{room.h() - 1};
      const auto vert_start{room.t() + 1};

      i32 rot, x, y;
      const Dir dir{rand_dir()}; 

      if (dir == Dir::UP) {
        rot = 0;
        x = horz_start + rand() % horz_range;
        y = room.t(); 
      } else if (dir == Dir::DOWN) {
        rot = 90;
        x = room.r();
        y = vert_start + rand() % vert_range;
      } else if (dir == Dir::RIGHT) {
        rot = 180;
        x = horz_start + rand() % horz_range;
        y = room.b();
      } else if (dir == Dir::LEFT) {
        rot = 270;
        x = room.l();
        y = vert_start + rand() % vert_range;
      }

      if (has_clearance("door", x, y, floor, dir)) {
        found = true;
        
        if (rand() % 2 == 0) {
          set_solid(x, y, floor);
          set_tile("wal", x, y, floor, "door1-cls", rot);
        } else {
          set_solid(x, y, floor, false);
          set_tile("wal", x, y, floor, "door1-opn", rot);
        }
      }
    }
  }

  cout << " Floor " << floor << " doors placed" << endl;
}


void 
MapSystem::calculate_selected_tile()
{
  f32 screenx{(input_.mx - HALF_SCREEN_SIZE_X) / (f32)TILE_SIZE / camera_.zoom};
  f32 screeny{(input_.my - HALF_SCREEN_SIZE_Y) / (f32)TILE_SIZE / camera_.zoom};

  i32 targetx{static_cast<i32>(floor(screenx + camera_.pos.x))};
  i32 targety{static_cast<i32>(floor(screeny + camera_.pos.y))};

  if (select_tile(targetx, targety)) {
    string msg{"Selected: ["};
    msg += to_string(input_.selectx) + "," + to_string(input_.selecty) + ",";
    msg += to_string(input_.selectfloor) + "]"; 

    ::msg(log_, msg);
  } else {
    ::msg(log_, "Selected: [invalid]");
  }
}


const bool 
MapSystem::select_tile(i32 x, i32 y)
{
  const auto x_in_bounds{x >= 0 && x <= TILES_PER_LAYER - 1};
  const auto y_in_bounds{y >= 0 && y <= TILES_PER_LAYER - 1}; 

  if (x_in_bounds && y_in_bounds) {
    clear_selection();

    input_.selectx = x;
    input_.selecty = y;
    input_.selectfloor = map_.cur_floor;

    set_tile("ovr", x, y, map_.cur_floor, "select");

    return true;
  } else {
    return false;
  }
}


void 
MapSystem::clear_selection() 
{
  if (input_.selectx == -1 && input_.selecty == -1) {
    return;
  }

  set_active(
    "ovr", input_.selectx, input_.selecty, input_.selectfloor, false
  );
}


const bool 
MapSystem::room_collision(i32 floor, const Room& test_room) 
const 
{
  for (const auto& room : map_.blocks[floor]) {
    if (SDL_HasIntersection(&room.rect, &test_room.rect)) {
      return true;
    }
  }

  for (const auto& room : map_.rooms[floor]) {
    const auto intersection{SDL_HasIntersection(&room.rect, &test_room.rect)};

    if (&room.rect != &test_room.rect && intersection) {
      return true;
    }
  }

  return false;
}


const bool 
MapSystem::has_clearance(
  const string& category, i32 x, i32 y, i32 floor, Dir dir
) 
const 
{
  i8 dx1, dy1;
  i8 dx2, dy2;
  i8 dx3, dy3;

  if (dir == Dir::UP) {
    dx1 =  0; dy1 = -1;
    dx2 = -1; dy2 =  0;
    dx3 =  1; dy3 =  0;
  } else if (dir == Dir::RIGHT) {
    dx1 =  1; dy1 =  0;
    dx2 =  0; dy2 = -1;
    dx3 =  0; dy3 =  1;
  } else if (dir == Dir::DOWN) {
    dx1 =  0; dy1 =  1;
    dx2 =  1; dy2 =  0;
    dx3 = -1; dy3 =  0;
  } else if (dir == Dir::LEFT) {
    dx1 = -1; dy1 =  0;
    dx2 =  0; dy2 =  1;
    dx3 =  0; dy3 = -1;
  }

  const auto& tiles{map_.floors[floor].layers["wal"].tiles};

  const auto front_clear{tiles[x + dx1][y + dy1].type == ""};
  const auto left_clear{tiles[x + dx2][y + dy2].category != "door"};
  const auto right_clear{tiles[x + dx3][y + dy3].category != "door"};

  return front_clear && left_clear && right_clear;
}


void 
MapSystem::set_tile(
  const string& layer, i32 x, i32 y, i32 floor, const string& type, 
  f32 rot, SDL_RendererFlip flip
) {
  Tile& tile{map_.floors[floor].layers[layer].tiles[x][y]};

  tile.active = true;
  tile.rot = rot;
  tile.flip = flip;

  if (map_.tile_data.find(type) != map_.tile_data.end()) {
    vector<string> type_vector; 
    boost::split(type_vector, type, boost::is_any_of("-"));

    tile.type = type_vector[0];
    tile.subtype = type_vector.size() <= 1 ? "" : type_vector[1];
    tile.category = map_.tile_data[type].category;
    tile.src.x = map_.tile_data[type].uv.x * TILE_SIZE;
    tile.src.y = map_.tile_data[type].uv.y * TILE_SIZE;
  } else {
    tile.src.x = 0;
    tile.src.y = 0;

    cerr << "Tile(" << x << "," << y << ") has invalid type: ";
    cerr << tile.type << endl;
  }
}


void 
MapSystem::set_solid(i32 x, i32 y, i32 floor, bool solid)
{
  Tile& tile{map_.floors[floor].layers["wal"].tiles[x][y]};
  tile.solid = solid;
}


void 
MapSystem::set_active(
  const std::string& layer, i32 x, i32 y, i32 floor, bool active
) {
  Tile& tile{map_.floors[floor].layers[layer].tiles[x][y]};
  tile.active = active;
}


const i32 
MapSystem::get_section(i32 floor) 
const
{
  if (floor > 2 * NUM_FLOORS / 3 && floor <= NUM_FLOORS) {
    return 3;
  } else if (floor > 1 * NUM_FLOORS / 3) {
    return 2;
  } else if (floor > 0 * NUM_FLOORS / 3) {
    return 1;
  } else {
    return -1;
  }
}

