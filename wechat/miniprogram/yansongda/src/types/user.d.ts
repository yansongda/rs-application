import type { RequestData } from "./http";

export interface User {
  phone: string;
  config?: UserConfig;
}

export interface UserConfig {
  avatar: string;
  nickname: string;
  slogan: string;
}
export interface DetailResponse {
  phone: string;
  config?: UserConfig;
}

export interface EditAvatarRequest extends RequestData {
  avatar: string;
}

export interface EditNicknameRequest extends RequestData {
  nickname: string;
}

export interface EditSloganRequest extends RequestData {
  slogan: string;
}
