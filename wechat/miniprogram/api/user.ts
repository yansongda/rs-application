import { CODE } from "@constant/error";
import { PATH } from "@constant/user";
import { HttpError } from "@models/error";
import error from "@utils/error";
import http from "@utils/http";
import logger from "@utils/logger";
import type {
  DetailResponse,
  EditAvatarRequest,
  EditNicknameRequest,
  EditSloganRequest,
} from "types/user";

const detail = async () => {
  try {
    return await http.post<DetailResponse>(PATH.DETAIL);
  } catch (e: unknown) {
    logger.error("查询用户详情失败", e);

    throw new HttpError(CODE.HTTP_API_USER_DETAIL, error.getErrorMessage(e));
  }
};

const editAvatar = async (avatar: string) => {
  try {
    return await http.post<null>(PATH.EDIT_AVATAR, {
      avatar,
    } as EditAvatarRequest);
  } catch (e: unknown) {
    logger.error("更新头像信息失败", e);

    throw new HttpError(
      CODE.HTTP_API_USER_EDIT_AVATAR,
      error.getErrorMessage(e),
    );
  }
};

const editNickname = async (nickname: string) => {
  try {
    return await http.post<null>(PATH.EDIT_NICKNAME, {
      nickname,
    } as EditNicknameRequest);
  } catch (e: unknown) {
    logger.error("更新昵称失败", e);

    throw new HttpError(
      CODE.HTTP_API_USER_EDIT_NICKNAME,
      error.getErrorMessage(e),
    );
  }
};

const editSlogan = async (slogan: string) => {
  try {
    return await http.post<null>(PATH.EDIT_SLOGAN, {
      slogan,
    } as EditSloganRequest);
  } catch (e: unknown) {
    logger.error("更新 slogan 失败", e);

    throw new HttpError(
      CODE.HTTP_API_USER_EDIT_SLOGAN,
      error.getErrorMessage(e),
    );
  }
};

export default { detail, editAvatar, editNickname, editSlogan };
