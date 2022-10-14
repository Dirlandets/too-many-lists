type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct ListIntoIter<T>(List<T>);
pub struct ListIter<'a, T> {
    next: Option<&'a Node<T>>,
}
pub struct ListIterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn into_iter(self) -> ListIntoIter<T> {
        ListIntoIter(self)
    }
    pub fn iter<'a>(&'a self) -> ListIter<'a, T> {
        ListIter {
            next: self.head.as_deref(),
        }
    }
    pub fn iter_mut<'a>(&'a mut self) -> ListIterMut<'a, T> {
        ListIterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Iterator for ListIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}
impl<'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr_link = self.head.take();
        while let Some(mut boxed_node) = curr_link {
            curr_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        {
            List::<i32>::new();
        }
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        assert_eq!(list.peek(), Some(&1));
        if let Some(value) = list.peek_mut() {
            *value = 42
        };

        assert_eq!(list.pop(), Some(42));

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        if let Some(value) = list.peek_mut() {
            *value = 42
        };
        assert_eq!(list.pop(), Some(42));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        iter.for_each(|val| println!("{}", val));
    }
    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        iter.for_each(|val| println!("{}", val));
    }
}
