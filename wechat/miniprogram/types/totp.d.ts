import type { HttpError } from "@models/error";
import type { RequestData } from "./http";

export interface Item {
  id: number;
  issuer: string;
  username: string;
  code: string;
  config: ItemConfig;
}

export interface ItemConfig {
  period: number;
}

export interface DetailRequest extends RequestData {
  id: number;
}

export interface EditIssuerRequest extends RequestData {
  id: number;
  issuer: string;
}

export interface EditUsernameRequest extends RequestData {
  id: number;
  username: string;
}

export interface CreateRequest extends RequestData {
  uri: string;
}

export interface DeleteRequest extends RequestData {
  id: number;
}

export interface ItemMessageEvent {
  detail: string;
}

export interface ItemDetailEvent {
  detail: number;
}

export interface ItemDeleteEvent {
  detail: number;
}
