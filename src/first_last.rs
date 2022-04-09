//Find first and last occurance of a target in a given array


pub fn first_last(list: Vec<i32>, target: i32) -> Vec<i32> {
    let mut first: i32 = -1;
    let mut last: i32 = -1;
    let mut first_found: bool = false;

    for x in 0..list.len() {
        if list[x] == target && !first_found {
            first = x as i32;
            first_found = true;
        }
        if list[x] == target{
            last = x as i32;
        }
    }

    let mut res: Vec<i32> = Vec::new();
    res.push(first.clone());
    res.push(last.clone());

    res
}