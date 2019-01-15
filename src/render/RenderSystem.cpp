#include <iostream>
#include <string>
#include <vector>
#include <fstream>
#include <algorithm>
#include <functional>

#include <SDL2/SDL.h>
#include <glm/glm.hpp>

#include "../../include/utility/Logging.h"
#include "../../include/constants/RenderConstants.h"
#include "../../include/constants/MapConstants.h"
#include "../../include/constants/UIConstants.h"
#include "../../include/render/RenderSystem.h"

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

  cout << "RenderSystem shutdown" << endl;
}


void RenderSystem::init()
{
  cout << "RenderSystem initializing" << endl;

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
    cerr << SDL_GetError() << endl;
    return;
  }

  render_.window = SDL_CreateWindow(
    "Last Ditch", 
    SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 
    SCREEN_SIZE_X, SCREEN_SIZE_Y, 
    SDL_WINDOW_SHOWN
  );

  if (render_.window == nullptr) {
    cerr << SDL_GetError() << endl;
    return;
  }

  render_.renderer = SDL_CreateRenderer(
    render_.window, -1, 
    SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC
  );

  if (render_.renderer == nullptr){
    cerr << SDL_GetError() << endl;
    return;
  }

  SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "linear");
}


void RenderSystem::init_SDL_image()
{
  const i32 img_flags{IMG_INIT_PNG};
  
  if (!(IMG_Init(img_flags) & img_flags)) {
    cerr << IMG_GetError() << endl;
    return;
  }
}


void RenderSystem::init_SDL_ttf()
{
  if (TTF_Init()) {
    cerr << TTF_GetError() << endl;
    return;
  } 
}


SDL_Texture* RenderSystem::load_texture(const string& texturename)
{
  string filename{"data/tilesets/" + texturename + ".png"};
  SDL_Surface* sur{IMG_Load(filename.c_str())};

  if (!sur) { 
    cerr << IMG_GetError() << endl;
    return nullptr;
  }

  SDL_Texture* texture{SDL_CreateTextureFromSurface(render_.renderer, sur)};

  if (!texture) {
    cerr << SDL_GetError() << endl;
    return nullptr;
  }
  
  SDL_FreeSurface(sur);

  return texture;
}


TTF_Font* RenderSystem::load_font(const string& fontname, u32 size)
{
  string fontpath{"data/fonts/" + fontname + ".ttf"};
  TTF_Font* font{TTF_OpenFont(fontpath.c_str(), size)};

  if (!font) {
    cerr << TTF_GetError() << endl;
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
  render_.fonts["Small"] = load_font("FantasqueSansMono-Regular", 14);
  render_.fonts["Medium"] = load_font("FantasqueSansMono-Regular", 18);
  render_.fonts["Large"] = load_font("FantasqueSansMono-Regular", 22);
}


void RenderSystem::build_elements()
{
  for (auto& kv : ui_.scrollable_elements) 
    if (kv.second.changed) build_scrollable(kv.second);
  for (auto& kv : ui_.button_elements) 
    if (kv.second.changed) build_button(kv.second);
  for (auto& kv : ui_.button_set_elements)
    if (kv.second.changed) build_button_set(kv.second);
  for (auto& kv : ui_.window_elements) 
    if (kv.second.changed) build_window(kv.second);
  for (auto& kv : ui_.text_elements) 
    if (kv.second.changed) build_text(kv.second);
} 


void RenderSystem::build_window(Window& el)
{
  el.changed = false;

  build_scalable(el.base);
}


void RenderSystem::build_button(Button& el)
{
  el.changed = false;

  build_scalable(el.base);
  build_scalable(el.pressed);
  build_text(el.label);
}


void RenderSystem::build_button_set(ButtonSet& el)
{
  el.changed = false;

  for (auto& kv : el.buttons) build_button(kv.second);
}


void RenderSystem::build_text(Text& el)
{
  el.changed = false;

  SDL_Surface* sur{
    TTF_RenderUTF8_Blended(render_.fonts[el.font], el.content.c_str(), el.color)
  }; 

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
  i32 msg_limit{min(MESSAGE_DISPLAY_LIMIT, (i32)el.list.items.size())};

  for (auto i{0}; i < msg_limit; i++) {
    full_msg += el.list.items[i];
    if (i < msg_limit - 1) full_msg += "\n";
  }

  SDL_Surface* sur{TTF_RenderText_Blended_Wrapped(
    render_.fonts[el.list.font], full_msg.c_str(), 
    {255, 255, 255}, el.mask.w
  )};

  SDL_DestroyTexture(render_.textures[el.list.texture]);
  render_.textures[el.list.texture] = SDL_CreateTextureFromSurface(
    render_.renderer, sur
  ); 

  i32 scrollbar_height{(i32)(el.mask.h / (f32)sur->h * el.mask.h)};

  if (scrollbar_height > el.mask.h) {
    el.scrollbar.active = false;
    el.list.bounds = {el.mask.x, el.mask.y, sur->w, sur->h};
  } else {
    el.scrollbar.active = true;

    el.list.bounds = {
      el.mask.x, el.mask.y - (i32)(el.pos * sur->h), 
      sur->w, sur->h
    };

    el.scroll_range = el.base.bounds.h - 2 * el.base.border - scrollbar_height;

    el.scrollbar.bounds = {
      el.base.bounds.x + el.base.bounds.w - el.base.border - SCROLLBAR_WIDTH, 
      el.base.bounds.y + el.base.border + (i32)(el.pos * el.scroll_range), 
      SCROLLBAR_WIDTH, scrollbar_height
    };

    build_scrollbar(el.scrollbar);
  }

  build_scalable(el.base);
}


void RenderSystem::build_scrollbar(Scrollbar& el)
{
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


void RenderSystem::render_map() const
{
  const i32 lower{0};
  const i32 upper{TILES_PER_LAYER - 1};

  i32 x_min(max(lower, (i32)(camera_.pos.x - VIEW_X * camera_.inv_zoom))); 
  i32 y_min(max(lower, (i32)(camera_.pos.y - VIEW_Y * camera_.inv_zoom)));
  i32 x_max(min(upper, (i32)(camera_.pos.x + VIEW_X * camera_.inv_zoom)));
  i32 y_max(min(upper, (i32)(camera_.pos.y + VIEW_Y * camera_.inv_zoom))); 

  for (auto x{x_min}; x <= x_max; ++x) { 
    for (auto y{y_min}; y <= y_max; ++y) {
      render_tile("floor", x, y, map_.cur_floor);
      render_tile("wall", x, y, map_.cur_floor);
      render_tile("object", x, y, map_.cur_floor);
      render_tile("entity", x, y, map_.cur_floor);
      render_tile("overlay", x, y, map_.cur_floor);
    }
  }
}


void RenderSystem::render_ui() const
{
  if (input_.hud) {
    render_text(ui_.text_elements["floor_display"]);
    render_text(ui_.text_elements["time_display"]);
    render_text(ui_.text_elements["date_display"]);
    render_scrollable(ui_.scrollable_elements["message_window"]);
  }

  if (input_.menu) {
    render_window(ui_.window_elements["main"]);
    render_button_set(ui_.button_set_elements["main_buttons"]);
  }
}


void RenderSystem::render_button_set(ButtonSet& el) const
{
  for (auto& kv : el.buttons) render_button(kv.second);
}


void RenderSystem::render_tile(
  const string& layer, i32 x, i32 y, i32 floor
) const 
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
      &tile.src, &dst, tile.rot, nullptr, tile.flip
    ); 
  }
}


void RenderSystem::render_scrollable(Scrollable& el) const
{
  render_scalable(el.base);

  SDL_RenderSetClipRect(render_.renderer, &el.mask);

  SDL_RenderCopy(
    render_.renderer, render_.textures[el.list.texture], nullptr, 
    &el.list.bounds
  );

  SDL_RenderSetClipRect(render_.renderer, nullptr);

  if (el.scrollbar.active) render_scrollbar(el.scrollbar);
}


void RenderSystem::render_text(Text& el) const
{
  SDL_Texture*& texture{render_.textures[el.texture]};

  SDL_RenderCopy(render_.renderer, texture, nullptr, &el.bounds); 
}


void RenderSystem::render_button(Button& el) const
{
  el.active ? render_scalable(el.pressed) : render_scalable(el.base);

  render_text(el.label);
}


void RenderSystem::render_window(Window& el) const
{
  render_scalable(el.base);
}


void RenderSystem::render_scrollbar(Scrollbar& el) const
{
  SDL_Texture*& texture{render_.textures[el.texture]};

  SDL_RenderCopy(render_.renderer, texture, &el.src["t"], &el.dst["t"]);
  SDL_RenderCopy(render_.renderer, texture, &el.src["m"], &el.dst["m"]);
  SDL_RenderCopy(render_.renderer, texture, &el.src["b"], &el.dst["b"]);
}

void RenderSystem::render_scalable(Scalable& el) const
{
  SDL_Texture*& texture{render_.textures[el.texture]};

  SDL_RenderCopy(render_.renderer, texture, &el.src["bl"], &el.dst["bl"]);
  SDL_RenderCopy(render_.renderer, texture, &el.src["bm"], &el.dst["bm"]);
  SDL_RenderCopy(render_.renderer, texture, &el.src["br"], &el.dst["br"]);
  SDL_RenderCopy(render_.renderer, texture, &el.src["ll"], &el.dst["ll"]);
  SDL_RenderCopy(render_.renderer, texture, &el.src["mm"], &el.dst["mm"]);
  SDL_RenderCopy(render_.renderer, texture, &el.src["rr"], &el.dst["rr"]);
  SDL_RenderCopy(render_.renderer, texture, &el.src["tl"], &el.dst["tl"]);
  SDL_RenderCopy(render_.renderer, texture, &el.src["tm"], &el.dst["tm"]);
  SDL_RenderCopy(render_.renderer, texture, &el.src["tr"], &el.dst["tr"]);
}
