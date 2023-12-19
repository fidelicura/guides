#[allow(unused)]


fn bubble(arr: &mut Vec<i32>) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        (1..arr.len())
            .into_iter()
            .for_each(|i| {
                if arr[i - 1] < arr[i] {
                    arr.swap(i - 1, i);
                    swapped = true;
                }
            });
    }
}

fn insertion(arr: &mut Vec<i32>) {
    let start = 1;
    (start..arr.len())
        .into_iter()
        .for_each(|i| {
            let mut j = i;
            // we might implement binary search instead of while loop swap
            while j > 0 && arr[j - 1] < arr[j] {
                arr.swap(j - 1, j);
                j -= 1;
            } 
        });
}

fn selection(arr: &mut Vec<i32>) {
    let len = arr.len();

    (0..len)
        .into_iter()
        .for_each(|i| {
            (i + 1..len)
                .into_iter()
                .for_each(|j| {
                    if arr[i] < arr[j] {
                        arr.swap(i, j);
                    }
                });

            // or (ascending order only, order-dependendent tests will fail)

            // let (mut j, _) = arr[i..]
            //     .iter()
            //     .enumerate()
            //     .min_by_key(|&(_, val)| val)
            //     .expect("vec is empty");
            // j += i;
           
            // if arr[i] != arr[j] {
            //     arr.swap(i, j);
            // }
        });
}

fn quick(arr: &mut Vec<i32>) {}

fn heap(arr: &mut Vec<i32>) {}

fn merge(arr: &mut Vec<i32>) {}


fn sort_to_test(arr: &mut Vec<i32>) {
    selection(arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr = vec![];
        sort_to_test(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element_array() {
        let mut arr: Vec<i32> = vec![5];
        sort_to_test(&mut arr);
        assert_eq!(arr, vec![5]);
    }

    #[test]
    fn test_sorted_positive_array() {
        let mut arr = vec![4, 3, 2, 1];
        sort_to_test(&mut arr);
        assert_eq!(arr, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_sorted_negative_array() {
        let mut arr = vec![-1, -2, -3, -4];
        sort_to_test(&mut arr);
        assert_eq!(arr, vec![-1, -2, -3, -4]);
    }

    #[test]
    fn test_sorted_both_array() {
        let mut arr = vec![4, 3, -1, -2];
        sort_to_test(&mut arr);
        assert_eq!(arr, vec![4, 3, -1, -2]);
    }

    #[test]
    fn test_unsorted_positive_array() {
        let mut arr = vec![3, 1, 4, 2];
        sort_to_test(&mut arr);
        assert_eq!(arr, vec![4, 3, 2, 1]);
    }
    
    #[test]
    fn test_unsorted_negative_array() {
        let mut arr = vec![-3, -1, -4, -2];
        sort_to_test(&mut arr);
        assert_eq!(arr, vec![-1, -2, -3, -4]);
    }
    
    #[test]
    fn test_unsorted_both_array() {
        let mut arr = vec![3, -1, 4, -2];
        sort_to_test(&mut arr);
        assert_eq!(arr, vec![4, 3, -1, -2]);
    }
}
