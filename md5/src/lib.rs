use md5::Digest;
use md5::Md5;
use std::fs::File;
use std::io::{Read, Write};

#[inline(always)]
pub fn run() {
    print!("请导入文件/文件夹路径(如果需要多个执行请输入一个 .list文件路径)回车执行: ");
    std::io::stdout().flush().unwrap();
    let mut cmd = String::new();
    std::io::stdin().read_line(&mut cmd).expect("无法读取操作");
    // 判断是否需要读取文件
    if cmd.contains(".list") {
        let mut file = match std::fs::File::open(cmd.trim()) {
            Ok(file) => file,
            Err(_) => {
                println!("无法打开文件");
                return;
            }
        };
        let mut buffer = String::new();
        match file.read_to_string(&mut buffer) {
            Ok(_) => {}
            Err(_) => {
                println!("无法读取文件");
                return;
            }
        }
        cmd = buffer;
    }
    // 按空格切割
    let paths: Vec<&str> = cmd.trim().split("\n").collect();
    let mut result: String = String::new();
    let mut error: String = String::new();
    // 遍历路径
    for path in paths {
        let path = path.trim();
        if path.is_empty() {
            continue;
        }
        let is_dir = match std::fs::metadata(path) {
            Ok(meta) => meta.is_dir(),
            Err(_) => false,
        };
        if is_dir {
            // 判断是否文件夹如果是文件夹读取文件夹内所有文件
            let paths = std::fs::read_dir(path).unwrap();
            for path in paths {
                let path = path.unwrap().path();
                let path = path.to_str().unwrap();
                // 读取文件md5
                let md5 = read_file_to_md5(path);
                match md5 {
                    Ok(md5) => {
                        println!("{}: {}", path, md5);
                        result += &format!("{}：{}\n", path, md5);
                    }
                    Err(err) => {
                        println!("失败{}: {}", path, err);
                        error += &format!("{}：{}\n", path, err);
                    }
                }
            }
        } else {
            // 读取文件md5
            let md5 = read_file_to_md5(path);
            match md5 {
                Ok(md5) => {
                    println!("{}: {}", path, md5);
                    result += &format!("{}：{}\n", path, md5);
                }
                Err(err) => {
                    println!("失败{}: {}", path, err);
                    error += &format!("{}：{}\n", path, err);
                }
            }
        }
    }
    // 输出结果到文件
    // 文件名为当前时间
    let now = chrono::Local::now();
    if !result.is_empty() {
        let path = format!("md5-{}.txt", now.format("%Y-%m-%d-%H-%M-%S"));
        match std::fs::write(&path, result) {
            Ok(_) => println!("成功结果已保存到{}", path),
            Err(_) => println!("无法保存结果"),
        }
    }
    if !error.is_empty() {
        let path = format!("md5-error-{}.txt", now.format("%Y-%m-%d-%H-%M-%S"));
        match std::fs::write(&path, error) {
            Ok(_) => println!("失败结果已保存到{}", path),
            Err(_) => println!("无法保存结果"),
        }
    }
}

/**
 * 读取网络文件并计算md5
 */
// #[inline(always)]
// pub fn read_url_to_md5(path: &str) -> Result<String, &str> {
//     let response = match reqwest::blocking::get(path) {
//         Ok(response) => match response.bytes() {
//             Ok(response) => response,
//             Err(_) => return Err("读取文件失败"),
//         },
//         Err(_) => return Err("读取文件失败"),
//     };
//     let mut context = Md5::new();
//     context.update(&response);
//     let digest = context.finalize();
//     let digest = format!("{:x}", digest);
//     Ok(digest)
// }

/**
 * 读取文件并计算md5
 */
#[inline(always)]
pub fn read_file_to_md5(path: &str) -> Result<String, &str> {
    // 路径格式化 去除双引号
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err("无法打开文件"),
    };
    let mut buffer = [0u8; 1024];
    let mut context = Md5::new();
    loop {
        let n = match file.read(&mut buffer) {
            Ok(n) => n,
            Err(_) => return Err("无法读取文件"),
        };
        if n == 0 {
            break;
        }
        context.update(&buffer[..n]);
    }
    let digest = context.finalize();
    let digest = format!("{:x}", digest);
    Ok(digest)
}
