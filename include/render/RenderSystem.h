#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <string>
#include <unordered_map>

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>

#include "../utility/Types.h"
#include "../interface/Input.h"
#include "../render/Render.h"
#include "../render/Camera.h"
#include "../map/Map.h"

class RenderSystem
{
public:
  RenderSystem(
    Input& input, Render& render, Camera& camera, Map& map
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

  void render_tile(const std::string& layer, i32 x, i32 y, i32 floor);

  void render_scalable(Scalable& el);

  void render_window(const std::string& id);
  void render_button(const std::string& id);
  void render_text(const std::string& id);
  void render_scrollable(const std::string& id);

  void render_map();
  void render_ui();

  void render_messages();

  Render& render_;
  Input& input_;
  Camera& camera_;
  Map& map_;

};

#endif
