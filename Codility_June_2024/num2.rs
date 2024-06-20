// use std::cmp::max;
// use std::collections::HashSet;
fn main(){
    println!("{:?}", solution(&mut String::from("BAABA"), &mut vec![2, 4, 1, 1, 2]));   //[2,4]
    println!("{:?}", solution(&mut String::from("ABAB"), &mut vec![10,5,10,15]));       //[0,15]
    println!("{:?}", solution(&mut String::from("B"), &mut vec![100]));       //[100,0]
}

// pub fn solution(r: &mut String, v: &mut Vec<i32>) -> Vec<i32> {
//     let mut a = 0;
//     let mut b = 0;
//     let r_chars = r.chars().collect::<Vec<char>>();

//     // Iterate directly over indices and elements since we need both
//     for (i, &amount) in v.iter().enumerate() {
//         match r_chars[i] {
//             'A' => {
//                 if b < amount {
//                     b = amount; // Set b to the required amount if it's not enough
//                 } else {
//                     a += amount;
//                     b -= amount;
//                 }
//             },
//             'B' => {
//                 if a < amount {
//                     a = amount; // Set a to the required amount if it's not enough
//                 } else {
//                     a -= amount;
//                     b += amount;
//                 }
//             },
//             _ => panic!("Invalid character in string R"),
//         }
//     }

//     vec![a, b]
// }

pub fn solution(r: &mut String, v: &mut Vec<i32>) -> Vec<i32> {
    // let s:HashSet<i32> = v.iter().cloned().collect();
    // for n in v.iter_mut() {
    //     if !s.contains(&*n) {
    //         s.insert(n);
    //     }
    // }
    // println!("{:?}",s);
    let mut result: Vec<i32>=Vec::new();
    let mut a=0;
    let mut b=0;
    let mut noo:bool=true;

    loop {
        let mut aa=a;
        let mut bb=b;
        noo=true;
        for (direction,amount) in r.chars().zip(v.iter()){
            if direction=='A' && bb>=*amount {
                aa+=*amount;
                bb-=*amount;
            } else if direction=='B' && aa>=*amount {
                aa-=*amount;
                bb+=*amount;
            } else if direction=='A' && bb<*amount {
                b=*amount;
                noo=false;
                break;
            } else if direction=='B' && aa<*amount {
                a=*amount;
                noo=false;
                break;                
            }
            if noo==true {
                break;
            }
        }
        if noo==true {
            break;
        }
    }
    
    result.push(a);
    result.push(b);
    result
}

//difficult

// fn solution(r: &str, v: &[i32]) -> Vec<i32> {  
//     let mut bank_a_balance = 0;  
//     let mut bank_b_balance = 0;  
//     let mut max_a_needed = 0;  
//     let mut max_b_needed = 0;  
  
//     // 遍历转账列表，模拟转账过程  
//     for (i, &amount) in v.iter().enumerate() {  
//         // 根据接收方银行标识更新相应银行的余额  
//         match r.as_bytes()[i] as char {  
//             'A' => {  
//                 // 转账给银行A，需要确保银行B有足够的余额  
//                 if bank_b_balance < amount {  
//                     // 如果银行B余额不足，则需要增加银行B的初始余额  
//                     max_b_needed = max(max_b_needed, amount - bank_b_balance);  
//                 }  
//                 bank_b_balance -= amount;  
//                 bank_a_balance += amount;  
//             },  
//             'B' => {  
//                 // 转账给银行B，需要确保银行A有足够的余额  
//                 if bank_a_balance < amount {  
//                     // 如果银行A余额不足，则需要增加银行A的初始余额  
//                     max_a_needed = max(max_a_needed, amount - bank_a_balance);  
//                 }  
//                 bank_a_balance -= amount;  
//                 bank_b_balance += amount;  
//             },  
//             _ => panic!("Invalid bank identifier in string R"),  
//         }  
  
//         // 跟踪任何时刻两个银行余额的最小值，确保它们不会降至0以下  
//         max_a_needed = max(max_a_needed, -bank_a_balance);  
//         max_b_needed = max(max_b_needed, -bank_b_balance);  
//     }  
//     let result = vec![bank_a_balance, bank_b_balance];  
//     result  

// }  

// pub fn solution(r: &mut String, v: &mut Vec<i32>) -> Vec<i32> {
//     // Implement your solution here
//     let mut max=0;
//     for i in v.iter(){
//         if *i>max {
//             max=*i;
//         }
//     }
//     let mut result: Vec<i32>=Vec::new();
//     let a=0;let b=0;
//     let mut okk=true;
//     for a in 0..max{
//         for b in 0..max{
//             let mut aa=a;
//             let mut bb=b;
//             for (direction,amount) in r.chars().zip(v.iter()){
//                 if direction=='A' {
//                     if bb<amount {
//                         break;
//                     }
//                     aa+=amount;
//                     bb-=amount;
//                 } else if direction=='B' {
//                     if aa<amount {
//                         break;
//                     }
//                     aa-=amount;
//                     bb+=amount;
//                 }            
//             }
//         }
//     }

//     let mut aa=a;
//     let mut bb=b;

//     loop {
//         for(derection, amount) in r.chars().zip(v.iter()){
//             if direction=='A' {
//                 if bb<amount {
//                     bb=amount;
//                     break;
//                 }
//                 aa+=amount;
//                 bb-=amount;
//             } else if direction=='B' {
//                 if aa<amount {
//                     aa=amount;
//                     break;
//                 }
//                 aa-=amount;
//                 bb+=amount;
//             }
//         }
//         if(aa>=0&&bb>0){
//             break;
//         }
//     }
    
//     result.push(a);
//     result.push(b);
//     result
// }
