interface Window {
  ipc: {
    postMessage: (msg: string) => void
  },
}