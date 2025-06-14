# Development

### Overview

A Civilization Garden

## Simulation

### Physics

#### Dynamic Entity Collision Algorithm

1. **Physics AABB and Displacement**  
   The dynamic agent has a world-space AABB. Its movement for the current tick is expressed as a displacement vector, typically computed using velocity and acceleration. This displacement is used for collision resolution.

2. **Axis-Separated Movement**  
   Collision resolution is performed one axis at a time (X, Z, Y), applying only the displacement along that axis. Resolving Y last improves wall sliding consistency.

3. **Sweep and Resolve**  
   For each axis:  
    1. Apply the displacement along that axis to the AABB. 2. Check for overlaps with nearby solid voxels. 3. If overlapping, compute the penetration depth and resolve by pushing the AABB back along that axis. 4. Zero out velocity on that axis if a collision occurred (optional, depending on contact handling).

4. **Thread-Safe Design**  
   The AABB used in physics is separate from the display position. Rendering and other threads can safely read the display position at any time, without being exposed to intermediate collision states.

5. **Final Sync**  
   After resolving all axes and completing the physics step, the agent’s display position (typically its center) is updated to match the center of the resolved AABB.

### Decision

**Prompt**

There is a voxel world that is separated into cube-shaped chunks of blocks.

We are now generating two layers of navigation graph. First, we have a chunk::Graph that produces a block::Node for every empty space within the chunk that has a solid surface underneath it that can be stood upon. These nodes are then connected by block::Edges that define which nodes an Agent can move between and the cost of making that move.

This is our primary goal right now. To implement a patfinding algorithm across these local chunk::Graphs that can eventually be knitted together to pathfind across larger parts of the world.

Second, there is also a world::Graph that produces a chunk::Node for every chunk in the World, and then defines chunk::Edges for these nodes. A chunk::Edge is created for every matching pair of blocks that can be used to step between two unique chunks. This means a chunk::Node can have multiple edges connecting it to a neighbor chunk.

That's the main task right now, but this pathfinding system needs to fit into a broader decision-making system that will allow Agents to make complex decisions about the World. This Simulation system will need to accept Agent work in relatively small batches and assign the work to a dedicated thread_pool that can perform the calculation without blocking the rest of the Simulation, and then return the result when it is complete for the Agent to act upon.

To simplify the overall problem, we can start by creating the foundation of the Decision system, and then getting Agents to request simple pathfinding tasks just within a single test Chunk, so that we don't need to tackle anything with the 1st layer of the navigation Graph quite yet.

## Interface
