use sfml::system::Vector2f;

/*
clever raycasting
https://stackoverflow.com/a/16391873/5850070
https://wrf.ecse.rpi.edu/Research/Short_Notes/pnpoly.html
*/
pub fn polygon_contains(point: Vector2f, polygon: &Vec<Vector2f>) -> bool {
  let polygon_len = polygon.len();
  let mut i = 0;
  let mut j = polygon_len - 1;
  let mut inside = false;
  while i < polygon_len {
    if (polygon[i].y > point.y) != (polygon[j].y > point.y) {
      let diff =
        (polygon[j].x - polygon[i].x) * (point.y - polygon[i].y) / (polygon[j].y - polygon[i].y);
      if point.x < diff + polygon[i].x {
        inside = !inside;
      }
    }
    j = i;
    i += 1;
  }
  inside
}

pub fn polygon_area(polygon: &Vec<Vector2f>) -> f32 {
  let mut area = 0.0;
  let len = polygon.len();
  for i in 0..len {
    let start = polygon[i];
    let end = polygon[if i == len - 1 { 0 } else { i + 1 }];
    area += start.x * end.y - start.y * end.x;
  }
  (area / 2.0).abs()
}
