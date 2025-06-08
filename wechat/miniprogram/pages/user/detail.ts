import { STORAGE } from "@constant/app";
import { DEFAULT } from "@constant/user";
import type { User } from "types/user";
import type { WxGetStorageSuccess } from "types/wechat";

Page({
  data: {
    config: {
      nickname: "",
      avatar: "",
      slogan: "",
    },
  },
  async onShow() {
    const storage: WxGetStorageSuccess<User> = await wx.getStorage({
      key: STORAGE.USER,
    });

    this.setData({
      config: {
        nickname: storage.data.config?.nickname ?? DEFAULT.CONFIG.NICKNAME,
        avatar: storage.data.config?.avatar ?? DEFAULT.CONFIG.AVATAR,
        slogan: storage.data.config?.slogan ?? DEFAULT.CONFIG.SLOGAN,
      },
    });
  },
});
