import { RequestData } from './http'

export interface DetailResponse {
  open_id: string
  avatar: string
  nickname: string
  slogan: string
}

export interface UpdateRequest extends RequestData {
  avatar?: string
  nickname?: string
  slogan?: string
}

export interface UpdateResponse {
  open_id: string
  avatar: string
  nickname: string
  slogan: string
}

export interface UpdateResult {
  isGlobalDataUpdated: boolean
  user?: User
}

export interface User {
  avatar: string
  nickname: string
  slogan: string
}
