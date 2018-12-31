#ifndef WINDOW_H
#define WINDOW_H

#include <SDL2/SDL.h>

#include "Element.h"
#include "../Types.h"
#include "../map/MapConstants.h"

struct Window: public Element
{
  Window()
    : texture{nullptr}
    , tl_src{}
    , tm_src{}
    , tr_src{}
    , ll_src{}
    , mm_src{}
    , rr_src{}
    , bl_src{}
    , bm_src{}
    , br_src{}
    , tl_dst{}
    , tm_dst{}
    , tr_dst{}
    , ll_dst{}
    , mm_dst{}
    , rr_dst{}
    , bl_dst{}
    , bm_dst{}
    , br_dst{}
  {
    i32 size{TILE_SIZE / 4};

    tl_src = {0, 0, size, size};
    tm_src = {size, 0, size, size};
    tr_src = {2 * size, 0, size, size};
    ll_src = {0, size, size, size};
    mm_src = {size, size, size, size};
    rr_src = {2 * size, size, size, size};
    bl_src = {0, 2 * size, size, size};
    bm_src = {size, 2 * size, size, size};
    br_src = {2 * size, 2 * size, size, size};
  }

  SDL_Texture* texture;

  SDL_Rect tl_src, tm_src, tr_src;
  SDL_Rect ll_src, mm_src, rr_src;
  SDL_Rect bl_src, bm_src, br_src;

  SDL_Rect tl_dst, tm_dst, tr_dst;
  SDL_Rect ll_dst, mm_dst, rr_dst;
  SDL_Rect bl_dst, bm_dst, br_dst;
};

#endif
