#include "UISystem.h"

#include <iostream>
#include <string>

#include "../constants/RenderConstants.h"

using namespace std;

UISystem::UISystem(Input& input, Render& render, Map& map)
  : input_{input}
  , render_{render}
  , map_{map}
  , fonts_{}
  , text_elements_{}
  , window_elements_{}
{
}


void UISystem::Initialize()
{
  InitializeSDLTTF();
  LoadFonts();

  SetupFloorDisplay();
}


void UISystem::Update()
{
  if (map_.floor_changed) {
    text_elements_["floor_display"].text = to_string(map_.cur_floor + 1);
    BuildTextElement(text_elements_["floor_display"]);
  }

  for (auto kv : text_elements_) RenderTextElement(kv.second);
}


void UISystem::InitializeSDLTTF()
{
  if (TTF_Init()) {
    cout << "TTF_Init: " << TTF_GetError() << endl;  
    return;
  } 
}


void UISystem::LoadFonts()
{
  fonts_["Fantasque-Small"] = LoadFont("FantasqueSansMono-Regular", 14);
  fonts_["Fantasque-Medium"] = LoadFont("FantasqueSansMono-Regular", 18);
  fonts_["Fantasque-Large"] = LoadFont("FantasqueSansMono-Regular", 22);
}


TTF_Font* UISystem::LoadFont(string fontname, unsigned size)
{
  string font_path{"assets/fonts/" + fontname + ".ttf"};
  TTF_Font* font{TTF_OpenFont(font_path.c_str(), size)};

  if (!font) {
    cout << "TTF_OpenFont error: " << TTF_GetError() << endl;
    return nullptr;
  }

  return font;
}


void UISystem::BuildTextElement(TextElement& element)
{
  SDL_Surface* surface{TTF_RenderUTF8_Blended(
    element.font, element.text.c_str(), element.color
  )}; 

  element.rect.w = surface->w;
  element.rect.h = surface->h;

  if (surface == nullptr) {
    cerr << "TTF_RenderUTF8_Blended error: " << TTF_GetError() << endl; 
  } else {
    element.texture = SDL_CreateTextureFromSurface(render_.renderer, surface); 
  }
}


void UISystem::RenderTextElement(const TextElement& text_element)
{
  SDL_RenderCopy(
    render_.renderer, text_element.texture, nullptr, &text_element.rect
  ); 
}


void UISystem::SetupFloorDisplay()
{
  text_elements_["floor_display"].rect.x = 4;
  text_elements_["floor_display"].rect.y = 4;
  text_elements_["floor_display"].font = fonts_["Fantasque-Small"];
  text_elements_["floor_display"].text = to_string(map_.cur_floor + 1);

  BuildTextElement(text_elements_["floor_display"]);
}


