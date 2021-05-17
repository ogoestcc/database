use crate::models;

use tokio_pg_mapper::FromTokioPostgresRow;

use tokio_postgres::row::Row;

pub fn user(row: &Row, prefix: Option<&str>) -> models::Users {
    models::Users::from_row_ref_prefixed(row, prefix.unwrap_or("")).unwrap()
}

pub fn user_and_preferences(
    row: &Row,
    user_prefix: Option<&str>,
    content_prefix: Option<&str>,
) -> (models::Users, models::Contents) {
    let user = models::Users::from_row_ref_prefixed(row, user_prefix.unwrap_or("")).unwrap();
    let content = models::Contents::from_row_ref_prefixed(row, content_prefix.unwrap_or("")).unwrap();
    (user, content)
}

#[allow(dead_code)]
pub fn user_and_ratings(
    row: &Row,
    user_prefix: Option<&str>,
    rating_prefix: Option<&str>,
) -> (models::Users, models::Ratings) {
    let user = models::Users::from_row_ref_prefixed(row, user_prefix.unwrap_or("")).unwrap();
    let rating = models::Ratings::from_row_ref_prefixed(row, rating_prefix.unwrap_or("")).unwrap();
    (user, rating)
}

#[allow(dead_code)]
pub fn user_ratings_contents(
    row: &Row,
    user_prefix: Option<&str>,
    rating_prefix: Option<&str>,
    content_prefix: Option<&str>,
) -> (models::Users, models::Ratings, models::Contents) {

    let user = models::Users::from_row_ref_prefixed(row, user_prefix.unwrap_or("")).unwrap();
    let rating = models::Ratings::from_row_ref_prefixed(row, rating_prefix.unwrap_or("")).unwrap();
    let content = models::Contents::from_row_ref_prefixed(row, content_prefix.unwrap_or("")).unwrap();

    (user, rating, content)
}
