#ifndef GAME_H
#define GAME_H

#include <iostream>
#include <Eigen/Geometry>
#include "SDL.h" 

class Game {

public:
  Game();
  ~Game();

  void init(
    const char* title, 
    int xpos, int ypos, int width, int height, 
    bool fullscreen
  );
  void handle_events();
  void update();
  void render();
  void clean();

  bool running() { return is_running; }

private:
  bool is_running;
  SDL_Window* window;
  SDL_Renderer* renderer;

};

#endif // GAME_H

