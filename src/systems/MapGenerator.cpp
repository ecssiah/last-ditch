#include "MapGenerator.h"

#include <cstdlib>
#include <ctime>

#include "../components/Room.h"
#include "../constants/MapConstants.h"

using namespace std;

MapGenerator::MapGenerator(Map& map)
  : map_(map)
  , rooms_(NUM_FLOORS, vector<Room>())
  , blocked_rooms_(NUM_FLOORS, vector<Room>())
{
  srand(time(nullptr));

  for (auto floor{0}; floor < NUM_FLOORS; ++floor) DefineBlockedRooms(floor);
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
        SetTile("overlay", x, y, floor, "selection");
      }
    }

    SeedRooms(floor, 20);
    ExpandRooms(floor);
    BuildRooms(floor);
  }
}

void MapGenerator::SeedRooms(unsigned floor, unsigned num_rooms)
{
  for (auto i{0}; i < num_rooms; ++i) {
    bool room_collision{true};

    Room test_room;
    test_room.floor_type = "floor1";
    test_room.wall_type = "wall1";

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
  for (const auto& room : blocked_rooms_[floor]) {
    if (Intersects(room, test_room)) return true;
  }
  for (const auto& room : rooms_[floor]) {
    if (room != test_room && Intersects(room, test_room)) return true;
  }

  return false;
}

void MapGenerator::ExpandRooms(unsigned floor)
{
  for (auto i{0}; i < 100; ++i) {
    Room& room = rooms_[floor][rand() % (rooms_[floor].size() - 1)]; 

    auto choice{rand() % 4};
    switch (choice) {
      case 0: room.l--; break;
      case 1: room.r++; break;
      case 2: room.t--; break;
      case 3: room.b++; break;
    }
    
    if (RoomCollision(floor, room)) {
      switch (choice) {
        case 0: room.l++; break;
        case 1: room.r--; break;
        case 2: room.t++; break;
        case 3: room.b--; break;
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

bool MapGenerator::Intersects(const Room& r1, const Room& r2)
{
  auto lr_check{r1.l < r2.r && r1.r > r2.l};
  auto tb_check{r1.t < r2.b && r1.b > r2.t};

  return lr_check && tb_check ? true : false;
}

bool MapGenerator::Intersects(
  const Room& r1, unsigned l, unsigned r, unsigned t, unsigned b
) {
  auto lr_check{r1.l < r && r1.r > l};
  auto tb_check{r1.t < b && r1.b > t};

  return lr_check && tb_check ? true : false;
}

void MapGenerator::SetTile(
  string layer, 
  int x, int y, int floor, 
  string type, 
  float rotation, SDL_RendererFlip flip
) {
  Tile& tile = map_.floors[floor].layers[layer].tiles[x][y];

  if (TileData.find(type) != TileData.end()) {
    tile.active = true;
    tile.rotation = rotation;
    tile.flip = flip;

    tile.src.x = TileData[type].uv[0] * TILE_SIZE;  
    tile.src.y = TileData[type].uv[1] * TILE_SIZE;
  } else {
    cerr << "Tile(" << x << "," << y << ") has invalid type: " << type << endl; 
  }
}
