import type { PendingTransaction } from '../types'

type Props = { pending: PendingTransaction[] }

export function MempoolPanel({ pending }: Props) {
  return (
    <section className="rounded-xl border border-slate-800 bg-slate-900/50 p-4">
      <h2 className="mb-3 text-sm font-semibold uppercase tracking-wide text-slate-400">
        Mempool
      </h2>
      {pending.length === 0 ? (
        <p className="text-sm text-slate-500">Sin transacciones pendientes</p>
      ) : (
        <ul className="space-y-2 text-xs text-slate-300">
          {pending.map((t) => (
            <li key={t.id} className="rounded-md bg-slate-950/60 px-2 py-1 font-mono">
              {t.from_account_id} → {t.to_account_id} · {t.amount}
            </li>
          ))}
        </ul>
      )}
    </section>
  )
}
