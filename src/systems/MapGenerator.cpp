#include "MapGenerator.h"

#include <cstdlib>
#include <ctime>
#include <algorithm>
#include <boost/algorithm/string.hpp>

#include "../components/Room.h"
#include "../constants/MapConstants.h"

using namespace std;

MapGenerator::MapGenerator(Map& map)
  : map_(map)
  , rooms_(NUM_FLOORS, vector<Room>())
  , blocked_rooms_(NUM_FLOORS, vector<Room>())
  , num_rooms_(100)
  , expansion_iterations_(1200)
{
  srand(MAP_SEED);

  for (auto floor{0}; floor < NUM_FLOORS; ++floor) DefineBlockedRooms(floor);
}

void MapGenerator::GenerateMap(string name)
{
  SetTile(
    "entity", TILES_PER_LAYER / 2, TILES_PER_LAYER / 2, 0, "test_character1"
  );

  for (auto floor{0}; floor < NUM_FLOORS; ++floor) {
    for (auto x{0}; x < TILES_PER_LAYER; ++x) { 
      for (auto y{0}; y < TILES_PER_LAYER; ++y) {
        auto on_x_border{x < 3 || x > TILES_PER_LAYER - 4};
        auto on_y_border{y < 3 || y > TILES_PER_LAYER - 4}; 
        auto on_x_main{
          x > TILES_PER_LAYER / 2 - 4 && x < TILES_PER_LAYER / 2 + 3
        };
        auto on_y_main{
          y > TILES_PER_LAYER / 2 - 4 && y < TILES_PER_LAYER / 2 + 3
        };

        if (on_x_border || on_y_border || on_x_main || on_y_main) {
          SetTile("floor", x, y, floor, "concrete-dark");
        } else {
          SetTile("floor", x, y, floor, "concrete-light");
        }

        // Debugging Grid
        /* SetTile("overlay", x, y, floor, "selection"); */
      }
    }

    SeedRooms(floor, num_rooms_);
    ExpandRooms(floor);
    BuildRooms(floor);
    FinishRooms(floor);
  }
}

void MapGenerator::SeedRooms(unsigned floor, unsigned num_rooms)
{
  for (auto i{0}; i < num_rooms; ++i) {
    bool room_collision{true};

    Room test_room;
    test_room.floor_type = "floor1";
    test_room.wall_type = "wall2";

    while (room_collision) {
      test_room.l = rand() % (TILES_PER_LAYER - 8) + 3;
      test_room.t = rand() % (TILES_PER_LAYER - 8) + 3;
      test_room.r = test_room.l + 2;
      test_room.b = test_room.t + 2;

      room_collision = RoomCollision(floor, test_room);
    }

    rooms_[floor].push_back(test_room);
  }
}

bool MapGenerator::RoomCollision(unsigned floor, const Room& test_room) 
{
  for (const auto& room : blocked_rooms_[floor]) 
    if (Intersects(room, test_room)) return true;

  for (const auto& room : rooms_[floor]) 
    if (room != test_room && Intersects(room, test_room)) return true;

  return false;
}

void MapGenerator::ExpandRooms(unsigned floor)
{
  // Randomize room expansion
  // srand(time(nullptr));

  for (auto i{0}; i < expansion_iterations_; ++i) {
    Room& room = rooms_[floor][rand() % rooms_[floor].size()]; 

    bool found = false;
    vector<int> dirs = {0, 1, 2, 3}; 

    while (!found && dirs.size() > 0) {
      int choice{dirs[(int)(rand() % dirs.size())]};

      switch (choice) {
        case 0: room.l--; break;
        case 1: room.r++; break;
        case 2: room.t--; break;
        case 3: room.b++; break;
      }

      if (RoomCollision(floor, room)) {
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

void MapGenerator::BuildRooms(unsigned floor)
{
  for (const auto& room : rooms_[floor]) {
    for (auto x{room.l}; x <= room.r; ++x) {
      SetTile("wall", x, room.t, floor, room.wall_type + "-str"); 
      SetTile("wall", x, room.b, floor, room.wall_type + "-str");
    }

    for (auto y{room.t + 1}; y <= room.b - 1; ++y) {
      SetTile("wall", room.l, y, floor, room.wall_type + "-str", 90); 
      SetTile("wall", room.r, y, floor, room.wall_type + "-str", 90);
    }
  } 

  cout << "Floor " << floor << " rooms built successfully" << endl;
}


void MapGenerator::FinishRooms(unsigned floor)
{
  for (auto x{3}; x < TILES_PER_LAYER - 3; ++x) {
    for (auto y{3}; y < TILES_PER_LAYER - 3; ++y) {
      Tile& tile = map_.floors[floor].layers["wall"].tiles[x][y];

      if (tile.type != "") {
        Tile& utile = map_.floors[floor].layers["wall"].tiles[x][y - 1];
        Tile& dtile = map_.floors[floor].layers["wall"].tiles[x][y + 1];
        Tile& ltile = map_.floors[floor].layers["wall"].tiles[x - 1][y];
        Tile& rtile = map_.floors[floor].layers["wall"].tiles[x + 1][y];

        vector<string> tstrings;
        boost::split(tstrings, tile.type, boost::is_any_of("-"));

        vector<string> ustrings;
        boost::split(ustrings, utile.type, boost::is_any_of("-"));
        vector<string> dstrings;
        boost::split(dstrings, dtile.type, boost::is_any_of("-"));
        vector<string> lstrings;
        boost::split(lstrings, ltile.type, boost::is_any_of("-"));
        vector<string> rstrings;
        boost::split(rstrings, rtile.type, boost::is_any_of("-"));

        bool umatch{tstrings[0] == ustrings[0]};  
        bool dmatch{tstrings[0] == dstrings[0]};  
        bool lmatch{tstrings[0] == lstrings[0]};  
        bool rmatch{tstrings[0] == rstrings[0]};  

        if (umatch && lmatch && dmatch && rmatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-int");
        } else if (umatch && rmatch && dmatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-tee");
        } else if (rmatch && dmatch && lmatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-tee", 90);
        } else if (dmatch && lmatch && umatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-tee", 180);
        } else if (lmatch && umatch && rmatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-tee", 270);
        } else if (umatch && rmatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-cor");
        } else if (rmatch && dmatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-cor", 90);
        } else if (dmatch && lmatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-cor", 180);
        } else if (lmatch && umatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-cor", 270);
        } else if (lmatch && rmatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-str");
        } else if (umatch && dmatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-str", 90);
        } else if (umatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-end");
        } else if (rmatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-end", 90);
        } else if (dmatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-end", 180);
        } else if (lmatch) {
          SetTile("wall", x, y, floor, tstrings[0] + "-end", 270);
        }
      }
    }
  }

  for (auto& room : rooms_[floor]) {
    unsigned count{0};
    bool found{false};

    while (!found && count++ < 40) {
      auto choice{rand() % 4}; 
      auto& tiles{map_.floors[floor].layers["wall"].tiles};

      if (choice == 0) {
        auto place{(rand() % (room.r - room.l - 1)) + room.l + 1};
        
        if (tiles[place][room.t - 1].type == "") {
          found = true;
          SetTile("wall", place, room.t, floor, "door1");
        }
      } else if (choice == 1) {
        auto place{(rand() % (room.b - room.t - 1)) + room.t + 1};

        if (tiles[room.r + 1][place].type == "") {
          found = true;
          SetTile("wall", room.r, place, floor, "door1", 90);
        }
      } else if (choice == 2) {
        auto place{(rand() % (room.r - room.l - 1)) + room.l + 1};
        
        if (tiles[place][room.b + 1].type == "") {
          found = true;
          SetTile("wall", place, room.b, floor, "door1");
        }
      } else if (choice == 3) {
        auto place{(rand() % (room.b - room.t - 1)) + room.t + 1};

        if (tiles[room.l - 1][place].type == "") {
          found = true;
          SetTile("wall", room.l, place, floor, "door1", 90);
        }
      }
    }
  }
}


bool MapGenerator::Intersects(
  const Room& r1, unsigned l, unsigned r, unsigned t, unsigned b
) {
  auto lr_check{r1.l < r && r1.r > l};
  auto tb_check{r1.t < b && r1.b > t};

  return lr_check && tb_check ? true : false;
}

bool MapGenerator::Intersects(const Room& r1, const Room& r2)
{
  return Intersects(r1, r2.l, r2.r, r2.t, r2.b);
}

void MapGenerator::DefineBlockedRooms(unsigned floor)
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

void MapGenerator::SetTile(
  string layer, 
  int x, int y, int floor, 
  string type, 
  float rotation, SDL_RendererFlip flip
) {
  Tile& tile = map_.floors[floor].layers[layer].tiles[x][y];

  if (TileData.find(type) != TileData.end()) {
    tile.type = type;
    tile.active = true;
    tile.rotation = rotation;
    tile.flip = flip;

    tile.src.x = TileData[type].uv[0] * TILE_SIZE;  
    tile.src.y = TileData[type].uv[1] * TILE_SIZE;
  } else {
    cerr << "Tile(" << x << "," << y << ") has invalid type: " << type << endl; 
  }
}
