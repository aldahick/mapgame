# mapgame

Play strategy, on a map.

## Playing

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install dependencies: `apt install libsfml-dev build-essential pkg-config`
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
  - [AFG](https://data.humdata.org/dataset/geoboundaries-admin-boundaries-for-afghanistan)
  - [CHN](https://github.com/junwang23/geoCN/blob/master/geojson/china_provinces.json)
  - [RUS](https://data.humdata.org/dataset/geoboundaries-admin-boundaries-for-russian-federation)
  - [USA](https://github.com/PublicaMundi/MappingAPI/blob/master/data/geojson/us-states.json)
