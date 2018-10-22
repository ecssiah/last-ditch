#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <string>
#include <unordered_map>
#include <GLFW/glfw3.h>

#include "../components/Render.h"
#include "../components/Input.h"
#include "../components/Camera.h"
#include "../components/Map.h"
#include "../components/Chunk.h"

class RenderSystem
{
public:
  RenderSystem(Input& input, Render& render, Camera& camera, Map& map);
  ~RenderSystem();

  void Initialize();
  void Update();

private:
  void RunTests();
  void RenderChunk(Chunk& chunk);

  void LoadTexture(const std::string& filename);

  Render& render_;
  Input& input_;
  Camera& camera_;
  Map& map_;

  std::unordered_map<std::string, unsigned> textures;

  unsigned shader_program_; 
  unsigned VBO_, VAO_;
};

#endif // RENDER_SYSTEM_H
