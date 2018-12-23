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


void UISystem::Initialize()
{
  InitializeSDLTTF();
  LoadFonts();

  SetupMainWindow();
  SetupMainButtons();

  SetupFloorDisplay();
  SetupTimeDisplay();
  SetupDateDisplay();
}


void UISystem::Update()
{
  if (input_.menu) {
    RenderWindowElement(window_elements_["main_window"]);

    RenderButtonElement(button_elements_["info"]);
    RenderButtonElement(button_elements_["save"]);
    RenderButtonElement(button_elements_["options"]);
  }

  BuildTextElements();

  for (auto kv : text_elements_) RenderTextElement(kv.second);
}


void UISystem::BuildTextElements()
{
  if (map_.floor_changed) {
    text_elements_["floor_display"].text = to_string(map_.cur_floor + 1);
    BuildTextElement(text_elements_["floor_display"]);
  }

  if (time_.time_changed) {
    text_elements_["time_display"].text = FormatTime();
    BuildTextElement(text_elements_["time_display"]);
  }

  if (time_.date_changed) {
    text_elements_["date_display"].text = FormatDate();
    BuildTextElement(text_elements_["date_display"]);
  }
}


void UISystem::InitializeSDLTTF()
{
  if (TTF_Init()) {
    cout << "TTF_Init: " << TTF_GetError() << endl;  
    return;
  } 
}


void UISystem::LoadFonts()
{
  fonts_["Fantasque-Small"] = LoadFont("FantasqueSansMono-Regular", 14);
  fonts_["Fantasque-Medium"] = LoadFont("FantasqueSansMono-Regular", 18);
  fonts_["Fantasque-Large"] = LoadFont("FantasqueSansMono-Regular", 22);
}


TTF_Font* UISystem::LoadFont(string fontname, unsigned size)
{
  string fontpath{"assets/fonts/" + fontname + ".ttf"};
  TTF_Font* font{TTF_OpenFont(fontpath.c_str(), size)};

  if (!font) {
    cout << "TTF_OpenFont error: " << TTF_GetError() << endl;
    return nullptr;
  }

  return font;
}


void UISystem::BuildWindowElement(Window& el)
{
  auto size{TILE_SIZE / 4};

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


void UISystem::BuildButtonElement(Button& el)
{
  auto size{TILE_SIZE / 4};

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


void UISystem::BuildTextElement(Text& el)
{
  SDL_Surface* sur{TTF_RenderUTF8_Blended(el.font, el.text.c_str(), el.color)}; 

  el.rect.w = sur->w;
  el.rect.h = sur->h;

  if (sur == nullptr) {
    cerr << "TTF_RenderUTF8_Blended error: " << TTF_GetError() << endl; 
  } else {
    el.texture = SDL_CreateTextureFromSurface(render_.renderer, sur); 
  }
}


void UISystem::RenderTextElement(const Text& el)
{
  SDL_RenderCopy(render_.renderer, el.texture, nullptr, &el.rect); 
}


void UISystem::RenderWindowElement(const Window& el)
{
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

void UISystem::RenderButtonElement(const Button& el)
{
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
}


void UISystem::SetupMainWindow()
{
  window_elements_["main_window"].rect.x = 0.1 * SCREEN_SIZE_X;
  window_elements_["main_window"].rect.y = 0.1 * SCREEN_SIZE_Y;
  window_elements_["main_window"].rect.w = 0.8 * SCREEN_SIZE_X;  
  window_elements_["main_window"].rect.h = 0.8 * SCREEN_SIZE_Y;  

  BuildWindowElement(window_elements_["main_window"]);
}


void UISystem::SetupMainButtons()
{
  auto width{120};
  auto height{40};

  button_elements_["info"].text = "Info";
  button_elements_["info"].rect.x = .25 * SCREEN_SIZE_X - width / 2;
  button_elements_["info"].rect.y = .12 * SCREEN_SIZE_Y;
  button_elements_["info"].rect.w = width;
  button_elements_["info"].rect.h = height;

  BuildButtonElement(button_elements_["info"]);

  button_elements_["save"].text = "Save/Load";
  button_elements_["save"].rect.x = .50 * SCREEN_SIZE_X - width / 2;
  button_elements_["save"].rect.y = .12 * SCREEN_SIZE_Y;
  button_elements_["save"].rect.w = width;
  button_elements_["save"].rect.h = height;

  BuildButtonElement(button_elements_["save"]);

  button_elements_["options"].text = "Options";
  button_elements_["options"].rect.x = .75 * SCREEN_SIZE_X - width / 2;
  button_elements_["options"].rect.y = .12 * SCREEN_SIZE_Y;
  button_elements_["options"].rect.w = width;
  button_elements_["options"].rect.h = height;

  BuildButtonElement(button_elements_["options"]);
}


void UISystem::SetupFloorDisplay()
{
  text_elements_["floor_display"].font = fonts_["Fantasque-Small"];
  text_elements_["floor_display"].text = to_string(map_.cur_floor + 1);

  BuildTextElement(text_elements_["floor_display"]);
  
  text_elements_["floor_display"].rect.x = 4;
  text_elements_["floor_display"].rect.y = 4;
}


void UISystem::SetupTimeDisplay()
{
  text_elements_["time_display"].font = fonts_["Fantasque-Small"];
  text_elements_["time_display"].text = FormatTime();

  BuildTextElement(text_elements_["time_display"]);

  auto& rect{text_elements_["time_display"].rect};

  rect.x = SCREEN_SIZE_X - rect.w - 4;
  rect.y = 4;
}


string UISystem::FormatTime()
{
  stringstream ss;
  ss << setfill('0');
  ss << setw(2) << time_.hour << ":";
  ss << setw(2) << time_.minute << ":"; 
  ss << setw(2) << time_.second; 

  return ss.str();
}


void UISystem::SetupDateDisplay()
{
  text_elements_["date_display"].font = fonts_["Fantasque-Small"];
  text_elements_["date_display"].text = FormatDate();

  BuildTextElement(text_elements_["date_display"]);

  auto& rect{text_elements_["date_display"].rect};

  rect.x = SCREEN_SIZE_X - rect.w - 4;
  rect.y = 16;
}


string UISystem::FormatDate()
{
  stringstream ss;
  ss << setfill('0');
  ss << setw(2) << time_.day << "/"; 
  ss << setw(2) << time_.month << "/";
  ss << setw(2) << time_.year;

  return ss.str();
}


