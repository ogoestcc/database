use std::{collections::HashMap, sync::Arc};

use models::wherables::AlertRatings;

use crate::{
    models::{self, wherables},
    services::{traits, types::alerts::ratings as service},
};

pub async fn get<DB: traits::Alerts>(
    db_connection: Arc<DB>,
    request: service::Request,
) -> service::Response {
    log::debug!("Request {:?}", request);

    let alert_where =
        request
            .alert_where
            .clone()
            .map_or(Default::default(), |w| wherables::Alert {
                id: w.id.clone(),
                content: w.content.clone(),
            });

    let rating_where = request
        .rating_where
        .clone()
        .map_or(Default::default(), |w| wherables::Rating {
            user_id: None,
            alert_id: w.alert_id,
            like: w.like,
            dislike: w.dislike,
            critical: w.critical,
        });

    let r#where = AlertRatings::from((alert_where, rating_where));

    let alerts: Vec<models::AlertRatings> = db_connection.get(r#where).await;
    let alerts: Vec<service::AlertsRatings> = alerts
        .iter()
        .map(|alert| service::AlertsRatings {
            alert: From::from(&alert.alert),
            ratings: alert.ratings.iter().map(From::from).collect(),
        })
        .collect();

    service::Response {
        metadata: service::Metadata {
            total: alerts.len() as u64,
            alert_where: request.alert_where,
            rating_where: request.rating_where,
        },
        alerts,
    }
}
