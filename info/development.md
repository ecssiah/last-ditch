# Development

## Simulation

### Physics

#### Collision Algorithm
1. **Physics AABB and Velocity**  
The dynamic agent has a world-space AABB and a velocity vector. All physics and collision resolution is applied to this AABB.

2.	**Axis-Separated Movement**  
Movement and collision resolution are performed one axis at a time (X, Z, Y), ensuring stable, predictable results and natural sliding along walls. Y is resolved last to make wall sliding more consistent.

3.	**Sweep and Resolve**  
For each axis:
    - Apply the velocity on that axis to the AABB.
    - Check for overlaps with nearby solid voxels.
    - If overlapping, compute the penetration depth and resolve by pushing the AABB back along that axis.
    - Zero out velocity along that axis if a collision occurred.

4.	**Thread-Safe Design**  
The AABB used in physics is separate from the display position. Rendering and other threads can safely read the display position at any time, without being exposed to intermediate collision states.

5.	**Final Sync**  
After resolving all axes and completing the physics step, the agent’s display position (typically its center) is updated to match the center of the resolved AABB.

## Interface
