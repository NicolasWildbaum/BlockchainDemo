import { useState } from 'react'

import * as api from '../api/endpoints'
import { AccountPanel } from '../components/AccountPanel'
import { BlockCard } from '../components/BlockCard'
import { MempoolPanel } from '../components/MempoolPanel'
import { MinePanel } from '../components/MinePanel'
import { StatusBanner } from '../components/StatusBanner'
import { useDashboardInit } from '../hooks/useDashboardInit'
import { useDemoStore } from '../store/useDemoStore'

export default function Dashboard() {
  useDashboardInit()
  const { apiOk, accounts, blocks, pending, loading, error, validation, refresh, setValidation } =
    useDemoStore()
  const [txFrom, setTxFrom] = useState(1)
  const [txTo, setTxTo] = useState(2)
  const [txAmt, setTxAmt] = useState(10)
  const [txMsg, setTxMsg] = useState<string | null>(null)
  const [txBusy, setTxBusy] = useState(false)

  async function onCreateTx() {
    setTxBusy(true)
    setTxMsg(null)
    try {
      await api.createTransaction({
        from_account_id: txFrom,
        to_account_id: txTo,
        amount: txAmt,
      })
      setTxMsg('Transacción enviada al mempool')
      await refresh()
    } catch (e) {
      setTxMsg(e instanceof Error ? e.message : 'error')
    } finally {
      setTxBusy(false)
    }
  }

  async function onValidate() {
    try {
      const v = await api.validateChain()
      setValidation(v)
    } catch (e) {
      setValidation({
        valid: false,
        issues: [e instanceof Error ? e.message : 'error'],
      })
    }
  }

  async function onReset() {
    try {
      await api.resetDemo()
      setValidation(null)
      await refresh()
    } catch (e) {
      setValidation({
        valid: false,
        issues: [e instanceof Error ? e.message : 'error'],
      })
    }
  }

  return (
    <div className="mx-auto flex min-h-screen max-w-7xl flex-col gap-6 px-4 py-8">
      <header className="flex flex-col gap-2 border-b border-slate-800 pb-6">
        <p className="text-xs font-medium uppercase tracking-widest text-sky-400/90">
          Simulación educativa
        </p>
        <h1 className="text-3xl font-semibold tracking-tight text-white">Blockchain demo</h1>
        <p className="max-w-2xl text-sm text-slate-400">
          Rust + Axum en el backend, React + Vite en el frontend. Estado en memoria, PoW por prefijo
          hex y mempool antes de minar.
        </p>
        <StatusBanner apiOk={apiOk} loading={loading} error={error} />
      </header>

      <div className="grid gap-6 lg:grid-cols-[minmax(0,320px)_1fr]">
        <aside className="flex flex-col gap-4">
          <AccountPanel accounts={accounts} />
          <MempoolPanel pending={pending} />
          <section className="rounded-xl border border-slate-800 bg-slate-900/50 p-4">
            <h2 className="mb-3 text-sm font-semibold uppercase tracking-wide text-slate-400">
              Nueva transacción
            </h2>
            <div className="grid grid-cols-3 gap-2 text-xs">
              <label className="text-slate-500">
                Desde
                <input
                  type="number"
                  min={1}
                  max={6}
                  value={txFrom}
                  onChange={(e) => setTxFrom(Number(e.target.value))}
                  className="mt-1 w-full rounded border border-slate-700 bg-slate-950 px-2 py-1"
                />
              </label>
              <label className="text-slate-500">
                Hacia
                <input
                  type="number"
                  min={1}
                  max={6}
                  value={txTo}
                  onChange={(e) => setTxTo(Number(e.target.value))}
                  className="mt-1 w-full rounded border border-slate-700 bg-slate-950 px-2 py-1"
                />
              </label>
              <label className="text-slate-500">
                Monto
                <input
                  type="number"
                  min={1}
                  value={txAmt}
                  onChange={(e) => setTxAmt(Number(e.target.value))}
                  className="mt-1 w-full rounded border border-slate-700 bg-slate-950 px-2 py-1"
                />
              </label>
            </div>
            <button
              type="button"
              disabled={txBusy}
              onClick={() => void onCreateTx()}
              className="mt-3 w-full rounded-lg bg-slate-100 px-4 py-2 text-sm font-medium text-slate-900 hover:bg-white disabled:opacity-50"
            >
              {txBusy ? 'Enviando…' : 'Añadir al mempool'}
            </button>
            {txMsg && <p className="mt-2 text-xs text-slate-400">{txMsg}</p>}
          </section>
          <MinePanel />
          <div className="flex flex-wrap gap-2">
            <button
              type="button"
              onClick={() => void onValidate()}
              className="rounded-lg border border-slate-600 px-4 py-2 text-sm text-slate-200 hover:bg-slate-800"
            >
              Validar cadena
            </button>
            <button
              type="button"
              onClick={() => void onReset()}
              className="rounded-lg border border-rose-500/40 px-4 py-2 text-sm text-rose-200 hover:bg-rose-950/40"
            >
              Reset demo
            </button>
          </div>
          {validation && (
            <div
              className={`rounded-lg border px-3 py-2 text-xs ${
                validation.valid
                  ? 'border-emerald-500/40 bg-emerald-950/30 text-emerald-100'
                  : 'border-rose-500/40 bg-rose-950/30 text-rose-100'
              }`}
            >
              <p className="font-semibold">{validation.valid ? 'Cadena válida' : 'Cadena inválida'}</p>
              {validation.issues.length > 0 && (
                <ul className="mt-1 list-inside list-disc text-[11px] opacity-90">
                  {validation.issues.map((issue) => (
                    <li key={issue}>{issue}</li>
                  ))}
                </ul>
              )}
            </div>
          )}
        </aside>

        <main>
          <h2 className="mb-4 text-sm font-semibold uppercase tracking-wide text-slate-400">
            Cadena (flujo ← anterior · más reciente →)
          </h2>
          <div className="flex gap-4 overflow-x-auto pb-4">
            {blocks.map((b) => (
              <BlockCard key={b.index} block={b} />
            ))}
          </div>
        </main>
      </div>
    </div>
  )
}
