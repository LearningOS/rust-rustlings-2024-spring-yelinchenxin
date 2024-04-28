/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T: Clone + std::cmp::PartialOrd>(array: &mut [T]){
	//TODO
    let mut vec_len = array.len();
    let mut x = 0;
    
    while x < vec_len - 1 {
        let mut y = 0;
        while y < vec_len - x -1 {
            if array[y] > array[y+1] {
                let mut temp_num = array[y].clone();
                array[y] = array[y+1].clone();
                array[y+1] = temp_num;
            }
            y += 1;
        }
        x += 1;
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