//use std::sync::mpsc;
//use std::thread;

fn actual_sort<'a>(array: Vec<&'a u32>) -> Vec<&'a u32> {
    if array.len() <= 1 {
        return match array.get(0) {
            Some(n) => { vec![n] },
            None => { vec![] }
        };
    }

    let pivot = &array[0];
    let mut left: Vec<&u32> = Vec::new();
    let mut right: Vec<&u32> = Vec::new();

    for n in &array[1..] {
        if n >= pivot {
            right.push(n);
        } else {
            left.push(n);
        }
    }

    let mut return_value = actual_sort(left);
    return_value.push(pivot);
    return_value.append(&mut actual_sort(right));

    return_value
}

pub fn sort<'a>(array: &'a Vec<u32>) -> Vec<&'a u32> {
    if array.len() <= 1 {
        return match array.get(0) {
            Some(n) => { vec![n] },
            None => { vec![] }
        };
    }

    let pivot = &array[0];
    let mut left: Vec<&u32> = Vec::new();
    let mut right: Vec<&u32> = Vec::new();

    for n in &array[1..] {
        if n >= pivot {
            right.push(n);
        } else {
            left.push(n);
        }
    }

    let mut return_value = actual_sort(left);
    return_value.push(pivot);
    return_value.append(&mut actual_sort(right));

    return_value
}

//pub fn sort(array: Vec<u32>) -> Vec<u32> {
//    if array.len() <= 1 {
//        return array;
//    }
//
//    let iter = array.iter();
//
//    let (left, pivot, right) = separate(iter);
//
//    let (tx, rx) = mpsc::channel();
//
//    thread::spawn(move || {
//        tx.send(actual_sort(right)).unwrap();
//    });
//
//    let mut return_value = actual_sort(left.to_vec());
//    return_value.push(pivot);
//    return_value.append(&mut rx.recv().unwrap());
//
//    return_value
//}
