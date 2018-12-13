#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <string>
#include <unordered_map>

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>

#include "../components/Render.h"
#include "../components/Input.h"
#include "../components/Camera.h"
#include "../components/Map.h"

class RenderSystem
{
public:
  RenderSystem(Input& input, Render& render, Camera& camera, Map& map);
  ~RenderSystem();

  void Initialize();
  void Update();

private:
  void InitializeSDL();
  void InitializeSDLImage();
  void LoadTilesets();

  void RenderMap();

  SDL_Texture* LoadTexture(std::string texturename);

  Render& render_;
  Input& input_;
  Camera& camera_;
  Map& map_;

  SDL_Window* window_;
  SDL_Renderer* renderer_;

  SDL_Surface* surface_;

  SDL_Texture* map_tileset_;
  SDL_Texture* object_tileset_;
  SDL_Texture* character_tileset_;
};

#endif // RENDER_SYSTEM_H
