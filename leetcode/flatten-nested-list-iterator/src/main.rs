#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    values: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    pub fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut values = Vec::new();

        let nested = NestedInteger::List(nestedList);

        Self::populate_values(&nested, &mut values);

        values.reverse();
        NestedIterator { values }
    }

    fn populate_values(nested: &NestedInteger, values: &mut Vec<i32>) {
        match nested {
            NestedInteger::Int(x) => {
                values.push(*x);
            }
            NestedInteger::List(x) => {
                for item in x {
                    Self::populate_values(item, values);
                }
            }
        }
    }

    pub fn next(&mut self) -> i32 {
        self.values.pop().unwrap()
    }

    pub fn has_next(&self) -> bool {
        !self.values.is_empty()
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
            if answer.len() > 10 {
                break;
            }
        }
        assert_eq!(vec![1, 1, 2, 1, 1], answer);
    }

    #[test]
    fn test2() {
        let mut list = NestedIterator::new(vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![NestedInteger::Int(6)]),
            ]),
        ]);

        let mut answer = Vec::new();
        while list.has_next() {
            answer.push(list.next());
            if answer.len() > 10 {
                break;
            }
        }

        assert_eq!(vec![1, 4, 6], answer);
    }
}
