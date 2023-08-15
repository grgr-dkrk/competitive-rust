fn dp_knapsack(n: usize, w: usize, weight: &Vec<usize>, value: &Vec<usize>) -> usize {
	let mut dp = vec![vec![0; w + 1]; n + 1];

	for i in 0..n {
		for j in 0..=w {
			if j >= weight[i] {
				dp[i + 1][j] = std::cmp::max(dp[i][j - weight[i]] + value[i], dp[i][j]);
			} else {
				dp[i + 1][j] = dp[i][j];
			}
		}
	}

	dp[n][w]
}

#[cfg(test)]
mod tests {
	use super::dp_knapsack;
	#[test]
	fn test_knapsack() {
		let weight = vec![2, 1, 3, 2];
		let value = vec![3, 2, 4, 2];
		let n = weight.len();
		let w = 5;
		assert_eq!(dp_knapsack(n, w, &weight, &value), 7);
	}
}
