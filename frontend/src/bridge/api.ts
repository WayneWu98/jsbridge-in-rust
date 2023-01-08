import { ActionType } from './constant'
import { invoke } from './ipc'

enum DownloadCallbackType {
  Progress = 'onProgress',
  Success = 'onSuccess',
  Fail = 'onFail',
  Finally = 'onFinally',
}

interface DownloadConfig {
  url: string,
  [DownloadCallbackType.Progress]?: (data: { total: number, progress: number }) => void,
  [DownloadCallbackType.Success]?: () => void,
  [DownloadCallbackType.Fail]?: () => void,
  [DownloadCallbackType.Finally]?: () => void,
}
export const download = (config: DownloadConfig) => {
  const callback = ({ type, data }: { type: keyof typeof DownloadCallbackType , data: any}) => {
    config[DownloadCallbackType[type]]?.(data)
  }
  invoke(ActionType.DownloadFile, { data: config.url, callback })
}

export const getSystemInfo = () => {
  return new Promise(r => {
    invoke(ActionType.GetSystemInfo, { callback: r })
  })
}