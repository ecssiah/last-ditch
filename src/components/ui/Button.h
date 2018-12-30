#ifndef BUTTON_H
#define BUTTON_H

#include <SDL2/SDL.h>

#include "Element.h"
#include "../../Types.h"
#include "../../constants/MapConstants.h"

struct Button : public Element
{
  Button()
    : text{}
    , active{false}
    , active_texture{nullptr}
    , inactive_texture{nullptr}
    , active_tl_src{}
    , active_tm_src{}
    , active_tr_src{}
    , active_ll_src{}
    , active_mm_src{}
    , active_rr_src{}
    , active_bl_src{}
    , active_bm_src{}
    , active_br_src{}
    , inactive_tl_src{}
    , inactive_tm_src{}
    , inactive_tr_src{}
    , inactive_ll_src{}
    , inactive_mm_src{}
    , inactive_rr_src{}
    , inactive_bl_src{}
    , inactive_bm_src{}
    , inactive_br_src{}
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
    i32 base{TILE_SIZE};
    i32 size{TILE_SIZE / 4};

    active_tl_src = {base, 0, size, size};
    active_tm_src = {base + size, 0, size, size};
    active_tr_src = {base + 2 * size, 0, size, size};
    active_ll_src = {base, size, size, size};
    active_mm_src = {base + size, size, size, size};
    active_rr_src = {base + 2 * size, size, size, size};
    active_bl_src = {base, 2 * size, size, size};
    active_bm_src = {base + size, 2 * size, size, size};
    active_br_src = {base + 2 * size, 2 * size, size, size};

    base = 2 * TILE_SIZE;

    inactive_tl_src = {base, 0, size, size};
    inactive_tm_src = {base + size, 0, size, size};
    inactive_tr_src = {base + 2 * size, 0, size, size};
    inactive_ll_src = {base, size, size, size};
    inactive_mm_src = {base + size, size, size, size};
    inactive_rr_src = {base + 2 * size, size, size, size};
    inactive_bl_src = {base, 2 * size, size, size};
    inactive_bm_src = {base + size, 2 * size, size, size};
    inactive_br_src = {base + 2 * size, 2 * size, size, size};
  }

  std::string text;

  bool active;

  SDL_Texture* active_texture;
  SDL_Texture* inactive_texture;

  SDL_Rect active_tl_src, active_tm_src, active_tr_src;
  SDL_Rect active_ll_src, active_mm_src, active_rr_src;
  SDL_Rect active_bl_src, active_bm_src, active_br_src;

  SDL_Rect inactive_tl_src, inactive_tm_src, inactive_tr_src;
  SDL_Rect inactive_ll_src, inactive_mm_src, inactive_rr_src;
  SDL_Rect inactive_bl_src, inactive_bm_src, inactive_br_src;

  SDL_Rect tl_dst, tm_dst, tr_dst;
  SDL_Rect ll_dst, mm_dst, rr_dst;
  SDL_Rect bl_dst, bm_dst, br_dst;
};

#endif
