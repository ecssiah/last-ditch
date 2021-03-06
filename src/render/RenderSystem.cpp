#include <iostream>
#include <string>
#include <vector>
#include <fstream>
#include <algorithm>
#include <functional>

#include <SDL2/SDL.h>

#include "../utility/Logging.h"
#include "../constants/RenderConstants.h"
#include "../constants/MapConstants.h"
#include "../constants/UIConstants.h"
#include "../render/RenderSystem.h"

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


void 
RenderSystem::init()
{
  cout << "RenderSystem initializing" << endl;

  init_SDL();
  init_SDL_image();
  init_SDL_ttf();

  init_grid();

  load_fonts();
  load_tilesets();
}


void 
RenderSystem::update()
{
  SDL_RenderClear(render_.renderer);

  build_elements();

  render_map(); 
  render_ui();

  SDL_RenderPresent(render_.renderer);
}


void 
RenderSystem::init_SDL()
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
    render_.window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC
  );

  if (render_.renderer == nullptr){
    cerr << SDL_GetError() << endl;
    return;
  }

  SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "linear");
}


void 
RenderSystem::init_SDL_image()
{
  const i32 img_flags{IMG_INIT_PNG};
  const i32 initialized{IMG_Init(img_flags)};
  
  if ((initialized & img_flags) != img_flags) {
    cerr << IMG_GetError() << endl;
    return;
  }
}


void 
RenderSystem::init_SDL_ttf()
{
  if (TTF_Init() != 0) {
    cerr << TTF_GetError() << endl;
    return;
  } 
}


void 
RenderSystem::init_grid()
{
  render_.scale = camera_.zoom * TILE_SIZE; 

  render_.grid_dst = {
    0, 0, static_cast<i32>(render_.scale), static_cast<i32>(render_.scale)
  };
}


SDL_Texture* 
RenderSystem::load_texture(const string& texturename)
{
  string filepath{"data/tilesets/" + texturename + ".png"};

  SDL_Surface* surface{IMG_Load(filepath.c_str())};

  if (surface == nullptr) { 
    cerr << IMG_GetError() << endl;
    return nullptr;
  }

  SDL_Texture* texture{SDL_CreateTextureFromSurface(render_.renderer, surface)};

  if (texture == nullptr) {
    cerr << SDL_GetError() << endl;
    return nullptr;
  }
  
  SDL_FreeSurface(surface);

  return texture;
}


TTF_Font* 
RenderSystem::load_font(const string& fontname, u32 size)
{
  string fontpath{"data/fonts/" + fontname + ".ttf"};
  TTF_Font* font{TTF_OpenFont(fontpath.c_str(), size)};

  if (font == nullptr) {
    cerr << TTF_GetError() << endl;
    return nullptr;
  }

  return font;
}


void 
RenderSystem::load_tilesets()
{
  render_.textures["flr"] = load_texture("map_tileset"); 
  render_.textures["wal"] = render_.textures["flr"];
  render_.textures["obj"] = load_texture("object_tileset"); 
  render_.textures["ent"] = load_texture("entity_tileset"); 
  render_.textures["ovr"] = load_texture("overlay_tileset");
}


void 
RenderSystem::load_fonts()
{
  render_.fonts["Small"] = load_font("FantasqueSansMono-Regular", 14);
  render_.fonts["Medium"] = load_font("FantasqueSansMono-Regular", 18);
  render_.fonts["Large"] = load_font("FantasqueSansMono-Regular", 22);
}


void 
RenderSystem::build_elements()
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


void 
RenderSystem::build_window(Window& el)
{
  el.changed = false;

  build_scalable(el.base);
}


void 
RenderSystem::build_button(Button& el)
{
  el.changed = false;

  build_scalable(el.base);
  build_scalable(el.pressed);
  build_text(el.label);
}


void 
RenderSystem::build_button_set(ButtonSet& el)
{
  el.changed = false;

  for (auto& kv : el.buttons) build_button(kv.second);
}


void 
RenderSystem::build_text(Text& el)
{
  SDL_Surface* surface{
    TTF_RenderUTF8_Blended(render_.fonts[el.font], el.content.c_str(), el.color)
  }; 

  if (surface == nullptr) {
    cerr << TTF_GetError() << endl; 
    return;
  }

  el.changed = false;

  SDL_DestroyTexture(render_.textures[el.texture]);

  render_.textures[el.texture] = SDL_CreateTextureFromSurface(
    render_.renderer, surface
  ); 

  SDL_FreeSurface(surface);
}


void 
RenderSystem::build_scrollable(Scrollable& el)
{
  build_scalable(el.base);

  if (el.list.items.size() < 1) return;

  string full_msg;
  i32 msg_limit{
    min(MESSAGE_DISPLAY_LIMIT, static_cast<i32>(el.list.items.size()))
  };

  for (auto i{0}; i < msg_limit; i++) {
    full_msg += el.list.items[i];
    if (i < msg_limit - 1) full_msg += "\n";
  }

  SDL_Surface* surface{TTF_RenderText_Blended_Wrapped(
    render_.fonts[el.list.font], full_msg.c_str(), {255, 255, 255}, el.mask.w
  )};

  if (surface == nullptr) {
    cerr << TTF_GetError() << endl; 
    return;
  }

  el.changed = false;

  SDL_DestroyTexture(render_.textures[el.list.texture]);

  render_.textures[el.list.texture] = SDL_CreateTextureFromSurface(
    render_.renderer, surface
  ); 

  const f32 mask_content_ratio{el.mask.h / static_cast<f32>(surface->h)};
  const i32 scrollbar_height{
    static_cast<i32>(round(mask_content_ratio * el.mask.h))
  };

  if (scrollbar_height > el.mask.h) {
    el.scrollbar.active = false;
    el.list.bounds = {el.mask.x, el.mask.y, surface->w, surface->h};
  } else {
    el.scroll_range = el.base.bounds.h - 2 * el.base.border - scrollbar_height;

    const i32 scrollbar_offset{static_cast<i32>(el.pos * el.scroll_range)};

    el.scrollbar.active = true;
    el.scrollbar.bounds = {
      el.base.bounds.x + el.base.bounds.w - el.base.border - SCROLLBAR_WIDTH, 
      el.base.bounds.y + el.base.border + scrollbar_offset,
      SCROLLBAR_WIDTH, scrollbar_height
    };

    el.list.bounds = {
      el.mask.x, el.mask.y - static_cast<i32>(el.pos * surface->h), 
      surface->w, surface->h
    };

    build_scrollbar(el.scrollbar);
  }
}


void 
RenderSystem::build_scrollbar(Scrollbar& el)
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


void 
RenderSystem::build_scalable(Scalable& el) 
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


void 
RenderSystem::render_map() 
const
{
  const i32 lower{0};
  const i32 upper{TILES_PER_LAYER - 1};

  const i32 x_min{static_cast<i32>(camera_.pos.x - VIEW_X * camera_.inv_zoom)};
  const i32 x_max{static_cast<i32>(camera_.pos.x + VIEW_X * camera_.inv_zoom)};
  const i32 y_min{static_cast<i32>(camera_.pos.y - VIEW_Y * camera_.inv_zoom)};
  const i32 y_max{static_cast<i32>(camera_.pos.y + VIEW_Y * camera_.inv_zoom)};

  for (auto x{max(lower, x_min)}; x <= min(upper, x_max); x++) { 
    for (auto y{max(lower, y_min)}; y <= min(upper, y_max); y++) {
      render_tile("flr", x, y, map_.cur_floor);
      render_tile("wal", x, y, map_.cur_floor);
      render_tile("obj", x, y, map_.cur_floor);
      render_tile("ent", x, y, map_.cur_floor);
      render_tile("ovr", x, y, map_.cur_floor);

      if (render_.grid) render_grid(x, y);
    }
  }
}


void 
RenderSystem::render_ui() 
const
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


void 
RenderSystem::render_button_set(ButtonSet& el) 
const
{
  for (auto& kv : el.buttons) render_button(kv.second);
}


void 
RenderSystem::render_tile(const std::string& layer, i32 x, i32 y, i32 floor) 
const 
{
  const Tile& tile{map_.floors[floor].layers[layer].tiles[x][y]};

  if (tile.active) {
    SDL_Rect dst;
    dst.x = render_.scale * (x - camera_.pos.x) + HALF_SCREEN_SIZE_X; 
    dst.y = render_.scale * (y - camera_.pos.y) + HALF_SCREEN_SIZE_Y;
    dst.w = render_.scale;
    dst.h = render_.scale;

    SDL_RenderCopyEx(
      render_.renderer, render_.textures[layer], 
      &tile.src, &dst, tile.rot, nullptr, tile.flip
    ); 
  }
}


void 
RenderSystem::render_grid(i32 x, i32 y) 
const
{
  render_.grid_dst.x = render_.scale * (x - camera_.pos.x) + HALF_SCREEN_SIZE_X;
  render_.grid_dst.y = render_.scale * (y - camera_.pos.y) + HALF_SCREEN_SIZE_Y;

  SDL_RenderCopy(
    render_.renderer, render_.textures["ovr"], 
    &render_.grid_src, &render_.grid_dst
  );
}


void 
RenderSystem::render_scrollable(Scrollable& el) 
const
{
  render_scalable(el.base);

  SDL_RenderSetClipRect(render_.renderer, &el.mask);

  SDL_RenderCopy(
    render_.renderer, render_.textures[el.list.texture], 
    nullptr, &el.list.bounds
  );

  SDL_RenderSetClipRect(render_.renderer, nullptr);

  if (el.scrollbar.active) render_scrollbar(el.scrollbar);
}


void 
RenderSystem::render_text(Text& el) 
const
{
  SDL_Texture*& texture{render_.textures[el.texture]};

  SDL_RenderCopy(render_.renderer, texture, nullptr, &el.bounds); 
}


void 
RenderSystem::render_button(Button& el) 
const
{
  el.active ? render_scalable(el.pressed) : render_scalable(el.base);

  render_text(el.label);
}


void 
RenderSystem::render_window(Window& el) 
const
{
  render_scalable(el.base);
}


void 
RenderSystem::render_scrollbar(Scrollbar& el) 
const
{
  SDL_Texture*& texture{render_.textures[el.texture]};

  SDL_RenderCopy(render_.renderer, texture, &el.src["t"], &el.dst["t"]);
  SDL_RenderCopy(render_.renderer, texture, &el.src["m"], &el.dst["m"]);
  SDL_RenderCopy(render_.renderer, texture, &el.src["b"], &el.dst["b"]);
}

void 
RenderSystem::render_scalable(Scalable& el) 
const
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
