use utoipa::OpenApi;

mod post_docs;
mod user_docs;
mod settings_docs;
mod comment_docs;
mod profiles_docs;
mod reporting_docs;
mod session_docs;
mod userassets_docs;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Posts endpoints
        post_docs::search_posts,
        post_docs::create_post,
        post_docs::edit_post,
        post_docs::get_post_by_id,
        post_docs::delete_post,
        post_docs::get_latest_posts,
        post_docs::get_popular_posts,
        post_docs::get_premium_posts,
        post_docs::get_random_posts,
        post_docs::get_post,
        post_docs::like_post,
        post_docs::dislike_post,
        post_docs::get_a_random_post,

        // User endpoints
        user_docs::get_user_posts,
        user_docs::patch_user_settings,
        user_docs::get_user_settings,
        user_docs::verify_user_email,
        user_docs::request_password_reset,
        user_docs::reset_password,

        // Comment endpoints
        comment_docs::get_comments,
        comment_docs::post_comment,
        comment_docs::like_comment,
        comment_docs::dislike_comment,
        comment_docs::reply_to_comment,
        comment_docs::get_replies,
        comment_docs::like_reply,
        comment_docs::dislike_reply,

        // Settings endpoints
        settings_docs::change_password,

        // profiles
        profiles_docs::get_lookup_user,
        profiles_docs::get_public_user,
        profiles_docs::get_private_user,
        profiles_docs::patch_profile,
        profiles_docs::follow_profile,
        profiles_docs::get_quicklookup,

        // Reporting endpoints
        reporting_docs::create_report,

        // Session endpoints
        session_docs::validate_docs,
        session_docs::register_docs,
        session_docs::login_docs,
        session_docs::logout_docs,

        // User asset endpoints
        userassets_docs::upload_profile_picture,
        userassets_docs::upload_banner
    ),
    components(
        schemas(
            // Posts
            crate::routes::internal::posts::types::UserPatchPost,

            // Users
            crate::models::settings::UserSettings,
            crate::models::profile::DBProfile,
            crate::models::user::User,

            // Comments
            crate::models::comment::Comment,
            crate::models::comment::CommentReplies,
            crate::models::comment::Reply,

            // Reports
            crate::models::report::Report,
            crate::models::report::ReportType,
            crate::models::report::ReportStatus,

            // Sessions
            crate::routes::internal::session::AuthResponse
        )
    ),
    tags(
        (name = "Posts", description = "All post-related endpoints"),
        (name = "Users", description = "All user-related endpoints"),
        (name = "Settings", description = "All settings-related endpoints"),
        (name = "Comments", description = "All comment-related endpoints"),
        (name = "Profile", description = "All profiles-related endpoints"),
        (name = "Reporting", description = "All reporting-related endpoints"),
        (name = "Session", description = "All session-related endpoints"),
        (name = "User Assets", description = "All userasset-related endpoints")
    )
)]
pub struct ApiDoc;
