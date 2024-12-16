DROP INDEX "miniprogram"."uk_user_open_id";

ALTER TABLE "miniprogram"."user" RENAME COLUMN "open_id" TO "phone";

CREATE UNIQUE INDEX "uk_user_phone" ON "miniprogram"."user" USING btree (
    "phone"
    );

CREATE TABLE "miniprogram"."third_user" (
     "id" int8 NOT NULL DEFAULT nextval('third_user_id_seq'::regclass),
     "user_id" int8 NOT NULL DEFAULT 0,
     "platform" varchar(32) COLLATE "pg_catalog"."default" NOT NULL,
     "third_id" varchar(128) COLLATE "pg_catalog"."default" NOT NULL,
     "config" jsonb,
     CONSTRAINT "third_user_pkey" PRIMARY KEY ("id"),
     CONSTRAINT "uk_platform_third_id" UNIQUE ("platform", "third_id")
)
;

ALTER TABLE "miniprogram"."third_user"
    OWNER TO "miniprogram";

CREATE INDEX "idx_user_id" ON "miniprogram"."third_user" USING btree (
    "user_id" "pg_catalog"."int8_ops" ASC NULLS LAST
    );

COMMENT ON COLUMN "miniprogram"."third_user"."user_id" IS '标识用户';

COMMENT ON COLUMN "miniprogram"."third_user"."platform" IS '平台';

COMMENT ON COLUMN "miniprogram"."third_user"."third_id" IS '平台对应用户标识';