#include "UISystem.h"

#include <iostream>
#include <string>

#include "../constants/RenderConstants.h"

using namespace std;

UISystem::UISystem(Input& input, Render& render, Map& map)
  : input_{input}
  , render_{render}
  , map_{map}
  , floor_text_color_{240, 240, 255}
{
}


void UISystem::Initialize()
{
  InitializeSDLTTF();
  LoadFonts();

  floor_text_dst_.x = 4;
  floor_text_dst_.y = 4;

  UpdateFloorDisplay();
}


void UISystem::Update()
{
  if (map_.floor_changed) {
    map_.floor_changed = false;
    UpdateFloorDisplay();
  }

  SDL_RenderCopy(
    render_.renderer, floor_text_tex_, nullptr, &floor_text_dst_
  ); 
}


void UISystem::UpdateFloorDisplay()
{
  string floor_text{to_string(map_.cur_floor + 1)};

  SDL_Surface* floor_display_sur = TTF_RenderUTF8_Blended(
    fonts_["Fantasque-Regular"], floor_text.c_str(), floor_text_color_ 
  ); 

  floor_text_dst_.w = floor_display_sur->w;
  floor_text_dst_.h = floor_display_sur->h;

  if (floor_display_sur == nullptr) {
    cerr << "TTF_RenderUTF8_Blended error: " << TTF_GetError() << endl; 
  } else {
    floor_text_tex_ = SDL_CreateTextureFromSurface(
      render_.renderer, floor_display_sur
    ); 
  }
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
  fonts_["OpenSans-Regular"] = LoadFont("OpenSans-Regular", 14);
  fonts_["Fantasque-Regular"] = LoadFont("FantasqueSansMono-Regular", 14);
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

