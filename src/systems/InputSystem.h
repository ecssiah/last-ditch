#ifndef INPUT_SYSTEM_H
#define INPUT_SYSTEM_H

#include <GLFW/glfw3.h>

#include "../components/Input.h"
#include "../components/Window.h"

class InputSystem
{
public:
  InputSystem(Input& input, Window& window);

  void Initialize();
  void Update();

  void KeyCallback(
    GLFWwindow* window, int key, int scancode, int action, int mods
  );
  void CursorPosCallback(
    GLFWwindow* window, double xpos, double ypos
  );

private:
  static void key_callback(
    GLFWwindow* window, int key, int scancode, int action, int mods
  );
  static void cursor_position_callback(
    GLFWwindow* window, double xpos, double ypos
  );

  Input& input_;
  Window& window_;
}; 

#endif // INPUT_SYSTEM_H
