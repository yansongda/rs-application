import { STORAGE, URL } from "@constant/app";
import { CODE, WECHAT_MESSAGE } from "@constant/error";
import { HttpError } from "@models/error";
import logger from "@utils/logger";
import type { Request, RequestData, RequestQuery, Response } from "types/http";
import type { WxRequestFail, WxRequestSuccess } from "types/wechat";

const formatUrl = (request: Request): void => {
	// 处理 url 的 query
	if (typeof request.query !== "undefined") {
		const query = request.query;

		const paramsArray: string[] = [];

		// biome-ignore lint: no-check
		Object.keys(request.query).forEach(
			(key) => query[key] && paramsArray.push(`${key}=${query[key]}`),
		);

		request.url += `${
			request.url.search(/\?/) === -1 ? "?" : "&"
		}${paramsArray.join("&")}`;
	}

	// 处理 url
	if (request.url.startsWith("/")) {
		request.url = URL.BASE + request.url;
	}
};

const formatHeaders = (request: Request): void => {
	if (typeof request.headers === "undefined") {
		request.headers = {};
	}

	request.headers.authorization =
		`Bearer ${wx.getStorageSync(STORAGE.ACCESS_TOKEN)}` || "";
};

const request = <T>(request: Request): Promise<T> => {
	formatUrl(request);
	formatHeaders(request);

	if (request.isUploadFile) {
		return wxUpload(request);
	}

	return wxRequest(request);
};

const wxRequest = <T>(request: Request) => {
	logger.info(
		"请求接口",
		request.url.indexOf("users/update") === -1 ? request : "用户更新",
	);

	return new Promise<T>((resolve, reject) => {
		wx.request({
			url: request.url,
			data: request.data || {},
			header: request.headers ?? {},
			timeout: request.timeout || 5000,
			method: request.method || "GET",
			success: (res: WxRequestSuccess<T>) => {
				logger.info(
					"接口请求成功",
					request.url.indexOf("users/detail") === -1 ? res : "用户详情",
				);

				if (Number(res.data.code) === 0) {
					resolve(res.data.data);
				}

				reject(new HttpError(Number(res.data.code), res.data.message));
			},
			fail: (err: WxRequestFail) => {
				logger.warning("接口请求失败", err);

				reject(
					new HttpError(
						err.errno,
						WECHAT_MESSAGE[err.errno as keyof typeof WECHAT_MESSAGE] ||
							`接口请求失败：${err.errMsg}`,
					),
				);
			},
		});
	});
};

const wxUpload = <T>(request: Request) => {
	logger.info("请求上传接口", request.url, request.headers);

	return new Promise<T>((resolve, reject) => {
		const filePath: string = (request.data?.filePath ?? "") as string;
		const name: string = (request.data?.name ?? "") as string;
		const formData = request.data ?? {};

		if (!filePath || !name) {
			reject(new HttpError(CODE.HTTP_PARAMS));
		}

		formData.filePath = undefined;
		formData.name = undefined;

		wx.uploadFile({
			url: request.url,
			filePath,
			name,
			formData,
			header: request.headers ?? {},
			timeout: request.timeout || 30000,
			success: (res) => {
				logger.info("接口请求成功", res);

				const response = JSON.parse(res.data) as Response<T>;

				if (response.code === 0) {
					resolve(response.data);
				}

				reject(new HttpError(Number(response.code), response.message));
			},
			fail: (err) => {
				logger.warning("接口请求失败", err);

				reject(new HttpError(undefined, `接口请求失败：${err.errMsg}`));
			},
		});
	});
};

const post = <T>(
	url: string,
	data?: RequestData,
	isUploadFile?: boolean,
): Promise<T> => {
	return request<T>({ url, data, isUploadFile, method: "POST" } as Request);
};

const get = <T>(url: string, query?: RequestQuery): Promise<T> => {
	return request<T>({ url, query, method: "GET" } as Request);
};

export default { request, post, get };
