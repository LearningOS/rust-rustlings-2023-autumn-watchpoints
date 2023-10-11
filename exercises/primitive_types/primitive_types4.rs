// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];
    //其中 starting_index 是 slice 的第一个位置，ending_index 则是 slice 最后一个位置的后一个值
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
