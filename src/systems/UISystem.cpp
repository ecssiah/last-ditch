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

  if (input_.menu) {
    RenderWindowElement(window_elements_["main_window"]);
  }

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


void UISystem::BuildWindowElement(WindowElement& element)
{
  auto element_size{TILE_SIZE / 4};

  element.tl_dst = {
    element.rect.x, element.rect.y, 
    element_size, element_size
  };
  element.tm_dst = {
    element.rect.x + element_size, element.rect.y, 
    element.rect.w - 2 * element_size, element_size
  }; 
  element.tr_dst = {
    element.rect.x + element.rect.w - element_size, element.rect.y, 
    element_size, element_size
  };
  element.ll_dst = {
    element.rect.x, element.rect.y + element_size,
    element_size, element.rect.h - 2 * element_size
  };
  element.mm_dst = {
    element.rect.x + element_size, element.rect.y + element_size,
    element.rect.w - 2 * element_size, element.rect.h - 2 * element_size
  };
  element.rr_dst = {
    element.rect.x + element.rect.w - element_size,
    element.rect.y + element_size,
    element_size, element.rect.h - 2 * element_size 
  };
  element.bl_dst = {
    element.rect.x, element.rect.y + element.rect.h - element_size,
    element_size, element_size
  }; 
  element.bm_dst = {
    element.rect.x + element_size, 
    element.rect.y + element.rect.h - element_size,
    element.rect.w - 2 * element_size, element_size
  };
  element.br_dst = {
    element.rect.x + element.rect.w - element_size, 
    element.rect.y + element.rect.h - element_size,
    element_size, element_size
  };
}


void UISystem::BuildTextElement(TextElement& element)
{
  SDL_Surface* surface{TTF_RenderUTF8_Blended(
    element.font, element.text.c_str(), element.color
  )}; 

  element.rect.w = surface->w;
  element.rect.h = surface->h;

  if (surface == nullptr) {
    cerr << "TTF_RenderUTF8_Blended error: " << TTF_GetError() << endl; 
  } else {
    element.texture = SDL_CreateTextureFromSurface(render_.renderer, surface); 
  }
}


void UISystem::RenderTextElement(const TextElement& element)
{
  SDL_RenderCopy(render_.renderer, element.texture, nullptr, &element.rect); 
}


void UISystem::RenderWindowElement(const WindowElement& element)
{
  SDL_RenderCopy(
    render_.renderer, overlay_texture_, &element.tl_src, &element.tl_dst
  );
  SDL_RenderCopy(
    render_.renderer, overlay_texture_, &element.tm_src, &element.tm_dst
  );
  SDL_RenderCopy(
    render_.renderer, overlay_texture_, &element.tr_src, &element.tr_dst
  );
  SDL_RenderCopy(
    render_.renderer, overlay_texture_, &element.ll_src, &element.ll_dst
  );
  SDL_RenderCopy(
    render_.renderer, overlay_texture_, &element.mm_src, &element.mm_dst
  );
  SDL_RenderCopy(
    render_.renderer, overlay_texture_, &element.rr_src, &element.rr_dst
  );
  SDL_RenderCopy(
    render_.renderer, overlay_texture_, &element.bl_src, &element.bl_dst
  );
  SDL_RenderCopy(
    render_.renderer, overlay_texture_, &element.bm_src, &element.bm_dst
  );
  SDL_RenderCopy(
    render_.renderer, overlay_texture_, &element.br_src, &element.br_dst
  );
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


