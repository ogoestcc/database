use sea_query::Iden;

pub trait Table: Sized {
    fn select() -> &'static [Self];
    fn select_table() -> &'static [(Self, Self)];
}

#[derive(Iden, Clone)]
#[iden = "alerts"] // table name
pub enum Alerts {
    Table,
    Id,
    CvssScore,
    Description,
    PublishedAt,
    UpdatedAt,
    Provider,
    Product,
}

impl Table for Alerts {
    fn select() -> &'static [Self] {
        &[
            Self::Id,
            Self::CvssScore,
            Self::Description,
            Self::PublishedAt,
            Self::UpdatedAt,
            Self::Provider,
            Self::Product,
        ]
    }

    fn select_table() -> &'static [(Self, Self)] {
        &[
            (Self::Table, Self::Id),
            (Self::Table, Self::CvssScore),
            (Self::Table, Self::Description),
            (Self::Table, Self::PublishedAt),
            (Self::Table, Self::UpdatedAt),
            (Self::Table, Self::Provider),
            (Self::Table, Self::Product),
        ]
    }
}

#[derive(Iden, Clone)]
#[iden = "alerts_views"]
pub enum AlertsViews {
    Table,
    UserId,
    AlertId,
    Favorited,
}

impl Table for AlertsViews {
    fn select() -> &'static [Self] {
        &[Self::UserId, Self::AlertId, Self::Favorited]
    }

    fn select_table() -> &'static [(Self, Self)] {
        &[
            (Self::Table, Self::UserId),
            (Self::Table, Self::AlertId),
            (Self::Table, Self::Favorited),
        ]
    }
}

#[derive(Iden, Clone)]
#[iden = "users"]
pub enum Users {
    Table,
    Id,
    Email,
    Password,
    Active,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

impl Table for Users {
    fn select() -> &'static [Self] {
        &[
            Self::Id,
            Self::Email,
            Self::Password,
            Self::Active,
            Self::CreatedAt,
            Self::UpdatedAt,
            Self::DeletedAt,
        ]
    }

    fn select_table() -> &'static [(Self, Self)] {
        &[
            (Self::Table, Self::Id),
            (Self::Table, Self::Email),
            (Self::Table, Self::Password),
            (Self::Table, Self::Active),
            (Self::Table, Self::CreatedAt),
            (Self::Table, Self::UpdatedAt),
            (Self::Table, Self::DeletedAt),
        ]
    }
}

impl Users {
    pub fn select_without_password() -> &'static [(Self, Self)] {
        &[
            (Self::Table, Self::Id),
            (Self::Table, Self::Email),
            (Self::Table, Self::Active),
            (Self::Table, Self::CreatedAt),
            (Self::Table, Self::UpdatedAt),
            (Self::Table, Self::DeletedAt),
        ]
    }
}

#[derive(Iden, Clone)]
#[iden = "users_contents"]
pub enum UsersContents {
    Table,
    UserId,
    ContentId,
    Relevance,
}

impl Table for UsersContents {
    fn select() -> &'static [Self] {
        &[Self::UserId, Self::ContentId, Self::Relevance]
    }

    fn select_table() -> &'static [(Self, Self)] {
        &[
            (Self::Table, Self::UserId),
            (Self::Table, Self::ContentId),
            (Self::Table, Self::Relevance),
        ]
    }
}

#[derive(Iden, Clone)]
#[iden = "contents"]
pub enum Contents {
    Table,
    Id,
    Description,
    IsProduct,
    Active,
}

impl Table for Contents {
    fn select() -> &'static [Self] {
        &[Self::Id, Self::Description, Self::IsProduct, Self::Active]
    }

    fn select_table() -> &'static [(Self, Self)] {
        &[
            (Self::Table, Self::Id),
            (Self::Table, Self::Description),
            (Self::Table, Self::IsProduct),
            (Self::Table, Self::Active),
        ]
    }
}

#[derive(Iden, Clone)]
#[iden = "ratings"]
pub enum Ratings {
    Table,
    UserId,
    AlertId,
    Like,
    Dislike,
    Critical,
    CreatedAt,
}

impl Table for Ratings {
    fn select() -> &'static [Self] {
        &[
            Self::UserId,
            Self::AlertId,
            Self::Like,
            Self::Dislike,
            Self::Critical,
            Self::CreatedAt,
        ]
    }

    fn select_table() -> &'static [(Self, Self)] {
        &[
            (Self::Table, Self::UserId),
            (Self::Table, Self::AlertId),
            (Self::Table, Self::Like),
            (Self::Table, Self::Dislike),
            (Self::Table, Self::Critical),
            (Self::Table, Self::CreatedAt),
        ]
    }
}
