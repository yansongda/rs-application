import { AxiosResponse } from '@ohos/axios';
import { IItem } from '../types/Item';
import http, { Response } from '../utils/Http';

export interface CreateRequest {
  uri: string;
}

export interface CreateResponse extends IItem {}

export class Totp {
  static async create(uri: string): Promise<CreateResponse> {
    const response: AxiosResponse<Response<CreateResponse>> =
      await http.post<CreateResponse, AxiosResponse<Response<CreateResponse>>, CreateRequest>('api/v1/totp/create',
        {
          uri,
        })

    if (response.data?.data?.id) {
      return response.data.data;
    }

    return Promise.reject(new Error('create totp failed'));
  }
}