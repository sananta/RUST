#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`; 
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    let mut stack = Vec::new(); 
    let mut postfix = Vec::new();
    let length = tokens.len(); 
    if tokens[0] == InfixToken::Operator(Operator::Add) || tokens[0] == InfixToken::Operator(Operator::Sub) || tokens[0] == InfixToken::Operator(Operator::Mul) || tokens[0] == InfixToken::Operator(Operator::Div) || tokens[0] == InfixToken::RightParen { 
        return None; 
    }
    for i in 0..length { 
       /*  if tokens[i] == InfixToken::Operator(Operator::Add) || tokens[i] == InfixToken::Operator(Operator::Sub) || tokens[i] == InfixToken::Operator(Operator::Mul) || tokens[i] == InfixToken::Operator(Operator::Div) || tokens[i] == InfixToken::LeftParen { 
            if i > 0 { 
                if tokens[i - 1] == InfixToken::Operator(Operator::Add) || tokens[i - 1] == InfixToken::Operator(Operator::Sub) || tokens[i - 1] == InfixToken::Operator(Operator::Mul) || tokens[i - 1] == InfixToken::Operator(Operator::Div) || tokens[i - 1] == InfixToken::RightParen { 
                    return None; 
                }
            }
        }
*/
        if tokens[i] == InfixToken::Operator(Operator::Add) || tokens[i] == InfixToken::Operator(Operator::Sub) || tokens[i] == InfixToken::Operator(Operator::Mul) || tokens[i] == InfixToken::Operator(Operator::Div) || tokens[i] == InfixToken::RightParen {
            if i > 0 { 
                if tokens[i - 1] == InfixToken::Operator(Operator::Add) || tokens[i - 1] == InfixToken::Operator(Operator::Sub) || tokens[i - 1] == InfixToken::Operator(Operator::Mul) || tokens[i - 1] == InfixToken::Operator(Operator::Div) || tokens[i - 1] == InfixToken::LeftParen {
                    return None; 
                }
            }
        }
    }
    if tokens[length - 1] == InfixToken::Operator(Operator::Add) || tokens[length - 1] == InfixToken::Operator(Operator::Sub) || tokens[length - 1] == InfixToken::Operator(Operator::Mul) || tokens[length - 1] == InfixToken::Operator(Operator::Div) || tokens[length - 1] == InfixToken::LeftParen { 
        return None; 
    }
    let mut num = 0;
    for i in 0..length {
        if tokens[i] == InfixToken::LeftParen { 
            num = num + 1; 
        }
        if tokens[i] == InfixToken::RightParen {
            num = num + 1; 
        }
    }
    if num != 0 && num % 2 != 0 { 
        return None; 
    } 
    for i in tokens { 
        let mut z = 0; 
        match *i { 
            InfixToken::Operand(isize) => postfix.push(PostfixToken::Operand(isize)),
            InfixToken::Operator(Operator::Add) => z = 1, 
            InfixToken::Operator(Operator::Sub) => z = 2,
            InfixToken::Operator(Operator::Mul) => z = 3,
            InfixToken::Operator(Operator::Div) => z = 4,
            InfixToken::LeftParen => stack.push(InfixToken::LeftParen),
            InfixToken::RightParen => z = 5,  
        }
        if z == 1 { 
            if stack.len() == 0 { 
                stack.push(InfixToken::Operator(Operator::Add)); 
            } 
            else { 
                while stack.len() != 0 && stack[stack.len() - 1] != InfixToken::LeftParen {
                    let x = stack.pop().unwrap(); 
                    match x { 
                        InfixToken::Operand(isize) => {},
                        InfixToken::Operator(Operator::Add) => postfix.push(PostfixToken::Operator(Operator::Add)), 
                        InfixToken::Operator(Operator::Sub) => postfix.push(PostfixToken::Operator(Operator::Sub)),
                        InfixToken::Operator(Operator::Mul) => postfix.push(PostfixToken::Operator(Operator::Mul)),
                        InfixToken::Operator(Operator::Div) => postfix.push(PostfixToken::Operator(Operator::Div)),
                        InfixToken::LeftParen => {}, 
                        InfixToken::RightParen => {},  
                    }
                }
                stack.push(InfixToken::Operator(Operator::Add));
            }
        }
        if z == 2 { 
            if stack.len() == 0 { 
                stack.push(InfixToken::Operator(Operator::Sub)); 
            } 
            else { 
                while stack.len() != 0 && stack[stack.len() - 1] != InfixToken::LeftParen {
                    let x = stack.pop().unwrap(); 
                    match x { 
                        InfixToken::Operand(isize) => {},
                        InfixToken::Operator(Operator::Add) => postfix.push(PostfixToken::Operator(Operator::Add)), 
                        InfixToken::Operator(Operator::Sub) => postfix.push(PostfixToken::Operator(Operator::Sub)),
                        InfixToken::Operator(Operator::Mul) => postfix.push(PostfixToken::Operator(Operator::Mul)),
                        InfixToken::Operator(Operator::Div) => postfix.push(PostfixToken::Operator(Operator::Div)),
                        InfixToken::LeftParen => {}, 
                        InfixToken::RightParen => {},  
                    }
                }
                stack.push(InfixToken::Operator(Operator::Sub));
            }
        }
        if z == 3 { 
            if stack.len() == 0 { 
                stack.push(InfixToken::Operator(Operator::Mul)); 
            } 
            else { 
                let mut q = 0; 
                while stack.len() != 0 && stack[stack.len() - 1] != InfixToken::LeftParen && q != 2{
                    let x = stack.pop().unwrap(); 
                    match x { 
                        InfixToken::Operand(isize) => {},
                        InfixToken::Operator(Operator::Add) => {stack.push(x); q = 2}, 
                        InfixToken::Operator(Operator::Sub) => {stack.push(x); q = 2},
                        InfixToken::Operator(Operator::Mul) => postfix.push(PostfixToken::Operator(Operator::Mul)),
                        InfixToken::Operator(Operator::Div) => postfix.push(PostfixToken::Operator(Operator::Div)),
                        InfixToken::LeftParen => {}, 
                        InfixToken::RightParen => {},  
                    }
                }
                stack.push(InfixToken::Operator(Operator::Mul));
            }
        }
        if z == 4 { 
            if stack.len() == 0 { 
                stack.push(InfixToken::Operator(Operator::Div)); 
            } 
            else { 
                let mut q = 0; 
                while stack.len() != 0 && stack[stack.len() - 1] != InfixToken::LeftParen && q != 2{
                    let x = stack.pop().unwrap(); 
                    match x { 
                        InfixToken::Operand(isize) => {},
                        InfixToken::Operator(Operator::Add) => {stack.push(x); q = 2}, 
                        InfixToken::Operator(Operator::Sub) => {stack.push(x); q = 2},
                        InfixToken::Operator(Operator::Mul) => postfix.push(PostfixToken::Operator(Operator::Mul)),
                        InfixToken::Operator(Operator::Div) => postfix.push(PostfixToken::Operator(Operator::Div)),
                        InfixToken::LeftParen => {}, 
                        InfixToken::RightParen => {},  
                    }
                }
                stack.push(InfixToken::Operator(Operator::Div));
            }
        }
        if z == 5 {
            while stack.len() != 0 && stack[stack.len() - 1] != InfixToken::LeftParen { 
                let x = stack.pop().unwrap(); 
                match x { 
                    InfixToken::Operand(isize) => {},
                    InfixToken::Operator(Operator::Add) => postfix.push(PostfixToken::Operator(Operator::Add)), 
                    InfixToken::Operator(Operator::Sub) => postfix.push(PostfixToken::Operator(Operator::Sub)),
                    InfixToken::Operator(Operator::Mul) => postfix.push(PostfixToken::Operator(Operator::Mul)),
                    InfixToken::Operator(Operator::Div) => postfix.push(PostfixToken::Operator(Operator::Div)),
                    InfixToken::LeftParen => {}, 
                    InfixToken::RightParen => {},  
                }
            }
            stack.pop(); 
        }
    }
    while stack.len() != 0 { 
        let x = stack.pop().unwrap(); 
        match x { 
            InfixToken::Operand(isize) => {},
            InfixToken::Operator(Operator::Add) => postfix.push(PostfixToken::Operator(Operator::Add)), 
            InfixToken::Operator(Operator::Sub) => postfix.push(PostfixToken::Operator(Operator::Sub)),
            InfixToken::Operator(Operator::Mul) => postfix.push(PostfixToken::Operator(Operator::Mul)),
            InfixToken::Operator(Operator::Div) => postfix.push(PostfixToken::Operator(Operator::Div)),
            InfixToken::LeftParen => {}, 
            InfixToken::RightParen => {},  
        }
    }
    //println!("{:?}", postfix);
    Some(postfix)
}

/*
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`; 
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    let mut stack = Vec::new(); 
    let mut postfix = Vec::new(); 
    for i in tokens {
        match *i { 
            InfixToken::Operand(isize) => postfix.push(PostfixToken::Operand(isize)),
            InfixToken::Operator(Operator::Add) => stack.push(InfixToken::Operator(Operator::Add)), 
            InfixToken::Operator(Operator::Sub) => stack.push(InfixToken::Operator(Operator::Sub)),
            InfixToken::Operator(Operator::Mul) => stack.push(InfixToken::Operator(Operator::Mul)),
            InfixToken::Operator(Operator::Div) => stack.push(InfixToken::Operator(Operator::Div)),
            InfixToken::LeftParen => stack.push(InfixToken::LeftParen),
            InfixToken::RightParen => stack.push(InfixToken::RightParen),
        }
        let mut len = stack.len(); 
        if len >= 2 {
            if stack[len-1] == InfixToken::Operator(Operator::Sub) && stack[len-2] == InfixToken::Operator(Operator::Mul) { 
                let x = stack.pop().unwrap(); 
                stack.pop(); 
                stack.push(x); 
                postfix.push(PostfixToken::Operator(Operator::Mul)); 
                if stack.len() >= 2 {
                if stack[len-1] == InfixToken::Operator(Operator::Sub) && stack[len-2] == InfixToken::Operator(Operator::Add) { 
                    let x = stack.pop().unwrap(); 
                    stack.pop(); 
                    stack.push(x); 
                    postfix.push(PostfixToken::Operator(Operator::Add)); 
                }
                else if stack[len-1] == InfixToken::Operator(Operator::Sub) && stack[len-2] == InfixToken::Operator(Operator::Sub) { 
                    let x = stack.pop().unwrap(); 
                    stack.pop(); 
                    stack.push(x); 
                    postfix.push(PostfixToken::Operator(Operator::Sub)); 
                }
                }
            } 
            else if stack[len-1] == InfixToken::Operator(Operator::Sub) && stack[len-2] == InfixToken::Operator(Operator::Div) { 
                let x = stack.pop().unwrap(); 
                stack.pop(); 
                stack.push(x); 
                postfix.push(PostfixToken::Operator(Operator::Div));
                if stack.len() >= 2 {
                if stack[len-1] == InfixToken::Operator(Operator::Sub) && stack[len-2] == InfixToken::Operator(Operator::Add) { 
                    let x = stack.pop().unwrap(); 
                    stack.pop(); 
                    stack.push(x); 
                    postfix.push(PostfixToken::Operator(Operator::Add)); 
                }
                else if stack[len-1] == InfixToken::Operator(Operator::Sub) && stack[len-2] == InfixToken::Operator(Operator::Sub) { 
                    let x = stack.pop().unwrap(); 
                    stack.pop(); 
                    stack.push(x); 
                    postfix.push(PostfixToken::Operator(Operator::Sub)); 
                }
                }
            } 
            else if stack[len-1] == InfixToken::Operator(Operator::Add) && stack[len-2] == InfixToken::Operator(Operator::Mul) { 
                let x = stack.pop().unwrap(); 
                stack.pop(); 
                stack.push(x); 
                postfix.push(PostfixToken::Operator(Operator::Mul));
                if stack.len() >= 2 {
                if stack[len-1] == InfixToken::Operator(Operator::Add) && stack[len-2] == InfixToken::Operator(Operator::Sub) { 
                    let x = stack.pop().unwrap(); 
                    stack.pop(); 
                    stack.push(x); 
                    postfix.push(PostfixToken::Operator(Operator::Sub)); 
                }
                else if stack[len-1] == InfixToken::Operator(Operator::Add) && stack[len-2] == InfixToken::Operator(Operator::Add) { 
                    let x = stack.pop().unwrap(); 
                    stack.pop(); 
                    stack.push(x); 
                    postfix.push(PostfixToken::Operator(Operator::Add)); 
                }
                }
            } 
            else if stack[len-1] == InfixToken::Operator(Operator::Add) && stack[len-2] == InfixToken::Operator(Operator::Div) { 
                let x = stack.pop().unwrap(); 
                stack.pop(); 
                stack.push(x); 
                postfix.push(PostfixToken::Operator(Operator::Div));
                if stack.len() >= 2 {
                if stack[len-1] == InfixToken::Operator(Operator::Add) && stack[len-2] == InfixToken::Operator(Operator::Sub) { 
                    let x = stack.pop().unwrap(); 
                    stack.pop(); 
                    stack.push(x); 
                    postfix.push(PostfixToken::Operator(Operator::Sub)); 
                }
                else if stack[len-1] == InfixToken::Operator(Operator::Add) && stack[len-2] == InfixToken::Operator(Operator::Add) { 
                    let x = stack.pop().unwrap(); 
                    stack.pop(); 
                    stack.push(x); 
                    postfix.push(PostfixToken::Operator(Operator::Add)); 
                }
                }
            } 
            else if stack[len-1] == InfixToken::Operator(Operator::Add) && stack[len-2] == InfixToken::Operator(Operator::Add) { 
                let x = stack.pop().unwrap(); 
                stack.pop(); 
                stack.push(x);
                postfix.push(PostfixToken::Operator(Operator::Add)); 
            } 
            else if stack[len-1] == InfixToken::Operator(Operator::Sub) && stack[len-2] == InfixToken::Operator(Operator::Sub) { 
                let x = stack.pop().unwrap(); 
                stack.pop(); 
                stack.push(x);
                postfix.push(PostfixToken::Operator(Operator::Sub)); 
            } 
            else if stack[len-1] == InfixToken::Operator(Operator::Add) && stack[len-2] == InfixToken::Operator(Operator::Sub) { 
                let x = stack.pop().unwrap(); 
                stack.pop(); 
                stack.push(x);
                postfix.push(PostfixToken::Operator(Operator::Sub)); 
            } 
            else if stack[len-1] == InfixToken::Operator(Operator::Sub) && stack[len-2] == InfixToken::Operator(Operator::Add) { 
                let x = stack.pop().unwrap(); 
                stack.pop(); 
                stack.push(x);
                postfix.push(PostfixToken::Operator(Operator::Add)); 
            }  
            else if stack[len-1] == InfixToken::Operator(Operator::Div) && stack[len-2] == InfixToken::Operator(Operator::Mul) { 
                let x = stack.pop().unwrap(); 
                stack.pop(); 
                stack.push(x);
                postfix.push(PostfixToken::Operator(Operator::Mul)); 
            }  
            else if stack[len-1] == InfixToken::Operator(Operator::Mul) && stack[len-2] == InfixToken::Operator(Operator::Div) { 
                let x = stack.pop().unwrap(); 
                stack.pop(); 
                stack.push(x);
                postfix.push(PostfixToken::Operator(Operator::Div)); 
            }  
        }
        if stack[len-1] == InfixToken::RightParen { 
            stack.pop(); 
            len = len - 1; 
            while stack[len-1] != InfixToken::LeftParen { 
                let mut z = 0; 
                match stack[len-1] { 
                    InfixToken::Operand(isize) => z = 0,
                    InfixToken::Operator(Operator::Mul) => postfix.push(PostfixToken::Operator(Operator::Mul)), 
                    InfixToken::Operator(Operator::Div) => postfix.push(PostfixToken::Operator(Operator::Div)), 
                    InfixToken::Operator(Operator::Add) => postfix.push(PostfixToken::Operator(Operator::Add)), 
                    InfixToken::Operator(Operator::Sub) => postfix.push(PostfixToken::Operator(Operator::Sub)),
                    InfixToken::LeftParen => z = 0, 
                    InfixToken::RightParen => z = 0,
                }
                len = len - 1; 
            }
            stack.pop(); 
        }
    }
    //println!("{:?}", postfix); 
    Some(postfix)
}

*/

/*
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`; 
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    let mut stack = Vec::new(); 
    let mut postfix = Vec::new(); 
    for i in tokens {
        match *i { 
            InfixToken::Operand(isize) => postfix.push(PostfixToken::Operand(isize)),
            InfixToken::Operator(Operator::Add) => stack.push(InfixToken::Operator(Operator::Add)), 
            InfixToken::Operator(Operator::Sub) => stack.push(InfixToken::Operator(Operator::Sub)),
            InfixToken::Operator(Operator::Mul) => stack.push(InfixToken::Operator(Operator::Mul)),
            InfixToken::Operator(Operator::Div) => stack.push(InfixToken::Operator(Operator::Div)),
            InfixToken::LeftParen => stack.push(InfixToken::LeftParen),
            InfixToken::RightParen => stack.push(InfixToken::RightParen),
        }
        let mut len = stack.len(); 
        if len >= 2 {
            if stack[len-1] == InfixToken::Operator(Operator::Add) && stack[len-2] == InfixToken::Operator(Operator::Sub) { 
                let x = stack.pop().unwrap(); 
                stack.pop(); 
                stack.push(x);
                postfix.push(PostfixToken::Operator(Operator::Sub)); 
            } 
            else if stack[len-1] == InfixToken::Operator(Operator::Sub) && stack[len-2] == InfixToken::Operator(Operator::Add) { 
                let x = stack.pop().unwrap(); 
                stack.pop(); 
                stack.push(x);
                postfix.push(PostfixToken::Operator(Operator::Add)); 
            }  
            else if stack[len-1] == InfixToken::Operator(Operator::Div) && stack[len-2] == InfixToken::Operator(Operator::Mul) { 
                let x = stack.pop().unwrap(); 
                stack.pop(); 
                stack.push(x);
                postfix.push(PostfixToken::Operator(Operator::Mul)); 
            }  
            else if stack[len-1] == InfixToken::Operator(Operator::Mul) && stack[len-2] == InfixToken::Operator(Operator::Div) { 
                let x = stack.pop().unwrap(); 
                stack.pop(); 
                stack.push(x);
                postfix.push(PostfixToken::Operator(Operator::Div)); 
            }  
        }
        if stack[len-1] == InfixToken::RightParen { 
            stack.pop(); 
            len = len - 1; 
            while stack[len-1] != InfixToken::LeftParen { 
                let mut z = 0; 
                match stack[len-1] { 
                    InfixToken::Operand(isize) => z = 0,
                    InfixToken::Operator(Operator::Mul) => postfix.push(PostfixToken::Operator(Operator::Mul)), 
                    InfixToken::Operator(Operator::Div) => postfix.push(PostfixToken::Operator(Operator::Div)), 
                    InfixToken::Operator(Operator::Add) => postfix.push(PostfixToken::Operator(Operator::Add)), 
                    InfixToken::Operator(Operator::Sub) => postfix.push(PostfixToken::Operator(Operator::Sub)),
                    InfixToken::LeftParen => z = 0, 
                    InfixToken::RightParen => z = 0,
                }
                len = len - 1; 
            }
	    stack.pop(); 
        }
    }
    //println!("{:?}", postfix); 
    Some(postfix)
}
*/
