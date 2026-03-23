import type { ConfirmedTransaction } from '../types'

type Props = { transactions: ConfirmedTransaction[]; title?: string }

export function TransactionList({ transactions, title = 'Transacciones' }: Props) {
  return (
    <div>
      <h3 className="mb-1 text-xs font-medium uppercase text-slate-500">{title}</h3>
      {transactions.length === 0 ? (
        <p className="text-xs text-slate-600">Ninguna</p>
      ) : (
        <ul className="space-y-1 text-xs text-slate-300">
          {transactions.map((t) => (
            <li key={t.id} className="font-mono text-[11px]">
              {t.from_account_id}→{t.to_account_id} · {t.amount}
            </li>
          ))}
        </ul>
      )}
    </div>
  )
}
