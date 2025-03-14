# Rust 编程实战 - ECS - 推箱子小游戏

https://gitee.com/qingolo/rust-tutorials/tree/tutorial-2024/examples/sokoban/part-01

Game Sokoban
Player, Box, BoxTarget, Wall, Space

Part 01: ECS - specs

- create a world
- create entities, components
- create systems

Part 02: User Input (keyboard)

- dispatcher -> system.run_now()
- add ggez main window
- handle user inputs
- resource Move => InputKeyQueue
- EventHandler key_down_event() => save input keys into the queue
- SysMovePlayer => move player from the input keys
- EventHandler update() => run SysMovePlayer, SysShowPlayer

Links:
https://github.com/ggez/ggez
https://github.com/ggez/ggez/blob/master/examples/input_test.rs

Part 03: Rendering

- add images into Context
- GameImages as ggez Resource
- SysRender, EventHandler::draw()
  Challenges:
- coordinates: x >= 0, y >= 0, boundary
- draw wall, crate ...
- different character images for different input keys

Part 04: Animation

- add the crate keyframe in Cargo.toml
- create a system for animation updates
- add a resource for animation
- define key frames, update the system MovePlayer
- update the rendering system
  Challenges:
- animations for walking up, down, left
- use spritesheet to load image resources

Links:

- https://github.com/hannesmann/keyframe/blob/master/examples/visualizer.rs
- https://github.com/ggez/ggez/blob/master/examples/animation.rs
