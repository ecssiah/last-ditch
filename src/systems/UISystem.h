#ifndef UI_SYSTEM_H
#define UI_SYSTEM_H

#include <string>
#include <unordered_map>
#include <SDL2/SDL_ttf.h>

#include "../components/Map.h"
#include "../components/Input.h"

class UISystem
{
public:
  UISystem(Input& input, Map& map);

  void Initialize();
  void Update();

private:
  void LoadFonts();

  void InitializeSDLTTF();

  TTF_Font* LoadFont(std::string fontname);

  Input& input_;
  Map& map_;

  std::unordered_map<std::string, TTF_Font*> fonts_;

};

#endif // UI_SYSTEM_H
