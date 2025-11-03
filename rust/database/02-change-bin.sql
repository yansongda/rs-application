-- =============================================
-- 创建用户和模式
-- =============================================
CREATE USER "rs-application" WITH PASSWORD 'qoCDIv1RVTJycTNQKIaivVWVmRtsTRCr';

CREATE SCHEMA account;
CREATE SCHEMA tool;

-- =============================================
-- 账户模块表创建与数据迁移
-- =============================================
BEGIN;

-- access_token 表
CREATE TABLE account.access_token (LIKE miniprogram.access_token INCLUDING ALL);
INSERT INTO account.access_token SELECT * FROM miniprogram.access_token;
CREATE SEQUENCE account.access_token_id_seq;
ALTER TABLE account.access_token
    ALTER COLUMN id SET DEFAULT nextval('account.access_token_id_seq');
SELECT setval('account.access_token_id_seq',
              COALESCE((SELECT MAX(id) FROM account.access_token), 1));

-- third_user 表
CREATE TABLE account.third_user (LIKE miniprogram.third_user INCLUDING ALL);
INSERT INTO account.third_user SELECT * FROM miniprogram.third_user;
CREATE SEQUENCE account.third_user_id_seq;
ALTER TABLE account.third_user
    ALTER COLUMN id SET DEFAULT nextval('account.third_user_id_seq');
SELECT setval('account.third_user_id_seq',
              COALESCE((SELECT MAX(id) FROM account.third_user), 1));

-- user 表
CREATE TABLE account."user" (LIKE miniprogram."user" INCLUDING ALL);
INSERT INTO account."user" SELECT * FROM miniprogram."user";
CREATE SEQUENCE account.user_id_seq;
ALTER TABLE account."user"
    ALTER COLUMN id SET DEFAULT nextval('account.user_id_seq');
SELECT setval('account.user_id_seq',
              COALESCE((SELECT MAX(id) FROM account."user"), 1));

-- =============================================
-- 工具模块表创建与数据迁移
-- =============================================
-- totp 表
CREATE TABLE tool.totp (LIKE miniprogram.totp INCLUDING ALL);
INSERT INTO tool.totp SELECT * FROM miniprogram.totp;
CREATE SEQUENCE tool.totp_id_seq;
ALTER TABLE tool.totp
    ALTER COLUMN id SET DEFAULT nextval('tool.totp_id_seq');
SELECT setval('tool.totp_id_seq',
              COALESCE((SELECT MAX(id) FROM tool.totp), 1));

-- short_url 表
CREATE TABLE tool.short_url (LIKE miniprogram.short_url INCLUDING ALL);
INSERT INTO tool.short_url SELECT * FROM miniprogram.short_url;
CREATE SEQUENCE tool.short_url_id_seq;
ALTER TABLE tool.short_url
    ALTER COLUMN id SET DEFAULT nextval('tool.short_url_id_seq');
SELECT setval('tool.short_url_id_seq',
              COALESCE((SELECT MAX(id) FROM tool.short_url), 1));

COMMIT;

grant "rs-application" to yansongda_owner;
grant connect on database yansongda to "rs-application";
alter schema account owner to "rs-application";
alter schema tool owner to "rs-application";
alter table "account"."access_token" owner to "rs-application";
alter table "account"."third_user" owner to "rs-application";
alter table "account"."user" owner to "rs-application";
alter table "tool"."totp" owner to "rs-application";
alter table "tool"."short_url" owner to "rs-application";

alter sequence account.access_token_id_seq owner to "rs-application";
alter sequence "account"."third_user_id_seq" owner to "rs-application";
alter sequence "account"."user_id_seq" owner to "rs-application";
alter sequence "tool"."totp_id_seq" owner to "rs-application";
alter sequence "tool"."short_url_id_seq" owner to rs-application;
