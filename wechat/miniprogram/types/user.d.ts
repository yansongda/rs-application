import type { RequestData } from "./http";

export interface UserConfig {
  avatar: string;
  nickname: string;
  slogan: string;
}
export interface DetailResponse {
  phone: string;
  config?: UserConfig;
}

export interface EditRequest extends RequestData {
  phone: string;
  config?: UserConfig;
}

export interface User {
  phone: string;
  config?: UserConfig;
}
