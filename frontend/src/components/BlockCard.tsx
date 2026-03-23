import type { Block } from '../types'

import { TransactionList } from './TransactionList'

type Props = { block: Block; valid?: boolean }

function truncateHash(h: string, n = 12) {
  if (h.length <= n * 2) return h
  return `${h.slice(0, n)}…${h.slice(-n)}`
}

export function BlockCard({ block, valid = true }: Props) {
  return (
    <article
      className={`flex min-w-[280px] max-w-xs shrink-0 flex-col gap-3 rounded-xl border p-4 shadow-lg ${
        valid
          ? 'border-slate-700 bg-slate-900/80'
          : 'border-rose-600/50 bg-rose-950/20'
      }`}
    >
      <header className="flex items-center justify-between gap-2">
        <span className="text-lg font-semibold text-white">#{block.index}</span>
        <span
          className={`rounded-full px-2 py-0.5 text-[10px] font-semibold uppercase ${
            valid ? 'bg-emerald-500/20 text-emerald-300' : 'bg-rose-500/20 text-rose-200'
          }`}
        >
          {valid ? 'valid' : 'invalid'}
        </span>
      </header>
      <dl className="grid grid-cols-[auto_1fr] gap-x-2 gap-y-1 text-[11px] text-slate-400">
        <dt>nonce</dt>
        <dd className="font-mono text-slate-200">{block.nonce}</dd>
        <dt>diff</dt>
        <dd className="font-mono text-slate-200">{block.difficulty}</dd>
        <dt>prev</dt>
        <dd className="break-all font-mono text-slate-300" title={block.previous_hash}>
          {truncateHash(block.previous_hash)}
        </dd>
        <dt>hash</dt>
        <dd className="break-all font-mono text-sky-300" title={block.hash}>
          {truncateHash(block.hash)}
        </dd>
      </dl>
      <div className="rounded-lg border border-amber-500/20 bg-amber-950/20 p-2">
        <p className="text-[10px] font-semibold uppercase text-amber-200/80">Coinbase</p>
        <p className="font-mono text-xs text-amber-100">
          → miner {block.coinbase.miner_account_id} · +{block.coinbase.amount}
        </p>
      </div>
      <TransactionList transactions={block.transactions} />
    </article>
  )
}
