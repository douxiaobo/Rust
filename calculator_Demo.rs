use std::collections::HashMap;

#[derive(Debug)]
struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            top: 0,
            data: Vec::new()
        }
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.top += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if 0 == self.top { return None; }
        self.top -= 1;
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        if 0 == self.top { return None; }
        self.data.get(self.top - 1)
    }

    fn is_empty(&self) -> bool {
        0 == self.top
    }
}

// 同时检测多种括号
fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

// 检测括号是否匹配
fn par_checker3(infix: &str) -> bool {
    let mut char_list = Vec::new();
    for c in infix.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();
    while index < char_list.len() && balance {
        let c = char_list[index];
        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        }

        if ')' == c || ']' == c || '}' == c {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }

        index += 1;
    }

    balance && stack.is_empty()
}

// fn infix_to_postfix(infix: &str) -> Option<String> {
//     // 括号匹配检验
//     if !par_checker3(infix) { return None; }

//     // 设置各个符号的优先级
//     let mut prec = HashMap::new();
//     prec.insert("(", 1); prec.insert(")", 1);
//     prec.insert("+", 2); prec.insert("-", 2);
//     prec.insert("*", 3); prec.insert("/", 3);

//     // ops 保存操作符号、postfix 保存后缀表达式
//     let mut ops = Stack::new();
//     let mut postfix = Vec::new();
//     for token in infix.split_whitespace() {
//         if "0" <= token && token <= "9" {
//             // 0 - 9  范围字符入栈
//             postfix.push(token);
//         } else if "(" == token  {
//             // 遇到开括号，将操作符入栈
//             ops.push(token);
//         } else if ")" == token  {
//             // 遇到闭括号，将操作数入栈
//             let mut top = ops.pop().unwrap();
//             while top != "(" {
//                 postfix.push(top);
//                 top = ops.pop().unwrap();
//             }
//         } else {
//             // 比较符号的优先级来决定操作符号是否加入 postfix
//             while !ops.is_empty() &&
//                 prec[ops.peek().unwrap()] >= prec[token] {
//                 postfix.push(ops.pop().unwrap());
//             }
//             ops.push(token);
//         }
//     }

//     // 剩下的操作数入栈
//     while !ops.is_empty() {
//         postfix.push(ops.pop().unwrap())
//     }

//     // 出栈并组成字符串
//     let mut postfix_str = "".to_string();
//     for c in postfix {
//         postfix_str += &c.to_string();
//         postfix_str += " ";
//     }

//     Some(postfix_str)
// }

fn infix_to_postfix(infix: &str) -> Option<String> {
    // 括号匹配检验
    if !par_checker3(infix) { return None; }

    // 设置各个符号的优先级
    let mut prec = HashMap::new();
    prec.insert("(", 1); prec.insert(")", 1);
    prec.insert("+", 2); prec.insert("-", 2);
    prec.insert("*", 3); prec.insert("/", 3);

    // ops 保存操作符号、postfix 保存后缀表达式
    let mut ops = Stack::new();
    let mut postfix = Vec::new();
    for token in infix.split_whitespace() {
        if token.parse::<i32>().is_ok() {
            // 数字入栈
            postfix.push(token);
        } else if "(" == token  {
            // 遇到开括号，将操作符入栈
            ops.push(token);
        } else if ")" == token  {
            // 遇到闭括号，将操作数入栈
            let mut top = ops.pop().unwrap();
            while top != "(" {
                postfix.push(top);
                top = ops.pop().unwrap();
            }
        } else {
            // 比较符号的优先级来决定操作符号是否加入 postfix
            let token_prec = match prec.get(token) {
                Some(prec_value) => *prec_value,
                None => {
                    println!("Unknown operator: {}", token);
                    return None; // 或者抛出其他错误
                },
            };
            while !ops.is_empty() &&
                *prec.get(ops.peek().unwrap()).unwrap() >= token_prec {
                postfix.push(ops.pop().unwrap());
            }
            ops.push(token);
        }
    }

    // 剩下的操作数入栈
    while !ops.is_empty() {
        postfix.push(ops.pop().unwrap())
    }

    // 出栈并组成字符串
    let mut postfix_str = "".to_string();
    for c in postfix {
        postfix_str += &c.to_string();
        postfix_str += " ";
    }

    Some(postfix_str)
}

// 执行数学运算
fn do_calc(op: &str, op1: i32, op2: i32) -> i32 {
    if "+" == op  {
        op1 + op2
    } else if "-" == op {
        op1 - op2
    } else if "*" == op {
        op1 * op2
    } else if "/" == op {
        if 0 == op2 {
            panic!("ZeroDivisionError: Invalid operation!");
        }
        op1 / op2
    } else {
        panic!("OperatorError: Invalid operator: {:?}", op);
    }
}

fn postfix_eval(postfix: &str) -> Option<i32> {
    // 少于五个字符，不是有效的后缀表达式，因为表达式
    // 至少两个操作数加一个操作符，还需要两个空格隔开
    if postfix.len() < 5 { return None; }

    // 操作数栈
    let mut ops = Stack::new();
    for token in postfix.split_whitespace() {
        if "0" <= token  && token <= "9" {
            ops.push(token.parse::<i32>().unwrap());
        } else {
            // 对于减法和除法，顺序有要求
            // 所以先出栈的是第二个操作数
            let op2 = ops.pop().unwrap();
            let op1 = ops.pop().unwrap();
            let res = do_calc(token, op1, op2);
            ops.push(res);
        }
    }

    Some(ops.pop().unwrap())
}

fn main(){
    let args=std::env::args().collect::<Vec<String>>();
    if args.len()!=2{
        println!("Usage: {} <string>",args[0]);
        return;
    }
    let postfix=infix_to_postfix(&args[1].clone());
    match postfix {
        Some(ref val)=>println!("{}->{}",args[1],val),
        None=>println!("Invalid expression!"),        
    }
    let res=postfix_eval(&postfix.unwrap());
    match res {
        None=>println!("Invalid expression!"),
        Some(x)=>println!("{}={}",args[1],x),
    }
//     let postfix = infix_to_postfix(&args[1].clone());

//    match postfix {
//        None => println!("Invalid expression!"),
//        Some(postfix_expr) => {
//            let res = postfix_eval(&postfix_expr);
//            match res {
//                None => println!("Invalid expression!"),
//                Some(x) => println!("{}={}", args[1], x),
//            }
//        },
//    }
}

// use std::collections::VecDeque;

// fn infix_to_postfix(expression: &str) -> Option<String> {
//     // 定义运算符优先级映射
//     let precedence = |op: char| match op {
//         '+' | '-' => 1,
//         '*' | '/' => 2,
//         '(' => 0,
//         ')' => 3,
//         _ => panic!("Invalid operator"),
//     };

//     // 定义运算符是否左结合
//     let is_left_associative = |op: char| match op {
//         '+' | '-' | '*' | '/' => true,
//         _ => false,
//     };

//     // 定义辅助结构和变量
//     let mut output = VecDeque::new();
//     let mut operators = VecDeque::new();

//     // 遍历输入表达式
//     for token in expression.chars().collect::<Vec<char>>().chunks(1) {
//         let token = token[0];

//         // 处理空格
//         if token == ' ' {
//             continue;
//         }

//         // 处理数字和括号
//         if token.is_alphabetic() || token == '(' || token == ')' {
//             output.push_back(token);
//         } else {
//             // 处理运算符
//             while !operators.is_empty() && precedence(operators.back().unwrap()) >= precedence(token) && is_left_associative(token) {
//                 output.push_back(operators.pop_back().unwrap());
//             }
//             operators.push_back(token);
//         }
//     }

//     // 输出剩余运算符
//     while !operators.is_empty() {
//         output.push_back(operators.pop_back().unwrap());
//     }

//     // 将输出队列转换为字符串并返回
//     Some(output.into_iter().collect())
// }