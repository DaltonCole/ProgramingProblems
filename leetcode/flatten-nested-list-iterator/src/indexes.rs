#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    nestedList: NestedInteger,
    indexes: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut indexes = Vec::new();

        if nestedList.len() > 0 {
            indexes.push(0);
        }

        let mut first_list = &nestedList[0];

        while let NestedInteger::List(item) = first_list {
            indexes.push(0);
            first_list = &item[0];
        }

        NestedIterator {
            nestedList: NestedInteger::List(nestedList),
            indexes,
        }
    }

    fn next(&mut self) -> i32 {
        let mut stack = vec![&self.nestedList];

        println!("Indexes: {:?}", self.indexes);

        for &index in self.indexes.iter() {
            println!("{:?}", stack.iter().last().unwrap());
            match stack.iter().last().unwrap() {
                NestedInteger::Int(_) => panic!("?"),
                NestedInteger::List(x) => {
                    stack.push(&x[index]);
                }
            }
        }
        println!("Stack: {:?}", stack);
        let return_item = stack.pop().unwrap();

        // Go to next item
        while let Some(NestedInteger::List(x)) = stack.iter().last() {
            let last_index = self.indexes.iter_mut().last().unwrap();
            // Can keep going on current one
            if x.len() > *last_index + 1 {
                *last_index += 1;
                break;
            }
            // Start a new one
            else {
                self.indexes.pop();
                stack.pop();
            }
        }

        match return_item {
            NestedInteger::Int(x) => *x,
            NestedInteger::List(_) => panic!("??"),
        }
    }

    fn has_next(&self) -> bool {
        !self.indexes.is_empty()
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
