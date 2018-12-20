#ifndef UI_SYSTEM_H
#define UI_SYSTEM_H

#include <string>
#include <unordered_map>
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

#include "../components/Render.h"
#include "../components/Input.h"
#include "../components/map/Map.h"
#include "../components/ui/TextElement.h"
#include "../components/ui/WindowElement.h"

class UISystem
{
public:
  UISystem(Input& input, Render& render, Map& map);

  void Initialize();
  void Update();

private:
  void InitializeSDLTTF();
  TTF_Font* LoadFont(std::string fontname, unsigned size);
  void LoadFonts();

  void SetupFloorDisplay();

  void BuildTextElement(TextElement& element);
  void RenderTextElement(const TextElement& element);
  void RenderWindowElement(const WindowElement& element);

  Input& input_;
  Render& render_;
  Map& map_;

  std::unordered_map<std::string, TTF_Font*> fonts_;

  std::unordered_map<std::string, TextElement> text_elements_;
  std::unordered_map<std::string, WindowElement> window_elements_;

};

#endif // UI_SYSTEM_H
