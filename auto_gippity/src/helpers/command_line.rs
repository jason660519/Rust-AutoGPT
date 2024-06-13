use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout, Write};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent:{}", agent_pos);
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);
        stdout.execute(ResetColor).unwrap(); // Reset Color
    }
}

// Get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout = stdout();
    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    print!("{}", question);
    // Ensure the printed text is actually shown
    stdout.flush().unwrap();

    // Reset Color
    stdout.execute(ResetColor).unwrap();

    // Read User input
    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    // Trim whitespace and return
    user_response.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_prints_agent_msg() {
        PrintCommand::AICall
            .print_agent_message("Managing Agent", "Testing testing,processing something");
    }
}
