import type { RequestData } from "./http";

export interface Item {
  id: number;
  issuer: string;
  username: string;
  code: string;
  period: number;
  remainSeconds?: number;
}

export interface DetailRequest extends RequestData {
  id: number;
}

export interface UpdateRequest extends RequestData {
  id: number;
  issuer: string;
  username: string;
}

export interface CreateRequest extends RequestData {
  uri: string;
}

export interface DeleteRequest extends RequestData {
  id: number;
}
