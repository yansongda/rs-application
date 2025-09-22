create table third_config
(
    id         bigint unsigned AUTO_RANDOM
        primary key,
    platform   varchar(32) default 'wechat'          not null,
    third_id   varchar(128)                          not null comment '第三方id',
    config     json                                  not null,
    created_at timestamp   default CURRENT_TIMESTAMP not null,
    updated_at timestamp   default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP,
    constraint uk_third_config_platform_third_id
        unique (platform, third_id)
)
    comment '第三方配置';

-- 示例数据
insert into third_config (platform, third_id, config) values ('wechat', 'your_third_id', '{"app_id": "your_component_appid", "app_secret": "your_component_appsecret"}');