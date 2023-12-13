# controller-trigger-recorder

[./assets/acc.mp4](https://github.com/AntoniosBarotsis/controller-trigger-recorder/assets/50240570/1305a434-bbd1-43fb-b4f2-34cab67a008a)

> Video is actually a bit outdated, the window is now fully transparent and the top bezel is
> invisible but I'm too lazy to re-record it

Creates a transparent chart at the bottom of the screen that shows your controller trigger inputs
over time (as shown in the video).

## Issues and Stuff

I made 90% of this in one consecutive 7 hour session for "*fun*" so there's a couple of things that
are not great about it:

- You need to `ctrl-c` the process to exit it as I have removed the window bezel things that
  normally have the X button
- This is configured for a 1920x1080p screen and you can't move or stretch the window
- Refresh rate is not tunable
- Chart could look a bit better
- Probably early panics if you forget to connect your controller before running it
- No clue what happens when there's more than 1 controller (but why would that be the case anyway right?)
- One or two times I noticed some weird performance drop for a few seconds, no idea where that
  came from or if it is still a thing
- You might need to run games in borderless windowed instead of fullscreen
- Only tested on Windows, probably doesn't work on Linux, might work on Mac

Note that all of these are actually pretty easy to fix since the project is ~170 lines of code. You
can also open an issue that I may or may not work on 👍
