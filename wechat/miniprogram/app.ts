import { MESSAGE } from "@constant/error";
import { EE } from "@models/error";
import logger from "@utils/logger";
import app from "@utils/app";
import type {
	AppOnUnhandledRejection,
} from "types/wechat";

App({
	async onLaunch() {
    const valid = await app.valid();
    
    if (!valid) {
      await app.login();
    }
	},
	onShow() {
		app.upgrade();
	},
	async onError(e: string) {
		logger.error("小程序异常", e);

		await wx.showToast({ title: "小程序异常", icon: "error" });
	},
	async onUnhandledRejection(e: AppOnUnhandledRejection) {
		if (e.reason instanceof EE) {
			await wx.showToast({ title: MESSAGE[e.reason.code], icon: "error" });

			return;
		}

		logger.error("未知错误", e);

		await wx.showToast({ title: "出现未知错误", icon: "error" });
	},
});
