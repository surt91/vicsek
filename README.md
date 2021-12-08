#  Vicsek :bird::bird::bird:

This is an implementation of a (modified) [Vicsek model](https://doi.org/10.1103/PhysRevLett.75.1226)
for flocking behavior.

![flying spins](example.png)

It is implemented in Rust and will show an OpenGL animation. Via command line
argument one can create a .mp4 using `gnuplot` and `ffmpeg`.

Just start it with `cargo`, (you can get it at, eg., [rustup.rs](https://rustup.rs/)):

```bash
cargo run --release
```

## :whale: Docker

If you are running Linux and have an X server installed (if you do not know what
this means, it is probably true; XWayland does also work), you can also use the provided docker container:

```bash
docker-compose build
# this is needed to allow access to your X-server from within the Docker container
xhost +local:
docker-compose up
```
