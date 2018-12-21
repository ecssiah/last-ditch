#ifndef WINDOW_H
#define WINDOW_H

#include <SDL2/SDL.h>

#include "Element.h"
#include "../../constants/MapConstants.h"

struct Window: public Element
{
  Window()
    : texture{nullptr}
    , tl_src{0, 0, TILE_SIZE / 4, TILE_SIZE / 4}
    , tm_src{TILE_SIZE / 4, 0, TILE_SIZE / 2, TILE_SIZE / 4}
    , tr_src{3 * TILE_SIZE / 4, 0, TILE_SIZE / 4, TILE_SIZE / 4}
    , ll_src{0, TILE_SIZE / 4, TILE_SIZE / 4, 2 * TILE_SIZE / 4}
    , mm_src{TILE_SIZE / 4, TILE_SIZE / 4, TILE_SIZE / 2, TILE_SIZE / 2}
    , rr_src{3 * TILE_SIZE / 4, TILE_SIZE / 4, TILE_SIZE / 4, TILE_SIZE / 2}
    , bl_src{0, 3 * TILE_SIZE / 4, TILE_SIZE / 4, TILE_SIZE / 4}
    , bm_src{TILE_SIZE / 4, 3 * TILE_SIZE / 4, TILE_SIZE / 2, TILE_SIZE / 4}
    , br_src{3 * TILE_SIZE / 4, 3 * TILE_SIZE / 4, TILE_SIZE / 4, TILE_SIZE / 4}
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
