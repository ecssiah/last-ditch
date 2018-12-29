#ifndef ROOM_H
#define ROOM_H

#include <string>
#include <SDL2/SDL.h>

#include "../../Types.h"

struct Room
{
  Room()
  {}

  Room(I32 x, I32 y, I32 w, I32 h)
    : rect{x, y, w, h}
    , wall_type{}
    , floor_type{}
  {}

  I32 x() const { return rect.x; }
  I32 y() const { return rect.y; }
  I32 w() const { return rect.w; }
  I32 h() const { return rect.h; }

  I32 l() const { return rect.x; }
  I32 r() const { return rect.x + rect.w; }
  I32 t() const { return rect.y; }
  I32 b() const { return rect.y + rect.h; }

  SDL_Rect rect;

  std::string wall_type;
  std::string floor_type;
};


#endif
