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

        // 模拟窗口迭代
        for (x, y) in (1..(vec.len() - i)).enumerate() {
            if vec[x] > vec[y] {
                // let temp = vec[x];
                // vec[x] = vec[y];
                // vec[y] = temp;
                vec.swap(x, y);

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
            // let temp = vec[idx];
            // vec[idx] = vec[i];
            // vec[i] = temp;
            vec.swap(idx, i);
        }
    }
}

/// # 归并排序
///
/// 用递归的方式，将大的数组拆成两个子数组，然后对子数组进行排序合并。
///
/// 对子数组递归调用归并排序，在后续合并时，子数组已经是有序的。
/// 只需要依序比较子数组的头部元素，将较小的填入新建的数组即可。
///
/// ```
/// # use algorithm_study::sorting::merge_sort;
/// #
/// let mut vec_0: Vec<i32> = Vec::new();
/// let mut vec_1 = vec![1];
/// let mut vec_2 = vec![2,1];
/// let mut vec_3 = vec![3,5,2,4,1];
///
/// assert_eq!(merge_sort(vec_0), Vec::new());
/// assert_eq!(merge_sort(vec_1), vec![1]);
/// assert_eq!(merge_sort(vec_2), vec![1,2]);
/// assert_eq!(merge_sort(vec_3), vec![1,2,3,4,5]);
/// ```
///
pub fn merge_sort<T>(mut vec: Vec<T>) -> Vec<T>
where
    T: Copy + Ord,
{
    let len = vec.len();

    // 递归终止条件
    if len <= 1 {
        return vec;
    }

    // 数组分成两段，向下递归
    // 将返回的数组转换成可以Peek的迭代器，方便合并
    let mut second_half = merge_sort(vec.split_off(vec.len() / 2))
        .into_iter()
        .peekable();
    let mut first_half = merge_sort(vec).into_iter().peekable();

    let mut sorted: Vec<T> = Vec::with_capacity(len);

    // 合并数组
    loop {
        match (first_half.peek(), second_half.peek()) {
            (Some(&x), Some(&y)) if x > y => {
                sorted.push(y);
                second_half.next();
            }
            (Some(&x), Some(_)) => {
                sorted.push(x);
                first_half.next();
            }
            (None, _) => {
                for i in second_half {
                    sorted.push(i);
                }
                return sorted;
            }
            (_, None) => {
                for i in first_half {
                    sorted.push(i);
                }
                return sorted;
            }
        }
    }
}

/// # 快速排序
///
/// 选择一个元素作为分区点，将小于它的元素移动到它左侧，大于的移到右侧
/// 然后对左侧&右侧的子数组递归调用快速排序
///
/// ```
/// # use algorithm_study::sorting::quick_sort;
/// #
/// let mut vec_0: Vec<i32> = Vec::new();
/// let mut vec_1 = vec![1];
/// let mut vec_2 = vec![2,1];
/// let mut vec_3 = vec![3,5,2,4,1];
///
/// quick_sort(&mut vec_0);
/// quick_sort(&mut vec_1);
/// quick_sort(&mut vec_2);
/// quick_sort(&mut vec_3);
///
/// assert_eq!(vec_0, Vec::new());
/// assert_eq!(vec_1, vec![1]);
/// assert_eq!(vec_2, vec![1,2]);
/// assert_eq!(vec_3, vec![1,2,3,4,5]);
/// ```
///
pub fn quick_sort<T>(vec: &mut Vec<T>)
where
    T: Copy + Ord,
{
    quick_sort_with_idx(vec, 0, vec.len());

    fn quick_sort_with_idx<T>(vec: &mut Vec<T>, start: usize, end: usize)
    where
        T: Copy + Ord,
    {
        if start + 1 >= end {
            return;
        }

        let mut lm = start + 1;
        let mut rm = end - 1;

        loop {
            // 找到大于pivot的左侧元素
            // 先比较索引，短路计算，避免溢出
            while lm <= rm && vec[lm] <= vec[start] {
                lm += 1;
            }

            // 找到小于pivot的右侧元素
            while lm <= rm && vec[rm] >= vec[start] {
                rm -= 1;
            }

            // 碰头了，说明没找到
            if lm > rm {
                break;
            }

            // 交换
            vec.swap(lm, rm);
        }

        // 交换
        vec.swap(start, lm - 1);

        // 分区递归
        quick_sort_with_idx(vec, start, lm);
        quick_sort_with_idx(vec, lm + 1, end);
    }
}
