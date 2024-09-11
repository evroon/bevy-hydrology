# Bevy hydrology

<p align="center">
  <img
    width="800"
    src="misc/preview.gif"
    alt="Hydrology simulation in Bevy"
  />
</p>

Hydrology simulation in Bevy based on [this blog
post](https://nickmcd.me/2020/04/15/procedural-hydrology).

It simulates particles falling on the terrain, which descend down the slope. The particles affect
the terrain by erosion and sedimentation.

> [!NOTE]
> The erosion and sedimentation is (contrary to the original article) fully simulated in a [compute shader](https://github.com/evroon/bevy-hydrology/blob/master/assets/shaders/erosion.wgsl).


# Usage
To quickly run it to see how it works, clone this repo and run `cargo run --release`:
```bash
git clone git@github.com:evroon/bevy-hydrology.git
cd bevy-hydrology
cargo run --release
```

# License
Licensed under [MIT](https://choosealicense.com/licenses/mit/): [LICENSE](LICENSE).
