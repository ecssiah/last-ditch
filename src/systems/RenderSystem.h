#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <string>
#include <unordered_map>

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>

#include "../Types.h"
#include "../components/Render.h"
#include "../components/Input.h"
#include "../components/Camera.h"
#include "../components/map/Map.h"

class RenderSystem
{
public:
  RenderSystem(
    Input& input, Render& render, Camera& camera, Map& map
  );
  ~RenderSystem();

  void init();
  void update();
  void display();

private:
  void init_SDL();
  void init_SDL_image();
  void load_tilesets();
  SDL_Texture* load_texture(const std::string& texturename);

  void render_map();
  void render_tile(const std::string& layer, I32 x, I32 y);

  Render& render_;
  Input& input_;
  Camera& camera_;
  Map& map_;

};

#endif
