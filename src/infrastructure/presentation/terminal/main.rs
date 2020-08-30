use crate::{
    common::TodoNotesError,
    queries::{GetTodoByIdInput, GetTodoByIdOutput, Query},
};

pub async fn main<T: Query<GetTodoByIdInput, Option<GetTodoByIdOutput>>>(
    get_todo_by_id: T,
) -> Result<(), TodoNotesError> {
    let input = GetTodoByIdInput::new("550eccfb-db22-4c09-9af0-d1941961b8ce".to_string())?;
    let output = get_todo_by_id.execute(input).await?;
    println!("{:?}", output);
    Ok(())
}
