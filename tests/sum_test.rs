use lust::sum::sum::{sum, sum_all, sum_slice, sum_vec};

#[test]
fn sum_of_fixed_length_array() {
    let numbers = [1, 2, 3, 4, 5];
    let result = sum(numbers);
    assert_eq!(result, 15)
}
#[test]
fn sum_of_variable_length_array() {
    let numbers = vec![1, 2, 3];
    let result = sum_vec(numbers);
    assert_eq!(result, 6)
}
#[test]
fn sum_of_a_slice() {
    let numbers = &[3, 4, 5];
    let result = sum_slice(numbers);
    assert_eq!(result, 12)
}

#[test]
fn sum_of_list_of_lists() {
    let numbers1 = &[1, 2, 3];
    let numbers2 = &[2, 3, 4];
    let result = sum_all(&[numbers1, numbers2]);
    assert_eq!(result, &[6, 9])
}
#[test]
fn capacity_wont_exeed() {
    let capacity = 6;
    let mut y_points = Vec::with_capacity(capacity);
    for point in 1..5 {
        y_points.push(point);
    }
    assert_eq!(y_points.capacity(), capacity)
}
