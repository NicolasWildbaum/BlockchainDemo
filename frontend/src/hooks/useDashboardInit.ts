import { useEffect } from 'react'

import { useDemoStore } from '../store/useDemoStore'

export function useDashboardInit() {
  const refresh = useDemoStore((s) => s.refresh)
  useEffect(() => {
    void refresh()
  }, [refresh])
}
