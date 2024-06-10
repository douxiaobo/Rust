use std::sync::mpsc;
use std::thread; // 用于线程间通信

fn compute(id: usize, data: &mut i32) {
    // 假设这里是一个耗时的计算
    *data *= 2; // 简单的示例
    println!("Thread {} finished computation", id);
}

fn main() {
    let mut data1 = 1;
    let mut data2 = 2;
    let mut data3 = 3;
    let mut data4 = 4;

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let (tx3, rx3) = mpsc::channel();
    let (tx4, rx4) = mpsc::channel();

    // 启动四个线程
    let t1 = thread::spawn(move || {
        compute(1, &mut data1);
        tx1.send(()).unwrap(); // 发送完成信号
    });
    let t2 = thread::spawn(move || {
        compute(2, &mut data2);
        tx2.send(()).unwrap();
    });
    let t3 = thread::spawn(move || {
        compute(3, &mut data3);
        tx3.send(()).unwrap();
    });
    let t4 = thread::spawn(move || {
        compute(4, &mut data4);
        tx4.send(()).unwrap();
    });

    // 等待所有线程完成
    rx1.recv().unwrap();
    rx2.recv().unwrap();
    rx3.recv().unwrap();
    rx4.recv().unwrap();

    println!("Data1: {}", data1);
    println!("Data2: {}", data2);
    println!("Data3: {}", data3);
    println!("Data4: {}", data4);

    // 确保线程退出
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();
}

// Last login: Mon Jun 10 17:39:35 on ttys002
// douxiaobo@192 Rust % zed .
// douxiaobo@192 Rust % rustc threaded_computation.rs
// douxiaobo@192 Rust % ./threaded_computation
// Thread 1 finished computation
// Thread 2 finished computation
// Thread 3 finished computation
// Thread 4 finished computation
// Data1: 1
// Data2: 2
// Data3: 3
// Data4: 4
// douxiaobo@192 Rust %
