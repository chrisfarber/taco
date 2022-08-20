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
    let mut stuff = vec![1, 2, 3, 5, 3, 2, 1];
    let wat = largest_element_of(&mut stuff);
    if let Some(numRef) = wat {
        *numRef = 42;
    }

    println!("stuff? {:?}", stuff);
}
