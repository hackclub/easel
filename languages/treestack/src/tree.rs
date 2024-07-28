use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, Deref, DerefMut, Mul, Sub},
};

#[derive(Clone, Default, Debug)]
pub struct TreeNode<T> {
    pub val: T,
    pub children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    pub fn eval(self, rhs: Self, op: fn(T, T) -> T) -> Self {
        let children =
            self.children.into_iter().zip(rhs.children).map(|(l, r)| l.eval(r, op)).collect();
        Self { val: op(self.val, rhs.val), children }
    }

    pub fn new(val: T) -> Self {
        Self { val, children: Vec::new() }
    }

    pub fn flatten(self) -> Vec<TreeNode<T>> {
        let mut new_vec = Vec::new();
        for child in self.children {
            new_vec.push(TreeNode::new(child.val));
            let mut children = child.children;
            new_vec.append(&mut children);
        }
        new_vec
    }
}

impl<T: Display + Debug> Display for TreeNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output = &self.draw(0);
        write!(f, "{}\x1b[0m", output)
    }
}

impl<T: Display + Debug> TreeNode<T> {
    fn draw(&self, depth: usize) -> String {
        if self.children.is_empty() {
            return format!("\x1b[0m{}", self.val);
        }

        let code = format!("\x1b[38;5;{}m", depth + 1);
        let above = format!("\x1b[38;5;{}m", depth);
        let children =
            self.children.iter().map(|n| n.draw(depth + 1)).collect::<Vec<String>>().join(", ");

        let children = format!("{code}[{}]{above}", children);

        // if depth == 0 { return children };

        format!("{}{}", self.val, children)
    }
}

impl<T: Mul<Output = T>> Mul for TreeNode<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.eval(rhs, Mul::mul)
    }
}

impl<T: Add<Output = T>> Add for TreeNode<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.eval(rhs, Add::add)
    }
}

impl<T: Sub<Output = T>> Sub for TreeNode<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.eval(rhs, Sub::sub)
    }
}

impl<T> Deref for TreeNode<T> {
    type Target = Vec<TreeNode<T>>;

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<T> DerefMut for TreeNode<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}
