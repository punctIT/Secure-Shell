use crate::command_system::common::{Command, get_unformated_text};
use std::io::Write;

/// Enumeration of shell operation types for command chaining and flow control.
/// 
/// `Operation` represents the different ways commands can be connected in a shell
/// pipeline, including data flow operations (pipes, redirections) and logical
/// control operations (AND, OR, sequential execution).
/// 
/// # Variants
/// 
/// - `Pipe`: Passes output from one command as input to the next
/// - `InputRedirection`: Redirects file content as input to a command
/// - `OutputRedirection`: Redirects command output to a file
/// - `AndLogic`: Executes next command only if current command succeeds
/// - `OrLogic`: Executes next command only if current command fails
/// - `CommandSeparator`: Sequential execution regardless of success/failure
/// 
/// # Examples
/// 
/// ```rust
/// // Pipe operation: ps aux | grep nginx
/// let pipe_op = Operation::from_str("|");
/// 
/// // Logical AND: mkdir test && cd test
/// let and_op = Operation::from_str("&&");
/// 
/// // Output redirection: ls > output.txt
/// let redirect_op = Operation::from_str(">");
/// ```
enum Operation {
    Pipe,
    InputRedirection,
    OutputRedirection,
    AndLogic,
    OrLogic,
    CommandSeparator,
}

impl Operation {
        /// Converts an operator string to the corresponding `Operation` enum variant.
    /// 
    /// This method maps shell operator strings to their semantic operation types,
    /// enabling type-safe handling of different shell operations.
    /// 
    /// # Parameters
    /// 
    /// - `cmd`: The operator string to parse
    /// 
    /// # Returns
    /// 
    /// The corresponding `Operation` enum variant. Unknown operators default
    /// to `CommandSeparator` for safe fallback behavior.
    /// 
    /// # Operator Mappings
    /// 
    /// | Operator | Operation Type | Behavior |
    /// |----------|----------------|----------|
    /// | `\|` | `Pipe` | Data flow between commands |
    /// | `<` | `InputRedirection` | File input to command |
    /// | `>` | `OutputRedirection` | Command output to file |
    /// | `&&` | `AndLogic` | Conditional execution on success |
    /// | `\|\|` | `OrLogic` | Conditional execution on failure |
    /// | `;` | `CommandSeparator` | Sequential execution |
    /// | `""` (empty) | `CommandSeparator` | End of command chain |
    /// | *other* | `CommandSeparator` | Safe fallback |
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// assert!(matches!(Operation::from_str("|"), Operation::Pipe));
    /// assert!(matches!(Operation::from_str("&&"), Operation::AndLogic));
    /// assert!(matches!(Operation::from_str("||"), Operation::OrLogic));
    /// assert!(matches!(Operation::from_str(">"), Operation::OutputRedirection));
    /// assert!(matches!(Operation::from_str("unknown"), Operation::CommandSeparator));
    /// ```
    fn from_str(cmd: &str) -> Self {
        match cmd {
            "|" => Operation::Pipe,
            "<" => Operation::InputRedirection,
            ">" => Operation::OutputRedirection,
            "&&" => Operation::AndLogic,
            "||" => Operation::OrLogic,
            ";" | "" => Operation::CommandSeparator,
            _ => Operation::CommandSeparator,
        }
    }
}

/// Handles shell operations and manages command output flow in a shell pipeline.
/// 
/// `OperationHandler` processes the results of command execution and determines
/// how to handle shell operations such as pipes, redirections, and logical operators.
/// It manages output accumulation, file operations, and control flow based on
/// command success/failure status.
/// 
/// # Responsibilities
/// 
/// - **Output Management**: Accumulates and formats command outputs
/// - **File Operations**: Handles output redirection to files
/// - **Flow Control**: Manages conditional execution based on success status
/// - **Pipeline Processing**: Coordinates data flow between piped commands
/// - **Error Handling**: Processes operation failures and error states
pub struct OperationHandler {
    output: String,
    last_output: String,
    commands: Vec<Command>,
    current_dir: std::path::PathBuf,
    last_succes: bool,
}
impl OperationHandler {
     /// Creates a new `OperationHandler` instance for processing shell operations.
    /// 
    /// # Parameters
    /// 
    /// - `output`: Current command output to be processed
    /// - `last_output`: Accumulated output from previous commands in the pipeline
    /// - `command`: Slice of commands in the current pipeline segment
    /// - `path`: Current working directory for file operations
    /// - `last_succes`: Success status of the last executed command
    /// 
    /// # Returns
    /// 
    /// A new `OperationHandler` instance ready to process the specified operation.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// // Create handler for output redirection
    /// let commands = vec![
    ///     Command { cmd: vec!["ls".to_string()], op: Some(">".to_string()) },
    ///     Command { cmd: vec!["output.txt".to_string()], op: None }
    /// ];
    /// 
    /// let handler = OperationHandler::new(
    ///     "file1.txt\nfile2.txt".to_string(),
    ///     "".to_string(),
    ///     &commands,
    ///     PathBuf::from("/home/user"),
    ///     true
    /// );
    /// ```
    pub fn new(
        output: String,
        last_output: String,
        command: &[Command],
        path: std::path::PathBuf,
        last_succes: bool,
    ) -> Self {
        OperationHandler {
            output,
            last_output,
            commands: command.to_owned(),
            current_dir: path,
            last_succes,
        }
    }
    /// Writes content to a file, handling directory creation and formatting.
    /// 
    /// This method handles output redirection by writing command output to a specified
    /// file. It automatically creates parent directories if they don't exist and
    /// removes formatting control sequences before writing.
    /// 
    /// # Parameters
    /// 
    /// - `content`: The content to write to the file (may contain formatting)
    /// - `name`: The filename to write to (relative to current directory)
    /// 
    /// # Returns
    /// 
    /// - `Ok(())`: File was written successfully
    /// - `Err(Box<dyn std::error::Error>)`: File operation failed
    /// 
    /// # Behavior
    /// 
    /// 1. **Path Resolution**: Creates full file path relative to current directory
    /// 2. **Directory Creation**: Creates parent directories if they don't exist
    /// 3. **File Creation**: Creates new file or truncates existing file
    /// 4. **Content Processing**: Removes formatting with `get_unformated_text()`
    /// 5. **Writing**: Writes clean content as bytes
    /// 
    /// # File Operations
    /// 
    /// - **Create**: Creates file if it doesn't exist
    /// - **Truncate**: Empties existing file before writing
    /// - **Write**: Writes content with UTF-8 encoding
    /// 
    /// # Examples
    /// 
    /// # Error Conditions
    /// 
    /// - **Permission Denied**: Insufficient permissions to create file or directories
    /// - **Disk Full**: No space available for file creation
    /// - **Invalid Path**: Invalid characters in filename or path
    /// - **I/O Errors**: Hardware or filesystem errors during write operation
    fn write_in_file(
        &self,
        content: &str,
        name: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.current_dir.join(name);
        if !file_path.parent().unwrap().exists() {
            std::fs::create_dir_all(file_path.parent().unwrap())?;
        }
        let mut file = std::fs::File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(file_path)?;
        file.write_all(get_unformated_text(content).as_bytes())?;
        Ok(())
    }

    /// Processes the shell operation and returns the resulting output state.
    /// 
    /// This method is the core operation processor that handles different shell
    /// operations and determines the next state of command execution. It processes
    /// the current operation type and returns information about output handling,
    /// execution flow, and success status.
    /// 
    /// # Returns
    /// 
    /// A tuple containing:
    /// - `String`: Current accumulated output for the pipeline
    /// - `String`: Final output to display (empty for most operations)
    /// - `bool`: Jump flag indicating whether to skip subsequent commands
    /// - `bool`: Success status for the operation
    /// 
    /// # Operation Processing
    /// 
    /// ## Command Separator (`;` or end of chain)
    /// - **Behavior**: Concatenates all outputs for final display
    /// - **Jump**: `false` (continue processing)
    /// - **Success**: `true` (always succeeds)
    /// - **Output**: Combined previous and current output
    /// 
    /// ## Output Redirection (`>`)
    /// - **Behavior**: Writes current output to specified file
    /// - **Jump**: `true` (operation complete)
    /// - **Success**: Based on file write success
    /// - **Output**: Empty (redirected to file)
    /// 
    /// ## Logical AND (`&&`)
    /// - **Behavior**: Continues only if previous command succeeded
    /// - **Jump**: `true` if previous command failed
    /// - **Success**: `false` if jumping, `true` if continuing
    /// - **Output**: Current output if continuing, empty if jumping
    /// 
    /// ## Input Redirection (`<`)
    /// - **Behavior**: Placeholder operation (input handled elsewhere)
    /// - **Jump**: `true` (operation complete)
    /// - **Success**: `true` (always succeeds)
    /// - **Output**: Empty
    /// 
    /// ## Logical OR (`||`)
    /// - **Behavior**: Continues only if previous command failed
    /// - **Jump**: `true` if previous command succeeded
    /// - **Success**: `true` always (OR logic handles failures)
    /// - **Output**: Current output if continuing, empty if jumping
    /// 
    /// ## Pipe (`|`)
    /// - **Behavior**: Accumulates output for next command input
    /// - **Jump**: `false` (continue processing)
    /// - **Success**: `true` (pipe operation succeeds)
    /// - **Output**: Accumulated output for pipeline
    /// 
    /// # Examples
    /// 
    /// ## Sequential Execution
    /// ```rust
    /// // Command: pwd ; ls
    /// let handler = OperationHandler::new(
    ///     "/home/user".to_string(),
    ///     "".to_string(),
    ///     &[Command { cmd: vec!["pwd".to_string()], op: Some(";".to_string()) }],
    ///     current_dir,
    ///     true
    /// );
    /// 
    /// let (current_out, final_out, jump, success) = handler.get_output();
    /// // current_out: ""
    /// // final_out: " /home/user" 
    /// // jump: false
    /// // success: true
    /// ```
    /// 
    /// ## Output Redirection
    /// ```rust
    /// // Command: ls > files.txt
    /// let handler = OperationHandler::new(
    ///     "file1.txt\nfile2.txt".to_string(),
    ///     "".to_string(),
    ///     &[
    ///         Command { cmd: vec!["ls".to_string()], op: Some(">".to_string()) },
    ///         Command { cmd: vec!["files.txt".to_string()], op: None }
    ///     ],
    ///     current_dir,
    ///     true
    /// );
    /// 
    /// let (current_out, final_out, jump, success) = handler.get_output();
    /// // current_out: ""
    /// // final_out: ""
    /// // jump: true
    /// // success: true (if file write succeeds)
    /// ```
    /// 
    /// ## Logical AND Success
    /// ```rust
    /// // Command: mkdir test && cd test (mkdir succeeded)
    /// let handler = OperationHandler::new(
    ///     "".to_string(), // mkdir output
    ///     "".to_string(),
    ///     &[Command { cmd: vec!["mkdir".to_string()], op: Some("&&".to_string()) }],
    ///     current_dir,
    ///     true // mkdir succeeded
    /// );
    /// 
    /// let (current_out, final_out, jump, success) = handler.get_output();
    /// // current_out: " "
    /// // final_out: ""
    /// // jump: false (continue to cd command)
    /// // success: true
    /// ```
    /// 
    /// ## Logical AND Failure
    /// ```rust
    /// // Command: mkdir /root/test && cd test (mkdir failed)
    /// let handler = OperationHandler::new(
    ///     "Permission denied".to_string(),
    ///     "".to_string(),
    ///     &[Command { cmd: vec!["mkdir".to_string()], op: Some("&&".to_string()) }],
    ///     current_dir,
    ///     false // mkdir failed
    /// );
    /// 
    /// let (current_out, final_out, jump, success) = handler.get_output();
    /// // current_out: " "
    /// // final_out: ""
    /// // jump: true (skip cd command)
    /// // success: false
    /// ```
    pub fn get_output(&self) -> (String, String, bool, bool) {
        let op_str = match &self.commands[0].op {
            Some(i) => i.as_str(),
            None => "",
        };
        //dbg!(&op_str);
        let op = Operation::from_str(op_str);
        match op {
            Operation::CommandSeparator => {
                let output = self.last_output.clone() + " " + self.output.as_str();
                ("".to_string(), output, false, true)
            }
            Operation::OutputRedirection => {
                if self
                    .write_in_file(&self.output, &self.commands[1].cmd[0])
                    .is_ok()
                {
                    return ("".to_string(), "".to_string(), true, true);
                }
                ("Error".to_string(), "".to_string(), true, false)
            }
            Operation::AndLogic => {
                let mut jump = false;
                let mut output = self.output.clone();
                let mut succes = true;
                if !self.last_succes {
                    jump = true;
                    output = "".to_string();
                    succes = false;
                }
                (
                    self.last_output.clone() + " " + output.as_str(),
                    "".to_string(),
                    jump,
                    succes,
                )
            }
            Operation::InputRedirection => ("".to_string(), "".to_string(), true, true),
            Operation::OrLogic => {
                let mut jump = false;
                let mut output = self.output.clone();
                let mut succes = true;
                if self.last_succes {
                    jump = true;
                    output = "".to_string();
                    succes = true;
                }
                (
                    self.last_output.clone() + " " + output.as_str(),
                    "".to_string(),
                    jump,
                    succes,
                )
            }
            Operation::Pipe => (
                self.last_output.clone() + " " + self.output.as_str(),
                "".to_string(),
                false,
                true,
            ),
        }
    }
}
