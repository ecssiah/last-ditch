#ifndef SCALABLE_H
#define SCALABLE_H

#include <iostream>
#include <SDL2/SDL.h>

#include "Element.h"
#include "../map/MapConstants.h"

struct Scalable : public Element 
{
  Scalable()
    : Scalable{"window1"}
  {}

  Scalable(const std::string& type)
    : size{TILE_SIZE / 4}
    , texture{nullptr}
    , src{}
    , dst{}
  {
    i32 basex, basey;

    if (TileData.find(type) != TileData.end()) {
      basex = {(i32)(TILE_SIZE * TileData[type].uv.x)};
      basey = {(i32)(TILE_SIZE * TileData[type].uv.y)};
    } else {
      std::cerr << "Scalable has invalid type: " << type << std::endl;

      basex = {(i32)(TILE_SIZE * TileData["missing_overlay"].uv.x)};
      basey = {(i32)(TILE_SIZE * TileData["missing_overlay"].uv.y)};
    }

    src["tl"] = {basex + 0 * size, basey + 0 * size, size, size};
    src["tm"] = {basex + 1 * size, basey + 0 * size, size, size};
    src["tr"] = {basex + 2 * size, basey + 0 * size, size, size};
    src["ll"] = {basex + 0 * size, basey + 1 * size, size, size};
    src["mm"] = {basex + 1 * size, basey + 1 * size, size, size};
    src["rr"] = {basex + 2 * size, basey + 1 * size, size, size};
    src["bl"] = {basex + 0 * size, basey + 2 * size, size, size};
    src["bm"] = {basex + 1 * size, basey + 2 * size, size, size};
    src["br"] = {basex + 2 * size, basey + 2 * size, size, size};
  }

  i32 size;

  SDL_Texture* texture;

  std::unordered_map<std::string, SDL_Rect> src;
  std::unordered_map<std::string, SDL_Rect> dst;

};

#endif