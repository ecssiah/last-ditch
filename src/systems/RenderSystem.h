#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
/* #include <SDL2/SDL_ttf.h> */

class RenderSystem
{
public:
  RenderSystem();
  ~RenderSystem();

  void Initialize();
  void Update(const double& dt);

private:
  SDL_Window* window;
  SDL_Renderer* renderer;
};

#endif // RENDER_SYSTEM_H
