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

  void init();
  void start_frame();
  void end_frame();

private:
  void tick();

  Input& input_;
  Render& render_;
  Time& time_;

  std::chrono::steady_clock::time_point start_;
  std::chrono::steady_clock::time_point end_;
};

#endif
