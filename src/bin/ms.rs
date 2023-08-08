use std::env;

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

	let sorted_numbers = merge_sort(numbers);

	println!("{:?}", sorted_numbers);
}

fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
	if arr.len() <= 1 {
		return arr;
	}

	let mid = arr.len() / 2;
	let left = merge_sort(arr[..mid].to_vec());
	let right = merge_sort(arr[mid..].to_vec());

	merge(left, right)
}

fn merge(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32> {
	let mut result = Vec::new();

	while !left.is_empty() && !right.is_empty() {
		if left[0] <= right[0] {
			result.push(left.remove(0));
		} else {
			result.push(right.remove(0));
		}
	}

	while !left.is_empty() {
		result.push(left.remove(0));
	}

	while !right.is_empty() {
		result.push(right.remove(0));
	}

	result
}

#[cfg(test)]
mod tests {
	use super::merge_sort;

	#[test]
	fn should_return_expect() {
		let arr = vec![7, 3, 1, 2, 4, 5, 9, 6, 8];
		let sorted_arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

		assert_eq!(merge_sort(arr), sorted_arr);
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

			assert_eq!(merge_sort(arr), sorted_arr);
		}
	}
}
