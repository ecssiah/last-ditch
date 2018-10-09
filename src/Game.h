#ifndef GAME_H
#define GAME_H

#include <iostream>
#include <Eigen/Geometry>
#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>

#include "GameObject.h"

class Game 
{
public:
  Game();
  ~Game();

  void Init(
    const char* title, 
    int xpos, int ypos, int width, int height, 
    bool fullscreen
  );
  void HandleEvents();
  void Update();
  void Render();
  void Clean();

  bool IsRunning() { return is_running; }

private:
  bool is_running;

  GameObject* player;

  SDL_Window* window;
  SDL_Renderer* renderer;
};

#endif // GAME_H

