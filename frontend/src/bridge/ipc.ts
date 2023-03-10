import { ActionType, EventType } from './constant'

interface Fn {
  (...params: any[]): any,
}

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
  callEnded?: boolean,
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

const onReceivedMsg = (window as any).onReceivedMsg = (msg: ReceivedMsg) => {
  const { callbackId, callEnded, event, data } = msg
  if (callbackId) {
    const cb = _CALLBACK_BUCKET.get(callbackId)
    if (cb) cb(data)
    if (callEnded) _CALLBACK_BUCKET.delete(callbackId)
    return
  }
  if (event) {
    const handlers = _EVENT_HANDLER_BUCKET.get(event)
    for (const handler of Array.from(handlers ?? []) ) {
      if (handler) handler(data)
    }
  }
}

on(EventType.ThemeChanged, theme => {
  console.log('ThemeChanged', theme);
})