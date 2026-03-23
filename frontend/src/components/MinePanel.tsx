import { useState } from 'react'

import * as api from '../api/endpoints'
import { useDemoStore } from '../store/useDemoStore'

export function MinePanel() {
  const refresh = useDemoStore((s) => s.refresh)
  const [minerId, setMinerId] = useState(1)
  const [busy, setBusy] = useState(false)
  const [msg, setMsg] = useState<string | null>(null)

  async function onMine() {
    setBusy(true)
    setMsg(null)
    try {
      const res = await api.mine({ miner_account_id: minerId })
      setMsg(`Bloque #${res.block.index} minado · ${res.included_transaction_ids.length} txs`)
      await refresh()
    } catch (e) {
      setMsg(e instanceof Error ? e.message : 'error')
    } finally {
      setBusy(false)
    }
  }

  return (
    <section className="rounded-xl border border-slate-800 bg-slate-900/50 p-4">
      <h2 className="mb-3 text-sm font-semibold uppercase tracking-wide text-slate-400">Minar</h2>
      <div className="flex flex-wrap items-end gap-2">
        <label className="text-xs text-slate-500">
          Minero (id)
          <input
            type="number"
            min={1}
            max={6}
            value={minerId}
            onChange={(e) => setMinerId(Number(e.target.value))}
            className="ml-2 w-16 rounded border border-slate-700 bg-slate-950 px-2 py-1 text-slate-100"
          />
        </label>
        <button
          type="button"
          disabled={busy}
          onClick={() => void onMine()}
          className="rounded-lg bg-sky-600 px-4 py-2 text-sm font-medium text-white hover:bg-sky-500 disabled:opacity-50"
        >
          {busy ? 'Minando…' : 'Minar bloque'}
        </button>
      </div>
      {msg && <p className="mt-2 text-xs text-slate-400">{msg}</p>}
    </section>
  )
}
