pub mod alerts;
pub mod ratings;
pub mod users;

pub use alerts::Alerts;
pub use ratings::Ratings;
pub use users::Users;
pub use users::UserRatings;

pub mod wherables;

// pub mod wherables {

//     pub use super::{
//         alerts::AlertWhere as Alert,
//         ratings::RatingWhere as Rating,
//         users::wherables::{UserRatings, UserWhere as User},
//     };
// }
