# SummerLab2021 CO2 visualizer

Real-time CO2 level visualizer, made for a workshop celebrated in [Summerlab2021-Tabakalera](https://www.tabakalera.eus/es/summerlab-2021). 
The visualizer pulls the data from a cloud-based storage and plots the data in a simple 2D graph.

## Dependencies

* `curl`
* `libxkbcommon-devel` (see [egui deps.](https://github.com/emilk/egui#demo))
* `libxcb-devel` (see [egui deps.](https://github.com/emilk/egui#demo))

## How to use

1. Change `DB_URL` and `API_KEY` variables in `src/lib.rs` (otherwise it won't compile).
2. run `cargo run` inside the repo. (for release mode build: `cargo build --release && ./target/release/sl21-co2 `)
