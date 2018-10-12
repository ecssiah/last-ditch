#include <algorithm>

#include "TimeSystem.h"
#include "../constants/SimulationConstants.h"

using namespace std;

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
  auto microseconds = 
    chrono::duration_cast<chrono::microseconds>(end_ - start_).count();

  dt_ = min(MAX_DELTA_TIME, 1e-6 * microseconds);

  if (!input_.pause)
  {
  }

  start_ = chrono::steady_clock::now();

  return dt_;
}

void TimeSystem::Tick()
{
  end_ = chrono::steady_clock::now();
}

