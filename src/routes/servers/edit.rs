use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn edit(Extension(_user): Extension<User>, Path(_server_id): Path<u64>) -> Result<()> {
    todo!()
}
