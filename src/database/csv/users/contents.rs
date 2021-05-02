
use crate::{
    database::{CSVDatabase, Database, Filter, Wherable},
    models::{self, users::UserContents},
};

#[async_trait::async_trait]
impl Database<UserContents> for CSVDatabase {
    async fn get<W>(&self, r#where: W) -> Vec<models::UserContents>
    where
        W: Wherable + Filter<models::UserContents> + Send + Sync,
    {
        let mut data = self.get_data::<models::UserContents, _>(
            r"../.dataset/8Kratings100users500alerts/users.csv",
            |_| true,
        );

        // let mut hash = HashMap::<i64, models::UserContents>::new();

        // for contents in data {
        //     let mut user: Users = Default::default();

        //     if let Some(user_rating) = hash.get_mut(&user.id) {
        //         user_rating.preferences.push(rating);
        //     } else {
        //         hash.insert(
        //             user.id,
        //             models::UserContents {
        //                 user,
        //                 preferences: contents
        //             },
        //         );
        //     }
        // }
        data.iter_mut()
            .filter_map(|user_rating| {
                if r#where.filter(user_rating) {
                    Some(user_rating.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
    // async fn get<W>(&self, r#where: W) -> Vec<UserContents>
    // where
    //     W: Wherable + Filter<Users> + Send + Sync,
    // {
    //     // let client = self.0.get().await.unwrap();

    //     // let rating_where = wherables::Rating {
    //     //     user_id: Some(r#":usr.id"#.to_string()),
    //     //     ..Default::default()
    //     // };

    //     // let select = queler::select::SelectBuilder::new()
    //     //     .from((Users::sql_table(), "usr"))
    //     //     .inner_join((Ratings::sql_table(), "rat"), rating_where.clause())
    //     //     .r#where(r#where.clause())
    //     //     .build();

    //     // log::debug!("{}", select);

    //     // let statement = client.prepare(select.to_string().as_str()).await.unwrap();

    //     // let mut hash = HashMap::<i64, UsersContents>::new();

    //     // for row in &client.query(&statement, &[]).await.unwrap() {
    //     //     let rating = Ratings::from_row_ref(row).unwrap();

    //     //     let user = Users::from_row_ref(row).unwrap();

    //     //     if let Some(user_rating) = hash.get_mut(&user.id) {
    //     //         user_rating.ratings.push(rating);
    //     //     } else {
    //     //         hash.insert(
    //     //             user.id,
    //     //             UsersContents {
    //     //                 user,
    //     //                 ratings: vec![rating],
    //     //             },
    //     //         );
    //     //     }
    //     // }

    //     // let mut ratings = vec![];
    //     // for (_, rating) in hash {
    //     //     ratings.push(rating);
    //     // }
    //     // ratings
    //     todo!()
    // }
}
