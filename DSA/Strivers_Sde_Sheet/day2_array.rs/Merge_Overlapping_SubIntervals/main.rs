impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0])); // Sort intervals by start time

        let mut merged: Vec<Vec<i32>> = vec![]; // Result vector

        for interval in intervals {
            if let Some(last) = merged.last_mut() {
                if last[1] >= interval[0] {
                    last[1] = last[1].max(interval[1]); // comapring for the max value
                } else {
                    merged.push(interval);
                }
            } else {
                merged.push(interval);
            }
        }
        merged
    }
}
