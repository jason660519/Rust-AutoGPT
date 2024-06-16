// #[macro_export]属性用来标记这个宏应该被导出，也就是说它可以被它所在crate的用户使用。
// macro_rules! 是宏的声明开始，get_function_string是宏的名称。

// =>后面跟着的是宏展开的结果。这里使用stringify!($func)将$func转换为字符串。
// 注意，stringify!在编译时执行，不会对其参数求值，因此，即使传给宏的是函数调用，
// 也只会得到函数名的字符串表示，而不会执行函数。


#[macro_export]
macro_rules! get_function_string{
    (&func:idnt)=>{{
        stringigy!($func)
    }};
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;

fn main() {
    let usr_req: String = get_user_response("What webserver are we building today?");
    dbg!(usr_req);
}
