# mapgame

Play strategy, on a map.

## Playing

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [SFML](https://docs.rs/sfml/latest/sfml/). On Ubuntu, do `apt install libsfml-dev build-essential pkg-config libssl-dev`
3. Execute `cargo run .`

## Todo

- [x] Continuous:
  - [x] Integration
  - [ ] Deployment (releases)
- [ ] Nations
  - [x] Download GeoJSON
  - [x] Load from GeoJSON
  - [ ] Color
- [ ] Provinces
  - [ ] Download GeoJSON
  - [x] Load from GeoJSON
- [ ] View
  - [ ] Zoom
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
- [ ] Networking
- [ ] Multiplayer

## Credits

- World map (nations): TODO
- Province maps:
  - [CHN](https://github.com/junwang23/geoCN/blob/master/geojson/china_provinces.json)
  - [USA](https://github.com/PublicaMundi/MappingAPI/blob/master/data/geojson/us-states.json)
