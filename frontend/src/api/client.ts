const PREFIX = '/api'

export type CoinbaseTx = {
  label: string
  sender: string
  recipient: string
  amount: number
  /** Emisión del protocolo (NICOCIN); no es firma de usuario. */
  is_protocol_coinbase?: boolean
}

export type TransferTx = {
  id: string
  sender: string
  recipient: string
  amount: number
  timestamp_ms: number
  public_key: string
  signature_hex: string
  status: string
}

export type Block = {
  index: number
  nonce: number
  data: string
  previous_hash: string
  hash: string
  transactions: TransferTx[]
  coinbase?: CoinbaseTx | null
}

export type User = {
  id: string
  name: string
  balance: number
  public_key: string
  address: string
}

export type BlockValidity = {
  index: number
  valid: boolean
  reasons: string[]
}

export type ValidationReport = {
  chain_valid: boolean
  blocks: BlockValidity[]
}

async function parseErr(res: Response): Promise<string> {
  try {
    const j = (await res.json()) as { error?: string }
    return j.error ?? res.statusText
  } catch {
    return res.statusText
  }
}

export async function getBlocks(): Promise<Block[]> {
  const res = await fetch(`${PREFIX}/blocks`)
  if (!res.ok) throw new Error(await parseErr(res))
  return res.json() as Promise<Block[]>
}

export async function getUsers(): Promise<User[]> {
  const res = await fetch(`${PREFIX}/users`)
  if (!res.ok) throw new Error(await parseErr(res))
  return res.json() as Promise<User[]>
}

export async function getMempool(): Promise<TransferTx[]> {
  const res = await fetch(`${PREFIX}/mempool`)
  if (!res.ok) throw new Error(await parseErr(res))
  return res.json() as Promise<TransferTx[]>
}

/** Alias de GET /api/transactions/pending (misma respuesta que /mempool). */
export async function getTransactionsPending(): Promise<TransferTx[]> {
  const res = await fetch(`${PREFIX}/transactions/pending`)
  if (!res.ok) throw new Error(await parseErr(res))
  return res.json() as Promise<TransferTx[]>
}

export async function postTransaction(body: {
  sender: string
  recipient: string
  amount: number
  demo_invalid_signature?: boolean
}): Promise<TransferTx> {
  const res = await fetch(`${PREFIX}/transactions`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!res.ok) throw new Error(await parseErr(res))
  return res.json() as Promise<TransferTx>
}

export async function putBlock(
  index: number,
  body: { data?: string; nonce?: number },
): Promise<Block> {
  const res = await fetch(`${PREFIX}/blocks/${index}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!res.ok) throw new Error(await parseErr(res))
  return res.json() as Promise<Block>
}

export async function mineBlock(index: number, minerId: string): Promise<Block[]> {
  const q = new URLSearchParams({ miner_id: minerId })
  const res = await fetch(`${PREFIX}/blocks/${index}/mine?${q}`, { method: 'POST' })
  if (!res.ok) throw new Error(await parseErr(res))
  return res.json() as Promise<Block[]>
}

export async function getValidate(): Promise<ValidationReport> {
  const res = await fetch(`${PREFIX}/validate`)
  if (!res.ok) throw new Error(await parseErr(res))
  return res.json() as Promise<ValidationReport>
}

export async function postReset(): Promise<Block[]> {
  const res = await fetch(`${PREFIX}/reset`, { method: 'POST' })
  if (!res.ok) throw new Error(await parseErr(res))
  return res.json() as Promise<Block[]>
}
