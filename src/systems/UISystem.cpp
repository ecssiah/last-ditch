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
  , overlay_texture_{nullptr}
{
}


void UISystem::Initialize(SDL_Texture* overlay_texture)
{
  overlay_texture_ = overlay_texture;

  InitializeSDLTTF();
  LoadFonts();

  SetupMainWindow();

  SetupFloorDisplay();
  SetupTimeDisplay();
  SetupDateDisplay();
}


void UISystem::Update()
{
  BuildTextElements();

  if (input_.menu) RenderWindowElement(window_elements_["main_window"]);

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
  string font_path{"assets/fonts/" + fontname + ".ttf"};
  TTF_Font* font{TTF_OpenFont(font_path.c_str(), size)};

  if (!font) {
    cout << "TTF_OpenFont error: " << TTF_GetError() << endl;
    return nullptr;
  }

  return font;
}


void UISystem::BuildWindowElement(WindowElement& el)
{
  auto el_size{TILE_SIZE / 4};

  el.tl_dst = { 
    el.rect.x, el.rect.y, 
    el_size, el_size 
  };
  el.tm_dst = {
    el.rect.x + el_size, el.rect.y, 
    el.rect.w - 2 * el_size, el_size
  }; 
  el.tr_dst = {
    el.rect.x + el.rect.w - el_size, 
    el.rect.y, el_size, el_size
  };
  el.ll_dst = {
    el.rect.x, el.rect.y + el_size, 
    el_size, el.rect.h - 2 * el_size
  };
  el.mm_dst = {
    el.rect.x + el_size, el.rect.y + el_size, 
    el.rect.w - 2 * el_size, el.rect.h - 2 * el_size
  };
  el.rr_dst = {
    el.rect.x + el.rect.w - el_size, el.rect.y + el_size,
    el_size, el.rect.h - 2 * el_size 
  };
  el.bl_dst = {
    el.rect.x, el.rect.y + el.rect.h - el_size,
    el_size, el_size
  }; 
  el.bm_dst = {
    el.rect.x + el_size, el.rect.y + el.rect.h - el_size,
    el.rect.w - 2 * el_size, el_size
  };
  el.br_dst = {
    el.rect.x + el.rect.w - el_size, el.rect.y + el.rect.h - el_size,
    el_size, el_size
  };
}


void UISystem::BuildTextElement(TextElement& el)
{
  SDL_Surface* surface{TTF_RenderUTF8_Blended(
    el.font, el.text.c_str(), el.color
  )}; 

  el.rect.w = surface->w;
  el.rect.h = surface->h;

  if (surface == nullptr) {
    cerr << "TTF_RenderUTF8_Blended error: " << TTF_GetError() << endl; 
  } else {
    el.texture = SDL_CreateTextureFromSurface(render_.renderer, surface); 
  }
}


void UISystem::RenderTextElement(const TextElement& el)
{
  SDL_RenderCopy(render_.renderer, el.texture, nullptr, &el.rect); 
}


void UISystem::RenderWindowElement(const WindowElement& el)
{
  SDL_RenderCopy(render_.renderer, overlay_texture_, &el.tl_src, &el.tl_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture_, &el.tm_src, &el.tm_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture_, &el.tr_src, &el.tr_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture_, &el.ll_src, &el.ll_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture_, &el.mm_src, &el.mm_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture_, &el.rr_src, &el.rr_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture_, &el.bl_src, &el.bl_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture_, &el.bm_src, &el.bm_dst);
  SDL_RenderCopy(render_.renderer, overlay_texture_, &el.br_src, &el.br_dst);
}


void UISystem::SetupMainWindow()
{
  window_elements_["main_window"].rect.x = 0.1 * SCREEN_SIZE_X;
  window_elements_["main_window"].rect.y = 0.1 * SCREEN_SIZE_Y;
  window_elements_["main_window"].rect.w = 0.8 * SCREEN_SIZE_X;  
  window_elements_["main_window"].rect.h = 0.8 * SCREEN_SIZE_Y;  

  BuildWindowElement(window_elements_["main_window"]);
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

  rect.x = SCREEN_SIZE_X - 4 - rect.w;
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

  rect.x = SCREEN_SIZE_X - 4 - rect.w;
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


