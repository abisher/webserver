mod auth;
mod path;
mod to_do;

use auth::auth_views_factory;
use actix_web::web::ServiceConfig;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
}