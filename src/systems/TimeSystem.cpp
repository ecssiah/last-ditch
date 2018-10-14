#include <algorithm>

#include "TimeSystem.h"
#include "../constants/SimulationConstants.h"

using namespace std;

TimeSystem::TimeSystem(Input& input, Window& window)
  : input_(input)
  , window_(window)
{
}

void TimeSystem::Initialize()
{
}

void TimeSystem::StartFrame()
{
  start_ = chrono::steady_clock::now();
}

void TimeSystem::EndFrame()
{
  end_ = chrono::steady_clock::now();

  auto microseconds = 
    chrono::duration_cast<chrono::microseconds>(end_ - start_).count();

  window_.dt = min(MAX_DELTA_TIME, 1e-6 * microseconds);

  if (!input_.pause)
  {
  }
}

