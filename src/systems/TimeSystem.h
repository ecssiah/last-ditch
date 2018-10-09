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
  Input& input;

  double dt;

  std::chrono::steady_clock::time_point start;
  std::chrono::steady_clock::time_point end;
};

#endif /* TIMESYSTEM_H */
