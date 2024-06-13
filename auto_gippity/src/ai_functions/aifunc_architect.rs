use ai_functions::ai_function;

#[ai_function]
// 这一段的用意在于为项目的开发提供一个高层次的需求概述，帮助开发者或自动化工具理解并决定需要实现哪些核心功能模块。
// 通过对项目描述的分析，这个函数能够提供一个包含有关项目所需关键功能的信息的对象，这有助于项目规划、工作分配和功能实现的初步阶段。
pub fn print_project_scope(_project_description: &str) {
    /// Input: Takes in a user request to build a website project description
    /// Function: Converts user request into JSON response of information items required for a website build.
    /// Important: At least one of the bool results must be true
    /// Output: Prints an object response in the following format:
    ///   {
    ///     "is_crud_required": bool, // true if site needs CRUD functionality
    ///     "is_user_login_and_logout": bool // true if site needs users to be able to log in and log out
    ///     "is_external_urls_required": bool // true if site needs to fetch data from third part providers
    ///   }
    /// Example 1:
    ///   user_request = "I need a full stack website that accepts users and gets stock price data"
    ///   prints:
    ///   {
    ///     "is_crud_required": true
    ///     "is_user_login_and_logout": true
    ///     "is_external_urls_required": bool true
    ///   }
    /// Example 2:
    ///   user_request = "I need a simple TODO app"
    ///   prints:
    ///   {
    ///     "is_crud_required": true
    ///     "is_user_login_and_logout": false
    ///     "is_external_urls_required": bool false
    ///   }
    println!(OUTPUT)
}

#[ai_function]
// 这个函数重要的用途在于自动化地辅助开发团队快速确定和集成必要的外部数据源，特别是在早期项目规刚开始时。
// 这有助于加速关键功能的原型制作，并确保开发过程中考虑到所有必要的外部服务。
// 通过自动识别和提供相关 API 端点，它能够降低手动搜索和验证这些信息的工作量。
// 需要注意的是，这个函数及相关描述是理论上的，真实实现可能需要对项目描述进行详细的解析，
// 并与已知的 API 资源进行匹配，这通常涉及到更复杂的逻辑和可能的配置数据。
pub fn print_site_urls(_project_description: &str) {
    /// Input: Takes in a project description of a website build
    /// Function: Outputs a list of external public API endpoints that should be used in the building of the website
    /// Important: Only selects url endpoint(s) which do not require any API Keys at all
    /// Output: Prints a list response of external urls in the following format:
    /// ["url1", "url2", "url3", ...]
    /// Example:
    ///   website_team_spec = "website_purpose: Some("\"Provides Crypto Price Data from Binance and Kraken\"",)"
    ///   prints:
    /// ["https://api.binance.com/api/v3/exchangeInfo", "https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1d"]
    println!(OUTPUT)
}
