use ansi_term::Colour::{Blue, Green, Purple, Red};
use num_enum::TryFromPrimitive;
use std::io::Write;

#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum FuncEnum {
    GetMd5 = 1,
}

const FUNCS: [(FuncEnum, &str); 1] = [(FuncEnum::GetMd5, "获取文件MD5!")];

fn main() {
    ansi_term::enable_ansi_support().unwrap();
    println!("{}", Purple.paint("[声明]"));
    println!();
    println!("  > 该工具仅用于学习交流, 如有侵权请联系删除");
    println!();
    println!(
        "   {}",
        Green.paint(r#"\^o^/ 请您输入指定数字执行指定操作 \^o^/"#)
    );
    println!();
    // 遍历 FUNCS
    for (key, label) in FUNCS.iter() {
        println!(
            "    {}. {}",
            Blue.paint((key.to_owned() as u8).to_string()),
            label
        );
    }
    println!();
    run()
}

fn run() {
    print!("{}", Green.paint("请输入一个数字指令回车执行："));
    std::io::stdout().flush().unwrap();
    // 让用户输入想执行的程序
    let mut cmd = String::new();
    std::io::stdin().read_line(&mut cmd).expect("无法读取操作");
    let cmd = cmd.trim().parse::<u8>().unwrap();
    let cmd = FuncEnum::try_from(cmd);
    if cmd.is_err() {
        println!("{}", Red.paint("无效的输入， 请输入数字指令。"));
        return run();
    }
    let cmd = cmd.unwrap();
    // 获取枚举值
    match cmd {
        FuncEnum::GetMd5 => md5::run(),
    }
    // 暂停程序不退出
    println!("Press Enter to exit...");
    std::io::stdin().read_line(&mut (String::new())).unwrap();
    std::process::exit(0);
}
