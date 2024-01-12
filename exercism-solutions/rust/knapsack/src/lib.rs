#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let max_weight_usize = max_weight as usize;
    let mut values = vec![vec![0; max_weight_usize + 1]; items.len() + 1];

    for item_idx in 1..=items.len() {
        let item = &items[item_idx - 1];
        let weight_usize = item.weight as usize;
        let value_usize = item.value as usize;

        for curr_capacity in 1..=max_weight_usize {
            let mut curr_value = values[item_idx - 1][curr_capacity];
            if curr_capacity >= weight_usize {
                curr_value = curr_value
                    .max(values[item_idx - 1][curr_capacity - weight_usize] + value_usize);
            }

            values[item_idx][curr_capacity] = curr_value;
        }
    }

    values[items.len()][max_weight_usize] as u32
}
