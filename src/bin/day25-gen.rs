fn main() {
  let items = [
    "monolith",
    "wreath",
    "mug",
    "astrolabe",
    "manifold",
    "sand",
    "mouse",
    "space law space brochure",
  ];
  for i in 0_u8..=255 {
    items.iter().for_each(|i| println!("drop {}", i));
    for j in 0..items.len() as u8 {
      if i & 1 << j > 0 {
        println!("take {}", items[j as usize]);
      }
    }
    println!("west");
  }
}
