import api from "@api/user";
import { STORAGE } from "@constant/app";
import { DEFAULT } from "@constant/user";
import error from "@utils/error";
import user from "@utils/user";
import Message from "tdesign-miniprogram/message/index";
import Toast from "tdesign-miniprogram/toast/index";
import type { User } from "types/user";
import type {
  ChooseAvatarButtonTap,
  WxGetFileSystemManagerReadFileSuccess,
  WxGetStorageSuccess,
} from "types/wechat";

Page({
  data: {
    avatar: "",
  },
  async onShow() {
    const storage: WxGetStorageSuccess<User> = await wx.getStorage({
      key: STORAGE.USER,
    });

    this.setData({
      avatar: storage.data.config?.avatar ?? DEFAULT.CONFIG.AVATAR,
    });
  },
  async onChooseAvatar(e: ChooseAvatarButtonTap<unknown, unknown>) {
    await wx.showLoading({ title: "上传中", icon: "loading", mask: true });

    const res = await wx.compressImage({
      src: e.detail.avatarUrl.toString(),
      quality: 50,
    });

    wx.getFileSystemManager().readFile({
      filePath: res.tempFilePath,
      encoding: "base64",
      success: async (res: WxGetFileSystemManagerReadFileSuccess) => {
        this.setData({ avatar: `data:image/jpeg;base64,${res.data}` });

        await wx.hideLoading();
      },
    });
  },
  async submit() {
    Toast({
      message: "更新中...",
      theme: "loading",
      duration: 5000,
      direction: "column",
      preventScrollThrough: true,
    });

    try {
      await api.editAvatar(this.data.avatar);

      // 同步完成之后更新下全局的用户信息状态
      await user.sync();

      Toast({
        message: "修改成功",
        theme: "success",
        duration: 1500,
        direction: "column",
        preventScrollThrough: true,
      });

      setTimeout(() => wx.navigateBack(), 1500);
    } catch (e: unknown) {
      Toast({
        message: "更新失败",
        theme: "error",
        duration: 100,
        direction: "column",
      });

      Message.error({
        content: `更新失败：${error.getErrorMessage(e)}`,
        duration: 5000,
        context: this,
        offset: [20, 32],
      });
    }
  },
  async cancel() {
    await wx.navigateBack();
  },
});
