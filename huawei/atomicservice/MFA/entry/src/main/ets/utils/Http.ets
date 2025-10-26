import axios, { AxiosError, AxiosResponse, InternalAxiosRequestConfig } from '@ohos/axios';
import { App, AppEnv, HttpDomain, LogDomain } from '../App';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { Authorization } from '../models/Auth';
import { PersistenceV2 } from '@kit.ArkUI';

const authorization =
  (): Authorization => PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() })!

const url = (): string => {
  const env = App.getEnv();

  if (AppEnv.TEST === env) {
    return HttpDomain.TEST;
  }

  return HttpDomain.PROD
}

const http = axios.create();

axios.defaults.baseURL = url();
axios.defaults.headers.common['Authorization'] = 'Bearer ' + authorization().accessToken;
axios.defaults.headers.common['User-Agent'] = 'yansongda/HarmonyOS <me@yansongda.cn>';
axios.defaults.headers.post['Content-Type'] = 'application/json';

axios.interceptors.request.use((config: InternalAxiosRequestConfig) => {
  hilog.info(LogDomain.UTILS, 'utils/http',
    '发送 http 请求，url: %{public}s, method: %{public}s, headers: %{public}s, request body: %{public}s', config.url,
    config.method,
    JSON.stringify(config.headers), JSON.stringify(config.data));

  return config;
}, (e: AxiosError) => {
  hilog.error(LogDomain.UTILS, 'utils/http',
    '发送 http 请求失败，message: %{public}s, url: %{public}s, method: %{public}s, headers: %{public}s, request body: %{public}s',
    e.message,
    e.config?.url, e.config?.method, JSON.stringify(e.config?.headers), JSON.stringify(e.config?.data));

  return Promise.reject(e);
});

// todo：1、请求异常；2、响应结构不对
axios.interceptors.response.use((response: AxiosResponse): AxiosResponse => {
  hilog.info(LogDomain.UTILS, 'utils/http',
    '接收 http 请求响应，url: %{public}s, method: %{public}s, response body: %{public}s, time: %{public}s',
    response.config.url,
    response.config.method, JSON.stringify(response.data), JSON.stringify(response.performanceTiming));

  return response;
}, (e: AxiosError) => {
  hilog.error(LogDomain.UTILS, 'utils/http',
    '接收 http 请求失败，message: %{public}s, url: %{public}s, method: %{public}s, request body: %{public}s', e.message,
    e.config?.url,
    e.config?.method, JSON.stringify(e.config?.data));

  return Promise.reject(e);
});

export interface Response<T> {
  code: number,
  message: string,
  data?: T,
}

export class HttpError extends Error {
}

export default http;