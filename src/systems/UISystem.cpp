#include "UISystem.h"

#include <iostream>
#include <string>

using namespace std;

UISystem::UISystem(Input& input, Map& map)
  : input_(input)
  , map_(map)
{
}


void UISystem::Initialize()
{
  InitializeSDLTTF();

  LoadFonts();
}


void UISystem::Update()
{

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

