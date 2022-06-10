fn main() {
    let org_arr = [1, 2, 3, 5, 6, 1, 8, 10, 11];
    let sub_arr = [6, 8, 10];

    let res = is_sub_array(&sub_arr, &org_arr);

    println!("result: {:#?}", res);
}

fn is_sub_array(sub_arr: &[i32], org_arr: &[i32]) -> bool {
    if sub_arr.len() == 0 {
        return true;
    } else {
        for frame in org_arr.windows(sub_arr.len()) {
            if frame == sub_arr {
                return true;
            }
        }
        return false;
    }
}

#[test]
fn test_empty_sub_array() {
    let sub_arr: [i32; 0] = [];
    let org_arr: [i32; 4] = [1, 2, 3, 4];

    assert_eq!(is_sub_array(&sub_arr, &org_arr), true);
}

#[test]
fn test_sub_array_larger_than_org_arr() {
    let sub_arr: [i32; 5] = [1, 2, 3, 4, 5];
    let org_arr: [i32; 4] = [1, 2, 3, 4];

    assert_eq!(is_sub_array(&sub_arr, &org_arr), false);
}

#[test]
fn test_sub_array_is_included_org_array() {
    let sub_arr: [i32; 2] = [2, 3];
    let org_arr: [i32; 4] = [1, 2, 3, 4];

    assert_eq!(is_sub_array(&sub_arr, &org_arr), true);
}

#[test]
fn test_sub_array_is_not_included_org_array() {
    let sub_arr: [i32; 2] = [4, 5];
    let org_arr: [i32; 4] = [1, 2, 3, 4];

    assert_eq!(is_sub_array(&sub_arr, &org_arr), false);
}
