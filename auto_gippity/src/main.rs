// #[macro_export]属性用来标记这个宏应该被导出，也就是说它可以被它所在crate的用户使用。
// macro_rules! 是宏的声明开始，get_function_string是宏的名称。

// =>后面跟着的是宏展开的结果。这里使用stringify!($func)将$func转换为字符串。
// 注意，stringify!在编译时执行，不会对其参数求值，因此，即使传给宏的是函数调用，
// 也只会得到函数名的字符串表示，而不会执行函数。
#[macro_export]
macro_rules! get_function_string {
    ($func:ident) => {{
        stringify!($func)
    }};
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::PrintCommand;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        panic!("Not enough arguments.");
    }
    let command = &args[1];
    let agent = &args[2];
    let message = &args[3];

    match command.as_str() {
        "AICall" => PrintCommand::AICall.print_agent_message(agent, message),
        "UnitTest" => PrintCommand::UnitTest.print_agent_message(agent, message),
        "Issue" => PrintCommand::Issue.print_agent_message(agent, message),
        _ => panic!("Unknown command."),
    }
}