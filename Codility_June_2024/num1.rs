fn main(){
    println!("{}",solution(14));    //19
    println!("{}",solution(10));    //11
    println!("{}",solution(99));    //9999
}

fn solution(n:i32)->i32{
    let mut num:i32=0;
    let mut tmp=i32::from(n);
    loop {
        num+=tmp%10;
        tmp=tmp/10;
        if tmp==0 {
            break;
        }
    }
    for i in n..std::i32::MAX {
        tmp=i;
        let mut result=0;
        loop{
            result+=tmp%10;
            tmp=tmp/10;
            if tmp==0 {
                break;
            }
        }
        if result==num*2 {
            return i as i32;
        }
        
    }
    -1
}

//OK