DROP INDEX "miniprogram"."uk_user_open_id";

ALTER TABLE "miniprogram"."user" RENAME COLUMN "open_id" TO "phone";

CREATE UNIQUE INDEX "uk_user_phone" ON "miniprogram"."user" USING btree (
    "phone"
    );