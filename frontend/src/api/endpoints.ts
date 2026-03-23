import type { Account, Block, ChainValidation, PendingTransaction } from '../types'
import { getJson, postJson } from './client'

export function health() {
  return getJson<{ status: string }>('/health')
}

export function listAccounts() {
  return getJson<Account[]>('/accounts')
}

export function listPending() {
  return getJson<PendingTransaction[]>('/transactions/pending')
}

export function listBlocks() {
  return getJson<Block[]>('/blocks')
}

export function validateChain() {
  return getJson<ChainValidation>('/blockchain/validate')
}

export function createTransaction(body: {
  from_account_id: number
  to_account_id: number
  amount: number
}) {
  return postJson<typeof body, PendingTransaction>('/transactions', body)
}

export function mine(body: {
  miner_account_id: number
  difficulty?: number
  max_transactions?: number
}) {
  return postJson<typeof body, { block: Block; included_transaction_ids: string[] }>('/mine', body)
}

export function resetDemo() {
  return postJson<Record<string, never>, { ok: boolean }>('/reset-demo', {})
}
