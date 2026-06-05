fn oddies (nums: Vec<i32>) -> i32 {
    let mut count = 0;
    for num in nums {
        if num % 2 == 1 {
            count += 1;
        }
    }
    count
}

#[test]
fn test_oddies() {
    assert_eq!(oddies(vec![1,2,3,4,5,6,7,8,9,10]), 5);
    assert_eq!(oddies(vec![1,2,3,4,5,6,7,8,9,10,11]), 0);
    assert_eq!(oddies(vec![1,2,3,4,5,6,7,8,9,10,11,12]), 2);
}

fn evenies (nums: Vec<i32>) -> i32 {
    let mut count = 0;
    for num in nums {
        if num % 2 == 0 {
            count += 1;
        }
    }
    count
}

#[test]
fn test_evenies() {
    assert_eq!(evenies(vec![1,2,3,4,5,6,7,8,9,10]), 5);
    assert_eq!(evenies(vec![1,2,3,4,5,6,7,8,9,10,11]), 0);
    assert_eq!(evenies(vec![1,2,3,4,5,6,7,8,9,10,11,12]), 2);
}
