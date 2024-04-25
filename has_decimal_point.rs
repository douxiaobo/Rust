fn main(){
    let args=std::env::args().collect::<Vec<String>>();
    if args.len()!=2{
        println!("Usage: {} <number>",args[0]);
        std::process::exit(1);
    }
    // let str=&args[1];
    // println!("{}",str);
    // let mut has_demimal_point=match str.find('.'){
    //     Some(_)=>true,
    //     None=>false,
    // };
    // match has_demimal_point{
    //     true=>println!("has decimal point"),
    //     false=>println!("no decimal point"),
    // }
    let mut int_part:u32=0;
    let mut frac_part:u32=0;
    let mut frac_digits=0;

    let mut has_demimal_point=false;

    for c in args[1].chars(){
        if c.is_ascii_digit()&&!has_demimal_point{  //  整数
            int_part=int_part*10+c.to_digit(10).unwrap();
        } else if c.is_ascii_digit()&&has_demimal_point{    //小数点
            frac_part=frac_part*10+c.to_digit(10).unwrap();
            frac_digits+=1;
        } else if c=='.'{
            has_demimal_point=true;
            // println!("has decimal point");
        } else {
            println!("error");            
        }
    }
    let num =match has_demimal_point{
        true=>{
            let frac_part_f32=frac_part as f32;
            int_part as f32+frac_part_f32/10f32.powi(frac_digits)       //powi是幂函数，计算整数次幂
        },
        false=>int_part as f32,
    };
    println!("num:{}",num);
    
}