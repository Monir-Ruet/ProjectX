pub fn map_error(error: sqlx::Error) -> tonic::Status {
    match error {
        sqlx::Error::Database(db_err) => {
            if db_err.is_unique_violation() {
                tonic::Status::already_exists("already exists")
            } else if db_err.is_check_violation() {
                tonic::Status::invalid_argument("pre check failed")
            } else if db_err.is_foreign_key_violation() {
                tonic::Status::invalid_argument("foreign key violation")
            } else {
                tonic::Status::internal("failed to insert")
            }
        }
        _ => tonic::Status::internal("failed to create user"),
    }
}
