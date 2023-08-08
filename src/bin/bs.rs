use std::env;

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();

	if args.is_empty() {
		panic!("No arguments provided");
	}

	let mut nums: Vec<i32> = Vec::new();

	for arg in args {
		match arg.parse::<i32>() {
			Ok(num) => nums.push(num),
			Err(_) => panic!("Invalid argument: {}", arg),
		}
	}

	let sorted_nums = bubble_sort(nums);

	println!("{:?}", sorted_nums);
}

fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
	let n = arr.len();

	for i in 0..n {
		for j in 0..n - i - 1 {
			if arr[j] > arr[j + 1] {
				arr.swap(j, j + 1);
			}
		}
	}

	arr
}

#[cfg(test)]
mod tests {
	use super::bubble_sort;

	#[test]
	fn should_return_expect() {
		let arr = vec![7, 3, 1, 2, 4, 5, 9, 6, 8];
		let sorted_arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

		assert_eq!(bubble_sort(arr), sorted_arr);
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

			assert_eq!(bubble_sort(arr), sorted_arr);
		}
	}
}
