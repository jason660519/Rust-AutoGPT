// 引入Message结构体，用于代表消息内容。
use crate::models::general::llm::Message;
// 引入BasicTraits特征，定义了代理基本行为。
use crate::models::agent_basic::basic_traits::BasicTraits;

// 定义AgentState枚举，表示代理的不同状态。
#[derive(Debug, PartialEq)]
pub enum AgentState{
  Discovery,  // 发现状态
  Working,    // 工作状态
  UnitTesting,// 单元测试状态
  Finished    // 完成状态
}

// 定义BasicAgent结构体，代表基本的代理。
#[derive(Debug)]
pub struct BasicAgent{
    pub objective: String, // 代理的目标
    pub position: String,  // 代理的位置或角色
    pub state: AgentState, // 代理当前的状态
    pub memory: Vec<Message> // 代理的记忆，存放消息
}

// 为BasicAgent实现BasicTraits特征。
impl BasicTraits for BasicAgent{
    // 构造函数，用于初始化一个BasicAgent实例。
    fn new(objective: String, position: String) -> Self {
        Self { 
            objective, 
            position, 
            state: AgentState::Discovery, // 初始化状态为Discovery
            memory: Vec::from([]) // 初始化记忆为空
        }
    }

    // 更新代理的状态。
    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }
    // 获取代理的目标。
    fn get_objective(&self) -> &String {
        &self.objective
    }
    // 获取代理的位置或角色。
    fn get_position(&self) -> &String {
        &self.position
    }
    // 获取代理的当前状态。
    fn get_state(&self) -> &AgentState {
        &self.state
    }
    // 获取代理的记忆。
    fn get_memory(&self) -> &Vec<Message> {
        &self.memory
    }

}