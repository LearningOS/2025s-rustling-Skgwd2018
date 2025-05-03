/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd + Copy>(array: &mut [T]) {
    let n = array.len();
    merge(array, 0, n);
}
// 归并排序
fn merge<T: PartialOrd + Copy>(nums: &mut [T], start: usize, end: usize) {
    if start + 1 >= end { return; }

    let mid = (start + end) / 2;
    merge(nums, start, mid);
    merge(nums, mid, end);
    sort2(nums, start, mid, end);
}
fn sort2<T: PartialOrd + Copy>(nums: &mut [T], start: usize, mid: usize, end: usize) {
    let nums2 = nums[mid..end].to_vec();
    let (mut index1, mut index2, mut index) = (mid - 1, nums2.len() - 1, end - 1);

    loop {
        if nums[index1] > nums2[index2] {
            nums[index] = nums[index1];
            index -= 1;
            if index1 == start {
                nums[start..=(index2 + start)].copy_from_slice(&nums2[..=index2]);
                break;
            }

            index1 -= 1;
        } else {
            nums[index] = nums2[index2];
            index -= 1;
            if index2 == 0 { break; }

            index2 -= 1;
        }
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
