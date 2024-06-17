// 这段代码的核心是提供了一个代表解决方案架构师的代理（Agent），它异步执行任务，比如获取项目范围和确定项目需要访问的外部网址(URLs)。
// 这代理通过循环不同的状态（如“发现”和“单元测试”），根据项目描述和外部网址的检测情况来更新项目事实表(FactSheet)。
// 通过这样的结构，它演示了一个能够在网站开发中收集和处理必要信息的自动化流程。

// 引入所需的crate庫和模組
use crate::ai_functions::aifunc_architect::{print_project_scope, print_site_urls};
use crate::helpers::command_line::PrintCommand;
use crate::helpers::general::{ai_task_request_decoded, check_status_code};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;


// Solutions Architect  定義解決方案架構師結構體
#[derive(Debug)]
pub struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    // 構造器，初始化解決方案架構師的屬性
    pub fn new() -> Self {
        let attributes: BasicAgent = BasicAgent {
            objective: "Gathers information and design solutions for website development".to_string(),
            position: "Solutions Architect".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self { attributes }
    }

    // Retrieve Project Scope 獲取项目范围的异步方法
    async fn call_project_scope(&mut self, factsheet: &mut FactSheet) -> ProjectScope {
        let msg_context: String = format!("{}", factsheet.project_description);

        let ai_response: ProjectScope = ai_task_request_decoded::<ProjectScope>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_project_scope),
            print_project_scope,
        )
        .await;

        factsheet.project_scope = Some(ai_response.clone());
        self.attributes.update_state(AgentState::Finished);
        return ai_response;
    }

    // 检索项目中的外部URL的异步方法
    async fn call_determine_external_urls(&mut self,factsheet: &mut FactSheet,msg_context: String,) {
        let ai_response: Vec<String> = ai_task_request_decoded::<Vec<String>>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls,
        )
        .await;

        factsheet.external_urls = Some(ai_response);
        self.attributes.state = AgentState::UnitTesting;
    }
}




// 为`AgentSolutionArchitect`实现`SpecialFunctions`特质（Trait）
#[async_trait]
impl SpecialFunctions for AgentSolutionArchitect {

    // 获取代理的属性
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    // 执行函数定义
    async fn execute(&mut self,factsheet: &mut FactSheet,) -> Result<(), Box<dyn std::error::Error>> {

        // !!! WARNING - BE CAREFUL OF INFINITATE LOOPS !!!
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    let project_scope: ProjectScope = self.call_project_scope(factsheet).await;

                    // Confirm if external urls  判断是否需要外部URLs
                    if project_scope.is_external_urls_required {
                        self.call_determine_external_urls(factsheet, factsheet.project_description.clone(),).await;
                        self.attributes.state = AgentState::UnitTesting;
                    }
                }

                AgentState::UnitTesting => {
                    let mut exclude_urls: Vec<String> = vec![];

                    let client: Client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .unwrap();

                    // Defining urls to check  定义要检查的urls
                    let urls: &Vec<String> = factsheet
                        .external_urls.as_ref().expect("No URL object on factsheet");

                    // Find faulty urls  查找故障的urls
                    for url in urls {
                        let endpoint_str: String = format!("Testing URL Endpoint: {}", url);
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),endpoint_str.as_str(),);

                        // Perform URL Test  执行URL测试
                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    exclude_urls.push(url.clone())
                                }
                            }
                            Err(e) => println!("Error checking {}: {}", url, e),
                        }
                    }

                    // Exclude any faulty urls 排除任何故障的urls
                    if exclude_urls.len() > 0 {
                        let new_urls: Vec<String> = factsheet
                            .external_urls
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter(|url| !exclude_urls.contains(&url))
                            .cloned()
                            .collect();
                        factsheet.external_urls = Some(new_urls);
                    }

                    // Confirm done 确认任务完成
                    self.attributes.state = AgentState::Finished;
                }

                // Default to Finished state 默认设置状态为完成
                _ => {
                    self.attributes.state = AgentState::Finished;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 异步测试Solutions Architect的功能
    #[tokio::test]
    async fn tests_solution_architect() {
        let mut agent: AgentSolutionArchitect = AgentSolutionArchitect::new();

        let mut factsheet: FactSheet = FactSheet {
      project_description: "Build a full stack website with user login and logout that shows latest Forex prices".to_string(),
      project_scope: None,
      external_urls: None,
      backend_code: None,
      api_endpoint_schema: None,
    };

        agent.execute(&mut factsheet).await.expect("Unable to execute Solutions Architect Agent");
        assert!(factsheet.project_scope != None);
        assert!(factsheet.external_urls.is_some());

        dbg!(factsheet);
    }
}


// Q&A: 代理在异步执行任务时是如何更新项目事实表的？
// 代理在异步执行任务时，通过调用特定的异步方法来更新项目事实表（FactSheet）。下面我会解释代理如何进行这一操作：
// 初始化代理和项目事实表：首先，代理(AgentSolutionArchitect)被初始化，并且一个空的项目事实表(FactSheet)也被创建。
// 这个事实表用于存储与项目相关的各种信息，如项目范围、需要检索的外部URLs等。
// 异步获取项目范围：代理通过调用call_project_scope方法，异步发送请求以获取项目的范围信息。
// 在这个方法里，代理使用项目描述（从FactSheet中提取）作为参数，向一个假设的AI任务请求函数ai_task_request_decoded发送请求。
// 请求的响应被解析为ProjectScope类型，并存储回项目事实表中的project_scope字段里。这个过程异步发生，不会阻塞代理的其他操作。
// 异步检索外部URLs：如果项目范围表明需要外部URLs，代理接着调用call_determine_external_urls方法。
// 同样，这个方法异步工作，使用项目描述作为参数来请求外部URLs的信息。
// 收到的URLs列表被更新到项目事实表中的external_urls字段。
// 更新代理状态和错误处理：每完成一个任务，代理的状态都会更新（比如从Discovery到UnitTesting，最终到Finished）。
// 如果在任何异步任务中发生错误（如网络请求失败），则会通过Rust的错误处理机制来解决。
// 单元测试外部URLs：在UnitTesting状态下，代理会检查记录在事实表中的外部URLs，确认这些URLs是否工作正常。
// 这个过程中，可能发现的无效URLs会从列表中移除，确保项目事实表中只保留有效的外部链接。
// 这个过程展示了代理如何通过对FactSheet的不断更新来异步执行任务，从而实现项目信息的收集和处理。
// 每个异步方法的调用都基于当前项目事实表的状态，确保了信息的准确性和及时性。