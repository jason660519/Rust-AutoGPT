use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder, RequestBuilder};
use std::{env, process::Output};

// Call Large language model(ie gpt-4)
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract API key information
    let api_key: String =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in environment variables");
    // let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in environment variables");

    // Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers = HeaderMap::new();

    // 添加API密钥到请求头中
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // // Create Open AI Org header   添加OpenAI组织信息到请求头中

    // headers.insert(
    //     "OpenAI-Organization",
    //     HeaderValue::from_str(api_org.as_str())
    //         .map_err(|e| -> Box<dyn std::error::Error + Send>{ Box::new(e) })?
    // );

    // Create client 創建 HTTP 客戶端
    let client: Client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create chat completion  創建聊天請求/创建聊天完成对象
    let chat_completion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature: 0.1,  // 低温度值有助于生成更一致、预测性的回答
    };

    // Extract API Response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Send Response 返回回應的文本內容
    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn tests_call_to_openai() {
        // 創建測試訊息
        let message: Message = Message {
            role: "user".to_string(), // 訊息的角色是 user
            content: "Hi there, this is a test. Give me a short response.".to_string(), // 訊息內容
        };

        // 將訊息放入向量中
        let messages: Vec<Message> = vec![message];

        // 調用 call_gpt 函數
        let res: Result<String, Box<dyn std::error::Error + Send>> = call_gpt(messages).await;

        // 檢查回應結果
        match res {
            Ok(res_str) => {
                dbg!(res_str); // 輸出回應內容到調試控制台
                assert!(true); // 測試通過
            }
            Err(_) => {
                assert!(false); // 測試失敗
            }
        }
    }
}


    // Create api key header
    // HeaderValue::from_str 是一個方法，用於從字符串創建一個 HeaderValue 對象。
    // &format!("Bearer {}", api_key) 使用 format! 宏來創建一個字符串，這個字符串的格式是 "Bearer <API_KEY>"。
    // 這是許多 API 認證中常見的格式，其中 Bearer 是固定的字符串，後面接著的是實際的 API 金鑰。
    // 例如，假設 api_key 是 abc123，那麼 format!("Bearer {}", api_key) 會生成字符串 "Bearer abc123"。

    //   為什麼要包裝成 Box<dyn std::error::Error + Send>
    //   多態性（Polymorphism）： Box<dyn std::error::Error> 允許我們存儲任何類型的錯誤，只要它實現了 std::error::Error trait。
    //   這種多態性使得我們可以用一種統一的方式處理不同的錯誤類型。
    //   動態分發（Dynamic Dispatch）：使用 Box<dyn Trait> 的方式來實現動態分發，
    //   可以在運行時確定具體的錯誤類型，而不是在編譯時。這對於需要處理多種錯誤類型的場景非常有用。
    //   Send Trait：保證了這個錯誤可以在線程之間傳遞。這對於異步編程（如在 tokio 中使用）尤為重要，因為這樣可以確保錯誤在不同線程之間傳遞時是安全的。



// 这段代码 .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })? 
// 是Rust编程语言中的一种错误处理模式，使用在异步编程或错误转换场景中。让我们一步步来分解它：
// .map_err(...) 方法： 这是Result枚举提供的一个方法，被用来处理Result类型的错误值(Err)。
// Result是Rust中用来表示操作可能成功返回值也可能失败返回错误的类型。它有两个变体：Ok(T)，代表成功并包含结果值；Err(E)，代表错误并包含错误信息。
// .map_err(...) 接受一个闭包（closure）作为参数。这个闭包针对Err(E)情况下的错误值e执行某些操作，
// 然后返回一个新的值。这里的目的是将错误转换成另一种类型。

// 闭包（Closure）： |e| -> Box<dyn std::error::Error + Send> { Box::new(e) } 是一个闭包，它接受一个参数e，
// 并返回一个Box<dyn std::error::Error + Send>类型的值。 |e| 表示闭包的参数，这里e是map_err方法接收到的原始错误值。
// -> Box<dyn std::error::Error + Send> 指定了闭包的返回类型，即这个闭包会返回一个装箱的（Boxed）错误类型。
// **Box<dyn std::error::Error + Send>**：
// Box<dyn std::error::Error> 是一个装箱的（Boxed）错误值，dyn std::error::Error表示任意实现了std::error::Error trait的类型。
// 这是多态性的一种体现，可以存储任何错误类型。
// + Send 表示这个错误类型还实现了Send trait，即这个错误值可以安全地在线程间传递。这在异步编程中特别有用。
// ? 运算符： ? 是Rust中的一个错误传播运算符。如果Result是Ok(T)，它会返回T值，让程序继续执行。如果是Err(E)，
// 它会提前返回错误值，并由当前函数的调用者处理。
// 在这个代码段中，?运算符用于检查.map_err(...)的结果。如果结果是错误，通过?运算符将错误传递出去，由函数的调用者进行处理。

// 总结来说，.map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })? 
// 这段代码的作用是，将当前操作中遇到的任何错误转换为一个可以在线程间传递的通用错误类型，并利用?运算符来传递错误，以简化错误处理流程。


// map_err 方法用於將可能的錯誤轉換成另一種錯誤類型。
// 在這裡，|e| -> Box<dyn std::error::Error + Send>{ Box::new(e) } 是一個閉包，
// 用於將錯誤 e 包裝成 Box<dyn std::error::Error + Send> 類型。這樣做是為了滿足函數返回錯誤的要求。
// ? 操作符用於在出錯時提前返回錯誤，這樣當 from_str 方法失敗時，整個函數會返回一個錯誤，而不是繼續執行。

