
use rand::Rng;
use rand::thread_rng;

pub fn select_random_index<T>(vec: &Vec<T>) -> Option<usize> {
    if vec.len() > 0 {
        let index = thread_rng().gen_range(0, vec.len());
        Some(index)
    } else {
        None
    }
}

pub fn select_random_matching_index<T: Eq>(vec: &Vec<T>, target: T) -> Option<usize> {
    let mut indexes = Vec::new();
    for (i, element) in vec.iter().enumerate() {
        if *element == target {
            indexes.push(i);
        }
    }

    select_random_index(&indexes).map(|index| indexes[index])
}
