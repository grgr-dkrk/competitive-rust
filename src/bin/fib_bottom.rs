fn fib_bottom(n: u32) -> u32 {
	let mut dp = vec![0, 1];
	for i in 2..=n {
		dp.push(dp[(i - 1) as usize] + dp[(i - 2) as usize]);
	}
	return dp[n as usize];
}

#[cfg(test)]
mod tests {
	use super::fib_bottom;

	#[test]
	fn test_fib_bottom() {
		assert_eq!(fib_bottom(0), 0);
		assert_eq!(fib_bottom(1), 1);
		assert_eq!(fib_bottom(2), 1);
		assert_eq!(fib_bottom(3), 2);
		assert_eq!(fib_bottom(4), 3);
		assert_eq!(fib_bottom(5), 5);
		assert_eq!(fib_bottom(6), 8);
		assert_eq!(fib_bottom(7), 13);
		assert_eq!(fib_bottom(8), 21);
		assert_eq!(fib_bottom(9), 34);
		assert_eq!(fib_bottom(10), 55);
	}
}
