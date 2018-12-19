#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <string>
#include <unordered_map>

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <SDL2/SDL_ttf.h>

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
  void InitializeSDLTTF();
  void LoadTilesets();
  void LoadFonts();

  void RenderMap();
  void RenderTile(std::string layer, int x, int y);

  SDL_Texture* LoadTexture(std::string texturename);
  TTF_Font* LoadFont(std::string fontname);

  Render& render_;
  Input& input_;
  Camera& camera_;
  Map& map_;

  SDL_Window* window_;
  SDL_Renderer* renderer_;

  std::unordered_map<std::string, TTF_Font*> fonts_;
  std::unordered_map<std::string, SDL_Texture*> tilesets_;
};

#endif // RENDER_SYSTEM_H
