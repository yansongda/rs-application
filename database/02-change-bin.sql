create user rs_application with password '123';

create schema account AUTHORIZATION rs_application;

create table "account"."access_token" like "miniprogram"."access_token";
insert into "account"."access_token" select * from "miniprogram"."access_token";
alter table "account"."access_token" owner to rs_application;

create table "account"."third_user" like "miniprogram"."third_user";
insert into "account"."third_user" select * from "miniprogram"."third_user";
alter table "account"."third_user" owner to rs_application;

create table "account"."user" like "miniprogram"."user";
insert into "account"."user" select * from "miniprogram"."user";
alter table "account"."user" owner to rs_application;

create schema tool AUTHORIZATION rs_application;

create table "tool"."totp" like "miniprogram"."totp";
insert into "tool"."totp" select * from "miniprogram"."totp";
alter table "tool"."totp" owner to rs_application;

create table "tool"."short_url" like "miniprogram"."short_url";
insert into "tool"."short_url" select * from "miniprogram"."short_url";
alter table "tool"."short_url" owner to rs_application;

select setval('account.access_token_id_seq', nextval('account.access_token_id_seq'))
select setval('account.user_id_seq', nextval('account.user_id_seq'))
select setval('account.third_user_id_seq', nextval('account.third_user_id_seq'))
select setval('tool.short_url_id_seq', nextval('tool.short_url_id_seq'))
select setval('tool.totp_id_seq', nextval('tool.totp_id_seq'))
