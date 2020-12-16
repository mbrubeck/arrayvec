#![cfg(feature="unstable-const-generics")]

extern crate arrayvec;

use arrayvec::r#const::ArrayVec;
use std::mem;
use arrayvec::CapacityError;


#[test]
fn test_push_pop() {

    let mut vec: ArrayVec<Vec<i32>, 3> = ArrayVec::new();

    vec.push(vec![1, 2, 3, 4]);
    vec.push(vec![10]);
    vec.push(vec![-1, 13, -2]);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.pop(), Some(vec![-1, 13, -2]));
    assert_eq!(vec.len(), 2);

    /*
    for elt in &vec {
        assert_eq!(elt.iter().fold(0, Add::add), 10);
    }

    let sum_len = vec.into_iter().map(|x| x.len()).fold(0, Add::add);
    assert_eq!(sum_len, 8);
    */
}

