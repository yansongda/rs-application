import type { RequestData } from "./http";

export interface CreateRequest extends RequestData {
  url: string;
}

export interface CreateResponse {
  url: string;
  short: string;
}
