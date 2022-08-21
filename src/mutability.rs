fn largest_element_of<'a, T: PartialOrd>(v: &'a mut Vec<T>) -> Option<&'a mut T> {
    let mut largest: Option<&'a mut T> = None;
    for item in v {
        if let Some(current) = &mut largest {
            if *current < &mut *item {
                largest = Some(item);
            }
        } else {
            largest = Some(item);
        }
    }
    largest
}

pub fn try_crazy_stuff() {
    let mut stuff = vec![1, 2, 3, 5, 27, 3, 2, 1];
    let wat = largest_element_of(&mut stuff);
    if let Some(num) = wat {
        *num = 42;
    }

    println!("stuff? {:?}", stuff);
}

#[cfg(test)]
mod tests {
    use super::largest_element_of;

    #[test]
    fn gets_the_largest() {
        let mut nums = vec![17, 18, 20, 1, 32, 12, 19, 7];
        let largest = largest_element_of(&mut nums);
        assert_eq!(*largest.unwrap(), 32);
    }
}
