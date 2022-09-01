/// # 斐波那契数列
///
/// 以迭代器形式实现的无限数据结构。
///
/// ```
/// use algorithm_study::fibonacci::Fibonacci;
///
/// let mut fib = Fibonacci::new();
///
/// assert_eq!(fib.next(), Some(1));
/// assert_eq!(fib.next(), Some(1));
/// assert_eq!(fib.next(), Some(2));
/// assert_eq!(fib.next(), Some(3));
/// assert_eq!(fib.next(), Some(5));
/// assert_eq!(fib.next(), Some(8));
/// ```
pub struct Fibonacci {
    pre_previous: u128,
    previous: u128,
    less_than_two: bool,
}

impl Fibonacci {
    /// 创建新的斐波那契数列。
    ///
    /// # Examples
    /// ---
    /// ```
    /// # use algorithm_study::fibonacci::Fibonacci;
    /// #
    /// let mut fib = Fibonacci::new();
    /// ```
    pub fn new() -> Fibonacci {
        Fibonacci {
            pre_previous: 0,
            previous: 1,
            less_than_two: true,
        }
    }
}

impl std::iter::Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<u128> {
        if self.less_than_two {
            if self.pre_previous == 0 {
                self.pre_previous = 1;
                return Some(1);
            }
            self.less_than_two = false;
            return Some(1);
        }

        let fib = self.previous.checked_add(self.pre_previous);
        self.pre_previous = self.previous;

        if let Some(i) = fib {
            self.previous = i;
        }

        fib
    }
}
