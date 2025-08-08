import React, { useMemo, useRef, useState } from 'react'
import { useWasmLoader } from '@/hooks/useWasmLoader'
import { conversionService } from '@/services/conversionService'
import type { ConversionOptions, ConversionResult } from '@/services/conversionService'
import toast from 'react-hot-toast'

const SUPPORTED_FORMATS = [
  'jpeg', 'png', 'webp', 'avif', 'bmp', 'tiff', 'gif', 'ico'
]

export function ConversionPage() {
  const { ready, error } = useWasmLoader()
  const [targetFormat, setTargetFormat] = useState<string>('webp')
  const [dragOver, setDragOver] = useState(false)
  const [file, setFile] = useState<File | null>(null)
  const [result, setResult] = useState<ConversionResult | null>(null)
  const [isConverting, setIsConverting] = useState(false)
  const [fromFormat, setFromFormat] = useState<string>('unknown')
  const inputRef = useRef<HTMLInputElement | null>(null)

  const statusText = useMemo(() => {
    if (error) return 'WASM 加载失败'
    if (!ready) return '正在加载 WebAssembly...'
    return 'WASM 已就绪'
  }, [ready, error])

  function handlePick() {
    inputRef.current?.click()
  }

  function handleFileSelected(f: File) {
    setFile(f)
    conversionService
      .detectFormat(f)
      .then(setFromFormat)
      .catch(() => setFromFormat('unknown'))
    setResult(null)
  }

  async function onFiles(e: React.ChangeEvent<HTMLInputElement>) {
    const f = e.target.files?.[0]
    if (f) handleFileSelected(f)
  }

  async function onDrop(e: React.DragEvent) {
    e.preventDefault()
    setDragOver(false)
    const f = e.dataTransfer.files?.[0]
    if (f) handleFileSelected(f)
  }

  async function startConvert() {
    if (!file) {
      toast.error('请先选择文件')
      return
    }
    if (!SUPPORTED_FORMATS.includes(targetFormat)) {
      toast.error('请选择目标格式')
      return
    }
    setIsConverting(true)
    setResult(null)
    try {
      const opts: ConversionOptions = { preserveDimensions: true, preserveColorSpace: true }
      const r = await conversionService.convertImage(
        file,
        fromFormat,
        targetFormat,
        opts,
        (p: { stage?: string; error?: string }) => {
          if (p.stage === 'error' && p.error) toast.error(p.error)
        }
      )
      setResult(r)
      toast.success('转换完成')
    } catch (e: any) {
      toast.error(e?.message || '转换失败')
    } finally {
      setIsConverting(false)
    }
  }

  function download() {
    if (!result) return
    const blob = new Blob([result.data], { type: `image/${targetFormat}` })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `converted.${targetFormat}`
    a.click()
    URL.revokeObjectURL(url)
  }

  return (
    <div className="min-h-screen w-full bg-slate-50 text-slate-900">
      <div className="mx-auto max-w-5xl py-10 px-4">
        <header className="mb-8">
          <h1 className="text-2xl font-semibold">在线文档/图像转换器</h1>
          <p className="text-sm mt-1 text-slate-600">专注格式转换，Rust + WASM 驱动高性能</p>
          <p className="text-xs mt-2">{statusText}</p>
        </header>

        <div
          onDragOver={(e) => { e.preventDefault(); setDragOver(true) }}
          onDragLeave={() => setDragOver(false)}
          onDrop={onDrop}
          className={`border-2 border-dashed rounded-md p-8 text-center transition ${dragOver ? 'border-blue-500 bg-blue-50' : 'border-slate-300 bg-white'}`}
        >
          <input ref={inputRef} onChange={onFiles} type="file" accept="image/*" className="hidden" />
          <p className="mb-3">点击或拖拽文件到此处上传</p>
          <button onClick={handlePick} className="inline-flex items-center rounded bg-blue-600 text-white px-4 py-2 text-sm disabled:opacity-50" disabled={!ready}>选择文件</button>
          {file && (
            <p className="mt-3 text-xs text-slate-600">已选择：{file.name}（检测到格式：{fromFormat}）</p>
          )}
        </div>

        <div className="mt-6 grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="md:col-span-2 bg-white rounded-md border p-4">
            <h2 className="font-medium mb-3">转换设置</h2>
            <div className="flex items-center gap-3">
              <label className="text-sm">目标格式</label>
              <select className="border rounded px-2 py-1 text-sm" value={targetFormat} onChange={(e) => setTargetFormat(e.target.value)}>
                {SUPPORTED_FORMATS.map(f => <option key={f} value={f}>{f.toUpperCase()}</option>)}
              </select>
              <button onClick={startConvert} disabled={!file || !ready || isConverting} className="ml-auto inline-flex items-center rounded bg-emerald-600 text-white px-4 py-2 text-sm disabled:opacity-50">
                {isConverting ? '转换中...' : '开始转换'}
              </button>
            </div>
          </div>
          <div className="bg-white rounded-md border p-4">
            <h2 className="font-medium mb-3">结果与下载</h2>
            <div className="text-sm space-y-2">
              <p>原始大小：{file ? `${(file.size/1024).toFixed(2)} KB` : '-'}</p>
              <p>转换后大小：{result ? `${(result.convertedSize/1024).toFixed(2)} KB` : '-'}</p>
              <p>耗时：{result ? `${result.conversionTimeMs.toFixed(0)} ms` : '-'}</p>
              <p>压缩比：{result ? result.compressionRatio.toFixed(3) : '-'}</p>
              <button onClick={download} disabled={!result} className="inline-flex items-center rounded bg-slate-900 text-white px-3 py-1 text-sm disabled:opacity-50">下载</button>
            </div>
          </div>
        </div>

        <footer className="mt-10 text-center text-xs text-slate-500">
          设计参考 `Conholdate.Conversion` 单页交互，专注上传-&gt;选择目标格式-&gt;转换-&gt;下载流程。
        </footer>
      </div>
    </div>
  )
}

