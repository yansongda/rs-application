import api from "@api/totp";
import { CODE } from "@constant/error";
import type { HttpError } from "@models/error";
import { WeixinError } from "@models/error";
import { substr } from "@utils/string";
import type { Tap } from "miniprogram/types/wechat";
import Message from "tdesign-miniprogram/message/index";
import Toast from "tdesign-miniprogram/toast/index";
import type { Item } from "types/totp";

interface Dataset {
  id: string;
}

Page({
  data: {
    dialogVisible: false,
    dialogDataId: 0,
    intervalIdentity: -1,
    items: [] as Item[],
  },
  onShow() {
    this.setupRefreshInterval();

    this.all();
  },
  onHide() {
    this.clearRefreshInterval();
  },
  onUnload() {
    this.clearRefreshInterval();
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
            issuer: substr(item.issuer, 7),
            username: substr(item.username, 50),
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

    this.clearRefreshInterval();

    await wx.navigateTo({ url: `/pages/totp/detail/index?id=${id}` });
  },
  delete(e: Tap<Dataset, Dataset>) {
    const dialogDataId = Number(e.currentTarget.dataset.id);

    this.setData({ dialogVisible: true, dialogDataId });
  },
  refreshCode(id: number, index: number) {
    api
      .detail(id)
      .then((response) =>
        this.setData({ [`items[${index}].code`]: response.code }),
      )
      .catch((e: HttpError) =>
        Message.error({
          content: `更新验证码失败：${e.message}`,
          duration: 5000,
          offset: [20, 32],
          context: this,
        }),
      );
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
  setupRefreshInterval() {
    const intervalIdentity = setInterval(async () => {
      for (const item of this.data.items) {
        const index = this.data.items.indexOf(item);
        const period = item.config.period ?? 30;

        let remainSeconds = period - new Date().getSeconds();
        if (remainSeconds <= 0) {
          remainSeconds += period;
        }

        this.setData({ [`items[${index}].remainSeconds`]: remainSeconds });

        if (remainSeconds === period) {
          this.refreshCode(item.id, index);
        }
      }
    }, 1000);

    this.setData({ intervalIdentity });
  },
  clearRefreshInterval() {
    if (this.data.intervalIdentity > 0) {
      clearInterval(this.data.intervalIdentity);
    }

    this.setData({ intervalIdentity: -1 });
  },
});

export default {};
