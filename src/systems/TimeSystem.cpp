#include <algorithm>

#include "TimeSystem.h"
#include "../constants/SimulationConstants.h"

TimeSystem::TimeSystem(Input& input)
  : input_(input)
  , dt_(0.0)
{
}

void TimeSystem::Initialize()
{
}

double TimeSystem::Update()
{
  auto microseconds(
    std::chrono::duration_cast<std::chrono::microseconds>(end_ - start_).count()
  );

  dt_ = std::min(MAX_DELTA_TIME, 1e-6 * microseconds);

  if (!input_.pause)
  {
  }

  start_ = std::chrono::steady_clock::now();

  return dt_;
}

void TimeSystem::Tick()
{
  end_ = std::chrono::steady_clock::now();
}

