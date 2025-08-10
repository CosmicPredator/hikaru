pub mod exec;
pub mod download;

#[derive(Debug)]
pub struct ExecError(String);

pub trait Action<T> {
    async fn execute(&mut self) -> Result<T, ExecError>;
}
