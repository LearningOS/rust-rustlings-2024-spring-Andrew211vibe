/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE

fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let (mut l, mut r) = (lo, hi);
    let pivot = lo;

    while l < r {
        while l < r && arr[r] >= arr[pivot] { r -= 1; }
        while l < r && arr[l] <= arr[pivot] { l += 1; }
        if l < r { arr.swap(l, r); }
    }
    arr.swap(pivot, l);
    l
}

fn quick_sort<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let pivot = partition(arr, lo, hi);
        if pivot != 0 {
            quick_sort(arr, lo, pivot - 1);
        }
        quick_sort(arr, pivot + 1, hi);
    }
}

fn sort<T: PartialOrd>(array: &mut [T]){
	if array.len() > 1 {
        quick_sort(array, 0, array.len() - 1);
    }
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