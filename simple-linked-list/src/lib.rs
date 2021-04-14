use std::iter::FromIterator;

/// Useful resources
/// https://rust-unofficial.github.io/too-many-lists/index.html
/// https://doc.rust-lang.org/book/ch15-01-box.html#using-a-boxt-to-store-data-on-the-heap

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        fn get_len<T>(node: &Box<Node<T>>, len: usize) -> usize {
            return match &node.next {
                None => len,
                Some(n) => get_len(&n, len + 1)
            };
        }

        return match &self.head {
            None => 0,
            Some(h) => get_len(h, 1)
        };
    }

    /// or using mem_replace https://rust-unofficial.github.io/too-many-lists/first-push.html
    pub fn push(&mut self, _element: T) {
        let new_head = Node { data: _element, next: self.head.take() };
        self.head = Some(Box::new(new_head))
    }

    /// or using mem_replace https://rust-unofficial.github.io/too-many-lists/first-pop.html
    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(h) => {
                self.head = h.next;
                Some(h.data)
            }
            _ => None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        match self.head {
            None => self,
            Some(_) => {
                let mut reversed: SimpleLinkedList<T> = SimpleLinkedList::new();
                let mut current = self.head;
                while let Some(node) = current {
                    reversed.push(node.data);
                    current = node.next
                }
                reversed
            }
        }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item=T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for element in _iter {
            list.push(element);
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = self.rev().head;
        while let Some(node) = current {
            result.push(node.data);
            current = node.next;
        }
        result
    }
}
