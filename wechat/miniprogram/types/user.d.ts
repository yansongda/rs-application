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

export interface EditResponse {
  open_id: string;
  avatar: string;
  nickname: string;
  slogan: string;
}

export interface User {
  phone: string;
  config?: UserConfig;
}
