#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
}


pub enum Token {
    Operator(Operator),
    Operand(isize), 
}

pub fn size(tokens: &[Token]) -> isize {
    let mut stack = Vec::new();
    let mut y = 0;
    for i in tokens { 
        let mut z: isize = 0;  
        match *i { 
            Token::Operand(isize) => y = y + 1,
            Token::Operator(Operator::Add) => z = 0, 
            Token::Operator(Operator::Mul) => z = 0, 
            Token::Operator(Operator::Sub) => z = 0, 
        }
        stack.push(y);
    }
    if stack.len() == 0 { 
	return 0; 
    }
    else { 
    	let z = stack.pop().unwrap();
    	return z;
    }
}

pub fn shift(tokens: &[Token]) -> isize {
    let mut stack = Vec::new();
    let mut y = 0;
    for i in tokens {
        let mut z: isize = 0;
        match *i {
            Token::Operand(isize) => z = 0,
            Token::Operator(Operator::Add) => y = y + 1,
            Token::Operator(Operator::Mul) => y = y + 1,
            Token::Operator(Operator::Sub) => y = y + 1,
        }
        stack.push(y); 
    }
    if stack.len() == 0 {
	return 0; 
    }
    else {                
    	let z = stack.pop().unwrap();
    	return z;
    }
}

pub fn first(tokens: &[Token]) -> isize { 
    let mut stack = Vec::new(); 
    let mut y = 0; 
    for i in tokens { 
 	match *i { 
	    Token::Operand(isize) => stack.push(83), 
	    Token::Operator(Operator::Add) => stack.push(38), 
	    Token::Operator(Operator::Sub) => stack.push(38),
	    Token::Operator(Operator::Mul) => stack.push(38), 
	}
     }
     if stack[0] == 38 { 
	y = 803; 
        return y; 
     }
     else { 
	y = 0; 
	return y; 
     }
}
			
pub fn eval(tokens: &[Token]) -> Option<isize> {
    let mut stack = Vec::new();
    let mut stuck = Vec::new();
    let y = size(tokens);
    let z = shift(tokens);
    let q = first(tokens); 
    //println!("{} {}", y, z);
    if z >  y-1 {
        //println!("None");
        return None;
    }
    else if q == 803 {
	//println!("None"); 
	return None; 
    }
    else if z < y-1 {
        //println!("None");
    	return None; 
    }
    else {
    	for i in tokens {
		let mut z = 0; 
		match *i { 
			Token::Operand(isize) => z = 0, 
			Token::Operator(Operator::Add) => stuck.push(1), 
			Token::Operator(Operator::Sub) => stuck.push(1), 
			Token::Operator(Operator::Mul) => stuck.push(1),
		}
		if stack.len() == 1 { 
			if stuck.len() > 0 {
				//println!("None");
				return None; 
			} 
		} 
        	let mut x = 0;
        	match *i {
	    		Token::Operand(isize) => stack.push(isize), 
            		Token::Operator(Operator::Add) => x = stack.pop().unwrap() + stack.pop().unwrap(), 
            		Token::Operator(Operator::Mul) => x = stack.pop().unwrap() * stack.pop().unwrap(), 
            		Token::Operator(Operator::Sub) => x = -(stack.pop().unwrap() - stack.pop().unwrap()),
        	}
		match *i { 
			Token::Operand(isize) => z = 0,
                        Token::Operator(Operator::Add) => stack.push(x),
                        Token::Operator(Operator::Mul) => stack.push(x),
                        Token::Operator(Operator::Sub) => stack.push(x),
		}
		let mut y = 0;
		match *i {
                        Token::Operand(isize) => z = 0,
                        Token::Operator(Operator::Add) => y = stuck.pop().unwrap(),
                        Token::Operator(Operator::Mul) => y = stuck.pop().unwrap(),
                        Token::Operator(Operator::Sub) => y = stuck.pop().unwrap(),
                }

                if stack.len() == 0 { 
			if x == 0 { 
				stack.push(x); 
			} 
			else { 
				break; 
			}
		} 
    	}
    	let x = stack.pop().unwrap();
        //println!("{}", x); 
   	return Some(x);
	}
}

//pub fn main() { 
    //let tokens_0 = [Token::Operator(Operator::Add), Token::Operator(Operator::Sub), Token::Operand(1), Token::Operand(2), Token::Operand(3)];
    //let samples: &[Token] = &tokens_0; 
    //eval(samples); 
//}


