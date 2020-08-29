// question 119
pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut last_row = vec![];

    let row_index = (row_index + 1) as usize;
    for i in 1..=row_index {
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
    }

    last_row
}
