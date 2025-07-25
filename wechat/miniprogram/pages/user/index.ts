import { DEFAULT } from "@constant/user";
import { substr } from "@utils/string";
import utils from "@utils/user";
import type { User } from "types/user";

Page({
  data: {
    config: {
      nickname: DEFAULT.CONFIG.NICKNAME,
      avatar: DEFAULT.CONFIG.AVATAR,
      slogan: DEFAULT.CONFIG.SLOGAN,
    },
  },
  async onShow() {
    const user: User = await utils.detail();

    this.setData({
      config: {
        nickname: substr(user.config?.nickname ?? DEFAULT.CONFIG.NICKNAME),
        avatar: user.config?.avatar ?? DEFAULT.CONFIG.AVATAR,
        slogan: substr(user.config?.slogan ?? DEFAULT.CONFIG.SLOGAN),
      },
    });
  },
  onHide() {},
  onReady() {},
});

export default {};
