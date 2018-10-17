#ifndef TIMESYSTEM_H
#define TIMESYSTEM_H

#include <chrono>

#include "../components/Input.h"
#include "../components/Render.h"

class TimeSystem
{
public:
  TimeSystem(Input& input, Render& render);

  void Initialize();
  void StartFrame();
  void EndFrame();

private:
  Input& input_;
  Render& render_;

  std::chrono::steady_clock::time_point start_;
  std::chrono::steady_clock::time_point end_;
};

#endif /* TIMESYSTEM_H */
