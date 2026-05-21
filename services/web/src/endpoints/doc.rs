use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    crate::endpoints::health::liveness,
    crate::endpoints::health::readiness,
    crate::endpoints::user::register_user,
    crate::endpoints::user::find_user_by_id,
    crate::endpoints::user::signin,
    crate::endpoints::user::refresh,
    crate::endpoints::user::sign_out,
    crate::endpoints::user::me,
    crate::endpoints::user::update_me,
    crate::endpoints::user::is_authenticated,
    crate::endpoints::user::signin_provider
))]
pub struct ApiDoc;
