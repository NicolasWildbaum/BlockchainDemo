import type { Account } from '../types'

type Props = { accounts: Account[] }

export function AccountPanel({ accounts }: Props) {
  return (
    <section className="rounded-xl border border-slate-800 bg-slate-900/50 p-4">
      <h2 className="mb-3 text-sm font-semibold uppercase tracking-wide text-slate-400">
        Cuentas
      </h2>
      <ul className="space-y-2 text-sm">
        {accounts.map((a) => (
          <li
            key={a.id}
            className="flex items-center justify-between gap-4 rounded-lg bg-slate-950/60 px-3 py-2"
          >
            <span className="font-medium text-slate-100">{a.name}</span>
            <span className="tabular-nums text-emerald-300">{a.balance}</span>
          </li>
        ))}
      </ul>
    </section>
  )
}
