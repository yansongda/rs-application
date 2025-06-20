import api from "@api/totp";
import { CODE } from "@constant/error";
import type { HttpError } from "@models/error";
import { WeixinError } from "@models/error";
import { substr } from "@utils/string";
import type { Tap } from "miniprogram/types/wechat";
import Message from "tdesign-miniprogram/message/index";
import Toast from "tdesign-miniprogram/toast/index";
import type { Item, RefreshCodeFailed } from "types/totp";

interface Dataset {
  id: string;
}

Page({
  data: {
    dialogVisible: false,
    dialogDataId: 0,
    items: [] as Item[],
  },
  onShow() {
    this.all();
  },
  onHide() {
    // this.clearRefreshInterval();
  },
  onUnload() {
    // this.clearRefreshInterval();
  },
  all() {
    Toast({
      message: "加载中...",
      theme: "loading",
      duration: 5000,
      direction: "column",
      preventScrollThrough: true,
    });

    api
      .all()
      .then((response) => {
        Toast({
          message: "加载成功",
          theme: "success",
          duration: 100,
          direction: "column",
        });

        this.setData({
          items: response.map((item) => ({
            ...item,
            issuer: substr(item.issuer, 8),
            username: substr(item.username, 15),
          })),
        });
      })
      .catch((e: HttpError) => {
        Toast({
          message: "加载失败",
          theme: "error",
          duration: 100,
          direction: "column",
        });

        Message.error({
          content: `加载失败：${e.message}`,
          duration: 5000,
          offset: [20, 32],
          context: this,
        });
      });
  },
  async create() {
    const scan = await wx.scanCode({ scanType: ["qrCode"] }).catch(() => {
      throw new WeixinError(CODE.WEIXIN_QR_CODE);
    });

    api
      .create(scan.result)
      .catch((e: HttpError) =>
        Message.error({
          content: e.message,
          duration: 5000,
          offset: [20, 32],
          context: this,
        }),
      )
      .finally(() => this.all());
  },
  async detail(e: Tap<Dataset, Dataset>) {
    const id = Number(e.currentTarget.dataset.id);

    await wx.navigateTo({ url: `/pages/totp/detail/index?id=${id}` });
  },
  delete(e: Tap<Dataset, Dataset>) {
    const dialogDataId = Number(e.currentTarget.dataset.id);

    this.setData({ dialogVisible: true, dialogDataId });
  },
  refreshCodeFailed(e: RefreshCodeFailed) {
    Message.error({
      content: `更新验证码失败：${e.detail.message}`,
      duration: 5000,
      offset: [20, 32],
      context: this,
    });
  },
  dialogConfirm(e: Tap<Dataset, Dataset>) {
    const id = Number(e.currentTarget.dataset.id);

    api
      .deleteTotp(id)
      .catch((e: HttpError) =>
        Message.error({
          content: `删除失败：${e.message}`,
          duration: 5000,
          offset: [20, 32],
          context: this,
        }),
      )
      .finally(() => {
        this.dialogCancel();
        this.all();
      });
  },
  dialogCancel() {
    this.setData({ dialogVisible: false, dialogDataId: 0 });
  },
});

export default {};
