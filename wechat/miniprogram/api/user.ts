import { CODE } from "@constant/error";
import { PATH } from "@constant/user";
import { HttpError } from "@models/error";
import error from "@utils/error";
import http from "@utils/http";
import logger from "@utils/logger";
import type { DetailResponse, EditRequest, EditResponse } from "types/user";

const detail = async () => {
  try {
    return await http.post<DetailResponse>(PATH.DETAIL);
  } catch (e: unknown) {
    logger.error("查询用户详情失败", e);

    throw new HttpError(CODE.HTTP_API_USER_DETAIL, error.getErrorMessage(e));
  }
};

const update = async (updated: EditRequest) => {
  try {
    return await http.post<EditResponse>(PATH.EDIT, updated);
  } catch (e: unknown) {
    logger.error("更新用户信息失败", e);

    throw new HttpError(CODE.HTTP_API_USER_UPDATE, error.getErrorMessage(e));
  }
};

export default { detail, edit: update };
