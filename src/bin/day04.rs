fn main() {
  let count = (284639..=748759)
    .filter(|i| {
      let a = i / 100_000 % 10;
      let b = i / 10_000 % 10;
      let c = i / 1_000 % 10;
      let d = i / 100 % 10;
      let e = i / 10 % 10;
      let f = i % 10;
      let increasing = a <= b && b <= c && c <= d && d <= e && e <= f;

      let double = if cfg!(feature = "part-one") {
        a == b || b == c || c == d || d == e || e == f
      } else if cfg!(feature = "part-two") {
        let ad = (a == b && b != c);
        let bd = (a != b && b == c && c != d);
        let cd = (b != c && c == d && d != e);
        let dd = (c != d && d == e && e != f);
        let ed = (e != d && e == f);
        match (ad, bd, cd, dd, ed) {
          (true, false, _, _, _) => true,
          (false, true, false, _, _) => true,
          (_, false, true, false, _) => true,
          (_, _, false, true, false) => true,
          (_, _, _, false, true) => true,
          _ => false,
        }
      } else {
        panic!("Please choose feature 'part-one' or 'part-two'");
      };
      increasing && double
    })
    .count();

  println!("Number of passwords: {}", count);
}
