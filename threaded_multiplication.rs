use std::sync::mpsc;
use std::thread; // 用于等待所有线程完成

fn compute_and_print(id: usize, a: i32, b: i32) {
    let result = a * b;
    println!(
        "Thread {} finished computation: {} * {} = {}",
        id, a, b, result
    );
}

fn main() {
    // 创建三个通道，用于等待每个线程完成
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let (tx3, rx3) = mpsc::channel();

    // 启动三个线程
    let t1 = thread::spawn(move || {
        compute_and_print(1, 10, 10);
        tx1.send(()).unwrap(); // 发送完成信号
    });

    let t2 = thread::spawn(move || {
        compute_and_print(2, 100, 100);
        tx2.send(()).unwrap();
    });

    let t3 = thread::spawn(move || {
        compute_and_print(3, 1000, 1000);
        tx3.send(()).unwrap();
    });

    // 等待所有线程完成
    rx1.recv().unwrap();
    rx2.recv().unwrap();
    rx3.recv().unwrap();

    // 确保线程退出（尽管在上面的代码中，join是隐式的，因为我们在主线程中等待了接收）
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}

// douxiaobo@192 Rust % Zed threaded_multiplication.rs
// douxiaobo@192 Rust % rustc threaded_multiplication.rs
// douxiaobo@192 Rust % ./threaded_multiplication
// Thread 1 finished computation: 10 * 10 = 100
// Thread 3 finished computation: 1000 * 1000 = 1000000
// Thread 2 finished computation: 100 * 100 = 10000
// douxiaobo@192 Rust %
