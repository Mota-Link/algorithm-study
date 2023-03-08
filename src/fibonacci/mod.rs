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
/// 
pub struct Fibonacci {
    pre_previous: u128,
    previous: u128,
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
    /// 
    pub fn new() -> Fibonacci {
        Fibonacci {
            pre_previous: 0,
            previous: 0,
        }
    }
}

impl std::iter::Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<u128> {
        match (self.pre_previous, self.previous) {
            (0, 0) => {
                self.previous = 1;
                Some(1)
            }
            (0, 1) => {
                self.pre_previous = 1;
                Some(1)
            }
            _ => {
                let fib = self.previous.checked_add(self.pre_previous);
                self.pre_previous = self.previous;

                if let Some(i) = fib {
                    self.previous = i;
                }

                fib
            }
        }
    }
}

