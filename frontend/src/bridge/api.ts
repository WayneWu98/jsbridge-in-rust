import { ActionType } from './constant'
import { invoke } from './ipc'

interface DownloadConfig {
  url: string,
  onProgress?: () => void,
  onSuccess?: () => void,
  onFail?: () => void,
  onFinally?: () => void,
}
export const download = ({ url }: DownloadConfig) => {
  const callback = (...params: any[]) => {
    console.log('params', ...params);
  }
  invoke(ActionType.DownloadFile, { data: url, callback })
}