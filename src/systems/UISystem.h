#ifndef UI_SYSTEM_H
#define UI_SYSTEM_H

#include <string>
#include <unordered_map>
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

#include "../components/Render.h"
#include "../components/Input.h"
#include "../components/Time.h"
#include "../components/map/Map.h"
#include "../components/ui/Text.h"
#include "../components/ui/Window.h"
#include "../components/ui/Button.h"

class UISystem
{
public:
  UISystem(Input& input, Render& render, Map& map, Time& time);

  void Initialize();
  void Update();

private:
  void InitializeSDLTTF();
  TTF_Font* LoadFont(std::string fontname, unsigned size);
  void LoadFonts();

  void SetupMainWindow();
  void SetupMainButtons();
  void SetupFloorDisplay();
  void SetupTimeDisplay();
  void SetupDateDisplay();

  std::string FormatTime();
  std::string FormatDate();

  void UpdateMainText();

  void BuildWindowElement(std::string id);
  void BuildTextElement(std::string id);
  void BuildButtonElement(std::string id);

  void RenderWindowElement(std::string id);
  void RenderTextElement(std::string id);
  void RenderButtonElement(std::string id);

  Input& input_;
  Render& render_;
  Map& map_;
  Time& time_;

  std::unordered_map<std::string, TTF_Font*> fonts_;

  std::unordered_map<std::string, Text> text_elements_;
  std::unordered_map<std::string, Window> window_elements_;
  std::unordered_map<std::string, Button> button_elements_;

};

#endif
