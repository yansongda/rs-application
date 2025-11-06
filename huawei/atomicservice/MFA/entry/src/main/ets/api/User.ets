import { AxiosResponse } from '@ohos/axios';
import { User as UserModel } from '../models/User';
import http, { HttpError, Response } from '../utils/Http';

export interface EditAvatarRequest {
  avatar: string;
}

export interface EditNicknameRequest {
  nickname: string;
}

export class User {
  static async detail(): Promise<UserModel> {
    const response: AxiosResponse<Response<UserModel>> =
      await http.post<UserModel, AxiosResponse<Response<UserModel>>, void>('api/v1/user/detail');

    if (response.data?.data?.id) {
      return response.data.data;
    }

    return Promise.reject(new HttpError(response.data.message));
  }

  static async editAvatar(avatar: string): Promise<void> {
    const response: AxiosResponse<Response<void>> =
      await http.post<void, AxiosResponse<Response<void>>, EditAvatarRequest>('api/v1/user/edit/avatar',
        {
          avatar,
        });

    if (0 === response.data?.code) {
      return;
    }

    return Promise.reject(new HttpError(response.data.message));
  }

  static async editNickname(nickname: string): Promise<void> {
    const response: AxiosResponse<Response<void>> =
      await http.post<void, AxiosResponse<Response<void>>, EditNicknameRequest>('api/v1/user/edit/nickname',
        {
          nickname,
        });

    if (0 === response.data?.code) {
      return;
    }

    return Promise.reject(new HttpError(response.data.message));
  }
}