# Conways Game of Life

A simple implemention of the classic cellular automaton.

To run:

```
cargo run
```

If you are using Wayland then you will have to run:

```
WINIT_UNIX_BACKEND=x11 cargo run
```

This is a workaround for an issue using Winit on Wayland.

The speed is set to create a new generation every 0.5 second.

It uses ggez for the 2D graphics, as this looks to be the easiest way to put simple grpahics together.
