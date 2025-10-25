import axios, { AxiosError, AxiosResponse, InternalAxiosRequestConfig } from '@ohos/axios';
import { App, AppEnv, HttpDomain, LogDomain } from '../App';
import { Authorization } from '../models/Auth';
import { PersistenceV2 } from '@kit.ArkUI';
import { hilog } from '@kit.PerformanceAnalysisKit';

const authorization: Authorization =
  PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() })!

const url = (): string => {
  const env = App.getEnv();

  if (AppEnv.TEST === env) {
    return HttpDomain.TEST;
  }

  return HttpDomain.PROD
}

const http = axios.create();

axios.defaults.baseURL = url();
axios.defaults.headers.common['Authorization'] = 'Bearer ' + authorization.accessToken;
axios.defaults.headers.common['User-Agent'] = 'yansongda/HarmonyOS <me@yansongda.cn>';
axios.defaults.headers.post['Content-Type'] = 'application/json';

axios.interceptors.request.use((config: InternalAxiosRequestConfig) => {
  hilog.info(LogDomain.UTILS, 'utils/http',
    '发送 http 请求，url: %{public}s, method: %{public}s, headers: %{public}s, body: %{public}s', config.url,
    config.method,
    JSON.stringify(config.headers), JSON.stringify(config.data));

  return config;
}, (e: AxiosError) => {
  hilog.error(LogDomain.UTILS, 'utils/http', '发送 http 请求失败，code: %{public}s, message: %{public}s', e.code,
    e.message);

  return Promise.reject(e);
});

axios.interceptors.response.use((response: AxiosResponse): AxiosResponse => {
  hilog.info(LogDomain.UTILS, 'utils/http',
    '接收 http 请求，url: %{public}s, method: %{public}s, body: %{public}s, time: %{public}s', response.config.url,
    response.config.method, JSON.stringify(response.data), JSON.stringify(response.performanceTiming));

  return response;
}, (e: AxiosError) => {
  hilog.error(LogDomain.UTILS, 'utils/http', '接收 http 请求失败，code: %{public}s, message: %{public}s', e.code,
    e.message)

  return Promise.reject(e);
});

export interface Response<T> {
  code: number,
  message: string,
  data?: T,
}

export default http;