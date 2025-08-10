use std::process::Command;

use crate::actions::{Action, ExecError};

pub struct ExecAction {
    command: String,
    args: Vec<String>,
    pub output_as: String,
}

impl ExecAction {
    pub fn new(command: &str, args: Vec<&str>, output_as: &str) -> Self {
        Self {
            command: command.to_string(),
            args: args.iter().map(|arg| arg.to_string()).collect(),
            output_as: output_as.to_string(),
        }
    }
}

impl Action<()> for ExecAction {
    /// Executes the command in shell and returns the resultant stdout.
    async fn execute(&mut self) -> Result<(), ExecError> {
        let mut command = Command::new("sh");
        command.arg("-c");
        command.arg(&self.command);
        self.args.iter().for_each(|arg| {
            command.arg(arg);
        });
        let output = command
            .output()
            .map_err(|_| ExecError("command execution failed".to_string()))?;
        let result = String::from_utf8(output.stdout)
            .map_err(|_| ExecError("unable to collect stdout".to_string()))?;
        self.output_as = result;
        Ok(())
    }
}