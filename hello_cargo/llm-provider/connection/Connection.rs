use crate::query::QuerySpecification;
use crate::cursor::CursorSpecification;
use crate::cursor::Cursor;



pub trait Connection {
    fn session_id(&self) -> String;

    fn query<T>(&self, spec: QuerySpecification<T>) -> Vec<T>;

    fn start_transaction(&self);

    fn commit_transaction(&self);

    fn rollback_transaction(&self);

    fn release(&self, err: Option<&dyn std::error::Error>);
} 