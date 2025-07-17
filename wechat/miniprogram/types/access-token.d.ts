import type { RequestData } from "./http";

export interface LoginRequest extends RequestData {
  platform: string;
  code: string;
}

export interface LoginResponse {
  access_token: string;
}
