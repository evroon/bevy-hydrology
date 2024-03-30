# Bevy hydrology

<p align="center">
  <img
    width="800"
    src="misc/preview.jpg"
    alt="Hydrology simulation in Bevy"
  />
</p>

Hydrology simulation in Bevy based on [this blog
post](https://nickmcd.me/2020/04/15/procedural-hydrology).

It simulates particles falling on the terrain, which descend down the slope. The particles affect
the terrain by erosion and sedimentation.


# Usage
To quickly run bracket to see how it works, clone it and run `docker-compose up`:
```bash
git clone git@github.com:evroon/bevy-hydrology.git
cd bevy-hydrology
cargo run --release
```

# License
Licensed under [MIT](https://choosealicense.com/licenses/mit/): [LICENSE](LICENSE).
