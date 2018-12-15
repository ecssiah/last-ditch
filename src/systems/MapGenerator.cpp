#include "MapGenerator.h"

#include <cstdlib>
#include <ctime>

#include "../components/Room.h"
#include "../constants/MapConstants.h"

using namespace std;

MapGenerator::MapGenerator(Map& map)
  : map_(map)
  , rooms_(NUM_FLOORS, vector<Room>())
{
  srand(time(nullptr));
}

void MapGenerator::GenerateMap(string name)
{
  for (auto floor{0}; floor < NUM_FLOORS; ++floor) {
    for (auto x{0}; x < TILES_PER_LAYER; ++x) { 
      for (auto y{0}; y < TILES_PER_LAYER; ++y) {
        SetTile("floor", x, y, floor, "concrete");
      }
    }

    SeedRooms(floor, 10);
    ExpandRooms(floor);
    BuildRooms(floor);
  }
}

void MapGenerator::SeedRooms(unsigned floor, unsigned num_rooms)
{
  for (auto i{0}; i < num_rooms; ++i) {
    bool found{false};

    Room test_room;
    test_room.floor_type = "floor1";
    test_room.wall_type = "wall1";

    while (!found) {
      test_room.l = rand() % (TILES_PER_LAYER - 9) + 3;
      test_room.t = rand() % (TILES_PER_LAYER - 9) + 3;
      test_room.r = test_room.l + 3;
      test_room.b = test_room.t + 3;

      found = true;
      for (const auto& room : rooms_[floor]) {
        if (Intersects(room, test_room)) found = false;
      }
    }

    rooms_[floor].push_back(test_room);
  }
}

void MapGenerator::ExpandRooms(unsigned floor)
{

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
  if (r1.l < r2.r && r1.r > r2.l && r1.t < r2.b && r1.b > r2.t ) {
    cout << "Intersection!" << endl;
    return true;
  } else {
    return false;
  }
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
