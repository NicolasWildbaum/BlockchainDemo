export type TxHistoryKind = 'sent' | 'received' | 'coinbase_reward'

export interface TxHistoryEntry {
  tx_id: string
  kind: TxHistoryKind
  counterparty_id: number | null
  amount: number
  block_index: number
  timestamp: string
}

export interface Account {
  id: number
  name: string
  balance: number
  address: string
  transaction_history: TxHistoryEntry[]
}

export interface PendingTransaction {
  id: string
  from_account_id: number
  to_account_id: number
  amount: number
  created_at: string
}

export interface CoinbaseTx {
  tx_id: string
  miner_account_id: number
  amount: number
}

export interface ConfirmedTransaction {
  id: string
  from_account_id: number
  to_account_id: number
  amount: number
  timestamp: string
  block_index: number
}

export interface Block {
  index: number
  timestamp: string
  nonce: number
  previous_hash: string
  hash: string
  difficulty: number
  coinbase: CoinbaseTx
  transactions: ConfirmedTransaction[]
}

export interface ChainValidation {
  valid: boolean
  issues: string[]
}
