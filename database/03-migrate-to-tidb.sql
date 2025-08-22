create table account.access_token
(
    id           bigint unsigned                       not null AUTO_RANDOM
        primary key,
    user_id      bigint unsigned                       not null,
    platform     varchar(32) default 'wechat'          not null,
    access_token varchar(128)                          not null,
    data         json                                  null,
    created_at   timestamp   default CURRENT_TIMESTAMP not null,
    updated_at   timestamp   default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP,
    constraint uk_account_access_token
        unique (access_token)
);

create table account.third_user
(
    id         bigint unsigned                           not null AUTO_RANDOM
        primary key,
    user_id    bigint unsigned default 0                 not null,
    platform   varchar(32)                               not null comment '平台',
    third_id   varchar(128)                              not null comment '平台对应用户标识',
    config     json                                      null,
    created_at timestamp       default CURRENT_TIMESTAMP not null,
    updated_at timestamp       default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP
);

create index idx_account_platform_third_id
    on account.third_user (platform, third_id);

create index idx_account_user_id
    on account.third_user (user_id);

create table account.user
(
    id         bigint unsigned                     not null AUTO_RANDOM
        primary key,
    phone      varchar(18)                         null,
    config     json                                null,
    created_at timestamp default CURRENT_TIMESTAMP not null,
    updated_at timestamp default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP
);

create index idx_account_phone
    on account.user (phone);

create table tool.short_url
(
    id    bigint unsigned           not null AUTO_RANDOM
        primary key,
    short varchar(64)               not null,
    url   longtext                  null,
    visit bigint unsigned default 0 not null,
    constraint uk_tool_short
        unique (short)
);

create table tool.totp
(
    id         bigint unsigned                     not null AUTO_RANDOM
        primary key,
    user_id    bigint unsigned                     not null,
    username   varchar(128)                        not null,
    issuer     varchar(128)                        null,
    config     json                                not null,
    created_at timestamp default CURRENT_TIMESTAMP not null,
    updated_at timestamp default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP
);

create index idx_tool_user_id
    on tool.totp (user_id);

