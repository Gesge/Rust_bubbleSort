// 基础：固定类型
fn bubble_sort(vec: &mut Vec<i32>) {
    for i in 0..vec.len() {
        for j in 0..vec.len() - 1 - i {
            // 冒泡排序
            if vec[j] > vec[j + 1] {
                // 比较交换
                vec[j] = vec[j] ^ vec[j + 1];
                vec[j + 1] = vec[j] ^ vec[j + 1];
                vec[j] = vec[j] ^ vec[j + 1];
            }
        }
    }
}

// 提高：添加泛型
fn bubble_sort_template<T: PartialOrd + Copy>(list: &mut Vec<T>) -> &Vec<T> {
    for i in 0..list.len() {
        for x in 0..list.len() - 1 {
            // 元素换位
            if list[x] > list[x + 1] {
                list.swap(x, x + 1);
            }
        }
    }
    list
}


fn main() {
    // 基础部分：固定类型（i32）数组排序
    let mut list1 = vec![1, 8, 6, 5, 0, 9, 7, 10, 3];
    bubble_sort(&mut list1);
    println!("{:?}  ", list1);
    // 提高部分：使用template和PartialOrd实现对任意类型的排序
    let mut list2 = vec![1, 14, 6, 8, 22, 9, 7, 10, 5];
    bubble_sort_template(&mut list2);
    println!("{:?}  ", list2);

    let mut list3 = vec!['F', 'c', 'B', 'A', 'a', 'Y'];
    bubble_sort_template(&mut list3);
    println!("{:?}  ", list3);
}