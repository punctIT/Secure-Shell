use crate::command_system::command_runner::RunCommand;
use crate::command_system::common::Command;
use crate::command_system::common::get_commands;
use crate::command_system::operation_handler::OperationHandler;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A command handler for processing and executing shell-like commands with support for pipes,
/// redirections, and logical operators.
/// 
/// `CommandHandler` parses client input into a series of commands and executes them
/// sequentially while handling various shell operations such as:
/// - Input redirection (`<`)
/// - Output redirection (`>`)
/// - Pipes (`|`)
/// - Logical OR (`||`)
/// - Logical AND (`&&`)
/// 
/// The handler maintains the current working directory state and ensures all operations
/// are contained within a specified root directory for security.
pub struct CommandHandler {
    cmds: Vec<Command>,
    root: PathBuf,
    current_dir: PathBuf,
    users_list: Arc<RwLock<Vec<String>>>,
}
impl CommandHandler {
    /// Creates a new `CommandHandler` instance with the specified configuration.
    /// 
    /// # Parameters
    /// 
    /// - `client_input`: Raw command string from the client (e.g., "ls -la | grep txt")
    /// - `path`: Root directory path for sandboxing operations
    /// - `current_dir`: Current working directory for command execution
    /// - `users`: Thread-safe list of active users for session management
    /// 
    /// # Returns
    /// 
    /// A new `CommandHandler` instance ready to process commands.
    /// 
    /// # Behavior
    /// 
    /// - Parses the client input into individual commands using `get_commands()`
    /// - Canonicalizes paths when possible, falling back to original paths on error
    /// - Stores user list reference for multi-user command support
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let handler = CommandHandler::new(
    ///     "cd /tmp && ls".to_string(),
    ///     PathBuf::from("/home/secure"),
    ///     PathBuf::from("/home/secure/user"),
    ///     Arc::new(RwLock::new(vec!["alice".to_string()]))
    /// );
    /// ```
    pub fn new(
        client_input: String,
        path: PathBuf,
        current_dir: PathBuf,
        users: Arc<RwLock<Vec<String>>>,
    ) -> Self {
        CommandHandler {
            cmds: get_commands(client_input),
            root: std::fs::canonicalize(&path).unwrap_or(path),
            current_dir: std::fs::canonicalize(&current_dir).unwrap_or(current_dir),
            users_list: users,
        }
    }
    /// Executes all parsed commands sequentially with support for shell operations.
    /// 
    /// This method processes the command chain, handling various shell operators and
    /// maintaining state between commands. It supports:
    /// 
    /// - **Input redirection (`<`)**: Reads file content as input for the next command
    /// - **Output redirection (`>`)**: Redirects command output to a file
    /// - **Pipes (`|`)**: Passes output from one command as input to the next
    /// - **Logical operators (`||`, `&&`)**: Conditional command execution
    /// 
    /// # Returns
    /// 
    /// - `Some(String)`: Combined output from all executed commands
    /// - `None`: If no output was generated or an error occurred
    /// 
    /// # Command Flow
    /// 
    /// 1. **Input Processing**: Check for input redirection and read file content
    /// 2. **Command Execution**: Run individual commands via `RunCommand`
    /// 3. **Operation Handling**: Process shell operators via `OperationHandler`
    /// 4. **State Management**: Update current directory and success status
    /// 5. **Output Accumulation**: Combine results based on operators
    /// 
    /// # Special Behaviors
    /// 
    /// - **Pipe operations**: Command output becomes input for the next command
    /// - **Jump commands**: Some operations may skip subsequent commands
    /// - **Success tracking**: Logical operators depend on previous command success
    /// - **Directory changes**: `cd` commands update the current working directory
    /// 
    async fn run_commands(&mut self) -> Option<String> {
        let mut final_result: Option<String> = None;
        let mut jump_cmd = false;

        let mut last_succes = true;

        let mut current_dir = self.current_dir.clone();
        let _ = current_dir;
        let mut cmd_succes = true;
        let mut result: Option<String> = None;

        let mut cmds_output = String::from("");
        let mut input: Option<String> = None;
        for (i, cmd) in self.cmds.clone().iter().enumerate() {
            let op = cmd.op.clone().unwrap_or("".to_string());
            if op == "<" {
                if self.cmds[i].cmd.is_empty() {
                    return None;
                }
                input = Some(
                    std::fs::read_to_string(self.current_dir.join(&self.cmds[i + 1].cmd[0]))
                        .unwrap_or("".to_string()),
                );
            }
            if op == ">" && self.cmds[i].cmd.is_empty() {
                return None;
            }
            if !jump_cmd {
                let mut runner = RunCommand::new(
                    self.current_dir.clone(),
                    self.root.clone(),
                    cmd,
                    input.clone(),
                    self.users_list.clone(),
                );
                (current_dir, result, cmd_succes) = runner.test().await;
                self.current_dir = current_dir; //cd 
            }
            let sliced_cmds: Vec<Command> = self.cmds[i..].to_vec();
            let operation = OperationHandler::new(
                result.clone().unwrap_or("".to_string()),
                cmds_output,
                &sliced_cmds,
                self.current_dir.clone(),
                last_succes & cmd_succes,
            );
            let (mut current_output, output, jump, op_succes) = operation.get_output();
            jump_cmd = jump;
            if op == "|" {
                input = result.clone();
                current_output = String::from("");
            }
            if op == ">" && jump_cmd {
                result = None;
            }
            if op == "||" && !jump_cmd {
                final_result = None;
                current_output = String::from("");
            }

            cmds_output = current_output;
            last_succes = op_succes & cmd_succes;
            //dbg!(&input);
            if !output.is_empty() {
                final_result = Some(final_result.unwrap_or("".to_string()) + output.as_str());
            }
            //dbg!(&cmds_output);
            //dbg!(&final_result);
        }
        final_result
    }
     /// Executes the parsed commands and returns formatted output with current directory.
    /// 
    /// This method serves as the main entry point for command execution. It:
    /// 1. Runs all parsed commands via `run_commands()`
    /// 2. Ensures the current directory stays within the root boundary
    /// 3. Formats the output with directory information
    /// 4. Returns both the formatted output and updated current directory
    /// 
    /// # Returns
    /// 
    /// A tuple containing:
    /// - `String`: Formatted command output with directory path in the format: `"output[-]:path[-]\r\n\r\n"`
    /// - `PathBuf`: Updated current working directory after command execution
    /// 
    /// # Security
    /// 
    /// The method enforces directory sandboxing by:
    /// - Checking if the current directory is within the root boundary
    /// - Resetting to root directory if a path traversal attempt is detected
    /// - Displaying relative paths from the root directory
    /// 
    /// # Output Format
    /// 
    /// The returned string follows the format:
    /// ```text
    /// command_output[-]:relative/path/from/root[-]\r\n\r\n
    /// ```
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let mut handler = CommandHandler::new(
    ///     "pwd".to_string(),
    ///     PathBuf::from("/home/secure"),
    ///     PathBuf::from("/home/secure/user"),
    ///     users_list
    /// );
    /// 
    /// let (output, new_dir) = handler.get_output().await;
    pub async fn get_output(&mut self) -> (String, PathBuf) {
        //dbg!(&self.cmds);
        let output = self.run_commands().await.unwrap_or("".to_string());

        let current_dir = match self.current_dir.strip_prefix(&self.root) {
            Ok(path) => path,
            Err(_) => {
                self.current_dir = self.root.clone();
                std::path::Path::new("")
            }
        };
        let reply = format!("{}[-]:{}[-]\r\n\r\n", output, current_dir.display());
        //dbg!(&reply);
        (reply, self.current_dir.clone())
    }
}
