import { URL } from "@constant/accessToken";
import { CODE } from "@constant/error";
import { HttpError } from "@models/error";
import error from "@utils/error";
import http from "@utils/http";
import logger from "@utils/logger";
import type { LoginRequest, LoginResponse } from "types/accessToken";

const login = async (code: string) => {
	try {
		return await http.post<LoginResponse>(URL.LOGIN, { code } as LoginRequest);
	} catch (e: unknown) {
		logger.error("登录接口请求失败", e);

		throw new HttpError(
			CODE.HTTP_API_ACCESS_TOKEN_LOGIN,
			error.getErrorMessage(e),
		);
	}
};

export default { login };
