import { STORAGE } from "@constant/app";
import { DEFAULT } from "@constant/user";
import { substr } from "@utils/string";
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
        nickname: substr(
          storage.data.config?.nickname ?? DEFAULT.CONFIG.NICKNAME,
        ),
        avatar: storage.data.config?.avatar ?? DEFAULT.CONFIG.AVATAR,
        slogan: substr(storage.data.config?.slogan ?? DEFAULT.CONFIG.SLOGAN),
      },
    });
  },
  async editAvatar() {
    await wx.navigateTo({ url: "/pages/user/edit/avatar" });
  },
});
