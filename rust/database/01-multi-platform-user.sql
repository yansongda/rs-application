create table third_user
(
    id       bigserial
        constraint pk_third_user_id
            primary key,
    user_id  bigint default 0 not null,
    platform varchar(32)      not null,
    third_id varchar(128)     not null,
    config   jsonb,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp with time zone default now() not null
);

comment on column third_user.platform is '平台';

comment on column third_user.third_id is '平台对应用户标识';

alter table third_user
    owner to miniprogram;

create index idx_user_id
    on third_user (user_id);

create unique index uk_third_user_platform_third_id
    on third_user (platform, third_id);

insert into third_user (user_id, platform, third_id)
    (select id, 'wechat', open_id from "miniprogram"."user");

alter table "user"
    rename column open_id to phone;

drop index uk_user_open_id;

update miniprogram.user set phone = '';

alter table "user"
drop column avatar;

alter table "user"
drop column nickname;

alter table "user"
drop column slogan;

create index idx_user_phone
    on "user" (phone);

alter table "user"
    add config jsonb;


alter table access_token
    add platform varchar(32) default 'wechat' not null;

truncate table access_token restart identity;

