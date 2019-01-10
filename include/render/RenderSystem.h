#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <string>
#include <unordered_map>

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>

#include "../ui/UI.h"
#include "../ui/Scalable.h"
#include "../ui/Scrollbar.h"
#include "../utility/Types.h"
#include "../interface/Input.h"
#include "../render/Render.h"
#include "../render/Camera.h"
#include "../map/Map.h"

class RenderSystem
{
public:
  RenderSystem(
    Input& input, Render& render, Camera& camera, Map& map, UI& ui
  );
  ~RenderSystem();

  void init();
  void update();

private:
  void init_SDL();
  void init_SDL_image();
  void init_SDL_ttf();

  SDL_Texture* load_texture(const std::string& texturename);
  TTF_Font* load_font(const std::string& fontname, u32 size);

  void load_fonts();
  void load_tilesets();

  void build_elements();

  void build_window(Window& el);
  void build_button(Button& el);
  void build_text(Text& el);
  void build_scrollable(Scrollable& el);
  void build_scrollbar(Scrollbar& el);
  void build_scalable(Scalable& el);

  void render_window(Window& el);
  void render_button(Button& el);
  void render_scrollable(Scrollable& el);
  void render_scrollbar(Scrollbar& el);
  void render_scalable(Scalable& el);
  void render_text(Text& el);

  void render_tile(const std::string& layer, i32 x, i32 y, i32 floor);

  void render_map();
  void render_ui();

  Render& render_;
  Input& input_;
  Camera& camera_;
  Map& map_;
  UI& ui_;

};

#endif
