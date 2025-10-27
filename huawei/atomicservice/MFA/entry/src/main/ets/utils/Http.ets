import axios, { AxiosError, AxiosResponse, InternalAxiosRequestConfig } from '@ohos/axios';
import { App, AppEnv, HttpDomain, LogDomain } from '../App';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { Authorization } from '../models/Auth';
import { PersistenceV2 } from '@kit.ArkUI';
import { application } from '@kit.AbilityKit';

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

http.defaults.baseURL = url();
http.defaults.timeout = 5000;
// http.defaults.headers.common['Authorization'] = 'Bearer ' + authorization().accessToken;
http.defaults.headers.common['User-Agent'] = 'yansongda/atomicservice/MFA-v' + App.getVersion() + ' <me@yansongda.cn>';
http.defaults.headers.post['Content-Type'] = 'application/json';

http.interceptors.request.use((config: InternalAxiosRequestConfig) => {
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

  let message = '网络错误，请检查网络状态';

  try {
    message =
      application.getApplicationContext().resourceManager.getStringSync($r('app.string.error_internet_error').id)
  } catch (e) {
  }

  // todo: 提示
  return Promise.reject(new HttpError(message));
});

http.interceptors.response.use((response: AxiosResponse): AxiosResponse => {
  hilog.info(LogDomain.UTILS, 'utils/http',
    '接收 http 请求响应，url: %{public}s, method: %{public}s, response body: %{public}s, time: %{public}s',
    response.config.url,
    response.config.method, JSON.stringify(response.data), JSON.stringify(response.performanceTiming));

  return response;
}, (e: AxiosError) => {
  hilog.error(LogDomain.UTILS, 'utils/http',
    '接收 http 请求失败，message: %{public}s, url: %{public}s, method: %{public}s, request body: %{public}s, response body: %{public}s',
    e.message, e.config?.url, e.config?.method, JSON.stringify(e.config?.data), JSON.stringify(e.response?.data)
  );

  // todo：1、请求异常；2、响应结构不对

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