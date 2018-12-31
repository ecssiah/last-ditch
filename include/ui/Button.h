#ifndef BUTTON_H
#define BUTTON_H

#include <SDL2/SDL.h>

#include "Element.h"
#include "../Types.h"
#include "../map/MapConstants.h"

struct Button : public Element
{
  Button()
    : text{}
    , active{false}
    , active_texture{nullptr}
    , inactive_texture{nullptr}
    , active_tl_src{}, active_tm_src{}, active_tr_src{}
    , active_ll_src{}, active_mm_src{}, active_rr_src{}
    , active_bl_src{}, active_bm_src{}, active_br_src{}
    , inactive_tl_src{}, inactive_tm_src{}, inactive_tr_src{}
    , inactive_ll_src{}, inactive_mm_src{}, inactive_rr_src{}
    , inactive_bl_src{}, inactive_bm_src{}, inactive_br_src{}
    , tl_dst{}, tm_dst{}, tr_dst{}
    , ll_dst{}, mm_dst{}, rr_dst{}
    , bl_dst{}, bm_dst{}, br_dst{}
  {
    i32 size{TILE_SIZE / 4};
    i32 active_base{1 * TILE_SIZE};
    i32 inactive_base{2 * TILE_SIZE};

    active_tl_src = {active_base + 0 * size, 0 * size, size, size};
    active_tm_src = {active_base + 1 * size, 0 * size, size, size};
    active_tr_src = {active_base + 2 * size, 0 * size, size, size};
    active_ll_src = {active_base + 0 * size, 1 * size, size, size};
    active_mm_src = {active_base + 1 * size, 1 * size, size, size};
    active_rr_src = {active_base + 2 * size, 1 * size, size, size};
    active_bl_src = {active_base + 0 * size, 2 * size, size, size};
    active_bm_src = {active_base + 1 * size, 2 * size, size, size};
    active_br_src = {active_base + 2 * size, 2 * size, size, size};

    inactive_tl_src = {inactive_base + 0 * size, 0 * size, size, size};
    inactive_tm_src = {inactive_base + 1 * size, 0 * size, size, size};
    inactive_tr_src = {inactive_base + 2 * size, 0 * size, size, size};
    inactive_ll_src = {inactive_base + 0 * size, 1 * size, size, size};
    inactive_mm_src = {inactive_base + 1 * size, 1 * size, size, size};
    inactive_rr_src = {inactive_base + 2 * size, 1 * size, size, size};
    inactive_bl_src = {inactive_base + 0 * size, 2 * size, size, size};
    inactive_bm_src = {inactive_base + 1 * size, 2 * size, size, size};
    inactive_br_src = {inactive_base + 2 * size, 2 * size, size, size};
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
