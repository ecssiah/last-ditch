#include <iostream>
#include <string>
#include <vector>
#include <fstream>
#include <algorithm>
#include <functional>

#include <SDL2/SDL.h>
#include <glm/glm.hpp>

#include "../../include/utility/Logging.h"
#include "../../include/render/RenderSystem.h"
#include "../../include/render/RenderConstants.h"
#include "../../include/map/MapConstants.h"
#include "../../include/ui/UIConstants.h"

using namespace std;

RenderSystem::RenderSystem(
  Input& input, Render& render, Camera& camera, Map& map, UI& ui
) 
  : input_{input}
  , render_{render}
  , camera_{camera}
  , map_{map}
  , ui_{ui}
{
}


RenderSystem::~RenderSystem()
{
  IMG_Quit();
  TTF_Quit();

  SDL_DestroyRenderer(render_.renderer);
  SDL_DestroyWindow(render_.window);
  SDL_Quit();

  ::mlog("RenderSystem shutdown");
}


void RenderSystem::init()
{
  ::mlog("RenderSystem initializing");

  init_SDL();
  init_SDL_image();
  init_SDL_ttf();

  load_fonts();
  load_tilesets();
}


void RenderSystem::update()
{
  SDL_RenderClear(render_.renderer);

  build_elements();

  render_map(); 
  render_ui();

  SDL_RenderPresent(render_.renderer);
}


void RenderSystem::init_SDL()
{
  if (SDL_Init(SDL_INIT_VIDEO) != 0) {
    ::elog("SDL_Init error: " + string(SDL_GetError()));
    return;
  }

  render_.window = SDL_CreateWindow(
    "Last Ditch", 
    SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 
    SCREEN_SIZE_X, SCREEN_SIZE_Y, 
    SDL_WINDOW_SHOWN
  );

  if (render_.window == nullptr) {
    ::elog("SDL_CreateWindow error: " + string(SDL_GetError()));
    return;
  }

  render_.renderer = SDL_CreateRenderer(
    render_.window, -1, 
    SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC
  );

  if (render_.renderer == nullptr){
    ::elog("SDL_CreateRenderer error: " + string(SDL_GetError()));
    return;
  }

  SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "linear");
}


void RenderSystem::init_SDL_image()
{
  const i32 img_flags{IMG_INIT_PNG};
  
  if (!(IMG_Init(img_flags) & img_flags)) {
    ::elog("SDL_image error: " + string(IMG_GetError()));
    return;
  }
}


void RenderSystem::init_SDL_ttf()
{
  if (TTF_Init()) {
    ::elog("TTF_Init error: " + string(TTF_GetError()));
    return;
  } 
}


SDL_Texture* RenderSystem::load_texture(const string& texturename)
{
  string filename{"assets/textures/" + texturename + ".png"};
  SDL_Surface* sur{IMG_Load(filename.c_str())};

  if (!sur) { 
    ::elog("IMG_Load error: " + string(IMG_GetError()));
    return nullptr;
  }

  SDL_Texture* texture{SDL_CreateTextureFromSurface(render_.renderer, sur)};

  if (!texture) {
    ::elog("SDL_CreateTextureFromSurface error: " + string(SDL_GetError()));
    return nullptr;
  }
  
  SDL_FreeSurface(sur);

  return texture;
}


TTF_Font* RenderSystem::load_font(const string& fontname, u32 size)
{
  string fontpath{"assets/fonts/" + fontname + ".ttf"};
  TTF_Font* font{TTF_OpenFont(fontpath.c_str(), size)};

  if (!font) {
    ::elog("TTF_OpenFont error: " + string(TTF_GetError()));
    return nullptr;
  }

  return font;
}


void RenderSystem::load_tilesets()
{
  render_.textures["floor"] = load_texture("map_tileset"); 
  render_.textures["wall"] = render_.textures["floor"];
  render_.textures["object"] = load_texture("object_tileset"); 
  render_.textures["entity"] = load_texture("entity_tileset"); 
  render_.textures["overlay"] = load_texture("overlay_tileset");
}


void RenderSystem::load_fonts()
{
  render_.fonts["Fantasque-Small"] = load_font("FantasqueSansMono-Regular", 14);
  render_.fonts["Fantasque-Medium"] = load_font("FantasqueSansMono-Regular", 18);
  render_.fonts["Fantasque-Large"] = load_font("FantasqueSansMono-Regular", 22);
  render_.fonts["Inconsolata-Small"] = load_font("Inconsolata-Regular", 14);
}


void RenderSystem::build_elements()
{
  for (auto& kv : ui_.scrollable_elements) 
    if (kv.second.changed) build_scrollable(kv.second);
  for (auto& kv : ui_.button_elements) 
    if (kv.second.changed) build_button(kv.second);
  for (auto& kv : ui_.window_elements) 
    if (kv.second.changed) build_window(kv.second);
  for (auto& kv : ui_.text_elements) 
    if (kv.second.changed) build_text(kv.second);
  for (auto& kv : ui_.scalable_elements) 
    if (kv.second.changed) build_scalable(kv.second);
} 


void RenderSystem::build_window(Window& el)
{
  el.changed = false;

  build_scalable(el.base);
}


void RenderSystem::build_button(Button& el)
{
  el.changed = false;

  TTF_SizeText(
    render_.fonts[el.label.font], el.label.content.c_str(), 
    &el.label.bounds.w, &el.label.bounds.h
  );

  el.label.bounds.x = el.bounds.x + el.bounds.w / 2 - el.label.bounds.w / 2;
  el.label.bounds.y = el.bounds.y + el.bounds.h / 2 - el.label.bounds.h / 2;

  build_scalable(el.base);
  build_text(el.label);
}


void RenderSystem::build_text(Text& el)
{
  el.changed = false;

  SDL_Surface* sur{
    TTF_RenderUTF8_Blended(render_.fonts[el.font], el.content.c_str(), el.color)
  }; 

  // SHOULD ONLY HAPPEN FIRST TIME
  if (el.align == RIGHT_ALIGN) {
    el.bounds.x = SCREEN_SIZE_X - el.bounds.x - sur->w;
  }

  if (el.texture == "time_display") {
    ::print(el.bounds);
  }

  el.bounds.w = sur->w;
  el.bounds.h = sur->h;

  if (sur == nullptr) {
    cerr << "TTF_RenderUTF8_Blended error: " << TTF_GetError() << endl; 
  } else {
    SDL_DestroyTexture(render_.textures[el.texture]);
    render_.textures[el.texture] = SDL_CreateTextureFromSurface(
      render_.renderer, sur
    ); 
    SDL_FreeSurface(sur);
  }
}


void RenderSystem::build_scrollable(Scrollable& el)
{
  el.changed = false;

  string full_msg;
  i32 size{(i32)min(20.0, (f64)el.items.size())};

  for (auto i{0}; i < size; i++) {
    full_msg += el.items[i];
    if (i < size - 1) full_msg += "\n";
  }

  SDL_Surface* sur{TTF_RenderText_Blended_Wrapped(
    render_.fonts[el.body.font], full_msg.c_str(), 
    {255, 255, 255}, el.mask.w
  )};

  build_scalable(el.base);

  i32 scrollbar_h{(i32)(el.mask.h / (f64)sur->h * el.mask.h)};

  SDL_DestroyTexture(render_.textures[el.body.texture]);
  render_.textures[el.body.texture] = SDL_CreateTextureFromSurface(
    render_.renderer, sur
  ); 
  SDL_FreeSurface(sur);

  if (scrollbar_h > el.mask.h) {
    el.scrollbar.active = false;
    el.body.bounds = {el.mask.x, el.mask.y, sur->w, sur->h};
  } else {
    el.body.bounds = {
      el.mask.x, el.mask.y - (i32)(el.pos * (sur->h - el.mask.h)), 
      sur->w, sur->h
    };
    el.scrollbar.active = true;

    el.scroll_range = el.base.bounds.h - 2 * el.base.border - scrollbar_h;

    el.scrollbar.bounds = {
      el.base.bounds.x + el.base.bounds.w - el.base.border - SCROLLBAR_WIDTH, 
      el.base.bounds.y + el.base.border + (i32)(el.pos * el.scroll_range), 
      SCROLLBAR_WIDTH, scrollbar_h
    };

    build_scrollbar(el.scrollbar);
  }
}


void RenderSystem::build_scrollbar(Scrollbar& el)
{
  if (TileData.find(el.type) != TileData.end()) {
    el.basex = {(i32)(SCROLLBAR_WIDTH * TileData[el.type].uv.x)};
    el.basey = {(i32)(TILE_SIZE * TileData[el.type].uv.y)};
  } else {
    el.basex = 0;
    el.basey = 0;
    std::cerr << "Scrollbar has invalid type: " << el.type << std::endl;
  }

  el.src["t"] = {
    el.basex, el.basey + 0 * el.size, el.size, el.size
  };
  el.src["m"] = {
    el.basex, el.basey + 1 * el.size, el.size, el.size
  };
  el.src["b"] = {
    el.basex, el.basey + 2 * el.size, el.size, el.size
  };

  el.dst["t"] = { 
    el.bounds.x, el.bounds.y + 0 * el.size, 
    el.size, el.size 
  };
  el.dst["m"] = {
    el.bounds.x, el.bounds.y + 1 * el.size, 
    el.size, el.bounds.h - 2 * el.size
  }; 
  el.dst["b"] = {
    el.bounds.x, el.bounds.y + el.bounds.h - el.size, 
    el.size, el.size
  };

}


void RenderSystem::build_scalable(Scalable& el)
{
  if (TileData.find(el.type) != TileData.end()) {
    el.basex = {(i32)(TILE_SIZE * TileData[el.type].uv.x)};
    el.basey = {(i32)(TILE_SIZE * TileData[el.type].uv.y)};
    el.border = TileData[el.type].border;
  } else {
    el.basex = 0;
    el.basey = 0;
    std::cerr << "Scalable has invalid type: " << el.type << std::endl;
  }

  el.src["tl"] = {
    el.basex + 0 * el.size, el.basey + 0 * el.size, el.size, el.size
  };
  el.src["tm"] = {
    el.basex + 1 * el.size, el.basey + 0 * el.size, el.size, el.size
  };
  el.src["tr"] = {
    el.basex + 2 * el.size, el.basey + 0 * el.size, el.size, el.size
  };
  el.src["ll"] = {
    el.basex + 0 * el.size, el.basey + 1 * el.size, el.size, el.size
  };
  el.src["mm"] = {
    el.basex + 1 * el.size, el.basey + 1 * el.size, el.size, el.size
  };
  el.src["rr"] = {
    el.basex + 2 * el.size, el.basey + 1 * el.size, el.size, el.size
  };
  el.src["bl"] = {
    el.basex + 0 * el.size, el.basey + 2 * el.size, el.size, el.size
  };
  el.src["bm"] = {
    el.basex + 1 * el.size, el.basey + 2 * el.size, el.size, el.size
  };
  el.src["br"] = {
    el.basex + 2 * el.size, el.basey + 2 * el.size, el.size, el.size
  };

  el.dst["tl"] = { 
    el.bounds.x, el.bounds.y, 
    el.size, el.size 
  };
  el.dst["tm"] = {
    el.bounds.x + el.size, el.bounds.y, 
    el.bounds.w - 2 * el.size, el.size
  }; 
  el.dst["tr"] = {
    el.bounds.x + el.bounds.w - el.size, el.bounds.y, 
    el.size, el.size
  };
  el.dst["ll"] = {
    el.bounds.x, el.bounds.y + el.size, 
    el.size, el.bounds.h - 2 * el.size
  };
  el.dst["mm"] = {
    el.bounds.x + el.size, el.bounds.y + el.size, 
    el.bounds.w - 2 * el.size, el.bounds.h - 2 * el.size
  };
  el.dst["rr"] = {
    el.bounds.x + el.bounds.w - el.size, el.bounds.y + el.size,
    el.size, el.bounds.h - 2 * el.size 
  };
  el.dst["bl"] = {
    el.bounds.x, el.bounds.y + el.bounds.h - el.size,
    el.size, el.size
  }; 
  el.dst["bm"] = {
    el.bounds.x + el.size, el.bounds.y + el.bounds.h - el.size,
    el.bounds.w - 2 * el.size, el.size
  };
  el.dst["br"] = {
    el.bounds.x + el.bounds.w - el.size, el.bounds.y + el.bounds.h - el.size,
    el.size, el.size
  };
}


void RenderSystem::render_map()
{
  const f32 lower{0};
  const f32 upper{(f32)TILES_PER_LAYER - 1};

  i32 x_min(max(lower, camera_.pos.x - VIEW_X * camera_.inv_zoom)); 
  i32 y_min(max(lower, camera_.pos.y - VIEW_Y * camera_.inv_zoom));
  i32 x_max(min(upper, camera_.pos.x + VIEW_X * camera_.inv_zoom));
  i32 y_max(min(upper, camera_.pos.y + VIEW_Y * camera_.inv_zoom)); 

  for (auto x{x_min}; x <= x_max; ++x) { 
    for (auto y{y_min}; y <= y_max; ++y) {
      render_tile("floor", x, y, map_.cur_floor);
      render_tile("wall", x, y, map_.cur_floor);
      render_tile("object", x, y, map_.cur_floor);
      render_tile("entity", x, y, map_.cur_floor);
      render_tile("overlay", x, y, 1);
    }
  }
}


void RenderSystem::render_ui()
{
  if (input_.hud) {
    render_scrollable(ui_.scrollable_elements["message_window"]);

    render_text(ui_.text_elements["floor_display"]);
    render_text(ui_.text_elements["time_display"]);
    render_text(ui_.text_elements["date_display"]);
  }

  if (input_.menu) {
    render_window(ui_.window_elements["main"]);

    render_button(ui_.button_elements["info"]);
    render_button(ui_.button_elements["save"]);
    render_button(ui_.button_elements["options"]);
  }
}


void RenderSystem::render_messages()
{
}


void RenderSystem::render_tile(const string& layer, i32 x, i32 y, i32 floor)
{
  const Tile& tile{map_.floors[floor].layers[layer].tiles[x][y]};

  if (tile.active) {
    const f32 scale_factor{camera_.zoom * TILE_SIZE};

    SDL_Rect dst;
    dst.x = scale_factor * (x - camera_.pos.x) + HALF_SCREEN_SIZE_X; 
    dst.y = scale_factor * (y - camera_.pos.y) + HALF_SCREEN_SIZE_Y;
    dst.w = scale_factor;
    dst.h = scale_factor;

    SDL_RenderCopyEx(
      render_.renderer, render_.textures[layer], 
      &tile.src, &dst, tile.rotation, nullptr, tile.flip
    ); 
  }
}


void RenderSystem::render_scrollable(Scrollable& el)
{
  render_scalable(el.base);

  SDL_RenderSetClipRect(render_.renderer, &el.mask);

  SDL_SetRenderTarget(render_.renderer, nullptr); 
  SDL_RenderCopy(render_.renderer, render_.textures[el.body.texture], nullptr, &el.body.bounds);

  SDL_RenderSetClipRect(render_.renderer, nullptr);

  if (el.scrollbar.active) render_scrollbar(el.scrollbar);
}


void RenderSystem::render_text(Text& el)
{
  SDL_RenderCopy(render_.renderer, render_.textures[el.texture], nullptr, &el.bounds); 
}


void RenderSystem::render_button(Button& el)
{
  el.active ? render_scalable(el.pressed) : render_scalable(el.base);
  render_text(el.label);
}


void RenderSystem::render_window(Window& el)
{
  render_scalable(el.base);
}


void RenderSystem::render_scrollbar(Scrollbar& el)
{
  SDL_RenderCopy(render_.renderer, render_.textures[el.texture], &el.src["t"], &el.dst["t"]);
  SDL_RenderCopy(render_.renderer, render_.textures[el.texture], &el.src["m"], &el.dst["m"]);
  SDL_RenderCopy(render_.renderer, render_.textures[el.texture], &el.src["b"], &el.dst["b"]);
}

void RenderSystem::render_scalable(Scalable& el)
{
  SDL_RenderCopy(render_.renderer, render_.textures[el.texture], &el.src["bl"], &el.dst["bl"]);
  SDL_RenderCopy(render_.renderer, render_.textures[el.texture], &el.src["bm"], &el.dst["bm"]);
  SDL_RenderCopy(render_.renderer, render_.textures[el.texture], &el.src["br"], &el.dst["br"]);
  SDL_RenderCopy(render_.renderer, render_.textures[el.texture], &el.src["ll"], &el.dst["ll"]);
  SDL_RenderCopy(render_.renderer, render_.textures[el.texture], &el.src["mm"], &el.dst["mm"]);
  SDL_RenderCopy(render_.renderer, render_.textures[el.texture], &el.src["rr"], &el.dst["rr"]);
  SDL_RenderCopy(render_.renderer, render_.textures[el.texture], &el.src["tl"], &el.dst["tl"]);
  SDL_RenderCopy(render_.renderer, render_.textures[el.texture], &el.src["tm"], &el.dst["tm"]);
  SDL_RenderCopy(render_.renderer, render_.textures[el.texture], &el.src["tr"], &el.dst["tr"]);
}
