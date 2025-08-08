import { useEffect, useState, useRef } from 'react'
// 通过 index.html 的 <script type="module"> 注入 window.rustimage_wasm

type WasmModule = any

export function useWasmLoader() {
  const [ready, setReady] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const loadingRef = useRef(false)

  useEffect(() => {
    if (loadingRef.current) return
    loadingRef.current = true

    async function load() {
      try {
        // 如果页面已注入，则等待 ready 事件；否则直接读取
        if (!(window as any).rustimage_wasm) {
          await new Promise<void>((resolve) => {
            const handler = () => { window.removeEventListener('rustimage-wasm-ready', handler); resolve() }
            window.addEventListener('rustimage-wasm-ready', handler)
            // 超时兜底
            setTimeout(() => { resolve() }, 3000)
          })
        }
        setReady(true)
      } catch (e: any) {
        setError(e?.message || 'Failed to load WASM module')
        setReady(false)
      }
    }

    load()
  }, [])

  return { ready, error }
}
