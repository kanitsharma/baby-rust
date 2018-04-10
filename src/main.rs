fn main() {
  let a = ['a', 'b', 'c'];
  let sliced = &a[1..2];
  println!("{:?}", sliced);
}
