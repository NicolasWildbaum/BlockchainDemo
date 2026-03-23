import { create } from 'zustand'

import * as api from '../api/endpoints'
import type { Account, Block, PendingTransaction } from '../types'

export type DemoStore = {
  apiOk: boolean | null
  accounts: Account[]
  blocks: Block[]
  pending: PendingTransaction[]
  validation: { valid: boolean; issues: string[] } | null
  loading: boolean
  error: string | null
  refresh: () => Promise<void>
  setValidation: (v: { valid: boolean; issues: string[] } | null) => void
}

export const useDemoStore = create<DemoStore>((set) => ({
  apiOk: null,
  accounts: [],
  blocks: [],
  pending: [],
  validation: null,
  loading: false,
  error: null,
  setValidation: (validation) => set({ validation }),
  refresh: async () => {
    set({ loading: true, error: null })
    try {
      const [h, accounts, blocks, pending] = await Promise.all([
        api.health(),
        api.listAccounts(),
        api.listBlocks(),
        api.listPending(),
      ])
      set({
        apiOk: h.status === 'ok',
        accounts,
        blocks,
        pending,
        loading: false,
      })
    } catch (e) {
      set({
        loading: false,
        error: e instanceof Error ? e.message : 'unknown error',
        apiOk: false,
      })
    }
  },
}))
