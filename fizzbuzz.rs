fn main() {
  for num in range(1, 101) {
    if is_fifteen(num) { println!("FizzBuzz"); }
    else if is_three(num) { println!("Fizz"); }
    else if is_five(num) { println!("Buzz"); }
    else { println!("{:d}", num); }
  }
}

fn is_three(num: int) -> bool {
  num % 3 == 0
}

fn is_five(num: int) -> bool {
  num % 5 == 0
}

fn is_fifteen(num: int) -> bool {
  is_three(num) && is_five(num)
}

#[test]
fn test_is_five_with_not_five() {
  if is_five(1) { fail!("One is not five!") }
}

#[test]
fn test_is_five_with_five() {
  assert!(is_five(5), "Five should be five!")
}

#[test]
fn test_is_fifteen_with_not_fifteen() {
  if is_fifteen(1) { fail!("One is not fifteen!") }
  else if is_fifteen(5) { fail!("Five is not fifteen!") }
  else if is_fifteen(3) { fail!("Three is not fifteen!") }
}

#[test]
fn test_is_fifteen_with_fifteen() {
  assert!(is_fifteen(15), "Fifteen should be fifteen!")
}

#[test]
fn test_is_three_with_not_three() {
  if is_three(1) { fail!("One is not three!") }
}

#[test]
fn test_is_three_with_three() {
  assert!(is_three(3), "Three should be three!")
}
