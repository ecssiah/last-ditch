#ifndef UI_SYSTEM_H
#define UI_SYSTEM_H

#include "../components/Input.h"

class UISystem
{
public:
  UISystem(Input& input);

  void Initialize();
  void Update();

private:
  Input& input_;

};

#endif // UI_SYSTEM_H
