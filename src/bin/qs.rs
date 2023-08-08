use std::env;

fn quick_sort(mut arr: Vec<i32>) -> Vec<i32> {
	if arr.len() <= 1 {
		return arr;
	}

	let pivot = arr.pop().unwrap();
	let mut left = Vec::new();
	let mut right = Vec::new();

	for x in arr {
		if x < pivot {
			left.push(x);
		} else {
			right.push(x);
		}
	}

	let mut sorted_left = quick_sort(left);
	let mut sorted_right = quick_sort(right);

	sorted_left.push(pivot);
	sorted_left.append(&mut sorted_right);

	sorted_left
}

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();

	if args.is_empty() {
		panic!("No arguments provided");
	}

	let mut numbers: Vec<i32> = Vec::new();

	for arg in args {
		match arg.parse::<i32>() {
			Ok(num) => numbers.push(num),
			Err(_) => panic!("Invalid argument: {}", arg),
		}
	}

	let sorted_numbers = quick_sort(numbers);

	println!("{:?}", sorted_numbers);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn should_return_expect() {
		let arr = vec![7, 3, 1, 2, 4, 5, 9, 6, 8];
		let sorted_arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

		assert_eq!(quick_sort(arr), sorted_arr);
	}

	#[test]
	fn should_return_expect_rand() {
		for _ in 0..100000 {
			let arr = (0..10).map(|_| rand::random::<i32>()).collect::<Vec<_>>();
			let sorted_arr = {
				let mut sorted_arr = arr.clone();
				sorted_arr.sort();
				sorted_arr
			};

			assert_eq!(quick_sort(arr), sorted_arr);
		}
	}
}
