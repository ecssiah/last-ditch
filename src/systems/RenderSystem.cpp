#include <iostream>
#include <GL/glew.h>

#include "RenderSystem.h"
#include "../constants/RenderConstants.h"
#define STB_IMAGE_IMPLEMENTATION
#include "../ext/stb/stb_image.h"

RenderSystem::RenderSystem() :
  window(nullptr),
  glcontext()
{
}

RenderSystem::~RenderSystem()
{
  SDL_DestroyWindow(window);
  SDL_GL_DeleteContext(glcontext);
  SDL_Quit();

  std::cout << "Render System Shutdown" << std::endl;
}

void RenderSystem::Initialize()
{
  if (SDL_Init(SDL_INIT_VIDEO) != 0)
    return;

  SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
  SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
  SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 2);
  SDL_GL_SetAttribute(SDL_GL_STENCIL_SIZE, 8);

  window = SDL_CreateWindow(
    "Last Ditch", 
    SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 
    Render::SCREEN_SIZE_X, Render::SCREEN_SIZE_Y, 
    SDL_WINDOW_OPENGL
  );

  glcontext = SDL_GL_CreateContext(window);

  glewExperimental = GL_TRUE;
  glewInit();

  GLuint vertex_buffer;
  glGenBuffers(1, &vertex_buffer);

  int width, height, nr_channels;
  unsigned char* data = stbi_load(
    "assets/textures/tileset1.png", &width, &height, &nr_channels, 0
  );

  unsigned int texture;
  glGenTextures(1, &texture);
  glBindTexture(GL_TEXTURE_2D, texture);
  
  glTexImage2D(
    GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_RGBA, GL_UNSIGNED_BYTE, data
  );

  stbi_image_free(data);

}

void RenderSystem::Update(const double& dt)
{
  SDL_GL_SwapWindow(window);
}
