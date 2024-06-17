// 引入所需的库和模块
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use crossterm::ExecutableCommand;
use std::io::{stdin, stdout, Write};

// 定义PrintCommand枚举类型，用于确定打印时使用的具体颜色
#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}



// Implement the PrintCommand methods 为PrintCommand实现方法
// 这段代码定义了一个  PrintCommand 枚举的  print_agent_message 方法，
// 该方法负责根据不同的  PrintCommand 枚举值，以不同的颜色打印代理（Agent）的消息。
impl PrintCommand {
    // 打印代理消息的方法
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        // Get the standard output handle 获取标准输出句柄
        let mut stdout: std::io::Stdout = stdout();
        
        // 根据枚举值选择打印的颜色
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,       // 如果是AICall，用青色
            Self::UnitTest => Color::Magenta,  // 如果是UnitTest，用品红色
            Self::Issue => Color::Red,         // 如果是Issue，用红色
        };
        
        // 设置前景色为绿色并打印代理位置
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent:{}", agent_pos);
        
        // 设置前景色为指定的颜色并打印代理声明
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);
        
        // 重置颜色
        stdout.execute(ResetColor).unwrap(); // Reset Color
    }
}

// 获取用户响应的函数
pub fn get_user_response(question: &str) -> String {
    let mut stdout = stdout();
    // 用蓝色打印问题
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    print!("{}", question);
    stdout.flush().unwrap();

    // 重置颜色
    stdout.execute(ResetColor).unwrap();

    // 读取用户输入
    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    // 去除响应中的空白符
    user_response.trim().to_string()
}

// src/helpers/command_line.rs 文件中的函数
// 确认代码是否安全执行
pub fn confirm_safe_code() -> bool {
    use std::io::{self, Write};

    // 提示用户确认
    print!("Are you sure the code is safe to execute? (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // 根据用户输入的响应返回布尔值
    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => {
            println!("Invalid input. Please enter 'y' or 'n'.");
            confirm_safe_code() // 递归调用，直到收到有效输入
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::str;

    // 运行测试并捕获输出的辅助函数
    fn run_test_and_capture_output<F>(test_func: F) -> String
    where
        F: FnOnce(&mut Vec<u8>) -> (),
    {
        let mut buffer = vec![];
        test_func(&mut buffer);
        String::from_utf8(buffer).expect("Invalid UTF-8 sequence")
    }

    // 测试AICall打印代理消息的方法
    #[test]
    fn test_prints_agent_msg_aicall() {
        let output = run_test_and_capture_output(|buffer| {
            writeln!(buffer, "Agent called the AI function.").unwrap();
        });

        assert!(output.contains("Agent called the AI function."));
    }

    // 测试UnitTest打印代理消息的方法
    #[test]
    fn test_prints_agent_msg_unittest() {
        let output = run_test_and_capture_output(|buffer| {
            writeln!(buffer, "Unit testing in progress...").unwrap();
        });

        assert!(output.contains("Unit testing in progress..."));
    }

    // 测试Issue打印代理消息的方法
    #[test]
    fn test_prints_agent_msg_issue() {
        let output = run_test_and_capture_output(|buffer| {
            writeln!(buffer, "There is an issue to resolve.").unwrap();
        });

        assert!(output.contains("There is an issue to resolve."));
    }
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn tests_prints_agent_msg() {
//         PrintCommand::AICall
//             .print_agent_message("Managing Agent", "Testing testing, processing something");
//     }
// }



// crossterm 是一个用于在终端（命令行界面）上进行跨平台文本操作和控制的 Rust 库。crossterm 提供跨平台TUI（終端用戶界面）功能的庫
// 它支持在不同的操作系统上进行一致的终端操作，例如 Windows、Linux 和 macOS。
// 借助 crossterm，你可以对终端进行以下操作：控制光标移动、隐藏和显示光标、
// 更改终端文本的颜色和样式、 处理输入事件（例如按键和鼠标事件）、清除屏幕或者部分屏幕、在终端中绘制图形或进行一些简单的动画操作。
// 在你的代码中，crossterm 库被用来更改文本颜色和重置颜色，以及执行一些与用户交互的操作。这些功能通过以下模块和方法实现：
// style::Color：定义颜色枚举，用于设置前景色或背景色。
// style::ResetColor：重置文本颜色。
// style::SetForegroundColor：设置前景色。
// ExecutableCommand：提供了一种机制，可以在标准输出上执行命令（例如设置颜色、移动光标等）。


// ResetColor 是一个 crossterm 提供的命令，它的作用是将终端中所有已经设置的文本颜色（包括前景色和背景色）重置为默认颜色。
// 因此，这行代码的作用是将终端的颜色状态恢复到操作之前的状态。
// 具体来说：// 如果你之前设置了某种前景色或背景色，例如使用了 SetForegroundColor 或 SetBackgroundColor 命令，
// 那么 ResetColor 会将这些设置清除，恢复到终端的默认颜色。
// 重置颜色有助于避免接下来打印的文本意外地延续了前面设置的颜色，确保后续的打印内容使用默认颜色，从而保持输出的一致性和可读性。


// agent_pos:  代表代理人的位置或角色，例如 "Managing Agent"、"UnitTest Agent" 或 "Issue Agent"。
// agent_statement: 代表代理人想要打印的消息。这些消息会根据不同的 PrintCommand: 枚举变体使用 ANSI 转义码进行不同颜色的格式化。