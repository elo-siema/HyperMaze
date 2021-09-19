# HyperMaze
An exploration in non-euclidian geometry. 2.5D maze game. Inspired by [CodeParade's Hyperbolica][1] and [ZenoRogue's HyperRogue][2].


![][image-1]


## How does it work?
The game takes place in a 2-dimensional space with Gaussian curvature = -1. It differs from "normal" euclidian geometry in that
Euclid's 5th axiom ([Paralell postulate][6]) is not preserved. This has a few interesting consequences:

1. Sum of angles in a triangle is less than 180 degrees
2. Non-intersecting lines have a point of minimum distance and diverge from both sides of that point
3. Ratio of circle's circumference to its diameter is greater than pi
4. Moving up, right, down, left leaves you where you started, but rotated

Projecting this space onto a computer screen, as well as applying transformations (translation, rotation) is tricky. The game does it as follows:

1. Map is stored as an SVG file, representing the Beltrami-Klein disk model. Pickups are represented by `<ellipse>` tags, and walls by `<line>`.

2. Next, the map is converted to [Minkowski hyperboloid model][8]. This is done so that transformations of the space with player movement are easy to implement and formulas are analogous to the ones used in Euclidian space. This approach was suggested by ZenoRogue, and after trying to research gyrovectors, I can definitely see why. [Very helpful StackExchange thread][9]

3. To render a frame, current state of the world is again converted to the Beltrami-Klein disk model, and then using polar coordinates in the Klein metric, to "normal" 3D scene in Euclidean space. This is then rendered by Macroquad.

4. When tab is pressed, a top-down minimap of a Klein disk is shown instead.


## How to Build
To build the desktop or the browser version you need to install the Rust  compiler first:

```bash
curl https://sh.rustup.rs -sSf | sh
```

During the installation, you may be asked for installation options. Just press Enter to select the default option. After the installation succeeded make sure that all environment variables are set up:

```bash
source ~/.profile
```

### The Desktop Version
You can build an run the desktop version by typing 

```bash
cargo run
```

### The Browser Version
TODO - there's some issue preventing the build. To be debugged.


[1]:	https://www.youtube.com/watch?v=EMKLeS-Uq_8
[2]:	https://roguetemple.com/z/hyper/
[3]:	https://github.com/hydrixos/raycaster-rust
[4]:	https://elo-siema.github.io/hyperbolic-raycaster-rust/
[5]:	https://emscripten.org
[6]:	https://emscripten.org
[7]:	https://en.wikipedia.org/wiki/Poincar%C3%A9_disk_model
[8]:	https://en.wikipedia.org/wiki/Hyperboloid_model
[9]:    https://math.stackexchange.com/questions/1862340/what-are-the-hyperbolic-rotation-matrices-in-3-and-4-dimensions?newreg=0a895728ef9c48ad814e2f06eafb3862
[10]:	https://github.com/hydrixos/raycaster-swift

[image-1]:	doc/screenshot.png
