#ifndef UI_SYSTEM_H
#define UI_SYSTEM_H

#include <string>
#include <unordered_map>
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

#include "Text.h"
#include "Window.h"
#include "Button.h"
#include "../Types.h"
#include "../render/Render.h"
#include "../Input.h"
#include "../Time.h"
#include "../map/Map.h"

class UISystem
{
public:
  UISystem(Input& input, Render& render, Map& map, Time& time);

  void init();
  void update();

private:
  void init_SDL_ttf();
  TTF_Font* load_font(const std::string& fontname, u32 size);
  void load_fonts();

  void setup_main_window();
  void setup_main_buttons();
  void setup_floor_display();
  void setup_time_display();
  void setup_date_display();

  std::string format_floor();
  std::string format_time();
  std::string format_date();

  void update_main_text();

  void build_window_element(const std::string& id);
  void build_text_element(const std::string& id);
  void build_button_element(const std::string& id);

  void render_window_element(const std::string& id);
  void render_text_element(const std::string& id);
  void render_button_element(const std::string& id);

  bool check_intersection(Element& el, i32 x, i32 y);

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