// use std::convert::TryInto;
fn main(){
    println!("{:?}", solution(&mut String::from("BAABA"), &mut vec![2, 4, 1, 1, 2]));   //[2,4]
    println!("{:?}", solution(&mut String::from("ABAB"), &mut vec![10,5,10,15]));       //[0,15]
    println!("{:?}", solution(&mut String::from("B"), &mut vec![100]));       //[100,0]
}
// #[warn(unused_assignments)]
fn solution(r: &mut String, v: &mut Vec<i32>) -> Vec<i32> {
    let n = v.len();
    // for i,j in (0..n).rev() {
    //     println!("{:?}", arr[i]);
    // }
    let mut a=0;
    let mut b=0;
    let mut aa=a;
    let mut bb=b;
    let mut i=(n-1) as usize;
    let mut j=(n-1) as usize;
    loop {        
        let char_at_j = r.chars().nth(j).unwrap();
        let val_at_j = v[j];
        let val_at_i = v[i];

        println!("i: {}, j: {}, char_at_j: {}, val_at_j: {}, val_at_i: {}, a: {}, b: {}, aa: {}, bb: {}", i, j, char_at_j, val_at_j,val_at_i, a, b, aa, bb);

        match char_at_j {
            'A' => {
                if bb >= val_at_j {
                    bb -= val_at_j;
                    aa += val_at_j;
                } else {
                    b = val_at_i;
                    if i > 0 {i-=1;}
                    aa=a;
                    bb=b;
                    j=n-1;
                    // Removed unnecessary assignments
                    if i==0 || j == 0 {
                        break;
                    } else {
                        continue;
                    }
                }
            },
            'B' => {
                if aa >= val_at_j {
                    aa -= val_at_j;
                    bb += val_at_j;
                } else {
                    a = val_at_i;
                    if i > 0 {i-=1;}
                    aa=a;
                    bb=b;
                    j=n-1;
                    // Removed unnecessary assignments
                    if i==0 || j == 0 {
                        break;
                    } else {
                        continue;
                    }
                }
            },
            _ => (),
        }


        if i==0 || j == 0 {
            break;
        }
        j -=1;
    }
    vec![a,b]
}

        // if char_at_j == 'A' && bb >= val_at_j {
        //     bb -= val_at_j;
        //     aa += val_at_j;
        // } else if char_at_j == 'A' && bb < val_at_j {
        //     b += val_at_i;
        //     i -= 1;
        //     aa = a;
        //     bb=b;
        //     break;
        // } else if char_at_j == 'B' && aa >= val_at_j {
        //     aa -= val_at_j;
        //     bb += val_at_j;
        // } else if char_at_j == 'B' && bb < val_at_j {
        //     a += val_at_i;
        //     i -= 1;
        //     aa=a;
        //     bb = b;
        //     break;
        // }