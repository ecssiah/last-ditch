#include "../../include/ui/UISystem.h"

#include <iostream>
#include <iomanip>
#include <string>
#include <sstream>
#include <algorithm>

#include "../../include/constants/UIConstants.h"
#include "../../include/constants/RenderConstants.h"
#include "../../include/constants/MapConstants.h"
#include "../../include/utility/Logging.h"
#include "../../include/ui/UI.h"

using namespace std;

UISystem::UISystem(Input& input, Map& map, Time& time, Log& log, UI& ui)
  : input_{input}
  , map_{map}
  , time_{time}
  , log_{log}
  , ui_{ui}
{
}


void UISystem::init()
{
  cout << "UISystem initializing" << endl;

  setup_floor_display();
  setup_time_display();
  setup_date_display();

  setup_main_window();
  setup_main_buttons();
  setup_message_window();
}


void UISystem::setup_main_window()
{
  auto& el{ui_.window_elements["main"]};
  el.changed = true;
  el.type = "window1";

  el.bounds.x = 0.1 * SCREEN_SIZE_X;
  el.bounds.y = 0.1 * SCREEN_SIZE_Y;
  el.bounds.w = 0.8 * SCREEN_SIZE_X;  
  el.bounds.h = 0.8 * SCREEN_SIZE_Y;  

  setup_window(el);
}


void UISystem::setup_main_buttons()
{
  auto& main_button_set{ui_.button_set_elements["main_buttons"]};
  auto& info{main_button_set.buttons["info"]};
  auto& save{main_button_set.buttons["save"]};
  auto& options{main_button_set.buttons["options"]};

  main_button_set.changed = true;

  info.active = true;
  info.type = "button2";
  info.bounds.x = .25 * SCREEN_SIZE_X - MAIN_BUTTON_WIDTH / 2;
  info.bounds.y = .11 * SCREEN_SIZE_Y;
  info.bounds.w = MAIN_BUTTON_WIDTH;
  info.bounds.h = MAIN_BUTTON_HEIGHT;

  info.label.font = "Small";
  info.label.content = "Info";
  info.label.texture = "info_button";

  save.type = "button2";
  save.bounds.x = .50 * SCREEN_SIZE_X - MAIN_BUTTON_WIDTH / 2;
  save.bounds.y = .11 * SCREEN_SIZE_Y;
  save.bounds.w = MAIN_BUTTON_WIDTH;
  save.bounds.h = MAIN_BUTTON_HEIGHT;

  save.label.font = "Small";
  save.label.content = "Save/Load";
  save.label.texture = "save_button";

  options.type = "button2";
  options.bounds.x = .75 * SCREEN_SIZE_X - MAIN_BUTTON_WIDTH / 2;
  options.bounds.y = .11 * SCREEN_SIZE_Y;
  options.bounds.w = MAIN_BUTTON_WIDTH;
  options.bounds.h = MAIN_BUTTON_HEIGHT;

  options.label.font = "Small";
  options.label.content = "Options";
  options.label.texture = "options_button";

  setup_button_set(main_button_set);
}


void UISystem::setup_message_window()
{
  auto& el{ui_.scrollable_elements["message_window"]};
  el.changed = true;
  el.id = "message_window";
  el.base_type = "window2";
  el.scrollbar_type = "scrollbar1";
  el.list_font = "Small";
  el.list_items = log_.msgs;
  el.pad = {MESSAGE_PADDING_X, MESSAGE_PADDING_Y};
  el.bounds = {
    SCREEN_SIZE_X - MESSAGE_WIN_SIZE_X, SCREEN_SIZE_Y - MESSAGE_WIN_SIZE_Y,
    MESSAGE_WIN_SIZE_X, MESSAGE_WIN_SIZE_Y
  };

  setup_scrollable(el);
}


void UISystem::setup_floor_display()
{
  auto& el{ui_.text_elements["floor_display"]};
  el.changed = true;
  el.id = "floor_display";
  el.font = "Small";
  el.texture = el.id;
  el.content = format_floor();
  el.bounds.w = FONT_WIDTH_SMALL * el.content.size();
  el.bounds.h = FONT_HEIGHT_SMALL;
  el.bounds.x = UI_PADDING;
  el.bounds.y = UI_PADDING;
}


void UISystem::setup_time_display()
{
  auto& el{ui_.text_elements["time_display"]};
  el.changed = true;
  el.id = "time_display";
  el.font = "Small";
  el.texture = el.id;
  el.content = format_time();
  el.bounds.w = FONT_WIDTH_SMALL * el.content.size();
  el.bounds.h = FONT_HEIGHT_SMALL;
  el.bounds.x = SCREEN_SIZE_X - UI_PADDING - el.bounds.w;
  el.bounds.y = UI_PADDING;
}


void UISystem::setup_date_display()
{
  auto& el{ui_.text_elements["date_display"]};
  el.changed = true;
  el.id = "date_display";
  el.font = "Large";
  el.texture = el.id;
  el.content = format_date();
  el.bounds.w = FONT_WIDTH_SMALL * el.content.size();
  el.bounds.h = FONT_HEIGHT_SMALL;
  el.bounds.x = SCREEN_SIZE_X - UI_PADDING - el.bounds.w;
  el.bounds.y = UI_PADDING + FONT_HEIGHT_SMALL;
}


void UISystem::setup_window(Window& el)
{
  el.base.type = "window1";
  el.base.texture = "overlay";
  el.base.bounds = el.bounds;

  setup_scalable(el.base);
}


void UISystem::setup_button(Button& el)
{
  el.changed = true;

  el.base.type = el.type + "-off";
  el.base.texture = "overlay";
  el.base.bounds = el.bounds;

  el.pressed.type = el.type + "-on";
  el.pressed.texture = "overlay";
  el.pressed.bounds = el.bounds;

  el.label.font = "Medium";
  el.label.bounds.w = FONT_WIDTH_MEDIUM * el.label.content.size();
  el.label.bounds.h = FONT_HEIGHT_MEDIUM;
  el.label.bounds.x = el.bounds.x + el.bounds.w / 2 - el.label.bounds.w / 2;
  el.label.bounds.y = el.bounds.y + el.bounds.h / 2 - el.label.bounds.h / 2;

  setup_scalable(el.base);
  setup_scalable(el.pressed);
}


void UISystem::setup_button_set(ButtonSet& el)
{
  for (auto& kv : el.buttons) setup_button(kv.second);
}


void UISystem::setup_scrollable(Scrollable& el)
{
  el.base.type = el.base_type;
  el.base.texture = "overlay";
  el.base.bounds = el.bounds;

  el.list.font = el.list_font;
  el.list.texture = el.id;
  el.list.items = el.list_items;

  el.scrollbar.type = el.scrollbar_type;
  el.scrollbar.texture = "overlay";

  el.mask = {
    el.bounds.x + el.pad.x, el.bounds.y + el.pad.y, 
    el.bounds.w - 2 * el.pad.x - SCROLLBAR_WIDTH, el.bounds.h - 2 * el.pad.y
  };

  setup_scalable(el.base);
  setup_scrollbar(el.scrollbar);
}


void UISystem::setup_scrollbar(Scrollbar& el)
{
  if (map_.tile_data.find(el.type) != map_.tile_data.end()) {
    el.basex = {(i32)(SCROLLBAR_WIDTH * map_.tile_data[el.type].uv.x)};
    el.basey = {(i32)(TILE_SIZE * map_.tile_data[el.type].uv.y)};
  } else {
    el.basex = 0;
    el.basey = 0;
    std::cerr << "Scrollbar has invalid type: " << el.type << std::endl;
  }

  el.src["t"] = {
    el.basex, el.basey + 0 * el.size, el.size, el.size
  };
  el.src["m"] = {
    el.basex, el.basey + 1 * el.size, el.size, el.size
  };
  el.src["b"] = {
    el.basex, el.basey + 2 * el.size, el.size, el.size
  };
}


void UISystem::setup_scalable(Scalable& el)
{
  if (map_.tile_data.find(el.type) != map_.tile_data.end()) {
    el.basex = {(i32)(TILE_SIZE * map_.tile_data[el.type].uv.x)};
    el.basey = {(i32)(TILE_SIZE * map_.tile_data[el.type].uv.y)};
    el.border = map_.tile_data[el.type].border;
  } else {
    el.basex = 0;
    el.basey = 0;
    std::cerr << "Scalable has invalid type: " << el.type << std::endl;
  }

  el.src["tl"] = {
    el.basex + 0 * el.size, el.basey + 0 * el.size, el.size, el.size
  };
  el.src["tm"] = {
    el.basex + 1 * el.size, el.basey + 0 * el.size, el.size, el.size
  };
  el.src["tr"] = {
    el.basex + 2 * el.size, el.basey + 0 * el.size, el.size, el.size
  };
  el.src["ll"] = {
    el.basex + 0 * el.size, el.basey + 1 * el.size, el.size, el.size
  };
  el.src["mm"] = {
    el.basex + 1 * el.size, el.basey + 1 * el.size, el.size, el.size
  };
  el.src["rr"] = {
    el.basex + 2 * el.size, el.basey + 1 * el.size, el.size, el.size
  };
  el.src["bl"] = {
    el.basex + 0 * el.size, el.basey + 2 * el.size, el.size, el.size
  };
  el.src["bm"] = {
    el.basex + 1 * el.size, el.basey + 2 * el.size, el.size, el.size
  };
  el.src["br"] = {
    el.basex + 2 * el.size, el.basey + 2 * el.size, el.size, el.size
  };
}


void UISystem::update()
{
  resolve_selections();

  update_menu();
  update_hud();
  update_messages();
}


void UISystem::resolve_selections()
{
  if (input_.menu) {
    update_main_buttons();
  } else if (input_.hud) {
    update_message_window();
  }
}


void UISystem::update_menu()
{
}


void UISystem::update_hud()
{
  if (map_.floor_changed) {
    auto& el{ui_.text_elements["floor_display"]};
    el.changed = true;
    el.content = format_floor();
  }

  if (time_.time_changed) {
    auto& el{ui_.text_elements["time_display"]};
    el.changed = true;
    el.content = format_time();
  }

  if (time_.date_changed) {
    auto& el{ui_.text_elements["date_display"]};
    el.changed = true;
    el.content = format_date();
  }
}


void UISystem::update_messages()
{
  if (log_.changed) {
    log_.changed = false;

    update_scrollable_items(
      ui_.scrollable_elements["message_window"], log_.msgs
    );
  }
}


void UISystem::update_main_buttons()
{
  if (input_.lclick) {
    input_.lclick = false;

    update_button_set(ui_.button_set_elements["main_buttons"]);
  }
}


void UISystem::update_message_window()
{
  auto& el{ui_.scrollable_elements["message_window"]};

  if (input_.lreleased) {
    el.scrollbar.selected = false;
  } else if (input_.touch_points == 2) {
    const bool msg_win_contained{check_intersection(input_.mx, input_.my, el)};

    if (msg_win_contained) update_scrollable(el, -SCROLL_SPEED * input_.tdy);
  } else {
    const bool msg_win_clicked{check_intersection(input_.mx, input_.my, el)};

    if (msg_win_clicked) {
      if (el.scrollbar.active) {
        const bool scrollbar_clicked{
          check_intersection(input_.mx, input_.my, el.scrollbar)
        };

        if (input_.lclick && scrollbar_clicked) el.scrollbar.selected = true;

        if (el.scrollbar.selected)
          update_scrollable(el, input_.mdy / (f32)el.scroll_range);
      }

      input_.lclick = false;
    }
  }
}


void UISystem::update_button_set(ButtonSet& el)
{
  el.changed = true;

  string choice;
  for (auto& kv : el.buttons)
    if (check_intersection(input_.mx, input_.my, kv.second))
      choice = kv.first;

  if (!choice.empty())
    for (auto& kv : el.buttons)
      kv.second.active = (kv.first == choice) ? true : false; 
}


void UISystem::update_scrollable(Scrollable& el, f32 ds)
{
  el.changed = true;

  f32 test_pos{el.pos + ds};
  el.pos = max(0.0f, min(test_pos, 1.0f));
}


void UISystem::update_scrollable_items(Scrollable& el, vector<string> items)
{
  el.changed = true;

  el.pos = 0.0;
  el.list_items = el.list.items = items;
}


const bool UISystem::check_intersection(i32 x, i32 y, Element& el) const
{
  const bool lcheck{input_.mx > el.bounds.x};
  const bool rcheck{input_.mx < el.bounds.x + el.bounds.w};
  const bool tcheck{input_.my > el.bounds.y};
  const bool bcheck{input_.my < el.bounds.y + el.bounds.h};

  return lcheck && rcheck && tcheck && bcheck;
}


const string UISystem::format_time() const
{
  stringstream ss;
  ss << setfill('0');
  ss << setw(2) << time_.hour << ":";
  ss << setw(2) << time_.minute << ":"; 
  ss << setw(2) << time_.second; 

  return ss.str();
}


const string UISystem::format_date() const
{
  stringstream ss;
  ss << setfill('0');
  ss << setw(2) << time_.day << "/"; 
  ss << setw(2) << time_.month << "/";
  ss << setw(2) << time_.year;

  return ss.str();
}


const string UISystem::format_floor() const
{
  return to_string(map_.cur_floor) + "F";
}

