use std::time::Instant;

fn main() {
  let start = Instant::now();
  let raw_pixels =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day08.txt")).replace("\n", "");

  let width = 25;
  let height = 6;
  let pixel_layers = raw_pixels
    .as_bytes()
    .chunks(width * height)
    .map(|c| {
      c.iter()
        .copied()
        .map(|char_digit| char_digit - '0' as u8)
        .collect::<Vec<u8>>()
    })
    .collect::<Vec<_>>();

  let lowest_layer = pixel_layers
    .iter()
    .min_by_key(|l| l.iter().filter(|d| **d == 0).count())
    .unwrap();
  let ones = lowest_layer.iter().filter(|d| **d == 1).count();
  let twos = lowest_layer.iter().filter(|d| **d == 2).count();

  println!("ones X twos for lowest layer: {}", ones * twos);

  let pixels = pixel_layers
    .iter()
    .fold(vec![2_u8; width * height], |mut pixels, layer| {
      pixels
        .iter_mut()
        .zip(layer.iter().copied())
        .for_each(|(p, layer_pixel)| {
          if *p == 2 && layer_pixel < 2 {
            *p = layer_pixel;
          }
        });
      pixels
    });

  println!();
  pixels.chunks(width).for_each(|line| {
    line
      .iter()
      .copied()
      .for_each(|p| print!("{}", if p == 0 { " " } else { "#" }));
    println!();
  });
  println!();

  println!("Took {:?}", start.elapsed());
}
