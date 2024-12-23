// adapted from https://www.algobreath.com/notes/next-permutation-in-rust
pub fn next_permutation<T: PartialOrd>(nums: &mut [T]) {
    // Step 1: Find the pivot
    let mut i = nums.len() - 1;
    while i > 0 && nums[i - 1] >= nums[i] {
        i -= 1;
    }

    if i > 0 {
        // Step 2: Find the successor to pivot
        let mut j = nums.len() - 1;
        while nums[j] <= nums[i - 1] {
            j -= 1;
        }

        // Step 3: Swap
        nums.swap(i - 1, j);
    }

    // Reverse the elements from i to the end of the array
    nums[i..].reverse();
}
