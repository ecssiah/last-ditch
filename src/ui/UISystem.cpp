#include "../../include/ui/UISystem.h"

#include <iostream>
#include <iomanip>
#include <string>
#include <sstream>
#include <algorithm>

#include "../../include/utility/Logging.h"
#include "../../include/ui/UIConstants.h"
#include "../../include/render/RenderConstants.h"

using namespace std;

UISystem::UISystem(Input& input, Render& render, Map& map, Time& time, Log& log)
  : input_{input}
  , render_{render}
  , map_{map}
  , time_{time}
  , log_{log}
{
}


void UISystem::init()
{
  mlog("UISystem initializing");

  setup_main_window();
  setup_main_buttons();
  setup_message_window();

  setup_floor_display();
  setup_time_display();
  setup_date_display();
}


void UISystem::update()
{
  update_menu();
  update_main_text();
  update_messages();
}


bool UISystem::check_intersection(i32 x, i32 y, Element& el)
{
  auto lcheck{input_.mx > el.bounds.x};
  auto rcheck{input_.mx < el.bounds.x + el.bounds.w};
  auto tcheck{input_.my > el.bounds.y};
  auto bcheck{input_.my < el.bounds.y + el.bounds.h};

  return lcheck && rcheck && tcheck && bcheck;
}


void UISystem::update_menu()
{
  if (input_.menu) {
    if (input_.lclick) {
      auto& info_btn{render_.button_elements["info"]};    
      auto& save_btn{render_.button_elements["save"]};    
      auto& options_btn{render_.button_elements["options"]};    

      if (check_intersection(input_.mx, input_.my, info_btn)) {
        info_btn.active = true;
        save_btn.active = false;
        options_btn.active = false;
      } else if (check_intersection(input_.mx, input_.my, save_btn)) {
        info_btn.active = false;
        save_btn.active = true;
        options_btn.active = false;
      } else if (check_intersection(input_.mx, input_.my, options_btn)) {
        info_btn.active = false;
        save_btn.active = false;
        options_btn.active = true;
      }
    }
  }
}


void UISystem::update_main_text()
{
  if (map_.floor_changed) {
    render_.text_elements["floor_display"].text = format_floor();
    build_text("floor_display");
  }

  if (time_.time_changed) {
    render_.text_elements["time_display"].text = format_time();
    build_text("time_display");
  }

  if (time_.date_changed) {
    render_.text_elements["date_display"].text = format_date();
    build_text("date_display");
  }
}


void UISystem::update_messages()
{
  if (log_.changed) {
    log_.changed = false;

  }
}


void UISystem::setup_main_window()
{
  auto& main_win{render_.window_elements["main"]};
  main_win.type = "window1";
  main_win.bounds.x = 0.1 * SCREEN_SIZE_X;
  main_win.bounds.y = 0.1 * SCREEN_SIZE_Y;
  main_win.bounds.w = 0.8 * SCREEN_SIZE_X;  
  main_win.bounds.h = 0.8 * SCREEN_SIZE_Y;  

  build_window("main");
}


void UISystem::setup_main_buttons()
{
  auto width{120};
  auto height{32};

  auto& info_btn{render_.button_elements["info"]};
  info_btn.active = true;
  info_btn.type = "button2";
  info_btn.text = "Info";
  info_btn.bounds.x = .25 * SCREEN_SIZE_X - width / 2;
  info_btn.bounds.y = .11 * SCREEN_SIZE_Y;
  info_btn.bounds.w = width;
  info_btn.bounds.h = height;

  build_button("info");

  auto& save_btn{render_.button_elements["save"]};
  save_btn.type = "button2";
  save_btn.text = "Save/Load";
  save_btn.bounds.x = .50 * SCREEN_SIZE_X - width / 2;
  save_btn.bounds.y = .11 * SCREEN_SIZE_Y;
  save_btn.bounds.w = width;
  save_btn.bounds.h = height;

  build_button("save");

  auto& options_btn{render_.button_elements["options"]};
  options_btn.type = "button2";
  options_btn.text = "Options";
  options_btn.bounds.x = .75 * SCREEN_SIZE_X - width / 2;
  options_btn.bounds.y = .11 * SCREEN_SIZE_Y;
  options_btn.bounds.w = width;
  options_btn.bounds.h = height;

  build_button("options");
}


void UISystem::setup_message_window()
{
  auto& el{render_.scrollable_elements["message_window"]};
  el.base.type = "window1";
  el.scrollbar.type = "scrollbar1";

  string full_msg;
  for (const auto& msg : log_.msgs) full_msg += msg + "\n"; 

  SDL_Surface* target_sur{TTF_RenderText_Blended_Wrapped(
    render_.fonts["Fantasque-Small"], 
    full_msg.c_str(), {255, 255, 255}, 
    MESSAGE_WIN_SIZE_X - 2 * MESSAGE_PADDING_X
  )};

  el.base.bounds = {
    SCREEN_SIZE_X - MESSAGE_WIN_SIZE_X, SCREEN_SIZE_Y - MESSAGE_WIN_SIZE_Y,
    MESSAGE_WIN_SIZE_X, MESSAGE_WIN_SIZE_Y
  };
  el.bounds = {
    el.base.bounds.x + MESSAGE_PADDING_X, el.base.bounds.y + MESSAGE_PADDING_Y,
    target_sur->w, target_sur->h
  };

  el.texture = SDL_CreateTextureFromSurface(render_.renderer, target_sur); 

  el.mask = {
    el.bounds.x, el.bounds.y, 
    el.bounds.w, MESSAGE_WIN_SIZE_Y - 2 * MESSAGE_PADDING_Y
  };

  build_scrollable("message_window");
}


void UISystem::setup_floor_display()
{
  auto& el{render_.text_elements["floor_display"]};
  el.font = render_.fonts["Fantasque-Small"];
  el.text = format_floor();

  TTF_SizeText(el.font, el.text.c_str(), &el.bounds.w, &el.bounds.h);

  el.bounds.x = 4;
  el.bounds.y = 4;

  build_text("floor_display");
}


void UISystem::setup_time_display()
{
  auto& el{render_.text_elements["time_display"]};
  el.font = render_.fonts["Fantasque-Small"];
  el.text = format_time();

  TTF_SizeText(el.font, el.text.c_str(), &el.bounds.w, &el.bounds.h);

  el.bounds.x = SCREEN_SIZE_X - el.bounds.w - 4;
  el.bounds.y = 4;

  build_text("time_display");
}


void UISystem::setup_date_display()
{
  auto& el{render_.text_elements["date_display"]};
  el.font = render_.fonts["Fantasque-Small"];
  el.text = format_date();

  TTF_SizeText(el.font, el.text.c_str(), &el.bounds.w, &el.bounds.h);

  el.bounds.x = SCREEN_SIZE_X - el.bounds.w - 4;
  el.bounds.y = 16;

  build_text("date_display");
}


string UISystem::format_floor()
{
  return to_string(map_.cur_floor + 1) + "F";
}


string UISystem::format_time()
{
  stringstream ss;
  ss << setfill('0');
  ss << setw(2) << time_.hour << ":";
  ss << setw(2) << time_.minute << ":"; 
  ss << setw(2) << time_.second; 

  return ss.str();
}


string UISystem::format_date()
{
  stringstream ss;
  ss << setfill('0');
  ss << setw(2) << time_.day << "/"; 
  ss << setw(2) << time_.month << "/";
  ss << setw(2) << time_.year;

  return ss.str();
}


void UISystem::build_scrollable(const string& id)
{
  auto& el{render_.scrollable_elements[id]};

  build_scalable(el.base);
}


void UISystem::build_window(const string& id)
{
  auto& el{render_.window_elements[id]};
  el.base.type = el.type;
  el.base.bounds = el.bounds;

  build_scalable(el.base);
}


void UISystem::build_button(const string& id)
{
  auto& el{render_.button_elements[id]};
  el.base.type = el.type + "-off";
  el.base.bounds = el.bounds;
  el.pressed.type = el.type + "-on";
  el.pressed.bounds = el.bounds;

  build_scalable(el.base);
  build_scalable(el.pressed);

  auto& text_el{render_.text_elements[id]};

  text_el.text = el.text;
  text_el.font = render_.fonts["Fantasque-Medium"];

  TTF_SizeText(
    text_el.font, text_el.text.c_str(), &text_el.bounds.w, &text_el.bounds.h
  );

  text_el.bounds.x = el.bounds.x + el.bounds.w / 2 - text_el.bounds.w / 2;
  text_el.bounds.y = el.bounds.y + el.bounds.h / 2 - text_el.bounds.h / 2;

  build_text(id);
}


void UISystem::build_text(const string& id)
{
  auto& el{render_.text_elements[id]};

  SDL_Surface* sur{TTF_RenderUTF8_Blended(el.font, el.text.c_str(), el.color)}; 

  if (sur == nullptr) {
    cerr << "TTF_RenderUTF8_Blended error: " << TTF_GetError() << endl; 
  } else {
    el.texture = SDL_CreateTextureFromSurface(render_.renderer, sur); 
  }
}


void UISystem::build_scalable(Scalable& el) 
{
  el.texture = render_.textures["overlay"];

  if (TileData.find(el.type) != TileData.end()) {
    el.basex = {(i32)(TILE_SIZE * TileData[el.type].uv.x)};
    el.basey = {(i32)(TILE_SIZE * TileData[el.type].uv.y)};
  } else {
    std::cerr << "Scalable has invalid type: " << el.type << std::endl;

    el.basex = 0;
    el.basey = 0;
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

  el.dst["tl"] = { 
    el.bounds.x, el.bounds.y, 
    el.size, el.size 
  };
  el.dst["tm"] = {
    el.bounds.x + el.size, el.bounds.y, 
    el.bounds.w - 2 * el.size, el.size
  }; 
  el.dst["tr"] = {
    el.bounds.x + el.bounds.w - el.size, el.bounds.y, 
    el.size, el.size
  };
  el.dst["ll"] = {
    el.bounds.x, el.bounds.y + el.size, 
    el.size, el.bounds.h - 2 * el.size
  };
  el.dst["mm"] = {
    el.bounds.x + el.size, el.bounds.y + el.size, 
    el.bounds.w - 2 * el.size, el.bounds.h - 2 * el.size
  };
  el.dst["rr"] = {
    el.bounds.x + el.bounds.w - el.size, el.bounds.y + el.size,
    el.size, el.bounds.h - 2 * el.size 
  };
  el.dst["bl"] = {
    el.bounds.x, el.bounds.y + el.bounds.h - el.size,
    el.size, el.size
  }; 
  el.dst["bm"] = {
    el.bounds.x + el.size, el.bounds.y + el.bounds.h - el.size,
    el.bounds.w - 2 * el.size, el.size
  };
  el.dst["br"] = {
    el.bounds.x + el.bounds.w - el.size, el.bounds.y + el.bounds.h - el.size,
    el.size, el.size
  };
}

