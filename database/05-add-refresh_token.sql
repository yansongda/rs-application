alter table account.access_token
    add expired_at timestamp null after data;
alter table account.access_token
    add third_id varchar(128) not null after platform;
update account.access_token set third_id = 'wx36601dc74412c674'


create table account.refresh_token
(
    id            bigint unsigned                       not null
        AUTO_RANDOM
        primary key,
    access_token_id bigint unsigned                     not null,
    refresh_token varchar(128)                          not null,
    expired_at    timestamp                             null,
    created_at    timestamp   default current_timestamp not null,
    updated_at    timestamp   default current_timestamp not null on update current_timestamp,
    constraint uk_account_access_token_id
            unique (access_token_id),
    constraint uk_account_refresh_token
        unique (refresh_token)
);

create index idx_account_user_id
    on refresh_token (user_id);