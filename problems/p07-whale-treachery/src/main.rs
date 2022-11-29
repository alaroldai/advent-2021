/// Did part 1 in python with:
/// xs = [input]
/// sum(abs(x - median(xs)) for x in xs)
/// 
/// Similarly part 2:
/// sum(csum(abs(x - int(mean(xs)))) for x in xs)
/// Figuring out to use `int` instead of `round` took a bit of time though.

fn main() {
  println!("Hello, world!");
}
