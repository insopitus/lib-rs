fn coin_change(coins: &[u32], target: u32) -> u32 {
    todo!()
}
fn count(amounts: &[u32]) -> u32 {
    amounts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::coin_change;

    #[test]
    fn basic() {
        assert_eq!(coin_change(&[1, 5, 10, 25], 100), 4);
        assert_eq!(coin_change(&[1, 5, 10, 25], 10), 1);
        assert_eq!(coin_change(&[1, 5, 10, 25], 6), 2);
        assert_eq!(coin_change(&[1, 5, 10, 25], 2), 2);
    }
}
