# PixApprox

A [genetic programming](https://en.wikipedia.org/wiki/Genetic_programming) experiment trying to approximate a picture with a function 

```
pixel = f(x, y)
```


A genetic programming experiment trying to approximate a picture with a function pixel = f(x, y)

Written in Rust for fearless concurrency. 

# Example output

## Filled Circle
![](/images/filled_circle.png)

Video of progress:

https://raw.githubusercontent.com/tomhal/pixapprox/main/examples/filled_circle.mp4

## Zebra skin
![](/images/zebra_skin_by_photolight.png)

Video of progress:

https://raw.githubusercontent.com/tomhal/pixapprox/main/examples/zebra1.mp4

## Pre-requisites

### Compiling and running
First install Rust. Read the instructions here: https://www.rust-lang.org/learn/get-started

### Editing
To edit Rust code, you can get [Visual Studio Code](https://code.visualstudio.com/).

When you open a `.rs` file, Visual Studio Code will suggest you install the `rust-analyzer` extension. After that you are all set.

## How to run

Since the program is computationally expensive, usually it is run with

```bash
cargo run --release
```

Remove `--release` to get a debug build with more safety checks enabled.

## Program output

The output of the program is written to the folder `result/`. 

It is preferred to empty that folder before running the application.

## Generate video from output files
Install ffmpeg and then run:
```bash
ffmpeg.exe -framerate 15 -i result/%05d.png -s:v 512x256 -c:v libx264 -profile:v high -crf 12 -pix_fmt yuv420p movie.mp4
```

[Explanation of the options](https://kb.tecplot.com/2019/02/27/use-ffmpeg-to-create-videos-from-pngs/)

# Program internals
## Values for coordinates and color
The values for x and y will go from -1.0 to 1.0 no matter what dimension of the picture.

The function output is limited to be within -1.0 and 1.0 and is then converted to a number between 0(black) and 255(white).

```rust
// State is where x and y are stored
let mut state = State::new(NVARS);

for y in 0..image.height {
    for x in 0..image.width {
        // Convert width and height from
        //   0..height/width
        // to
        //   -1.0 to +1.0
        state.vars[0] = (x as f32) / (image.width as f32) * 2.0 - 1.0;
        state.vars[1] = (y as f32) / (image.height as f32) * 2.0 - 1.0;

        let mut result = eval(prg, &state);

        // Limit the output to stay between -1.0 and 1.0
        result = result.min(1.0).max(-1.0);

        // Rescale the value to be from 0-255
        result = result * 127.0 + 128.0;

        let pix = result.trunc() as u8;
        image.data.push(pix);
    }
}
```

## Fitness function
The fitness function is the accumulated error^2 per pixel, generated images compared to goal image. 

Each error is the `abs(goal_pixel - generated_pixel)` so errors cannot compensate for errors elsewhere in the image.

The less total error the closer the generated image is to the goal image.

## Operators / Instructions

Sin(x) and cos(x) are evaluated as sin(2 \* pi \* x) and cos(2 \* pi \* x) so that x = -1.0 to 1.0 also gives a sin/cos output like that.

## Performance considerations
No profiling has been done yet.

### Compiler options
Some expensive optimization flags can be enabled in the `Cargo.toml` file. Remove the comments for `lto` and `codegen-units` to get additional performance.

### Avoidance of memory allocations
To avoid memory allocations the crate `SmallVec` is used where possible and reasonable. 

This will put several different kinds of collections onto the stack instead of the heap.

### Random number generation
Right now StdRng is being used in place of ThreadRng.

In the future measuring the difference in performance should be done since it's doubtful that cryptographically secure random number generation is really needed for the evolution.

# Saved links

## Genetic Programming video introduction
John Koza has a video describing what this program does. He calls it "Programmatic Image Compression". Watch it here: https://youtu.be/tTMpKrKkYXo?t=2155

## Convert single float to RGB
To get full RGB color instead of a grey-scale picture, this could be a solution:
https://stackoverflow.com/a/54106189
