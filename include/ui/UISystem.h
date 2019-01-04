#ifndef UI_SYSTEM_H
#define UI_SYSTEM_H

#include <string>
#include <unordered_map>
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

#include "Text.h"
#include "Scalable.h"
#include "Window.h"
#include "Button.h"
#include "../utility/Types.h"
#include "../utility/Log.h"
#include "../interface/Input.h"
#include "../time/Time.h"
#include "../render/Render.h"
#include "../map/Map.h"

class UISystem
{
public:
  UISystem(Input& input, Render& render, Map& map, Time& time, Log& log);

  void init();
  void update();

private:
  void setup_main_window();
  void setup_main_buttons();
  void setup_floor_display();
  void setup_time_display();
  void setup_date_display();

  std::string format_floor();
  std::string format_time();
  std::string format_date();

  void update_menu();
  void update_main_text();
  void update_messages();

  void build_window_element(const std::string& id);
  void build_text_element(const std::string& id);
  void build_button_element(const std::string& id);

  void build_scalable_element(Scalable& el);

  bool check_intersection(i32 x, i32 y, Element& el);

  Input& input_;
  Render& render_;
  Map& map_;
  Time& time_;
  Log& log_;

};

#endif
