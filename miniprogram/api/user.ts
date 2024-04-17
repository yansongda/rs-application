import http from '@utils/http'
import { URL } from '@constant/user'
import { CODE } from '@constant/error'
import logger from '@utils/logger'
import error from '@utils/error'
import { HttpError } from '@models/error'
import type { DetailResponse, UpdateRequest, UpdateResponse } from '@types/user'

const detail = async () => {
  try {
    return await http.post<DetailResponse>(URL.DETAIL)
  } catch (e: unknown) {
    logger.error('查询用户详情失败', e)

    throw new HttpError(CODE.HTTP_API_USER_DETAIL, error.getErrorMessage(e))
  }
}

const update = async (updated: UpdateRequest) => {
  try {
    return await http.post<UpdateResponse>(URL.UPDATE, updated)
  } catch (e: unknown) {
    logger.error('更新用户信息失败', e)

    throw new HttpError(CODE.HTTP_API_USER_UPDATE, error.getErrorMessage(e))
  }
}

export default { detail, update }
