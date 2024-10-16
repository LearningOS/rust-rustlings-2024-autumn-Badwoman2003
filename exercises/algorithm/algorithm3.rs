/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn partition<T: PartialOrd + Copy>(array: &mut [T], left: usize, right: usize) -> usize {
    let (mut i, mut j) = (left, right);
    while i < j {
        while (i < j) && (array[j] >= array[left]) {
            j -= 1;
        }
        while (i < j) && (array[i] <= array[left]) {
            i += 1;
        }
        array.swap(j, i);
    }
    array.swap(i, left);
    return i;
}
fn sort<T: PartialOrd + Copy>(array: &mut [T]) {
    //TODO
    if array.len() <= 1 {
        return;
    }
    let end = array.len() - 1;
    let sort_idx = partition(array, 0, array.len() - 1);
    sort(&mut array[0..=sort_idx]);
    sort(&mut array[(sort_idx + 1)..=end]);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
