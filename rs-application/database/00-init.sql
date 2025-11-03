create table miniprogram.user
(
    id         bigserial
        constraint pk_user_id
            primary key,
    open_id    varchar(128)                           not null,
    avatar     text,
    nickname   varchar(128)             default '微信用户'::character varying,
    slogan     varchar(246),
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null
);

create unique index uk_user_open_id
    on miniprogram.user (open_id);

create table miniprogram.totp
(
    id         bigserial
        constraint pk_totp_id
            primary key,
    user_id    bigint                                 not null,
    username   varchar(128)                           not null,
    issuer     varchar(128),
    secret     varchar(256)                           not null,
    config      jsonb                                          ,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null
);

create index idx_totp_user
    on miniprogram.totp (user_id);

create table miniprogram.short_url
(
    id         bigserial
        constraint pk_short_url_id
            primary key,
    short      varchar(64)                            not null
        constraint uk_short_url_short
            unique,
    url        text,
    visit      bigint                   default 0     not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null
);

create table miniprogram.access_token
(
    id           bigserial
        constraint pk_token_id
            primary key,
    user_id      bigint                                 not null,
    access_token varchar(128)                           not null
        constraint uk_token_token
            unique,
    data         jsonb                                  not null,
    created_at   timestamp with time zone default now() not null,
    updated_at   timestamp with time zone default now() not null
);
