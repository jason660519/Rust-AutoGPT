use crate::apis::call_request::call_gpt;
// 引入模块crate::apis::call_request::call_gpt，用于调用GPT的API请求。

use crate::helpers::command_line::PrintCommand;
// 引入模块crate::helpers::command_line::PrintCommand，用于打印命令行信息。

use crate::models::general::llm::Message;
// 引入模块crate::models::general::llm::Message，用于定义消息结构。

use reqwest::Client;
// 引入reqwest库中的Client，用于发送HTTP请求。

use serde::de::DeserializeOwned;
// 引入serde库中的DeserializeOwned特性，用于反序列化JSON数据。

use std::fs;
// 引入标准库std::fs，用于文件系统操作。

// 常量CODE_TEMPLATE_PATH，定义代码模板的文件路径。
const CODE_TEMPLATE_PATH: &str =
    "/Users/shaun/Code/TUTORIALS/rust_autogpt/web_template/src/code_template.rs";

// 常量WEB_SERVER_PROJECT_PATH，定义Web服务器项目的文件路径。
pub const WEB_SERVER_PROJECT_PATH: &str = "/Users/shaun/Code/TUTORIALS/rust_autogpt/web_template/";

// 常量EXEC_MAIN_PATH，定义可执行文件main.rs的文件路径。
pub const EXEC_MAIN_PATH: &str =
    "/Users/shaun/Code/TUTORIALS/rust_autogpt/web_template/src/main.rs";

// 常量API_SCHEMA_PATH，定义API模式文件的文件路径。
const API_SCHEMA_PATH: &str =
    "/Users/shaun/Code/TUTORIALS/rust_autogpt/auto_gippity/schemas/api_schema.json";

// 扩展AI函数以鼓励特定输出
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str: &str = ai_func(func_input);

    // 扩展字符串以鼓励只打印输出
    let msg: String = format!(
        "FUNCTION: {}
  INSTRUCTION: You are a function printer. You ONLY print the results of functions.
  Nothing else. No commentary. Here is the input to the function: {}.
  Print out what the function will return.",
        ai_function_str, func_input
    );

    // 返回消息
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// 执行对GPT的调用请求
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // 扩展AI函数
    let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

    // 打印当前状态
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // 获取LLM响应
    let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    // 返回成功或再次尝试
    match llm_response_res {
        Ok(llm_resp) => llm_resp,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice to call OpenAI"),
    }
}

// 执行对GPT的调用请求 - 解码后的
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;
    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode ai response from serde_json");
    return decoded_response;
}

// 检查请求URL是否合法
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// 获取代码模板内容
pub fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

// 获取可执行文件main.rs的内容
pub fn read_exec_main_contents() -> String {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

// 保存新的后端代码
pub fn save_backend_code(contents: &String) {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("Failed to write main.rs file");
}

// 保存JSON API端点模式
pub fn save_api_endpoints(api_endpoints: &String) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write API Endpoints to file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    // 测试扩展AI函数
    fn tests_extending_ai_function() {
        let extended_msg: Message =
            extend_ai_function(convert_user_input_to_goal, "dummy variable");
        dbg!(&extended_msg);
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    // 测试AI任务请求
    async fn tests_ai_task_request() {
        let ai_func_param: String =
            "Build me a webserver for making stock price api requests.".to_string();

        let res: String = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(res.len() > 20);
    }
}
// #[tokio::test]
//     async fn tests_ai_task_request(){
//         let ai_func_param:String = "Build me a webserver for making stock price api requests.".to_string();
//         let res=ai_task_request(
//             ai_func_param,
//             "Managing Agent",
//             "Defining user requirements",
//         convert_user_input_to_goal
//         ).await;
//         assert!(res.len()>20);
//     }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

//     #[test]
//     fn tests_extending_ai_function() {
//         let extended_msg: Message = extend_ai_function(convert_user_input_to_goal, "dummy variable");
//         dbg!(&extended_msg);
//         assert_eq!(extended_msg.role, "system".to_string());
//     }
// }
