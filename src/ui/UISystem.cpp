#include "../../include/ui/UISystem.h"

#include <iostream>
#include <iomanip>
#include <string>
#include <sstream>

#include "../../include/render/RenderConstants.h"
using namespace std;

UISystem::UISystem(Input& input, Render& render, Map& map, Time& time)
  : input_{input}
  , render_{render}
  , map_{map}
  , time_{time}
{
}


void UISystem::init()
{
  setup_main_window();
  setup_main_buttons();

  setup_floor_display();
  setup_time_display();
  setup_date_display();

  cout << "UISystem init" << endl;
}


void UISystem::update()
{
  update_main_text();

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


bool UISystem::check_intersection(i32 x, i32 y, Element& el)
{
  auto lcheck{input_.mx > el.rect.x};
  auto rcheck{input_.mx < el.rect.x + el.rect.w};
  auto tcheck{input_.my > el.rect.y};
  auto bcheck{input_.my < el.rect.y + el.rect.h};

  return lcheck && rcheck && tcheck && bcheck;
}


void UISystem::update_main_text()
{
  if (map_.floor_changed) {
    render_.text_elements["floor_display"].text = format_floor();
    build_text_element("floor_display");
  }

  if (time_.time_changed) {
    render_.text_elements["time_display"].text = format_time();
    build_text_element("time_display");
  }

  if (time_.date_changed) {
    render_.text_elements["date_display"].text = format_date();
    build_text_element("date_display");
  }
}


void UISystem::build_window_element(const string& id)
{
  auto size{TILE_SIZE / 4};
  auto& el{render_.window_elements[id]};

  el.tl_dst = { 
    el.rect.x, el.rect.y, 
    size, size 
  };
  el.tm_dst = {
    el.rect.x + size, el.rect.y, 
    el.rect.w - 2 * size, size
  }; 
  el.tr_dst = {
    el.rect.x + el.rect.w - size, el.rect.y, 
    size, size
  };
  el.ll_dst = {
    el.rect.x, el.rect.y + size, 
    size, el.rect.h - 2 * size
  };
  el.mm_dst = {
    el.rect.x + size, el.rect.y + size, 
    el.rect.w - 2 * size, el.rect.h - 2 * size
  };
  el.rr_dst = {
    el.rect.x + el.rect.w - size, el.rect.y + size,
    size, el.rect.h - 2 * size 
  };
  el.bl_dst = {
    el.rect.x, el.rect.y + el.rect.h - size,
    size, size
  }; 
  el.bm_dst = {
    el.rect.x + size, el.rect.y + el.rect.h - size,
    el.rect.w - 2 * size, size
  };
  el.br_dst = {
    el.rect.x + el.rect.w - size, el.rect.y + el.rect.h - size,
    size, size
  };
}


void UISystem::build_button_element(const string& id)
{
  auto size{TILE_SIZE / 4};
  auto& el{render_.button_elements[id]};

  el.tl_dst = { 
    el.rect.x, el.rect.y, 
    size, size 
  };
  el.tm_dst = {
    el.rect.x + size, el.rect.y, 
    el.rect.w - 2 * size, size
  }; 
  el.tr_dst = {
    el.rect.x + el.rect.w - size, el.rect.y, 
    size, size
  };
  el.ll_dst = {
    el.rect.x, el.rect.y + size, 
    size, el.rect.h - 2 * size
  };
  el.mm_dst = {
    el.rect.x + size, el.rect.y + size, 
    el.rect.w - 2 * size, el.rect.h - 2 * size
  };
  el.rr_dst = {
    el.rect.x + el.rect.w - size, el.rect.y + size,
    size, el.rect.h - 2 * size 
  };
  el.bl_dst = {
    el.rect.x, el.rect.y + el.rect.h - size,
    size, size
  }; 
  el.bm_dst = {
    el.rect.x + size, el.rect.y + el.rect.h - size,
    el.rect.w - 2 * size, size
  };
  el.br_dst = {
    el.rect.x + el.rect.w - size, el.rect.y + el.rect.h - size,
    size, size
  };

  auto& txt_el{render_.text_elements[id]};

  txt_el.font = render_.fonts["Fantasque-Medium"];
  txt_el.text = el.text;

  build_text_element(id);
  
  txt_el.rect.x = el.rect.x + el.rect.w / 2 - txt_el.rect.w / 2;
  txt_el.rect.y = el.rect.y + el.rect.h / 2 - txt_el.rect.h / 2;
}


void UISystem::build_text_element(const string& id)
{
  auto& el{render_.text_elements[id]};

  SDL_Surface* sur{TTF_RenderUTF8_Blended(el.font, el.text.c_str(), el.color)}; 

  el.rect.w = sur->w;
  el.rect.h = sur->h;

  if (sur == nullptr) {
    cerr << "TTF_RenderUTF8_Blended error: " << TTF_GetError() << endl; 
  } else {
    el.texture = SDL_CreateTextureFromSurface(render_.renderer, sur); 
  }
}


void UISystem::setup_main_window()
{
  render_.window_elements["main_window"].rect.x = 0.1 * SCREEN_SIZE_X;
  render_.window_elements["main_window"].rect.y = 0.1 * SCREEN_SIZE_Y;
  render_.window_elements["main_window"].rect.w = 0.8 * SCREEN_SIZE_X;  
  render_.window_elements["main_window"].rect.h = 0.8 * SCREEN_SIZE_Y;  

  build_window_element("main_window");
}


void UISystem::setup_main_buttons()
{
  auto width{120};
  auto height{32};

  render_.button_elements["info"].text = "Info";
  render_.button_elements["info"].rect.x = .25 * SCREEN_SIZE_X - width / 2;
  render_.button_elements["info"].rect.y = .11 * SCREEN_SIZE_Y;
  render_.button_elements["info"].rect.w = width;
  render_.button_elements["info"].rect.h = height;
  render_.button_elements["info"].active = true;

  build_button_element("info");

  render_.button_elements["save"].text = "Save/Load";
  render_.button_elements["save"].rect.x = .50 * SCREEN_SIZE_X - width / 2;
  render_.button_elements["save"].rect.y = .11 * SCREEN_SIZE_Y;
  render_.button_elements["save"].rect.w = width;
  render_.button_elements["save"].rect.h = height;

  build_button_element("save");

  render_.button_elements["options"].text = "Options";
  render_.button_elements["options"].rect.x = .75 * SCREEN_SIZE_X - width / 2;
  render_.button_elements["options"].rect.y = .11 * SCREEN_SIZE_Y;
  render_.button_elements["options"].rect.w = width;
  render_.button_elements["options"].rect.h = height;

  build_button_element("options");
}


void UISystem::setup_floor_display()
{
  render_.text_elements["floor_display"].font = render_.fonts["Fantasque-Small"];
  render_.text_elements["floor_display"].text = format_floor();

  build_text_element("floor_display");
  
  render_.text_elements["floor_display"].rect.x = 4;
  render_.text_elements["floor_display"].rect.y = 4;
}


void UISystem::setup_time_display()
{
  render_.text_elements["time_display"].font = render_.fonts["Fantasque-Small"];
  render_.text_elements["time_display"].text = format_time();

  build_text_element("time_display");

  auto& rect{render_.text_elements["time_display"].rect};

  rect.x = SCREEN_SIZE_X - rect.w - 4;
  rect.y = 4;
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


void UISystem::setup_date_display()
{
  render_.text_elements["date_display"].font = render_.fonts["Fantasque-Small"];
  render_.text_elements["date_display"].text = format_date();

  build_text_element("date_display");

  auto& rect{render_.text_elements["date_display"].rect};

  rect.x = SCREEN_SIZE_X - rect.w - 4;
  rect.y = 16;
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

