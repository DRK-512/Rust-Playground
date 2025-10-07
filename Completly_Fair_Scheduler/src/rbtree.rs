// A compact Left-Leaning Red-Black Tree (LLRB) implementation in Rust.
// LLRB is a simplified, equivalent form of red-black trees popularized by Sedgewick.
// It keeps red links leaning left and avoids 4-nodes via color flips.
//
// Features:
// - Generic over any Ord + Clone key type.
// - insert, contains, len, is_empty, clear
// - in-order traversal via IntoIterator
// - validate() to assert red-black invariants (debug usage)
//
// Note: This is an educational example focusing on clarity over micro-optimizations.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Color { Red, Black }

use Color::*;

#[derive(Debug)]
struct Node<T: Ord + Clone> {
    key: T,
    left: Link<T>,
    right: Link<T>,
    color: Color, // color of the link from parent to this node (root is always Black)
    size: usize,  // subtree size for convenience
}

type Link<T> = Option<Box<Node<T>>>;

impl<T: Ord + Clone> Node<T> {
    fn new_red(key: T) -> Box<Self> {
        Box::new(Node { key, left: None, right: None, color: Red, size: 1 })
    }

    fn is_red(n: &Link<T>) -> bool {
        matches!(n.as_deref().map(|x| x.color), Some(Red))
    }

    fn size(n: &Link<T>) -> usize {
        n.as_deref().map(|x| x.size).unwrap_or(0)
    }

    fn rotate_left(mut h: Box<Node<T>>) -> Box<Node<T>> {
        // h must have a red right link
        let mut x = h.right.take().expect("rotate_left on None right");
        h.right = x.left.take();
        x.left = Some(h);
        x.color = x.left.as_deref().unwrap().color; // inherit h's color
        x.left.as_mut().unwrap().color = Red;
        // fix sizes
        x.left.as_mut().unwrap().recalc();
        x.recalc();
        x
    }

    fn rotate_right(mut h: Box<Node<T>>) -> Box<Node<T>> {
        // h must have a red left link
        let mut x = h.left.take().expect("rotate_right on None left");
        h.left = x.right.take();
        x.right = Some(h);
        x.color = x.right.as_deref().unwrap().color; // inherit h's color
        x.right.as_mut().unwrap().color = Red;
        // fix sizes
        x.right.as_mut().unwrap().recalc();
        x.recalc();
        x
    }

    fn flip_colors(h: &mut Box<Node<T>>) {
        // h is black with two red children -> split 4-node
        h.color = match h.color { Red => Black, Black => Red };
        if let Some(l) = h.left.as_mut() { l.color = match l.color { Red => Black, Black => Red }; }
        if let Some(r) = h.right.as_mut() { r.color = match r.color { Red => Black, Black => Red }; }
    }

    fn recalc(&mut self) {
        self.size = 1 + Node::<T>::size(&self.left) + Node::<T>::size(&self.right);
    }
}

pub struct RBTree<T: Ord + Clone> { root: Link<T> }

impl<T: Ord + Clone> Default for RBTree<T> { fn default() -> Self { Self { root: None } } }

impl<T: Ord + Clone> RBTree<T> {
    pub fn new() -> Self { Self::default() }

    pub fn is_empty(&self) -> bool { self.root.is_none() }

    //pub fn len(&self) -> usize { Node::<T>::size(&self.root) }

    //pub fn clear(&mut self) { self.root = None; }

    pub fn contains(&self, key: &T) -> bool {
        let mut cur = self.root.as_deref();
        while let Some(n) = cur {
            if *key < n.key { cur = n.left.as_deref(); }
            else if *key > n.key { cur = n.right.as_deref(); }
            else { return true; }
        }
        false
    }

    pub fn insert(&mut self, key: T) {
        self.root = Self::insert_rec(self.root.take(), key);
        if let Some(root) = self.root.as_mut() { root.color = Black; }
    }

    fn insert_rec(h: Link<T>, key: T) -> Link<T> {
        let mut h = match h {
            None => return Some(Node::new_red(key)),
            Some(node) => node,
        };

        if key < h.key {
            h.left = Self::insert_rec(h.left.take(), key);
        } else if key > h.key {
            h.right = Self::insert_rec(h.right.take(), key);
        } else {
            // duplicate: overwrite or ignore. We'll ignore by default.
        }

        // Fix right-leaning red links
        if Node::is_red(&h.right) && !Node::is_red(&h.left) {
            h = Node::rotate_left(h);
        }
        // Fix two reds in a row on the left
        if Node::is_red(&h.left) && Node::is_red(&h.left.as_ref().unwrap().left) {
            h = Node::rotate_right(h);
        }
        // Split 4-nodes
        if Node::is_red(&h.left) && Node::is_red(&h.right) {
            Node::flip_colors(&mut h);
        }

        h.recalc();
        Some(h)
    }

    /// Returns a Vec of keys in-order (ascending). Useful for debugging.
    pub fn inorder(&self) -> Vec<T> {
        fn go<T: Ord + Clone>(n: &Link<T>, out: &mut Vec<T>) {
            if let Some(node) = n {
                go(&node.left, out);
                out.push(node.key.clone());
                go(&node.right, out);
            }
        }
        let mut v = Vec::new();
        go(&self.root, &mut v);
        v
    }

    // Verify basic LLRB invariants. Panics on violation (debug helper).
    //pub fn validate(&self) {
    //    if Node::<T>::is_red(&self.root) { panic!("root must be black"); }
    //    let black_height = self.black_height(self.root.as_deref());
    //    self.check(self.root.as_deref(), false, black_height);
    //}

    //fn black_height(&self, mut n: Option<&Node<T>>) -> usize {
    //    let mut h = 0;
    //    while let Some(node) = n {
    //        if node.color == Black { h += 1; }
    //        n = node.left.as_deref();
    //    }
    //    h
    //}

    //fn check(&self, n: Option<&Node<T>>, parent_red: bool, black_goal: usize) -> usize {
    //    if n.is_none() { return 0; }
    //    let node = n.unwrap();
    //    if parent_red && node.color == Red {
    //        panic!("red node has red parent");
    //    }
    //    if Node::<T>::is_red(&node.right) && !Node::<T>::is_red(&node.left) {
    //        panic!("right-leaning red link detected");
    //    }
    //    let left_b = self.check(node.left.as_deref(), node.color == Red, black_goal);
    //    let right_b = self.check(node.right.as_deref(), node.color == Red, black_goal);
    //    if left_b != right_b { panic!("black-height mismatch"); }
    //    let add = if node.color == Black { 1 } else { 0 };
    //    if left_b + add > black_goal { panic!("black-height exceeds goal"); }
    //    left_b + add
    //}
}

// Simple iterator for in-order traversal.
impl<T: Ord + Clone> IntoIterator for RBTree<T> {
    type Item = T;
    type IntoIter = RBIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        let mut it = RBIter { stack: Vec::new() };
        it.push_left(self.root);
        it
    }
}

pub struct RBIter<T: Ord + Clone> { stack: Vec<Box<Node<T>>> }

impl<T: Ord + Clone> RBTree<T> {
    pub fn remove(&mut self, key: &T) {
        if !self.contains(key) {
            return;
        }
        if !Node::is_red(&self.root.as_ref().unwrap().left)
            && !Node::is_red(&self.root.as_ref().unwrap().right) {
            if let Some(root) = self.root.as_mut() {
                root.color = Red;
            }
        }
        self.root = Self::remove_rec(self.root.take(), key);
        if let Some(root) = self.root.as_mut() {
            root.color = Black;
        }
    }

    fn move_red_left(mut h: Box<Node<T>>) -> Box<Node<T>> {
        Node::flip_colors(&mut h);
        if Node::is_red(&h.right.as_ref().unwrap().left) {
            h.right = h.right.take().map(|r| Node::rotate_right(r));
            h = Node::rotate_left(h);
            Node::flip_colors(&mut h);
        }
        h
    }

    fn move_red_right(mut h: Box<Node<T>>) -> Box<Node<T>> {
        Node::flip_colors(&mut h);
        if Node::is_red(&h.left.as_ref().unwrap().left) {
            h = Node::rotate_right(h);
            Node::flip_colors(&mut h);
        }
        h
    }

    fn min_node(h: &Box<Node<T>>) -> T {
        let mut cur = h;
        while let Some(left) = cur.left.as_ref() {
            cur = left;
        }
        cur.key.clone()
    }

    fn remove_min(mut h: Box<Node<T>>) -> Link<T> {
        if h.left.is_none() {
            return None;
        }
        if !Node::is_red(&h.left) && !Node::is_red(&h.left.as_ref().unwrap().left) {
            h = Self::move_red_left(h);
        }
        h.left = Self::remove_min(h.left.take().unwrap());
        Some(Self::fix_up(h))
    }

    fn remove_rec(h: Link<T>, key: &T) -> Link<T> {
        let mut h = h.unwrap();

        if *key < h.key {
            if h.left.is_some() {
                if !Node::is_red(&h.left) && !Node::is_red(&h.left.as_ref().unwrap().left) {
                    h = Self::move_red_left(h);
                }
                h.left = Self::remove_rec(h.left.take(), key);
            }
        } else {
            if Node::is_red(&h.left) {
                h = Node::rotate_right(h);
            }
            if *key == h.key && h.right.is_none() {
                return None;
            }
            if h.right.is_some() {
                if !Node::is_red(&h.right) && !Node::is_red(&h.right.as_ref().unwrap().left) {
                    h = Self::move_red_right(h);
                }
                if *key == h.key {
                    let min = Self::min_node(h.right.as_ref().unwrap());
                    h.key = min.clone();
                    h.right = Self::remove_min(h.right.take().unwrap());
                } else {
                    h.right = Self::remove_rec(h.right.take(), key);
                }
            }
        }
        Some(Self::fix_up(h))
    }

    fn fix_up(mut h: Box<Node<T>>) -> Box<Node<T>> {
        if Node::is_red(&h.right) {
            h = Node::rotate_left(h);
        }
        if Node::is_red(&h.left) && Node::is_red(&h.left.as_ref().unwrap().left) {
            h = Node::rotate_right(h);
        }
        if Node::is_red(&h.left) && Node::is_red(&h.right) {
            Node::flip_colors(&mut h);
        }
        h.recalc();
        h
    }
}

impl<T: Ord + Clone> RBIter<T> {
    fn push_left(&mut self, n: Link<T>) {
        let mut cur = n;
        while let Some(mut node) = cur {
            let left = node.left.take();
            self.stack.push(node);
            cur = left;
        }
    }
}

impl<T: Ord + Clone> Iterator for RBIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.stack.pop()?;
        let key = node.key.clone();
        let right = node.right.take();
        self.push_left(right);
        Some(key)
    }
}

// --- Demo ---
/*
fn main() {
    let mut tree = RBTree::new();
    let items = vec![
        10, 20, 30, 15, 5, 25, 1, 50, 60, 55, 2, 3, 4, 6, 7, 8, 9
    ];
    for x in &items { tree.insert(*x); }

    println!("len: {}", tree.len());
    println!("contains 25? {}", tree.contains(&25));
    println!("contains 99? {}", tree.contains(&99));

    println!("inorder: {:?}", tree.inorder());

    // Validate invariants (will panic if broken)
    tree.validate();

    // Iterate using IntoIterator
    let sorted: Vec<_> = tree.into_iter().collect();
    println!("sorted via iterator: {:?}", sorted);
}
*/
