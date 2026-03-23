import { useCallback, useEffect, useRef, useState } from 'react'

import * as api from './api/client'

function truncate(s: string, n = 10) {
  if (s.length <= n * 2) return s
  return `${s.slice(0, n)}…${s.slice(-n)}`
}

export default function App() {
  const [blocks, setBlocks] = useState<api.Block[]>([])
  const [users, setUsers] = useState<api.User[]>([])
  const [mempool, setMempool] = useState<api.TransferTx[]>([])
  const [report, setReport] = useState<api.ValidationReport | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [busy, setBusy] = useState(false)
  const [minerId, setMinerId] = useState('nico')

  const [txFrom, setTxFrom] = useState('nico')
  const [txTo, setTxTo] = useState('martin')
  const [txAmount, setTxAmount] = useState(10)
  const [demoBadSig, setDemoBadSig] = useState(false)

  const [drafts, setDrafts] = useState<Record<number, { data: string; nonce: string }>>({})
  const timers = useRef<Record<number, ReturnType<typeof setTimeout>>>({})
  const blocksRef = useRef(blocks)
  blocksRef.current = blocks

  const refresh = useCallback(async () => {
    setError(null)
    const [b, v, u, m] = await Promise.all([
      api.getBlocks(),
      api.getValidate(),
      api.getUsers(),
      api.getTransactionsPending(),
    ])
    setBlocks(b)
    setReport(v)
    setUsers(u)
    setMempool(m)
    const d: Record<number, { data: string; nonce: string }> = {}
    b.forEach((block, i) => {
      d[i] = { data: block.data, nonce: String(block.nonce) }
    })
    setDrafts(d)
  }, [])

  useEffect(() => {
    void refresh().catch((e) => setError(e instanceof Error ? e.message : 'error'))
  }, [refresh])

  function scheduleSave(index: number, next: { data: string; nonce: string }) {
    if (timers.current[index]) clearTimeout(timers.current[index])
    timers.current[index] = setTimeout(() => {
      const block = blocksRef.current[index]
      if (!block) return
      const body: { data?: string; nonce?: number } = {}
      if (next.data !== block.data) body.data = next.data
      if (next.nonce !== String(block.nonce)) {
        const nonceNum = Number(next.nonce)
        if (!Number.isFinite(nonceNum) || nonceNum < 0) return
        body.nonce = nonceNum
      }
      if (Object.keys(body).length === 0) return
      setBusy(true)
      api
        .putBlock(index, body)
        .then(() => refresh())
        .catch((e) => setError(e instanceof Error ? e.message : 'error'))
        .finally(() => setBusy(false))
    }, 280)
  }

  async function onMine(index: number) {
    setError(null)
    setBusy(true)
    try {
      const next = await api.mineBlock(index, minerId)
      setBlocks(next)
      const [v, u, m] = await Promise.all([
        api.getValidate(),
        api.getUsers(),
        api.getTransactionsPending(),
      ])
      setReport(v)
      setUsers(u)
      setMempool(m)
      const d: Record<number, { data: string; nonce: string }> = {}
      next.forEach((block, i) => {
        d[i] = { data: block.data, nonce: String(block.nonce) }
      })
      setDrafts(d)
    } catch (e) {
      setError(e instanceof Error ? e.message : 'error')
    } finally {
      setBusy(false)
    }
  }

  async function onReset() {
    setError(null)
    setBusy(true)
    try {
      await api.postReset()
      await refresh()
    } catch (e) {
      setError(e instanceof Error ? e.message : 'error')
    } finally {
      setBusy(false)
    }
  }

  async function onCreateTx() {
    setError(null)
    const amount = Math.floor(Number(txAmount))
    if (!Number.isFinite(amount) || amount < 1) {
      setError('Monto inválido: usa un entero ≥ 1')
      return
    }
    setBusy(true)
    try {
      await api.postTransaction({
        sender: txFrom,
        recipient: txTo,
        amount,
        demo_invalid_signature: demoBadSig,
      })
      await refresh()
    } catch (e) {
      setError(e instanceof Error ? e.message : 'error')
    } finally {
      setBusy(false)
    }
  }

  const validityByIndex = new Map<number, api.BlockValidity>()
  report?.blocks.forEach((b) => validityByIndex.set(b.index, b))

  return (
    <div className="mx-auto min-h-screen max-w-5xl px-4 py-10">
      <header className="mb-8 border-b border-slate-800 pb-6">
        <div className="flex items-end gap-4">
          <img
            src="/nicocin-logo.svg"
            alt="NICOCIN — protocolo demo"
            className="h-14 w-auto max-w-[min(100%,280px)] sm:h-16"
            width={280}
            height={64}
            decoding="async"
          />
        </div>
        <div className="mt-6 flex flex-wrap items-center gap-3">
          <label className="flex items-center gap-2 text-sm text-slate-400">
            Minero al pulsar Mine
            <select
              value={minerId}
              disabled={busy}
              onChange={(e) => setMinerId(e.target.value)}
              className="rounded-lg border border-slate-600 bg-slate-950 px-2 py-1 text-slate-100"
            >
              {users.map((u) => (
                <option key={u.id} value={u.id}>
                  {u.name} ({u.id})
                </option>
              ))}
            </select>
          </label>
          <button
            type="button"
            disabled={busy}
            onClick={() => void onReset()}
            className="rounded-lg border border-slate-600 px-4 py-2 text-sm text-slate-200 hover:bg-slate-800 disabled:opacity-50"
          >
            Reset cadena
          </button>
          {report && (
            <span
              className={`rounded-full px-3 py-1 text-xs font-semibold uppercase ${
                report.chain_valid
                  ? 'bg-emerald-500/20 text-emerald-300'
                  : 'bg-rose-500/20 text-rose-200'
              }`}
            >
              Cadena: {report.chain_valid ? 'válida' : 'inválida'}
            </span>
          )}
          {busy && <span className="text-xs text-slate-500">Sincronizando…</span>}
        </div>
        {error && (
          <p className="mt-3 rounded-lg border border-rose-500/40 bg-rose-950/40 px-3 py-2 text-sm text-rose-100">
            {error}
          </p>
        )}
      </header>

      <div className="mb-10 grid gap-6 lg:grid-cols-2">
        <section className="rounded-xl border border-slate-800 bg-slate-900/50 p-4">
          <h2 className="mb-3 text-sm font-semibold uppercase tracking-wide text-slate-400">Usuarios</h2>
          <ul className="space-y-2 text-sm">
            {users.map((u) => (
              <li
                key={u.id}
                className="flex flex-col gap-1 rounded-lg bg-slate-950/60 px-3 py-2 text-slate-100"
              >
                <div className="flex justify-between font-medium">
                  <span>
                    {u.name} <span className="text-xs font-normal text-slate-500">({u.id})</span>
                  </span>
                  <span className="tabular-nums text-emerald-300">{u.balance}</span>
                </div>
                <p className="font-mono text-[10px] text-slate-500" title={u.public_key}>
                  pk {truncate(u.public_key, 8)}
                </p>
                <p className="font-mono text-[10px] text-slate-600" title={u.address}>
                  {u.address}
                </p>
              </li>
            ))}
          </ul>
        </section>

        <section className="rounded-xl border border-slate-800 bg-slate-900/50 p-4">
          <h2 className="mb-3 text-sm font-semibold uppercase tracking-wide text-slate-400">
            Mempool + nueva transacción
          </h2>
          <div className="mb-3 flex flex-wrap gap-2 text-xs">
            <label className="text-slate-500">
              Desde
              <select
                value={txFrom}
                disabled={busy}
                onChange={(e) => setTxFrom(e.target.value)}
                className="ml-1 rounded border border-slate-700 bg-slate-950 px-2 py-1 text-slate-100"
              >
                {users.map((u) => (
                  <option key={u.id} value={u.id}>
                    {u.name}
                  </option>
                ))}
              </select>
            </label>
            <label className="text-slate-500">
              Hacia
              <select
                value={txTo}
                disabled={busy}
                onChange={(e) => setTxTo(e.target.value)}
                className="ml-1 rounded border border-slate-700 bg-slate-950 px-2 py-1 text-slate-100"
              >
                {users.map((u) => (
                  <option key={u.id} value={u.id}>
                    {u.name}
                  </option>
                ))}
              </select>
            </label>
            <label className="text-slate-500">
              Monto
              <input
                type="number"
                min={1}
                value={txAmount}
                disabled={busy}
                onChange={(e) => setTxAmount(Number(e.target.value))}
                className="ml-1 w-24 rounded border border-slate-700 bg-slate-950 px-2 py-1 text-slate-100"
              />
            </label>
            <label className="flex cursor-pointer items-center gap-2 text-slate-500">
              <input
                type="checkbox"
                checked={demoBadSig}
                disabled={busy}
                onChange={(e) => setDemoBadSig(e.target.checked)}
                className="rounded border-slate-600"
              />
              Demo: firma inválida
            </label>
            <button
              type="button"
              disabled={busy}
              onClick={() => void onCreateTx()}
              className="rounded-lg bg-slate-100 px-3 py-1 text-xs font-medium text-slate-900 hover:bg-white disabled:opacity-50"
            >
              Añadir al mempool
            </button>
          </div>
          {mempool.length === 0 ? (
            <p className="text-xs text-slate-500">Mempool vacío — al minar un bloque se vaciará al incluirlas.</p>
          ) : (
            <ul className="max-h-40 space-y-1 overflow-y-auto font-mono text-[11px] text-slate-300">
              {mempool.map((t) => (
                <li key={t.id} className="rounded bg-slate-950/60 px-2 py-1">
                  <span className="text-emerald-400/90">
                    {t.status === 'valid_pending' ? '✓ firmada' : `· ${t.status}`}
                  </span>{' '}
                  {t.sender} → {t.recipient} · {t.amount}{' '}
                  <span className="text-slate-600">({truncate(t.id, 6)})</span>
                  <span className="block text-slate-600" title={t.signature_hex}>
                    sig {truncate(t.signature_hex, 6)}
                  </span>
                </li>
              ))}
            </ul>
          )}
        </section>
      </div>

      <h2 className="mb-4 text-sm font-semibold uppercase tracking-wide text-slate-400">Bloques</h2>
      <div className="flex flex-col gap-6">
        {blocks.map((block, i) => {
          const dv = drafts[i] ?? { data: block.data, nonce: String(block.nonce) }
          const v = validityByIndex.get(block.index)
          const ok = v?.valid ?? false
          return (
            <article
              key={`${block.index}-${block.hash}`}
              className={`rounded-xl border p-5 shadow-lg ${
                ok ? 'border-slate-700 bg-slate-900/70' : 'border-rose-600/45 bg-rose-950/15'
              }`}
            >
              <div className="mb-4 flex flex-wrap items-center justify-between gap-2">
                <h3 className="text-lg font-semibold text-white">Bloque #{block.index}</h3>
                <div className="flex items-center gap-2">
                  <span
                    className={`rounded-full px-2 py-0.5 text-[10px] font-bold uppercase ${
                      ok ? 'bg-emerald-500/25 text-emerald-200' : 'bg-rose-500/25 text-rose-100'
                    }`}
                  >
                    {ok ? 'válido' : 'inválido'}
                  </span>
                  <button
                    type="button"
                    disabled={busy}
                    onClick={() => void onMine(i)}
                    className="rounded-lg bg-sky-600 px-3 py-1.5 text-sm font-medium text-white hover:bg-sky-500 disabled:opacity-50"
                  >
                    Mine
                  </button>
                </div>
              </div>

              {block.coinbase && (
                <div className="mb-4 rounded-lg border-2 border-amber-400/50 bg-amber-950/40 px-3 py-3">
                  <p className="text-[10px] font-bold uppercase tracking-wider text-amber-300">
                    {block.coinbase.label}
                  </p>
                  <p className="mt-1 font-mono text-sm text-amber-100">
                    <span className="text-amber-400/80">emisor</span> {block.coinbase.sender}{' '}
                    <span className="text-amber-400/80">→</span> {block.coinbase.recipient}{' '}
                    <span className="text-amber-400/80">monto</span> {block.coinbase.amount}
                  </p>
                  <p className="mt-1 text-[11px] text-amber-200/70">
                    Recompensa de bloque (coinbase). No es una transferencia firmada por un usuario: la crea el protocolo
                    al minar.
                  </p>
                </div>
              )}

              <div className="mb-4">
                <h4 className="mb-1 text-[10px] font-semibold uppercase text-slate-500">Transacciones en el bloque</h4>
                {(block.transactions ?? []).length === 0 ? (
                  <p className="text-xs text-slate-600">Ninguna</p>
                ) : (
                  <ul className="space-y-1 font-mono text-[11px] text-slate-300">
                    {(block.transactions ?? []).map((t) => (
                      <li key={t.id} className="rounded bg-slate-950/50 px-2 py-1">
                        <span className="text-emerald-400/80">firmada</span> {t.sender} → {t.recipient} · {t.amount}
                        <span className="block text-slate-600">sig {truncate(t.signature_hex, 6)}</span>
                      </li>
                    ))}
                  </ul>
                )}
              </div>

              <div className="grid gap-4 sm:grid-cols-2">
                <label className="block text-xs text-slate-500">
                  Data
                  <textarea
                    value={dv.data}
                    disabled={busy}
                    rows={2}
                    onChange={(e) => {
                      const data = e.target.value
                      setDrafts((prev) => ({ ...prev, [i]: { ...dv, data } }))
                      scheduleSave(i, { data, nonce: dv.nonce })
                    }}
                    className="mt-1 w-full rounded-lg border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100"
                  />
                </label>
                <label className="block text-xs text-slate-500">
                  Nonce
                  <input
                    type="number"
                    min={0}
                    value={dv.nonce}
                    disabled={busy}
                    onChange={(e) => {
                      const nonce = e.target.value
                      setDrafts((prev) => ({ ...prev, [i]: { ...dv, nonce } }))
                      scheduleSave(i, { data: dv.data, nonce })
                    }}
                    className="mt-1 w-full rounded-lg border border-slate-700 bg-slate-950 px-3 py-2 text-sm text-slate-100"
                  />
                </label>
              </div>

              <dl className="mt-4 grid gap-2 text-xs text-slate-400 sm:grid-cols-2">
                <div>
                  <dt className="text-slate-500">Previous hash</dt>
                  <dd className="break-all font-mono text-[11px] text-slate-300" title={block.previous_hash}>
                    {truncate(block.previous_hash, 12)}
                  </dd>
                </div>
                <div>
                  <dt className="text-slate-500">Hash</dt>
                  <dd className="break-all font-mono text-[11px] text-sky-300" title={block.hash}>
                    {truncate(block.hash, 14)}
                  </dd>
                </div>
              </dl>

              {!ok && v && v.reasons.length > 0 && (
                <ul className="mt-3 list-inside list-disc text-[11px] text-rose-200/90">
                  {v.reasons.map((r) => (
                    <li key={r}>{r}</li>
                  ))}
                </ul>
              )}
            </article>
          )
        })}
      </div>
    </div>
  )
}
