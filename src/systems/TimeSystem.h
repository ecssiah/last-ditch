#ifndef TIMESYSTEM_H
#define TIMESYSTEM_H

#include <chrono>

#include "../components/Input.h"
#include "../components/Render.h"
#include "../components/Time.h"

class TimeSystem
{
public:
  TimeSystem(Input& input, Render& render, Time& time);

  void Initialize();
  void StartFrame();
  void EndFrame();

private:
  void Tick();

  Input& input_;
  Render& render_;
  Time& time_;

  std::chrono::steady_clock::time_point start_;
  std::chrono::steady_clock::time_point end_;
};

#endif
