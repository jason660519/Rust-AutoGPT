use crate::models::general::llm::Message;
use crate::models::agent_basic::basic_traits::BasicTraits

#[derive(Debug, PartialEq)]
pub enum AgentState{
  Discovery,
  Working,
  UnitTesting,
  Finished
}
#[derive(Debug)]
pub struct BasicAgent{
    pub objective:String,
    pub position: String,
    pub state:AgentState,
    pub momory:Vec<Message>
}
impl BasicTraits for BasicAgent{
    fn new(objective:String, position:String)->Self{
        Self { 
            objective, 
            position, 
            state:AgentState::Discovery,
            memory:Vec::from([])
        }
    }

    fn update_state(&mut self, new_state:AgentState) {
        self.state=new_state;
    }
    fn get_objective(&self)-> &String {
        &self.objective
    }
    fn get_positon(&self)-> &String {
        &self.position
    }
    fn get_state(&self)-> &AgentState {
        &self.state
    }
    fn memory(&self)-> &Vec<Message> {
        &self.memory
    }

}