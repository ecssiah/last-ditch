# Development

## Simulation

### Physics

#### Dynamic Entity Collision Algorithm
1. **Physics AABB and Displacement**  
The dynamic agent has a world-space AABB. Its movement for the current tick is expressed as a displacement vector, typically computed using velocity and acceleration. This displacement is used for collision resolution.

2. **Axis-Separated Movement**  
Collision resolution is performed one axis at a time (X, Z, Y), applying only the displacement along that axis. Resolving Y last improves wall sliding consistency.

3. **Sweep and Resolve**  
For each axis:
    - Apply the displacement along that axis to the AABB.
    - Check for overlaps with nearby solid voxels.
    - If overlapping, compute the penetration depth and resolve by pushing the AABB back along that axis.
    - Zero out velocity on that axis if a collision occurred (optional, depending on contact handling).

4. **Thread-Safe Design**  
The AABB used in physics is separate from the display position. Rendering and other threads can safely read the display position at any time, without being exposed to intermediate collision states.

5. **Final Sync**  
After resolving all axes and completing the physics step, the agent’s display position (typically its center) is updated to match the center of the resolved AABB.

## Interface
