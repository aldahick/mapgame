# mapgame

Play strategy, on a map.

## Playing

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install dependencies: `apt install build-essential pkg-config unzip cmake`
  a. Also see [rust-sfml](https://github.com/jeremyletang/rust-sfml#prerequisites)'s dependencies.
3. Download [SFML 2.6.2](https://github.com/SFML/SFML/releases/2.6.2) and extract it to `./sfml`
4. Execute `cargo run`

## Todo

This is a really long list.

- [x] Continuous:
  - [x] Integration
  - [ ] Deployment (releases)
- [ ] Nations
  - [x] Load from GeoJSON
  - [ ] Color
- [ ] Provinces
  - [x] Load from GeoJSON
- [ ] View
  - [ ] Zoom
  - [ ] Map modes
- [ ] Basic UI components
- [ ] Resources
- [ ] Structures
  - [ ] Construction
  - [ ] Per-province management
    - [ ] Manual
    - [ ] Automated
  - [ ] Replacement
  - [ ] Destruction (?)
- [ ] Player
  - [x] Nation selection
  - [ ] HUD
    - [ ] Resources display
- [ ] Equipment
- [ ] Infantry
- [ ] AI
- [ ] Cars
- [ ] Boats
- [ ] Planes
- [ ] Multiplayer

## Credits

- World map (nations): TODO
- Province maps:
  - [AFG](https://data.humdata.org/dataset/geoboundaries-admin-boundaries-for-afghanistan)
  - [CHN](https://github.com/junwang23/geoCN/blob/master/geojson/china_provinces.json)
  - [RUS](https://data.humdata.org/dataset/geoboundaries-admin-boundaries-for-russian-federation)
  - [USA](https://github.com/PublicaMundi/MappingAPI/blob/master/data/geojson/us-states.json)
