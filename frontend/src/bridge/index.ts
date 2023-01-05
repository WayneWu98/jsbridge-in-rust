import { ActionType, EventType } from './constant'
export * from './constant'

type Fn = (...params: any[]) => any

let callbackId = 0;
const _CALLBACK_BUCKET = new Map<number, Fn>()
const _EVENT_HANDLER_BUCKET = new Map<EventType, Set<Fn>>()

export const invoke = <T>(actionType: ActionType, { data, callback }: {data?: T, callback?: Fn} = {}) => {
  
  if (callback) {
    callbackId++
    _CALLBACK_BUCKET.set(callbackId, callback)
  }
  window
    .ipc
    .postMessage(JSON.stringify({
      actionType,
      data,
      callbackId: callback ? callbackId : undefined
    }))
}

interface ReceivedMsg {
  callbackId?: number,
  event?: EventType,
  data?: any,
}

export const on = (event: EventType, fn: Fn) => {
  const handlers = _EVENT_HANDLER_BUCKET.get(event)
  if (handlers) return void handlers.add(fn)
  _EVENT_HANDLER_BUCKET.set(event, new Set([fn]))
}

export const off = (event: EventType, fn: Fn) => {
  _EVENT_HANDLER_BUCKET.get(event)?.delete(fn)
}

const onReceivedMsg = (window as any).onReceivedMsg = ({ callbackId, event, data }: ReceivedMsg) => {
  if (callbackId) {
    _CALLBACK_BUCKET.get(callbackId)?.(data)
    return
  }
  if (event) {
    const handlers = _EVENT_HANDLER_BUCKET.get(event)
    if (handlers) {
      for (const handler of Array.from(handlers) ) {
        handler(data)
      }
    }
  }
}

on(EventType.ThemeChanged, theme => {
  console.log('ThemeChanged', theme);
})