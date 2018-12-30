#ifndef ROOM_H
#define ROOM_H

#include <string>
#include <SDL2/SDL.h>

#include "../../Types.h"

struct Room
{
  Room()
  {}

  Room(i32 x, i32 y, i32 w, i32 h)
    : rect{x, y, w, h}
    , wall_type{}
    , floor_type{}
  {}

  i32 x() const { return rect.x; }
  i32 y() const { return rect.y; }
  i32 w() const { return rect.w; }
  i32 h() const { return rect.h; }

  i32 l() const { return rect.x; }
  i32 r() const { return rect.x + rect.w; }
  i32 t() const { return rect.y; }
  i32 b() const { return rect.y + rect.h; }

  SDL_Rect rect;

  std::string wall_type;
  std::string floor_type;
};


#endif
