#ifndef RENDER_H
#define RENDER_H

#include <string>
#include <unordered_map>
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

#include "../utility/Types.h"
#include "../render/RenderConstants.h"
#include "../ui/Text.h"
#include "../ui/Scalable.h"
#include "../ui/Window.h"
#include "../ui/Button.h"
#include "../ui/Scrollable.h"

struct Render
{
  Render() 
    : dt{FRAME_TIME} 
    , window{nullptr}
    , renderer{nullptr}
    , fonts{}
    , textures{}
    , text_elements{}
    , scalable_elements{}
    , window_elements{}
    , button_elements{}
    , scrollable_elements{}
  {}

  f32 dt;

  SDL_Window* window;
  SDL_Renderer* renderer;

  std::unordered_map<std::string, TTF_Font*> fonts;
  std::unordered_map<std::string, SDL_Texture*> textures;

  std::unordered_map<std::string, Text> text_elements;
  std::unordered_map<std::string, Scalable> scalable_elements;
  std::unordered_map<std::string, Window> window_elements;
  std::unordered_map<std::string, Button> button_elements;
  std::unordered_map<std::string, Scrollable> scrollable_elements;

};

#endif
