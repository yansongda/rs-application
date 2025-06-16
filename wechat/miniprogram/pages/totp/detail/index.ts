import api from "@api/totp";
import { substr } from "@utils/string";
import type { Tap } from "miniprogram/types/wechat";
import Toast from "tdesign-miniprogram/toast/index";

interface Query {
  id?: string;
}

interface Dataset {
  type: string;
}

Page({
  data: {
    dialogVisible: false,
    id: 0,
    issuer: "",
    username: "",
    config: { period: 30 },
  },
  onLoad(query: Query) {
    this.data.id = Number(query.id || 0);
  },
  onShow() {
    Toast({
      message: "加载中...",
      theme: "loading",
      duration: 5000,
      direction: "column",
    });

    api
      .detail(this.data.id)
      .then(({ id, issuer, username, config }) => {
        Toast({
          message: "加载成功",
          theme: "success",
          duration: 100,
          direction: "column",
        });

        this.setData({
          id,
          issuer: substr(issuer),
          username: substr(username),
          config,
        });
      })
      .catch(() => {
        Toast({
          message: "加载失败",
          theme: "error",
          duration: 100,
          direction: "column",
        });

        this.setData({ dialogVisible: true });
      });
  },
  async gotoEdit(e: Tap<Dataset, Dataset>) {
    let url = "";

    switch (e.currentTarget.dataset.type) {
      case "issuer":
        url = `/pages/totp/edit/issuer?id=${this.data.id}&issuer=${this.data.issuer}`;
        break;
      case "username":
        url = `/pages/totp/edit/username?id=${this.data.id}&username=${this.data.username}`;
        break;
      default:
        break;
    }

    console.log(url);

    if (url.length > 0) {
      await wx.navigateTo({ url });
    }
  },
  dialogConfirm() {
    this.setData({ dialogVisible: false });

    this.onShow();
  },
  dialogCancel() {
    this.setData({ dialogVisible: false });
  },
});
