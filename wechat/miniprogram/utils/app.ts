import accessToken from "@api/accessToken";
import { STORAGE } from "@constant/app";
import type { LoginResponse } from "miniprogram/types/accessToken";
import type {
  WxGetUpdateManagerOnCheckForUpdateResult,
  WxLoginSuccessCallbackResult,
} from "miniprogram/types/wechat";
import logger from "./logger";

const valid = async (): Promise<boolean> => {
  try {
    await wx.checkSession();
    await accessToken.valid();

    return true;
  } catch (e) {
    /* empty */
  }

  return false;
};

const login = async () => {
  await wx.showToast({
    title: "登录中...",
    icon: "loading",
    duration: 6000,
    mask: true,
  });

  wx.login({
    success: async (res: WxLoginSuccessCallbackResult) => {
      try {
        const loginResponse: LoginResponse = await accessToken.login(res.code);

        await wx.setStorage({
          key: STORAGE.ACCESS_TOKEN,
          data: loginResponse.access_token,
        });

        await wx.hideToast();
      } catch (e) {
        logger.error(e);

        restart("获取与设置基础信息失败，即将重启小程序", null);
      }
    },
    fail: () => {
      restart("微信登录 API 错误，即将重启小程序", null);
    },
  });
};

const restart = (
  content: string | null | undefined,
  path: string | null | undefined,
) => {
  wx.showModal({
    title: "提示",
    content: content ?? "小程序即将重启",
    showCancel: false,
    confirmText: "重启",
    success(res) {
      if (res.confirm) {
        wx.restartMiniProgram({ path: path ?? "/pages/home/index" });
      }
    },
  });
};

const upgrade = () => {
  const updateManager = wx.getUpdateManager();

  updateManager.onCheckForUpdate(
    (res: WxGetUpdateManagerOnCheckForUpdateResult) => {
      if (res.hasUpdate) {
        logger.info("小程序有最新版本，后续将自动更新");
      }
    },
  );

  updateManager.onUpdateReady(() => {
    wx.showModal({
      title: "更新提示",
      content: "新版本已经准备好，是否重启应用？",
      success(res) {
        if (res.confirm) {
          updateManager.applyUpdate();
        }
      },
    });
  });

  updateManager.onUpdateFailed(() => {
    logger.error("小程序更新下载异常");
  });
};

export default { valid, login, restart, upgrade };
