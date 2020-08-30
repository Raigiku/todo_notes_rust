use crate::common::TodoNotesError;
use async_trait::async_trait;

#[async_trait]
pub trait Query<I, O> {
    async fn execute(&self, input: I) -> Result<O, TodoNotesError>;
}
