#include <iostream>
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

RenderSystem::RenderSystem(Input& input, Window& window, Camera& camera) 
  : input_(input)
  , window_(window)
  , camera_(camera)
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
  shader_prog_ = GLLoadShader("assets/glsl/test.vert", "assets/glsl/test.frag");

  float vertices[] = {
    // positions          // colors           // texture coords
     0.5f,  0.5f, 0.0f,   1.0f, 0.0f, 0.0f,   1.0f, 1.0f,   // top right
     0.5f, -0.5f, 0.0f,   0.0f, 1.0f, 0.0f,   1.0f, 0.0f,   // bottom right
    -0.5f, -0.5f, 0.0f,   0.0f, 0.0f, 1.0f,   0.0f, 0.0f,   // bottom left
    -0.5f,  0.5f, 0.0f,   1.0f, 1.0f, 0.0f,   0.0f, 1.0f    // top left 
  };
  unsigned int indices[] = {
    0, 1, 3,
    1, 2, 3
  };

  unsigned int VBO, EBO;
  glGenVertexArrays(1, &VAO_);
  glGenBuffers(1, &VBO);
  glGenBuffers(1, &EBO);

  glBindVertexArray(VAO_);

  glBindBuffer(GL_ARRAY_BUFFER, VBO);
  glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, EBO);
  glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indices), indices, GL_STATIC_DRAW);

  // position attribute
  glVertexAttribPointer(
    0, 3, GL_FLOAT, GL_FALSE, 8 * sizeof(GL_FLOAT), (void*)0
  );
  glEnableVertexAttribArray(0);

  // color attribute
  glVertexAttribPointer(
    1, 3, GL_FLOAT, GL_FALSE, 8 * sizeof(GL_FLOAT), (void*)(3 * sizeof(float))
  );
  glEnableVertexAttribArray(1);

  // texture attribute
  glVertexAttribPointer(
    2, 2, GL_FLOAT, GL_FALSE, 8 * sizeof(GL_FLOAT), (void*)(6 * sizeof(float))
  );
  glEnableVertexAttribArray(2);

  // texture0 loading
  int width, height, nr_channels;
  stbi_set_flip_vertically_on_load(true);

  glGenTextures(1, &texture0_);
  glBindTexture(GL_TEXTURE_2D, texture0_);

  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);

  unsigned char* tex_data0 = stbi_load(
    "assets/textures/test_texture0.jpg", &width, &height, &nr_channels, 0
  );

  if (tex_data0)
  {
    glTexImage2D(
      GL_TEXTURE_2D, 0, GL_RGB, width, height, 
      0, GL_RGB, GL_UNSIGNED_BYTE, tex_data0
    );
    glGenerateMipmap(GL_TEXTURE_2D);
  } else {
    cout << "Failed to load texture0" << endl;
  }
  stbi_image_free(tex_data0);

  // texture1 loading
  glGenTextures(1, &texture1_);
  glBindTexture(GL_TEXTURE_2D, texture1_);

  unsigned char* tex_data1 = stbi_load(
    "assets/textures/test_texture1.jpg", &width, &height, &nr_channels, 0
  );

  if (tex_data1)
  {
    glTexImage2D(
      GL_TEXTURE_2D, 0, GL_RGB, width, height, 
      0, GL_RGB, GL_UNSIGNED_BYTE, tex_data1
    );
    glGenerateMipmap(GL_TEXTURE_2D);
  } else {
    cout << "Failed to load texture1" << endl;
  }
  stbi_image_free(tex_data1);

  glUseProgram(shader_prog_);

  glUniform1i(glGetUniformLocation(shader_prog_, "texture0"), 0); 
  glUniform1i(glGetUniformLocation(shader_prog_, "texture1"), 1); 

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

  window_.ptr = glfwCreateWindow(
    SCREEN_SIZE_X, SCREEN_SIZE_Y, 
    "Last Ditch", nullptr, nullptr
  );

  if (window_.ptr == nullptr)
  {
    cout << "Failed to create GLFW window" << endl;
    glfwTerminate();
    return;
  }

  glfwMakeContextCurrent(window_.ptr);

  glewExperimental = GL_TRUE;
  glewInit();

  RunTests();
}

void RenderSystem::Update(const double& dt)
{
  if (glfwWindowShouldClose(window_.ptr))
  {
    input_.exit = true;
    return;
  }

  glClearColor(0, 0, 0, 1.0f);
  glClear(GL_COLOR_BUFFER_BIT);

  glActiveTexture(GL_TEXTURE0);
  glBindTexture(GL_TEXTURE_2D, texture0_);
  glActiveTexture(GL_TEXTURE1);
  glBindTexture(GL_TEXTURE_2D, texture1_);

  glUseProgram(shader_prog_);

  glm::mat4 model {
    glm::rotate(
      glm::mat4(1.0), glm::radians(-55.0f), glm::vec3(1.0f, 0.0f, 0.0f)
    )
  };
  glm::mat4 view {
    glm::lookAt(camera_.pos, camera_.pos + camera_.z_dir, camera_.y_dir)
  };
  glm::mat4 projection {
    glm::perspective(
      glm::radians(45.0f), 
      (float)SCREEN_SIZE_X / (float)SCREEN_SIZE_Y, 
      0.1f, 100.0f
    )
  };

  int model_loc = glGetUniformLocation(shader_prog_, "model");
  int view_loc = glGetUniformLocation(shader_prog_, "view");
  int projection_loc = glGetUniformLocation(shader_prog_, "projection");

  glUniformMatrix4fv(model_loc, 1, GL_FALSE, glm::value_ptr(model));
  glUniformMatrix4fv(view_loc, 1, GL_FALSE, glm::value_ptr(view));
  glUniformMatrix4fv(projection_loc, 1, GL_FALSE, glm::value_ptr(projection));

  glBindVertexArray(VAO_);
  glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0);
      
  glfwSwapBuffers(window_.ptr);
  glfwPollEvents();
}
