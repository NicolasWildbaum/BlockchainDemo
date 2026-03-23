type Props = {
  apiOk: boolean | null
  loading: boolean
  error: string | null
}

export function StatusBanner({ apiOk, loading, error }: Props) {
  if (error) {
    return (
      <div className="rounded-lg border border-rose-500/40 bg-rose-950/40 px-4 py-3 text-sm text-rose-100">
        API: {error} (¿está el backend en <code className="text-rose-200">127.0.0.1:3000</code>?)
      </div>
    )
  }
  if (loading && apiOk === null) {
    return <div className="text-sm text-slate-400">Conectando con la API…</div>
  }
  if (apiOk) {
    return (
      <div className="rounded-lg border border-emerald-500/30 bg-emerald-950/30 px-4 py-2 text-sm text-emerald-100">
        API en línea
      </div>
    )
  }
  return null
}
