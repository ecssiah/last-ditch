#ifndef ROOM_H
#define ROOM_H

#include <string>
#include <SDL2/SDL.h>

#include "../Types.h"

struct Room
{
  Room()
  {}

  Room(i32 x, i32 y, i32 w, i32 h)
    : wall_type{}
    , floor_type{}
    , rect{x, y, w, h}
  {}

  inline const i32 x() const { return rect.x; }
  inline const i32 y() const { return rect.y; }
  inline const i32 w() const { return rect.w; }
  inline const i32 h() const { return rect.h; }

  inline const i32 l() const { return rect.x; }
  inline const i32 r() const { return rect.x + rect.w; }
  inline const i32 t() const { return rect.y; }
  inline const i32 b() const { return rect.y + rect.h; }

  std::string wall_type;
  std::string floor_type;

  SDL_Rect rect;

};


#endif
