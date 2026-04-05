impl Solution {
    pub fn calculation(opnd1: i32, opnd2: i32, optr: &str) -> i32 {
        match optr {
            "+" => opnd1 + opnd2,
            "-" => opnd1 - opnd2,
            "*" => opnd1 * opnd2,
            "/" => opnd1 / opnd2,
            _ => 0,
        }
    }

    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let opnd2 = stack.pop().unwrap();
                    let opnd1 = stack.pop().unwrap();

                    let result = Self::calculation(opnd1, opnd2, token.as_str());
                    stack.push(result);
                }
                _ => {
                    stack.push(token.parse::<i32>().unwrap());
                }
            }
        }

        stack.pop().unwrap()
    }
}