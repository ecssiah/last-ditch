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
  glDeleteVertexArrays(1, &VAO_);
  glfwTerminate();

  cout << "Render System Shutdown" << endl;
}

/////////////
// TESTING //
/////////////
void RenderSystem::RunTests()
{
  shader_program_ = GLLoadShader("assets/glsl/test.vert", "assets/glsl/test.frag");

  float vertices[] = {
    // positions         // texture coords
     0.5f,  0.5f, 0.0f,  1.0f, 1.0f,   // top right
     0.5f, -0.5f, 0.0f,  1.0f, 0.0f,   // bottom right
    -0.5f,  0.5f, 0.0f,  0.0f, 1.0f,   // top left 
     0.5f, -0.5f, 0.0f,  1.0f, 0.0f,   // bottom right
    -0.5f, -0.5f, 0.0f,  0.0f, 0.0f,   // bottom left
    -0.5f,  0.5f, 0.0f,  0.0f, 1.0f    // top left 
  };

  unsigned int VBO_;
  glGenVertexArrays(1, &VAO_);
  glGenBuffers(1, &VBO_);

  glBindVertexArray(VAO_);

  glBindBuffer(GL_ARRAY_BUFFER, VBO_);
  glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

  // position attribute
  glVertexAttribPointer(
    0, 3, GL_FLOAT, GL_FALSE, 5 * sizeof(GL_FLOAT), (void*)0
  );
  glEnableVertexAttribArray(0);

  // texture attribute
  glVertexAttribPointer(
    1, 2, GL_FLOAT, GL_FALSE, 5 * sizeof(GL_FLOAT), (void*)(3 * sizeof(float))
  );
  glEnableVertexAttribArray(1);

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
/////////////
// Testing //
/////////////


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

  if (render_.window == nullptr)
  {
    cout << "Failed to create GLFW window" << endl;
    glfwTerminate();
    return;
  }

  glfwMakeContextCurrent(render_.window);

  glewExperimental = GL_TRUE;
  glewInit();

  glEnable(GL_BLEND);
  glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

  RunTests();
}

void RenderSystem::Update()
{
  if (glfwWindowShouldClose(render_.window))
  {
    input_.exit = true;
    return;
  }

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

  glm::mat4 model {1.0f};
  glm::mat4 view {
    glm::lookAt(camera_.pos, camera_.pos + camera_.z_dir, camera_.y_dir)
  };
  glm::mat4 projection {
    glm::ortho(
      -1.0f / camera_.zoom * ASPECT_RATIO, 1.0f / camera_.zoom * ASPECT_RATIO, 
      -1.0f / camera_.zoom,                1.0f / camera_.zoom
    ) 
  };

  int model_loc = glGetUniformLocation(shader_program_, "model");
  int view_loc = glGetUniformLocation(shader_program_, "view");
  int projection_loc = glGetUniformLocation(shader_program_, "projection");

  glUniformMatrix4fv(model_loc, 1, GL_FALSE, glm::value_ptr(model));
  glUniformMatrix4fv(view_loc, 1, GL_FALSE, glm::value_ptr(view));
  glUniformMatrix4fv(projection_loc, 1, GL_FALSE, glm::value_ptr(projection));

  glBindVertexArray(VAO_);

  glDrawArrays(GL_TRIANGLES, 0, 6);
      
  glfwSwapBuffers(render_.window);
  glfwPollEvents();
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

  unsigned char* tex_data = stbi_load(
    filepath.c_str(), &width, &height, &channels, STBI_rgb_alpha 
  );

  if (tex_data)
  {
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
