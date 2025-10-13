alter table account.access_token
    add expired_at timestamp null after data;

create table account.refresh_token
(
    id            bigint unsigned                       not null
        AUTO_RANDOM
        primary key,
    access_token_id bigint unsigned                     not null,
    user_id       bigint unsigned                       not null,
    platform      varchar(32) default 'wechat'          not null,
    refresh_token varchar(128)                          not null,
    data          json                                  not null,
    expired_at    timestamp                             null,
    created_at    timestamp   default current_timestamp not null,
    updated_at    timestamp   default current_timestamp not null on update current_timestamp,
    constraint uk_account_refresh_token
        unique (refresh_token)
);

