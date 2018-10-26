#include <iostream>
#include <string>
#include <vector>
#include <iterator>
#include <fstream>
#include <functional>

#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>

#define STB_IMAGE_IMPLEMENTATION
#include "../external/stb/stb_image.h"

#include "RenderSystem.h"
#include "../constants/RenderConstants.h"
#include "../constants/MapConstants.h"
#include "../utils/GLCheckError.h"
#include "../utils/GLLoadShader.h"

using namespace std;

RenderSystem::RenderSystem(
  Input& input, Render& render, Camera& camera, Map& map
) 
  : input_(input)
  , render_(render)
  , camera_(camera)
  , map_(map)
{
}

RenderSystem::~RenderSystem()
{
  glDeleteVertexArrays(1, &vao_);
  glfwTerminate();

  cout << "Render System Shutdown" << endl;
}

void RenderSystem::RunTests()
{
  /* float vertices[] = { */
  /*   // positions         // texture coords */
  /*    0.5f,  0.5f, 0.0f,  1.0f / TILESET_WIDTH, 15.0f / TILESET_HEIGHT, // TR */ 
  /*    0.5f, -0.5f, 0.0f,  1.0f / TILESET_WIDTH, 14.0f / TILESET_HEIGHT, // BR */ 
  /*   -0.5f,  0.5f, 0.0f,  0.0f / TILESET_WIDTH, 15.0f / TILESET_HEIGHT, // TL */ 
  /*    0.5f, -0.5f, 0.0f,  1.0f / TILESET_WIDTH, 14.0f / TILESET_HEIGHT, // BR */ 
  /*   -0.5f, -0.5f, 0.0f,  0.0f / TILESET_WIDTH, 14.0f / TILESET_HEIGHT, // BL */
  /*   -0.5f,  0.5f, 0.0f,  0.0f / TILESET_WIDTH, 15.0f / TILESET_HEIGHT  // TL */
  /* }; */

  /* glGenVertexArrays(1, &vao_); */
  /* glGenBuffers(1, &VBO_); */

  /* glBindVertexArray(vao_); */

  /* glBindBuffer(GL_ARRAY_BUFFER, VBO_); */
  /* glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW); */

  /* // position attribute */
  /* glVertexAttribPointer( */
  /*   0, 3, GL_FLOAT, GL_FALSE, 5 * sizeof(GL_FLOAT), (void*)0 */
  /* ); */
  /* glEnableVertexAttribArray(0); */

  /* // model matrix attribute */

  /* // texcoords attribute */

  /* // texture attribute */
  /* glVertexAttribPointer( */
  /*   1, 2, GL_FLOAT, GL_FALSE, 5 * sizeof(GL_FLOAT), (void*)(3 * sizeof(float)) */
  /* ); */
  /* glEnableVertexAttribArray(1); */

  /* LoadTexture("character_tileset"); */
  /* LoadTexture("map_tileset"); */
  /* LoadTexture("object_tileset"); */

  /* glUseProgram(shader_program_); */

  /* glUniform1i(glGetUniformLocation(shader_program_, "character_tileset"), 0); */ 
  /* glUniform1i(glGetUniformLocation(shader_program_, "map_tileset"), 1); */ 
  /* glUniform1i(glGetUniformLocation(shader_program_, "object_tileset"), 2); */ 

  /* glBindBuffer(GL_ARRAY_BUFFER, 0); */
  /* glBindVertexArray(0); */
}

void RenderSystem::Initialize()
{
  glfwInit();
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
  glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);

  render_.window = glfwCreateWindow(
    SCREEN_SIZE_X, SCREEN_SIZE_Y, "Last Ditch", nullptr, nullptr
  );

  if (render_.window == nullptr) {
    cout << "Failed to create GLFW window" << endl;
    glfwTerminate();
    return;
  }

  glfwMakeContextCurrent(render_.window);

  glewExperimental = GL_TRUE;
  glewInit();

  glEnable(GL_BLEND);
  glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

  shader_program_ = GLLoadShader(
    "assets/glsl/map.vert", "assets/glsl/map.frag"
  );

  BuildMap();
}

void RenderSystem::BuildMap()
{
  // tile setup
  float tile_vertices[] = {
     0.5f,  0.5f,
     0.5f, -0.5f,
    -0.5f,  0.5f,
    -0.5f, -0.5f,
     0.5f, -0.5f,
  };

  glGenVertexArrays(1, &vao_);
  glBindVertexArray(vao_);

  glGenBuffers(1, &tile_vbo_);

  glBindBuffer(GL_ARRAY_BUFFER, tile_vbo_);

  glBufferData(
    GL_ARRAY_BUFFER, sizeof(tile_vertices), tile_vertices, GL_STATIC_DRAW
  );

  glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 0, 0);

  glGenBuffers(1, &map_vbo_);
  glBindBuffer(GL_ARRAY_BUFFER, map_vbo_);

  map_.attributes.insert(
    map_.attributes.end(), 
    {
      0.0f, 0.0f, 
      0.0f / TILESET_WIDTH, 13.0f / TILESET_HEIGHT, 
      1.0f / TILESET_WIDTH, 14.0f / TILESET_HEIGHT,
      1.0f, 0.0f, 
      0.0f / TILESET_WIDTH, 14.0f / TILESET_HEIGHT, 
      1.0f / TILESET_WIDTH, 15.0f / TILESET_HEIGHT,
      0.0f, 1.0f, 
      1.0f / TILESET_WIDTH, 13.0f / TILESET_HEIGHT, 
      2.0f / TILESET_WIDTH, 14.0f / TILESET_HEIGHT,
      1.0f, 1.0f, 
      1.0f / TILESET_WIDTH, 14.0f / TILESET_HEIGHT, 
      2.0f / TILESET_WIDTH, 15.0f / TILESET_HEIGHT,
    }
  );

  /* float map_test_array[] = { */
  /*   0.0f, 0.0f, 0.0f / TILESET_WIDTH, 14.0f / TILESET_HEIGHT, */ 
  /*   1.0f, 0.0f, 1.0f / TILESET_WIDTH, 14.0f / TILESET_HEIGHT */
  /* }; */

  glBufferData(
    GL_ARRAY_BUFFER, sizeof(GL_FLOAT) * map_.attributes.size(), 
    map_.attributes.data(), GL_STREAM_DRAW
  );
  /* glBufferData( */
  /*   GL_ARRAY_BUFFER, sizeof(map_test_array), map_test_array, GL_STREAM_DRAW */
  /* ); */

  glVertexAttribPointer(
    1, 2, GL_FLOAT, GL_FALSE, 6 * sizeof(GL_FLOAT), (void*)0
  );
  glVertexAttribPointer(
    2, 4, GL_FLOAT, GL_FALSE, 6 * sizeof(GL_FLOAT), (void*)(2 * sizeof(GL_FLOAT))
  );

  glEnableVertexAttribArray(0);
  glEnableVertexAttribArray(1);
  glEnableVertexAttribArray(2);

  glVertexAttribDivisor(0, 0);
  glVertexAttribDivisor(1, 1);
  glVertexAttribDivisor(2, 1);

  LoadTexture("character_tileset");
  LoadTexture("map_tileset");
  LoadTexture("object_tileset");

  glUseProgram(shader_program_);

  glUniform1i(glGetUniformLocation(shader_program_, "character_tileset"), 0); 
  glUniform1i(glGetUniformLocation(shader_program_, "map_tileset"), 1); 
  glUniform1i(glGetUniformLocation(shader_program_, "object_tileset"), 2); 

  glBindBuffer(GL_ARRAY_BUFFER, 0);
  glBindVertexArray(0);
}

void RenderSystem::Update()
{
  glClearColor(0, 0, 0, 1.0f);
  glClear(GL_COLOR_BUFFER_BIT);

  if (input_.debug) {
    glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
  } else { 
    glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
  }

  glActiveTexture(GL_TEXTURE0);
  glBindTexture(GL_TEXTURE_2D, textures["character_tileset"]);
  glActiveTexture(GL_TEXTURE1);
  glBindTexture(GL_TEXTURE_2D, textures["map_tileset"]);
  glActiveTexture(GL_TEXTURE2);
  glBindTexture(GL_TEXTURE_2D, textures["object_tileset"]);

  glUseProgram(shader_program_);

  glm::mat4 view {
    glm::lookAt(camera_.pos, camera_.pos + camera_.z_dir, camera_.y_dir)
  };
  glm::mat4 projection {
    glm::ortho(
      -1.0f / camera_.zoom * ASPECT_RATIO, 1.0f / camera_.zoom * ASPECT_RATIO, 
      -1.0f / camera_.zoom,                1.0f / camera_.zoom
    ) 
  };

  int view_loc {glGetUniformLocation(shader_program_, "view")};
  int projection_loc {glGetUniformLocation(shader_program_, "projection")};

  glUniformMatrix4fv(view_loc, 1, GL_FALSE, glm::value_ptr(view));
  glUniformMatrix4fv(projection_loc, 1, GL_FALSE, glm::value_ptr(projection));

  glBindVertexArray(vao_);

  glDrawArraysInstanced(GL_TRIANGLE_STRIP, 0, 5, 4);
      
  glfwSwapBuffers(render_.window);
}

void RenderSystem::LoadTexture(const string& filename)
{
  int width, height, channels;

  stbi_set_flip_vertically_on_load(true);

  glGenTextures(1, &textures[filename]);
  glBindTexture(GL_TEXTURE_2D, textures[filename]);

  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);

  const string filepath {"assets/textures/" + filename + ".png"};

  unsigned char* tex_data {
    stbi_load(filepath.c_str(), &width, &height, &channels, STBI_rgb_alpha)
  };

  if (tex_data) {
    glTexImage2D(
      GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_RGBA, GL_UNSIGNED_BYTE, 
      tex_data
    );
  } else {
    cout << "Failed to load: " << filename << endl;
  }

  glBindTexture(GL_TEXTURE_2D, 0);
  stbi_image_free(tex_data);
}
