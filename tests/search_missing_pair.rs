use rustius::search_missing_pair;

#[test]
fn test0() {
    let mut data = vec![12, 23, 12, -232, 12, 0, 23, 42, -232, 12];
    assert_eq!(search_missing_pair(&mut data), 42);
}

#[test]
fn test1() {
    let mut data = vec![0, 0, 0, 0, 0, -232, -232, 0, 1, 0, 0, 0];
    assert_eq!(search_missing_pair(&mut data), 1);
}
#[test]
fn test2() {
    let mut data = vec![1, 0, 0, 0, -232, 0, 0, -232, 1, 0, 0, 1];
    assert_eq!(search_missing_pair(&mut data), 1);
}
#[test]
fn test3() {
    let mut data = vec![
        3211, 1, 123, 164, 7685, -232, 164, 123, 3211, 1, 7685, -232, 0, 1,
    ];
    assert_eq!(search_missing_pair(&mut data), 1);
}
