#ifndef TIMESYSTEM_H
#define TIMESYSTEM_H

#include <chrono>

#include "../components/Input.h"

class TimeSystem
{
public:
  TimeSystem(Input& input);

  void Initialize();
  void Tick();
  double Update();

private:
  Input& input_;

  double dt_;

  std::chrono::steady_clock::time_point start_;
  std::chrono::steady_clock::time_point end_;
};

#endif /* TIMESYSTEM_H */
