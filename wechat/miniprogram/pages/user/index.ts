import { DEFAULT } from "@constant/user";
import utils from "@utils/user";
import type { User } from "types/user";

Page({
  data: {
    config: {
      nickname: "",
      avatar: "",
      slogan: "",
    },
  },
  async onShow() {
    const user: User = await utils.detail();

    this.setData({
      config: {
        nickname: user.config?.nickname ?? DEFAULT.CONFIG.NICKNAME,
        avatar: user.config?.avatar ?? DEFAULT.CONFIG.AVATAR,
        slogan: user.config?.slogan ?? DEFAULT.CONFIG.SLOGAN,
      },
    });
  },
  onHide() {},
  onReady() {},
});

export default {};
