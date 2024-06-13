#[cfg(test)]
mod test {
    // 引入必要的模块
    use std::sync::{Arc, Mutex, MutexGuard}; // 引入多线程同步工具：Arc（原子引用计数）、Mutex（互斥锁）和 MutexGuard（互斥锁保护的资源）。
    use std::fs::{OpenOptions, File}; // 引入文件操作相关模块：OpenOptions（文件选项）和 File（文件）。
    use std::io::prelude::*; // 引入输入输出（I/O）相关的标准库预声明模块。
    use std::thread::{JoinHandle, spawn}; // 引入线程操作相关模块，JoinHandle（线程句柄）和 spawn（用于产生新线程）。

    #[test]
    fn tests_concurrency() {
        // 创建一个Arc（原子引用计数）包装的Mutex（互斥锁）来保护一个文件。
        let file_mutex: Arc<Mutex<File>> = Arc::new(Mutex::new(
            OpenOptions::new() // 使用OpenOptions配置文件选项。
                .write(true)  // 允许写入。
                .create(true) // 如果文件不存在，创建该文件。
                .append(true) // 以附加模式写入到文件中。
                .open("increments.txt") // 打开名为"increments.txt"的文件。
                .unwrap(), // 确保文件成功打开，如果失败则引发恐慌（panic）。
        ));

        // 创建一个用于存储线程句柄（JoinHandle）的向量，用于等待所有线程完成。
        let mut handles: Vec<JoinHandle<()>> = vec![];

        // 创建10个线程，分别将0到9写入到文件中。
        for i in 0..10 {
            // 克隆Arc以便在线程间共享file_mutex。
            let file_mutex = Arc::clone(&file_mutex);
            // 使用spawn函数创建一个新线程。
            let handle = spawn(move || {
                // 获取Mutex保护的文件的锁，确保在其他线程完成之前当前线程拥有写入权限。
                let mut file = file_mutex.lock().unwrap();
                // 将数字写入文件，每个线程写入一个不同的数字。
                writeln!(file, "{}", i).unwrap();
            });
            // 将线程句柄保存到(handles)向量中。
            handles.push(handle);
        }

        // 等待所有线程完成。
        for handle in handles {
            handle.join().unwrap(); // 确保每个线程正确完成，若有线程执行失败则引发恐慌（panic）。
        }
    }
}