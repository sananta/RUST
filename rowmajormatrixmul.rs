#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

use std::{ops, fmt};
use std::ops::Add;

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
        let mut stack = Vec::new();
        for i in values {
            match *i {
                isize => stack.push(isize),
            }
        }
        Matrix {
           data: stack,
            row: row,
            col: col,
        }
    }
    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
        let mut stack = Vec::new();
        Matrix {
            data: stack,
            row: row,
            col: col,
        }
    }
    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
        let ref y = self.data;
        let x = &y;
        x
    }
    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
        let y = &mut self.data;
        y
    }
    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
        let (x, y) = (self.row, self.col);
        (x, y)
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {
            panic!();
        }
        let stack = self.data.to_vec();
        let stack2 = rhs.data.to_vec();
        let mut stack3 = Vec::new();
        for i in 0..stack.len() { 
            stack3.push(stack[i] + stack2[i]); 
        }
        Matrix {
            data: stack3,
            row: self.row,
            col: self.col,
        }
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {
            panic!();
        }
        let stack = self.data.to_vec();
        let stack2 = rhs.data.to_vec();
        let mut stack3 = Vec::new();
        for i in 0..stack.len() { 
            stack3.push(stack[i] - stack2[i]); 
        }
        Matrix {
            data: stack3,
            row: self.row,
            col: self.col,
        }
    }
}


impl<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul for Matrix<T> where T: std::fmt::Debug {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        if self.col != rhs.row { 
            panic!(); 
        }
        let mut stack = self.data.to_vec();
        let stack2 = rhs.data.to_vec();
        let x = self.row * rhs.col; 
        let mut stack3 = self.data.to_vec();
        let mut stack4 = self.data.to_vec(); 
        if stack3.len() < x { 
            let z = x - stack3.len(); 
            for i in 0..z { 
                stack3.push(stack[i]); 
            }
            //println!("{}", stack3.len()); 
        }
        if stack3.len() > x { 
            let z = stack3.len() - x; 
            for i in 0..z {
                stack3.pop(); 
            }
        }
        for i in 0..x { 
            stack3[i] = stack[1] - stack[1]; 
        }
        if stack4.len() < x { 
            let z = x - stack4.len(); 
            for i in 0..z { 
                stack4.push(stack[i]); 
            }
            //println!("{}", stack3.len()); 
        }
        if stack4.len() > x { 
            let z = stack4.len() - x; 
            for i in 0..z {
                stack4.pop(); 
            }
        }
        for i in 0..x { 
            stack4[i] = stack[1] - stack[1]; 
        }
        let mut e = 0; 
        for i in 0..self.row { 
            for j in 0..rhs.col { 
                stack3[i*self.row + j] = stack[1] - stack[1];
                for k in 0..self.row {
                    stack3[i*self.row + j] = stack3[i*self.row + j] + stack[i*self.row + k] * stack2[j + k*rhs.col]; 
                    if k == self.row-1 { 
                        stack4[e] = stack3[i*self.row + j]; 
                        e+=1; 
                    }
                }
            }
        } 
        Matrix { 
            data: stack4, 
            row: self.row, 
            col: rhs.col, 
        }    
    }    
}


impl<T: fmt::Display> fmt::Display for Matrix<T> where T: std::clone::Clone {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line. No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space, except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
        let mut s = String::new(); 
        let mut k = 0; 
        for i in 0..self.row { 
            for j in 0..self.col {
                
                s.push_str(&format!("{}", self.data[k]));
                if !(j == self.col-1) { 
                    s.push(' ');
                }
                k += 1; 
            }
            s.push('\n');
        }
        write!(f, "{}", s)           
    }
}
