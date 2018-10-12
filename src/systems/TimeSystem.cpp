#include <algorithm>

#include "TimeSystem.h"
#include "../constants/SimulationConstants.h"

TimeSystem::TimeSystem(Input& _input)
  : input(_input)
  , dt(0.0)
{
}

void TimeSystem::Initialize()
{
}

void TimeSystem::Tick()
{
  start = std::chrono::steady_clock::now();
}

double TimeSystem::Update()
{
  end = std::chrono::steady_clock::now();
  auto microseconds(
    std::chrono::duration_cast<std::chrono::microseconds>(end - start).count()
  );

  dt = std::min(MAX_DELTA_TIME, 1e-6 * microseconds);

  if (!input.pause)
  {
  }

  return dt;
}
