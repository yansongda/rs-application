import Message from 'tdesign-miniprogram/message/index'
import Toast from 'tdesign-miniprogram/toast/index'
import api from '@api/user'
import user from '@utils/user'
import error from '@utils/error'
import type {
  ChooseAvatarButtonTap,
  FormSubmit,
  WxGetFileSystemManagerReadFileSuccess,
  WxGetStorageSuccess
} from '@types/wechat'
import { STORAGE } from '@constant/app'
import type { UpdateRequest, User } from '@types/user'
import { DEFAULT } from '@constant/user'

interface FormData {
  avatar: string
  nickname: string
  slogan: string
}

Page({
  data: {
    avatar: DEFAULT.avatar,
    nickname: DEFAULT.nickname,
    slogan: DEFAULT.slogan
  },
  async onShow() {
    const storage: WxGetStorageSuccess<User> = await wx.getStorage({ key: STORAGE.USER })

    this.setData({
      avatar: storage.data.avatar,
      nickname: storage.data.nickname,
      slogan: storage.data.slogan
    })
  },
  async onChooseAvatar(e: ChooseAvatarButtonTap<unknown, unknown>) {
    await wx.showLoading({ title: '上传中', icon: 'loading', mask: true })

    const res = await wx.compressImage({ src: e.detail.avatarUrl.toString(), quality: 50 })

    wx.getFileSystemManager().readFile({
      filePath: res.tempFilePath,
      encoding: 'base64',
      success: async (res: WxGetFileSystemManagerReadFileSuccess) => {
        this.setData({ avatar: 'data:image/jpeg;base64,' + res.data })

        await wx.hideLoading()
      }
    })
  },
  async submit(e: FormSubmit<FormData>) {
    Toast({
      message: '更新中...',
      theme: 'loading',
      duration: 5000,
      direction: 'column',
      preventScrollThrough: true
    })

    try {
      await api.update(e.detail.value as UpdateRequest)

      // 同步完成之后更新下全局的用户信息状态
      await user.sync()

      Toast({
        message: '修改成功',
        theme: 'success',
        duration: 1500,
        direction: 'column',
        preventScrollThrough: true
      })

      setTimeout(() => wx.navigateBack(), 1500)
    } catch (e: unknown) {
      Toast({
        message: '更新失败',
        theme: 'error',
        duration: 100,
        direction: 'column'
      })

      Message.error({
        content: '更新失败：' + error.getErrorMessage(e),
        duration: 5000,
        context: this,
        offset: [20, 32]
      })
    }
  },
  async cancel() {
    await wx.navigateBack()
  }
})
