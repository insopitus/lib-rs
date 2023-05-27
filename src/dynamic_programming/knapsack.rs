// 0/1 knapsack problem
use std::{cmp, collections::HashMap};

#[derive(Clone, Copy)]
struct Item {
    value: i32,
    weight: i32,
}
fn get_hash_key(items: &[Item], max_weight: i32) -> String {
    format!("{},{}", &items.len(), max_weight)
}

fn solution(items: &[Item], max_weight: i32) -> i32 {
    // let memo: HashMap<String, i32> = HashMap::new();
    // if let Ok(value) = memo.get(&get_hash_key(items,max_weight)){
    //     return value
    // }
    println!("item-len: {}, max_weight: {}", items.len(), max_weight);
    let first_item = if let Some(i) = items.first() {
        i
    } else {
        return 0;
    };
    if max_weight <= 0 {
        return 0;
    }

    let result = cmp::max(
        solution(&items[1..], max_weight),
        if max_weight >= first_item.weight {
            solution(&items[1..], max_weight - first_item.weight) + first_item.value
        } else {
            0
        },
    );
    println!(
        "level: {},  result: {}, max_weight: {},",
        3 - items.len(),
        result,
        max_weight,
    );
    result
}
fn solution_cached(items: &[Item], max_weight: i32) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution_input1() {
        let items = vec![
            Item {
                value: 1,
                weight: 1,
            },
            Item {
                value: 2,
                weight: 2,
            },
            Item {
                value: 3,
                weight: 3,
            },
        ];
        assert_eq!(solution(&items, 5), 5);
        // assert_eq!(solution_v1(&items, 4), 2);
        // assert_eq!(solution_v1(&items, 8), 4);

        let items2 = vec![
            Item {
                value: 2,
                weight: 2,
            },
            Item {
                value: 1,
                weight: 2,
            },
            Item {
                value: 10,
                weight: 3,
            },
        ];
        assert_eq!(solution(&items2, 2), 2);
        assert_eq!(solution(&items2, 3), 10);
    }
}
