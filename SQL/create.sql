-- contents definition

-- Drop table

-- DROP TABLE contents;
 
CREATE TABLE contents (
    id varchar(100) NOT NULL,
    description varchar(255) NULL,
    is_product bool NOT NULL DEFAULT false,
    active bool NOT NULL DEFAULT true,
    CONSTRAINT contents_pk PRIMARY KEY (id)
);


-- users definition

-- Drop table

-- DROP TABLE users;

CREATE TABLE users (
    id serial NOT NULL,
    email varchar(255) NULL,
    "password" varchar(255) NULL,
    active bool NOT NULL DEFAULT true,
    created_at timestamp(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at timestamp(0) NULL DEFAULT NULL::timestamp without time zone,
    CONSTRAINT users_pk PRIMARY KEY (id)
);


-- alerts definition

-- Drop table

-- DROP TABLE alerts;

CREATE TABLE alerts (
    id varchar(20) NOT NULL,
    cvss_score float4 NULL DEFAULT 0.0,
    description text NOT NULL DEFAULT ''::text,
    published_at timestamp(0) NOT NULL,
    updated_at timestamp(0) NOT NULL,
    provider varchar(100) NOT NULL,
    product varchar(100) NOT NULL,
    CONSTRAINT alerts_pk PRIMARY KEY (id),
    CONSTRAINT alerts_fk_product FOREIGN KEY (product) REFERENCES contents(id),
    CONSTRAINT alerts_fk_provider FOREIGN KEY (provider) REFERENCES contents(id)
);


-- ratings definition

-- Drop table

-- DROP TABLE ratings;

CREATE TABLE ratings (
    user_id serial NOT NULL,
    alert_id varchar(20) NOT NULL,
    "like" bool NOT NULL DEFAULT false,
    dislike bool NOT NULL DEFAULT false,
    critical bool NOT NULL DEFAULT false,
    created_at timestamp(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT ratings_pk PRIMARY KEY (user_id, alert_id),
    CONSTRAINT ratings_fk_alert_id FOREIGN KEY (alert_id) REFERENCES alerts(id),
    CONSTRAINT ratings_fk_user_id FOREIGN KEY (user_id) REFERENCES users(id)
);


-- users_contents definition

-- Drop table

-- DROP TABLE users_contents;

CREATE TABLE users_contents (
    user_id serial NOT NULL,
    content_id varchar(50) NOT NULL,
    relevance float4 NOT NULL DEFAULT 1.0,
    CONSTRAINT users_contents_pk PRIMARY KEY (user_id, content_id),
    CONSTRAINT users_contents_fk_content_id FOREIGN KEY (content_id) REFERENCES contents(id),
    CONSTRAINT users_contents_fk_user_id FOREIGN KEY (user_id) REFERENCES users(id)
);


-- alerts_views definition

-- Drop table

-- DROP TABLE alerts_views;

CREATE TABLE alerts_views (
    user_id serial NOT NULL,
    alert_id varchar(20) NOT NULL,
    favorited bool NULL DEFAULT false,
    CONSTRAINT alerts_views_fk FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT alerts_views_fk_1 FOREIGN KEY (alert_id) REFERENCES alerts(id)
);
