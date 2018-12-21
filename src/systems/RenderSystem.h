#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <string>
#include <unordered_map>

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>

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

  void Initialize();
  void Update();
  void Display();

private:
  void InitializeSDL();
  void InitializeSDLImage();
  void LoadTilesets();
  SDL_Texture* LoadTexture(std::string texturename);

  void RenderMap();
  void RenderTile(std::string layer, int x, int y);

  Render& render_;
  Input& input_;
  Camera& camera_;
  Map& map_;

};

#endif
