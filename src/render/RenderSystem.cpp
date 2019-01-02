#include <iostream>
#include <string>
#include <vector>
#include <fstream>
#include <algorithm>
#include <functional>

#include <SDL2/SDL.h>
#include <glm/glm.hpp>

#include "../../include/render/RenderSystem.h"
#include "../../include/render/RenderConstants.h"
#include "../../include/map/MapConstants.h"

using namespace std;

RenderSystem::RenderSystem(
  Input& input, Render& render, Camera& camera, Map& map
) 
  : input_{input}
  , render_{render}
  , camera_{camera}
  , map_{map}
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
  cout << "RenderSystem init" << endl;

  init_SDL();
  init_SDL_image();
  init_SDL_ttf();

  load_fonts();
  load_tilesets();
}


void RenderSystem::init_SDL()
{
  if (SDL_Init(SDL_INIT_VIDEO) != 0) {
    cout << "SDL_Init Error: " << SDL_GetError() << endl;
    return;
  }

  render_.window = SDL_CreateWindow(
    "Last Ditch", 
    SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 
    SCREEN_SIZE_X, SCREEN_SIZE_Y, 
    SDL_WINDOW_SHOWN
  );

  if (render_.window == nullptr) {
    cout << "SDL_CreateWindow error: " << SDL_GetError() << endl;
    SDL_Quit();
    return;
  }

  render_.renderer = SDL_CreateRenderer(
    render_.window, -1, 
    SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC
  );

  if (render_.renderer == nullptr){
    SDL_DestroyWindow(render_.window);
    SDL_Quit();

    cout << "SDL_CreateRenderer error: " << SDL_GetError() << endl;
    return;
  }

  SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "linear");
}


void RenderSystem::init_SDL_image()
{
  const i32 img_flags{IMG_INIT_PNG};
  
  if (!(IMG_Init(img_flags) & img_flags)) {
    cout << "SDL_image error: " << IMG_GetError() << endl;
    return;
  }
}


void RenderSystem::init_SDL_ttf()
{
  if (TTF_Init()) {
    cout << "TTF_Init: " << TTF_GetError() << endl;  
    return;
  } 
}


SDL_Texture* RenderSystem::load_texture(const string& texturename)
{
  string filename{"assets/textures/" + texturename + ".png"};
  SDL_Surface* surface{IMG_Load(filename.c_str())};

  if (!surface) { 
    cout << "IMG_Load error: " << IMG_GetError() << endl;
    return nullptr;
  }

  SDL_Texture* texture{SDL_CreateTextureFromSurface(render_.renderer, surface)};

  if (!texture) {
    cout << "SDL_CreateTextureFromSurface error: " << SDL_GetError() << endl;
    return nullptr;
  }
  
  SDL_FreeSurface(surface);

  return texture;
}


TTF_Font* RenderSystem::load_font(const string& fontname, u32 size)
{
  string fontpath{"assets/fonts/" + fontname + ".ttf"};
  TTF_Font* font{TTF_OpenFont(fontpath.c_str(), size)};

  if (!font) {
    cout << "TTF_OpenFont error: " << TTF_GetError() << endl;
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
}


void RenderSystem::update()
{
  SDL_RenderClear(render_.renderer);

  render_map(); 
  render_ui();

  SDL_RenderPresent(render_.renderer);
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
      render_tile("floor", x, y);
      render_tile("wall", x, y);
      render_tile("object", x, y);
      render_tile("entity", x, y);
      render_tile("overlay", x, y);
    }
  }
}


void RenderSystem::render_ui()
{
  if (input_.menu) {
    render_window_element("main");

    render_button_element("info");
    render_button_element("save");
    render_button_element("options");
  }

  render_text_element("floor_display");
  render_text_element("time_display");
  render_text_element("date_display");
}


void RenderSystem::render_tile(const string& layer, i32 x, i32 y)
{
  const Tile& tile{map_.floors[map_.cur_floor].layers[layer].tiles[x][y]};

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


void RenderSystem::render_text_element(const string& id)
{
  const auto& el{render_.text_elements[id]};

  SDL_RenderCopy(render_.renderer, el.texture, nullptr, &el.bounds); 
}


void RenderSystem::render_button_element(const string& id)
{
  auto& el{render_.button_elements[id]};

  if (el.active) {
    render_scalable_element(el.pressed);
  } else {
    render_scalable_element(el.base);
  }

  render_text_element(id);
}


void RenderSystem::render_window_element(const string& id)
{
  auto& el{render_.window_elements[id]};

  render_scalable_element(el.base);
}


void RenderSystem::render_scalable_element(Scalable& el)
{
  SDL_RenderCopy(render_.renderer, el.texture, &el.src["tl"], &el.dst["tl"]);
  SDL_RenderCopy(render_.renderer, el.texture, &el.src["tm"], &el.dst["tm"]);
  SDL_RenderCopy(render_.renderer, el.texture, &el.src["tr"], &el.dst["tr"]);
  SDL_RenderCopy(render_.renderer, el.texture, &el.src["ll"], &el.dst["ll"]);
  SDL_RenderCopy(render_.renderer, el.texture, &el.src["mm"], &el.dst["mm"]);
  SDL_RenderCopy(render_.renderer, el.texture, &el.src["rr"], &el.dst["rr"]);
  SDL_RenderCopy(render_.renderer, el.texture, &el.src["bl"], &el.dst["bl"]);
  SDL_RenderCopy(render_.renderer, el.texture, &el.src["bm"], &el.dst["bm"]);
  SDL_RenderCopy(render_.renderer, el.texture, &el.src["br"], &el.dst["br"]);
}
