# 文件名称与主要功能说明

| 文件名称 | 主要功能 |
| -------- | -------- |
| **src/ai_fuctions/aifunc_architect.rs** | 负责定义与项目架构相关的AI功能，如项目范围分析和外部API端点识别。 |
| **src/ai_fuctions/aifunc_backend.rs** | 定义与后端开发相关的AI功能，包括生成、改进和修复后端代码，以及提取REST API端点。 |
| **src/ai_fuctions/aifunc_managing.rs** | 定义管理用户输入和生成项目目标的AI功能。 |
| **src/ai_fuctions/mod.rs** | 作为ai_fuctions目录的模块初始化文件，导入其他模块。 |
| **src/apis/call_request.rs** | 处理调用外部API的请求，特别是与大型语言模型（如GPT-4）的交互请求。 |
| **src/apis/mod.rs** | 作为apis目录的模块初始化文件，导入其他模块。 |
| **src/helplers/command_line.rs** | 提供在命令行中打印消息和获取用户输入的功能。 |
| **src/helplers/general.rs** | 提供通用的帮助函数，包括读取和保存文件内容、检查URL状态等。 |
| **src/helplers/mod.rs** | 作为helpers目录的模块初始化文件，导入其他模块。 |
| **src/models/agent_basic/basic_agents.rs** | 定义基本代理（Agent）的数据结构和实现方法。 |
| **src/models/agent_basic/basic_traits.rs** | 定义代理相关的基本特质（Traits）。 |
| **src/models/agent_basic/mod.rs** | 作为agent_basic目录的模块初始化文件，导入其他模块。 |
| **src/models/agents/agent_architect.rs** | 定义解决方案架构师（Solutions Architect）代理及其相关功能，用于收集和设计网站开发解决方案。 |
| **src/models/agents/agent_backend.rs** | 定义后端开发人员（Backend Developer）代理及其相关功能，用于开发和测试后端代码。 |
| **src/models/agents/agent_traits.rs** | 定义代理的方法和行为特质（Traits）。 |
| **src/models/agents/mod.rs** | 作为agents目录的模块初始化文件，导入其他模块。 |
| **src/models/agents_manager/managing_agent.rs** | 定义管理代理及其相关功能，用于协调不同代理的工作。 |
| **src/models/agents_manager/mod.rs** | 作为agents_manager目录的模块初始化文件，导入其他模块。 |
| **src/models/general/llm.rs** | 定义与大型语言模型（LLM）交互相关的数据结构。 |
| **src/models/general/mod.rs** | 作为general目录的模块初始化文件，导入其他模块。 |
| **src/models/mod.rs** | 作为models目录的模块初始化文件，导入其它模块。 |
| **src/main.rs** | 主程序入口文件，包含宏定义和主函数。 |
| **Auto_GIPPTY/.env** | 环境变量配置文件，存储API密钥等敏感信息。 |
| **Auto_GIPPTY/argo.toml** | 项目配置文件，定义依赖包和项目元数据。 |