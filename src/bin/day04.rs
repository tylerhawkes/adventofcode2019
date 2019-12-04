#[cfg(not(any(feature = "part-one", feature = "part-two")))]
compile_error!("Choose feature part-one or part-two");

fn main() {
  let count = (284639..=748759)
    .filter(|i| {
      let digits = (0..6)
        .rev()
        .map(|b| i / 10_u32.pow(b) % 10)
        .collect::<Vec<_>>();
      let increasing = digits.windows(2).all(|d| d[0] <= d[1]);
      #[cfg(feature = "part-one")]
      let double = digits.windows(2).any(|d| d[0] == d[1]);
      #[cfg(feature = "part-two")]
      // Check ends then the middle 3
      let double = (digits[0] == digits[1] && digits[1] != digits[2])
        || (digits[3] != digits[4] && digits[4] == digits[5])
        || digits
          .windows(4)
          .any(|d| d[0] != d[1] && d[1] == d[2] && d[2] != d[3]);
      increasing && double
    })
    .count();

  println!("Number of passwords: {}", count);
}
