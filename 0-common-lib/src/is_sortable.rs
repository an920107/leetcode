pub fn is_sortable_by_rotating<T>(arr: &[T]) -> bool
where
    T: Ord,
{
    let mut decreasing_count = 0;
    for window in arr.windows(2) {
        if window[1] < window[0] {
            decreasing_count += 1;
            if decreasing_count >= 2 {
                return false;
            }
        }
    }

    if decreasing_count == 0 {
        return true;
    }

    arr[arr.len() - 1] <= arr[0]
}

#[test]
fn test_is_sortable_by_rotating() {
    assert_eq!(is_sortable_by_rotating(&[0, 1, 2, 3]), true);
    assert_eq!(is_sortable_by_rotating(&[2, 3, 0, 1]), true);
    assert_eq!(is_sortable_by_rotating(&[2, 1, 0, 3]), false);
}
