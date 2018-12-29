#include "UISystem.h"

#include <iostream>
#include <iomanip>
#include <string>
#include <sstream>

#include "../constants/RenderConstants.h"

using namespace std;

UISystem::UISystem(Input& input, Render& render, Map& map, Time& time)
  : input_{input}
  , render_{render}
  , map_{map}
  , time_{time}
  , fonts_{}
  , text_elements_{}
  , window_elements_{}
{
}


void UISystem::init()
{
  init_SDL_ttf();
  load_fonts();

  setup_main_window();
  setup_main_buttons();

  setup_floor_display();
  setup_time_display();
  setup_date_display();
}


void UISystem::update()
{
  update_main_text();

  if (input_.menu) {
    if (input_.lclick) {
      auto& info_btn{button_elements_["info"]};    
      auto& save_btn{button_elements_["save"]};    
      auto& options_btn{button_elements_["options"]};    

      if (check_intersection(info_btn, input_.mx, input_.my)) {
        info_btn.active = true;
        save_btn.active = false;
        options_btn.active = false;
      } else if (check_intersection(save_btn, input_.mx, input_.my)) {
        info_btn.active = false;
        save_btn.active = true;
        options_btn.active = false;
      } else if (check_intersection(options_btn, input_.mx, input_.my)) {
        info_btn.active = false;
        save_btn.active = false;
        options_btn.active = true;
      }
    }

    render_window_element("main_window");

    render_button_element("info");
    render_button_element("save");
    render_button_element("options");
  }

  render_text_element("floor_display");
  render_text_element("time_display");
  render_text_element("date_display");
}


bool UISystem::check_intersection(Element& el, I32 x, I32 y)
{
  auto lcheck{input_.mx > el.rect.x};
  auto rcheck{input_.mx < el.rect.x + el.rect.w};
  auto tcheck{input_.my > el.rect.y};
  auto bcheck{input_.my < el.rect.y + el.rect.h};

  return lcheck && rcheck && tcheck && bcheck ? true : false;
}


void UISystem::update_main_text()
{
  if (map_.floor_changed) {
    text_elements_["floor_display"].text = format_floor();
    build_text_element("floor_display");
  }

  if (time_.time_changed) {
    text_elements_["time_display"].text = format_time();
    build_text_element("time_display");
  }

  if (time_.date_changed) {
    text_elements_["date_display"].text = format_date();
    build_text_element("date_display");
  }
}


void UISystem::init_SDL_ttf()
{
  if (TTF_Init()) {
    cout << "TTF_Init: " << TTF_GetError() << endl;  
    return;
  } 
}


void UISystem::load_fonts()
{
  fonts_["Fantasque-Small"] = load_font("FantasqueSansMono-Regular", 14);
  fonts_["Fantasque-Medium"] = load_font("FantasqueSansMono-Regular", 18);
  fonts_["Fantasque-Large"] = load_font("FantasqueSansMono-Regular", 22);
}


TTF_Font* UISystem::load_font(const string& fontname, U32 size)
{
  string fontpath{"assets/fonts/" + fontname + ".ttf"};
  TTF_Font* font{TTF_OpenFont(fontpath.c_str(), size)};

  if (!font) {
    cout << "TTF_OpenFont error: " << TTF_GetError() << endl;
    return nullptr;
  }

  return font;
}


void UISystem::build_window_element(const string& id)
{
  auto size{TILE_SIZE / 4};
  auto& el{window_elements_[id]};

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
  auto& el{button_elements_[id]};

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

  auto& txt_el{text_elements_[id]};

  txt_el.font = fonts_["Fantasque-Medium"];
  txt_el.text = el.text;

  build_text_element(id);
  
  txt_el.rect.x = el.rect.x + el.rect.w / 2 - txt_el.rect.w / 2;
  txt_el.rect.y = el.rect.y + el.rect.h / 2 - txt_el.rect.h / 2;
}


void UISystem::build_text_element(const string& id)
{
  auto& el{text_elements_[id]};

  SDL_Surface* sur{TTF_RenderUTF8_Blended(el.font, el.text.c_str(), el.color)}; 

  el.rect.w = sur->w;
  el.rect.h = sur->h;

  if (sur == nullptr) {
    cerr << "TTF_RenderUTF8_Blended error: " << TTF_GetError() << endl; 
  } else {
    el.texture = SDL_CreateTextureFromSurface(render_.renderer, sur); 
  }
}


void UISystem::render_text_element(const string& id)
{
  auto& el{text_elements_[id]};

  SDL_RenderCopy(render_.renderer, el.texture, nullptr, &el.rect); 
}


void UISystem::render_window_element(const string& id)
{
  auto& el{window_elements_[id]};
  auto* overlay_texture{render_.textures["overlay"]};

  SDL_RenderCopy(render_.renderer, overlay_texture, &el.tl_src, &el.tl_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture, &el.tm_src, &el.tm_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture, &el.tr_src, &el.tr_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture, &el.ll_src, &el.ll_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture, &el.mm_src, &el.mm_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture, &el.rr_src, &el.rr_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture, &el.bl_src, &el.bl_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture, &el.bm_src, &el.bm_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture, &el.br_src, &el.br_dst);
}

void UISystem::render_button_element(const string& id)
{
  auto& el{button_elements_[id]};
  auto* overlay_texture{render_.textures["overlay"]};

  if (el.active) {
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.active_tl_src, &el.tl_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.active_tm_src, &el.tm_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.active_tr_src, &el.tr_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.active_ll_src, &el.ll_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.active_mm_src, &el.mm_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.active_rr_src, &el.rr_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.active_bl_src, &el.bl_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.active_bm_src, &el.bm_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.active_br_src, &el.br_dst
    );
  } else {
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.inactive_tl_src, &el.tl_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.inactive_tm_src, &el.tm_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.inactive_tr_src, &el.tr_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.inactive_ll_src, &el.ll_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.inactive_mm_src, &el.mm_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.inactive_rr_src, &el.rr_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.inactive_bl_src, &el.bl_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.inactive_bm_src, &el.bm_dst
    );
    SDL_RenderCopy(
      render_.renderer, overlay_texture, &el.inactive_br_src, &el.br_dst
    );
  }

  render_text_element(id);
}


void UISystem::setup_main_window()
{
  window_elements_["main_window"].rect.x = 0.1 * SCREEN_SIZE_X;
  window_elements_["main_window"].rect.y = 0.1 * SCREEN_SIZE_Y;
  window_elements_["main_window"].rect.w = 0.8 * SCREEN_SIZE_X;  
  window_elements_["main_window"].rect.h = 0.8 * SCREEN_SIZE_Y;  

  build_window_element("main_window");
}


void UISystem::setup_main_buttons()
{
  auto width{120};
  auto height{32};

  button_elements_["info"].text = "Info";
  button_elements_["info"].rect.x = .25 * SCREEN_SIZE_X - width / 2;
  button_elements_["info"].rect.y = .11 * SCREEN_SIZE_Y;
  button_elements_["info"].rect.w = width;
  button_elements_["info"].rect.h = height;
  button_elements_["info"].active = true;

  build_button_element("info");

  button_elements_["save"].text = "Save/Load";
  button_elements_["save"].rect.x = .50 * SCREEN_SIZE_X - width / 2;
  button_elements_["save"].rect.y = .11 * SCREEN_SIZE_Y;
  button_elements_["save"].rect.w = width;
  button_elements_["save"].rect.h = height;

  build_button_element("save");

  button_elements_["options"].text = "Options";
  button_elements_["options"].rect.x = .75 * SCREEN_SIZE_X - width / 2;
  button_elements_["options"].rect.y = .11 * SCREEN_SIZE_Y;
  button_elements_["options"].rect.w = width;
  button_elements_["options"].rect.h = height;

  build_button_element("options");
}


void UISystem::setup_floor_display()
{
  text_elements_["floor_display"].font = fonts_["Fantasque-Small"];
  text_elements_["floor_display"].text = format_floor();

  build_text_element("floor_display");
  
  text_elements_["floor_display"].rect.x = 4;
  text_elements_["floor_display"].rect.y = 4;
}


void UISystem::setup_time_display()
{
  text_elements_["time_display"].font = fonts_["Fantasque-Small"];
  text_elements_["time_display"].text = format_time();

  build_text_element("time_display");

  auto& rect{text_elements_["time_display"].rect};

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
  text_elements_["date_display"].font = fonts_["Fantasque-Small"];
  text_elements_["date_display"].text = format_date();

  build_text_element("date_display");

  auto& rect{text_elements_["date_display"].rect};

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

