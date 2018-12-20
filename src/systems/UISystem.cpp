#include "UISystem.h"

#include <iostream>
#include <string>

using namespace std;

UISystem::UISystem(Input& input, Render& render, Map& map)
  : input_(input)
  , render_(render)
  , map_(map)
  , floor_text_color_({240, 240, 255})
{
}


void UISystem::Initialize()
{
  InitializeSDLTTF();
  LoadFonts();

  floor_text_dst_ = {0, 0, 20, 16};

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
  string floor_text{to_string(map_.cur_floor)};

  SDL_Surface* floor_display_sur = TTF_RenderUTF8_Blended(
    fonts_["OpenSans-Regular"], floor_text.c_str(), floor_text_color_ 
  ); 

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
  fonts_["OpenSans-Regular"] = LoadFont("OpenSans-Regular");
  fonts_["Fantasque-Regular"] = LoadFont("FantasqueSansMono-Regular");
}


TTF_Font* UISystem::LoadFont(string fontname)
{
  string font_path{"assets/fonts/" + fontname + ".ttf"};
  TTF_Font* font{TTF_OpenFont(font_path.c_str(), 14)};

  if (!font) {
    cout << "TTF_OpenFont error: " << TTF_GetError() << endl;
    return nullptr;
  }

  return font;
}

