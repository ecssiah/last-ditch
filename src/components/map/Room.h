#ifndef ROOM_H
#define ROOM_H

#include <string>
#include <SDL2/SDL.h>

struct Room
{
  Room()
  {}

  Room(int x, int y, int w, int h)
    : rect{x, y, w, h}
    , wall_type{}
    , floor_type{}
  {}

  int x() const { return rect.x; }
  int y() const { return rect.y; }
  int w() const { return rect.w; }
  int h() const { return rect.h; }

  int l() const { return rect.x; }
  int r() const { return rect.x + rect.w; }
  int t() const { return rect.y; }
  int b() const { return rect.y + rect.h; }

  SDL_Rect rect;

  std::string wall_type;
  std::string floor_type;
};


#endif
