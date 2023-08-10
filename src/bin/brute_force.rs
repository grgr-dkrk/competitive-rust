use std::io::{self, BufRead};

fn count_with_brute_force(n: i32, s: i32) -> i32 {
	let mut count = 0;

	for x1 in 1..=n {
		for x2 in 1..=n {
			if x1 + x2 <= s {
				count += 1;
			}
		}
	}

	count
}

fn main() {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines().map(|line| line.unwrap());

	let n: i32 = lines.next().unwrap().parse().unwrap();
	let s: i32 = lines.next().unwrap().parse().unwrap();

	let count = count_with_brute_force(n, s);

	println!("{}", count);
}

#[cfg(test)]
mod tests {
	use super::count_with_brute_force;

	#[test]
	fn should_be_return_correctly() {
		assert_eq!(count_with_brute_force(3, 4), 6);
		assert_eq!(count_with_brute_force(869, 120), 7140);
	}
}
