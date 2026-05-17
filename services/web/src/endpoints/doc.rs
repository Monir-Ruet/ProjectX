use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    crate::endpoints::health::liveness,
    crate::endpoints::health::readiness,
    crate::endpoints::user::register_user,
    crate::endpoints::user::find_user_by_id,
    crate::endpoints::user::update_user,
    crate::endpoints::user::delete_user_by_id,
    crate::endpoints::user::signin,
    crate::endpoints::user::refresh,
    crate::endpoints::user::logout,
    crate::endpoints::user::me,
))]
pub struct ApiDoc;
