#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <string>
#include <unordered_map>

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>

#include "../components/Render.h"
#include "../components/Input.h"
#include "../components/Camera.h"
#include "../components/Time.h"
#include "../components/map/Map.h"
#include "../systems/UISystem.h"

class RenderSystem
{
public:
  RenderSystem(
    Input& input, Render& render, Camera& camera, Map& map, Time& time
  );
  ~RenderSystem();

  void Initialize();
  void Update();

private:
  void InitializeSDL();
  void InitializeSDLImage();
  void LoadTilesets();

  void RenderMap();
  void RenderTile(std::string layer, int x, int y);

  UISystem ui_system_;

  SDL_Texture* LoadTexture(std::string texturename);

  Render& render_;
  Input& input_;
  Camera& camera_;
  Map& map_;

  std::unordered_map<std::string, SDL_Texture*> tilesets_;
};

#endif // RENDER_SYSTEM_H
