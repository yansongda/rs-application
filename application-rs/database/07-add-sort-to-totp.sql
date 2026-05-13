alter table tool.totp
    add column sort bigint unsigned not null default 0;

create index idx_tool_totp_user_sort
    on tool.totp (user_id, sort);

-- rollback
drop index idx_tool_totp_user_sort on tool.totp;
alter table tool.totp
    drop column sort;
