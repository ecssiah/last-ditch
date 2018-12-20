#ifndef UI_SYSTEM_H
#define UI_SYSTEM_H

#include <string>
#include <unordered_map>
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

#include "../components/Map.h"
#include "../components/Render.h"
#include "../components/Input.h"

class UISystem
{
public:
  UISystem(Input& input, Render& render, Map& map);

  void Initialize();
  void Update();

private:
  void LoadFonts();

  void InitializeSDLTTF();

  void UpdateFloorDisplay();

  TTF_Font* LoadFont(std::string fontname);

  Input& input_;
  Render& render_;
  Map& map_;

  SDL_Color floor_text_color_;
  SDL_Rect floor_text_dst_;
  SDL_Texture* floor_text_tex_;

  std::unordered_map<std::string, TTF_Font*> fonts_;

};

#endif // UI_SYSTEM_H
