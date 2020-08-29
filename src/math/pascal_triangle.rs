// question 118
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];

    let mut last_row = vec![];
    for i in 1..=num_rows as usize {
        let mut this_row = vec![0; i];
        match i {
            1 => {
                this_row[0] = 1;
            }

            2 => {
                this_row[0] = 1;
                this_row[1] = 1;
            }

            x => {
                this_row[0] = 1;
                this_row[x - 1] = 1;
                for i in 1..x - 1 {
                    this_row[i] = last_row[i - 1] + last_row[i];
                }
            }
        }

        last_row = this_row.clone();
        result.push(this_row);
    }

    result
}
