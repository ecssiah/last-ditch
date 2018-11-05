#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <string>
#include <unordered_map>

#include <SDL2/SDL.h>

#include "../components/Render.h"
#include "../components/Input.h"
#include "../components/Camera.h"
#include "../components/Map.h"
#include "../components/Chunk.h"

class RenderSystem
{
public:
  RenderSystem(Input& input, Render& render, Camera& camera, Map& map);
  ~RenderSystem();

  void Initialize();
  void Update();

private:
  Render& render_;
  Input& input_;
  Camera& camera_;
  Map& map_;


  SDL_Window* window;
};

#endif // RENDER_SYSTEM_H
