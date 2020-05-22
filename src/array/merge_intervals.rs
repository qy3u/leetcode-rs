// question 56
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.len() < 2 {
        return intervals;
    }

    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[1]);
    intervals.sort_by_key(|v| v[0]);

    let mut result: Vec<Vec<i32>> = vec![];
    let mut i = 0;
    while i < intervals.len() {
        // try to eat next
        let curr_begin = intervals[i][0];
        let mut curr_end = intervals[i][1];

        let mut j = i + 1;
        while j < intervals.len() {
            let next_begin = intervals[j][0];
            let next_end = intervals[j][1];

            if next_begin > curr_end {
                break;
            }

            curr_end = std::cmp::max(curr_end, next_end);
            j += 1;
        }

        result.push(vec![curr_begin, curr_end]);
        i = j;
    }
    result
}
