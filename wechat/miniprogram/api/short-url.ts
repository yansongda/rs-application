import { CODE } from "@constant/error";
import { PATH } from "@constant/short-url";
import { HttpError } from "@models/error";
import error from "@utils/error";
import http from "@utils/http";
import logger from "@utils/logger";
import type { CreateRequest, CreateResponse } from "types/short-url";

const create = async (url: string) => {
  try {
    return await http.post<CreateResponse>(PATH.CREATE, {
      url,
    } as CreateRequest);
  } catch (e: unknown) {
    logger.error("创建短链接失败", e);

    throw new HttpError(
      CODE.HTTP_API_SHORT_URL_CREATE,
      error.getErrorMessage(e),
    );
  }
};

export default { create };
