use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    crate::endpoints::health::liveness,
    crate::endpoints::health::readiness,

    //user
    crate::endpoints::user::register_user,
    crate::endpoints::user::find_user_by_id,
    crate::endpoints::user::find_user_by_email,
    crate::endpoints::user::signin,
    crate::endpoints::user::refresh,
    crate::endpoints::user::sign_out,
    crate::endpoints::user::me,
    crate::endpoints::user::update_me,
    crate::endpoints::user::is_authenticated,
    crate::endpoints::user::signin_provider,

    // passkey
    crate::endpoints::passkey::start_register,
    crate::endpoints::passkey::finish_register,
    crate::endpoints::passkey::start_authentication,
    crate::endpoints::passkey::finish_authentication,
))]

pub struct ApiDoc;
