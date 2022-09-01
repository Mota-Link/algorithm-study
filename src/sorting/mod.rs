/// # 冒泡排序
///
/// 每轮比较相邻两位，把大的元素交换到高位。
/// 优化：当一次冒泡没有元素交换行为，那说明已经按顺序了，可以提前退出。
///
/// ```
/// # use algorithm_study::sorting::bubble_sort;
/// #
/// let mut vec_0: Vec<i32> = Vec::new();
/// let mut vec_1 = vec![1];
/// let mut vec_2 = vec![3,5,2,4,1];
///
/// bubble_sort(&mut vec_0);
/// bubble_sort(&mut vec_1);
/// bubble_sort(&mut vec_2);
///
/// assert_eq!(vec_0, Vec::new());
/// assert_eq!(vec_1, vec![1]);
/// assert_eq!(vec_2, vec![1,2,3,4,5]);
/// ```
///
pub fn bubble_sort<T>(vec: &mut Vec<T>)
where
    T: Copy + Ord,
{
    // 冒泡行为标志
    let mut is_bubbled;

    for i in 0..vec.len() {
        is_bubbled = false;

        for j in 0..(vec.len() - i - 1) {
            if vec[j] > vec[j + 1] {
                let temp = vec[j];
                vec[j] = vec[j + 1];
                vec[j + 1] = temp;
                is_bubbled = true;
            }
        }

        // 若没有冒泡行为，说明已排好序，可提前退出
        if !is_bubbled {
            break;
        }
    }
}

/// # 插入排序
///
/// 数组分为两部分：前面是有序部分，初始值只有一个元素；
/// 后面是无序部分，每轮操作无序部分的第一个元素。
///
/// 可以理解为用冒泡的方式，从后往前交换直到找到合适的位置。
/// 但为了减少冒泡交换的开销，一次可以移位多个元素，最后确定位置了再填入。
///
/// ```
/// # use algorithm_study::sorting::insertion_sort;
/// #
/// let mut vec_0: Vec<i32> = Vec::new();
/// let mut vec_1 = vec![1];
/// let mut vec_2 = vec![3,5,2,4,1];
///
/// insertion_sort(&mut vec_0);
/// insertion_sort(&mut vec_1);
/// insertion_sort(&mut vec_2);
///
/// assert_eq!(vec_0, Vec::new());
/// assert_eq!(vec_1, vec![1]);
/// assert_eq!(vec_2, vec![1,2,3,4,5]);
/// ```
///
pub fn insertion_sort<T>(vec: &mut Vec<T>)
where
    T: Copy + Ord,
{
    // 遍历无序部分
    for i in 1..vec.len() {
        // 保存当前值
        let temp = vec[i];
        let mut idx: usize = 0;

        // 从高到低，遍历有序部分，方便移位
        for j in (0..i).rev() {
            // 比当前值大，向右挪一位
            // 比当前值小，就不挪，返回右边的空位
            if vec[j] > temp {
                vec[j + 1] = vec[j];
            } else {
                idx = j + 1;
                break;
            }
        }

        // 当前值填入空位
        vec[idx] = temp;
    }
}

/// # 选择排序
///
/// 每轮找到当前最小元素，交换到前面去
///
/// ```
/// # use algorithm_study::sorting::selection_sort;
/// #
/// let mut vec_0: Vec<i32> = Vec::new();
/// let mut vec_1 = vec![1];
/// let mut vec_2 = vec![3,5,2,4,1];
///
/// selection_sort(&mut vec_0);
/// selection_sort(&mut vec_1);
/// selection_sort(&mut vec_2);
///
/// assert_eq!(vec_0, Vec::new());
/// assert_eq!(vec_1, vec![1]);
/// assert_eq!(vec_2, vec![1,2,3,4,5]);
/// ```
///
pub fn selection_sort<T>(vec: &mut Vec<T>)
where
    T: Copy + Ord,
{
    // 当前最小元素的索引
    let mut idx;

    for i in 0..vec.len() {
        idx = i;

        // 遍历寻找最小元素
        for j in (i + 1)..vec.len() {
            if vec[j] < vec[idx] {
                idx = j;
            }
        }

        // 最小元素，交换到当前最前端
        if idx != i {
            let temp = vec[idx];
            vec[idx] = vec[i];
            vec[i] = temp;
        }
    }
}
