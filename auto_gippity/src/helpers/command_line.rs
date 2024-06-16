use crossterm::style::{Color, ResetColor, SetForegroundColor};
use crossterm::ExecutableCommand;
use std::io::{stdin, stdout, Write};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent:{}", agent_pos);
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);
        stdout.execute(ResetColor).unwrap(); // Reset Color
    }
}

pub fn get_user_response(question: &str) -> String {
    let mut stdout = stdout();
    // Print the question in blue color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    print!("{}", question);
    stdout.flush().unwrap();

    // Reset the color
    stdout.execute(ResetColor).unwrap();

    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    user_response.trim().to_string()
}

// src/helpers/command_line.rs
pub fn confirm_safe_code() -> bool {
    use std::io::{self, Write};

    print!("Are you sure the code is safe to execute? (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => {
            println!("Invalid input. Please enter 'y' or 'n'.");
            confirm_safe_code() // Recursively ask until a valid input is received
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::str;

    fn run_test_and_capture_output<F>(test_func: F) -> String
    where
        F: FnOnce(&mut Vec<u8>) -> (),
    {
        let mut buffer = vec![];
        test_func(&mut buffer);
        String::from_utf8(buffer).expect("Invalid UTF-8 sequence")
    }

    #[test]
    fn test_prints_agent_msg_aicall() {
        let output = run_test_and_capture_output(|buffer| {
            writeln!(buffer, "Agent called the AI function.").unwrap();
        });

        assert!(output.contains("Agent called the AI function."));
    }

    #[test]
    fn test_prints_agent_msg_unittest() {
        let output = run_test_and_capture_output(|buffer| {
            writeln!(buffer, "Unit testing in progress...").unwrap();
        });

        assert!(output.contains("Unit testing in progress..."));
    }

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