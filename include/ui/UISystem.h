#ifndef UI_SYSTEM_H
#define UI_SYSTEM_H

#include <string>
#include <unordered_map>
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

#include "UI.h"
#include "Text.h"
#include "Scalable.h"
#include "Scrollbar.h"
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
  UISystem(Input& input, Map& map, Time& time, Log& log, UI& ui);

  void init();
  void update();

private:
  void setup_main_window();
  void setup_main_buttons();
  void setup_message_window();

  void setup_floor_display();
  void setup_time_display();
  void setup_date_display();

  void setup_button(Button& el);
  void setup_scrollbar(Scrollbar& el);
  void setup_scalable(Scalable& el);

  void update_menu();
  void update_hud();
  void update_messages();

  void resolve_selections();

  void update_main_buttons(); 
  void update_message_window();

  bool check_intersection(i32 x, i32 y, Element& el);

  std::string format_floor();
  std::string format_time();
  std::string format_date();

  Input& input_;
  Map& map_;
  Time& time_;
  Log& log_;
  UI& ui_;

};

#endif
