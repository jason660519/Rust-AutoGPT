// 引入AgentState枚举，定义了代理可能处于的状态。
use crate::models::agent_basic::basic_agent::AgentState;
// 引入Message结构体，代表代理可能存储或处理的消息。
use crate::models::general::llm::Message;

// 定义一个名为BasicTraits的trait。
pub trait BasicTraits {
    // 定义一个关联函数（类似于其他编程语言中的静态方法），用于创建并返回trait实现者的实例。
    fn new(objective: String, position: String) -> Self;

    // 定义一个方法，用于更新实现此trait的实例的状态。
    fn update_state(&mut self, new_state: AgentState);

    // 定义一个方法，返回实现此trait的实例的目标。
    fn get_objective(&self) -> &String;

    // 定义一个方法，返回实现此trait的实例的位置。
    fn get_position(&self) -> &String;

    // 定义一个方法，返回实现此trait的实例当前的状态。
    fn get_state(&self) -> &AgentState;

    // 定义一个方法，返回与实现此trait的实例相关联的消息列表的引用。
    fn get_memory(&self) -> &Vec<Message>;
}