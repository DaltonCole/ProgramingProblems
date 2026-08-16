#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    nestedList: Vec<NestedInteger>,
    index: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(mut nestedList: Vec<NestedInteger>) -> Self {
        nestedList.reverse();
        NestedIterator {
            nestedList,
            index: 0,
        }
    }

    fn next(&self) -> i32 {
        let first_item = self.nestedList.iter().last().unwrap();

        match first_item {
            NestedInteger::Int(x) => {
                self.nestedList.pop().unwrap();
                *x
            }
            NestedInteger::List(x) => {
                let current_item = x[self.index];
                if x.len() == self.index + 1 {
                    self.nestedList.pop();
                }

                match current_item {
                    NestedInteger::Int(y) => y,
                    NestedInteger::List(_) => panic!("Won't get here"),
                }
            }
        }
    }

    fn has_next(&self) -> bool {
        !self.nestedList.is_empty()
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let mut list = NestedIterator::new(vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ]);
        let mut answer = Vec::new();
        while list.has_next() {
            answer.push(list.next());
        }
        assert_eq!(vec![1, 1, 2, 1, 1], answer);
    }

    #[test]
    fn test2() {
        let mut list = NestedIterator::new(vec![NestedInteger::List(vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![NestedInteger::Int(6)]),
            ]),
        ])]);

        let mut answer = Vec::new();
        while list.has_next() {
            answer.push(list.next());
        }

        assert_eq!(vec![1, 4, 6], answer);
    }
}
