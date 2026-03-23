const API_PREFIX = '/api'

async function parseError(res: Response): Promise<string> {
  try {
    const j = (await res.json()) as { error?: string }
    return j.error ?? res.statusText
  } catch {
    return res.statusText
  }
}

export async function getJson<T>(path: string): Promise<T> {
  const res = await fetch(`${API_PREFIX}${path}`)
  if (!res.ok) throw new Error(await parseError(res))
  return res.json() as Promise<T>
}

export async function postJson<TBody extends object, TRes>(
  path: string,
  body: TBody,
): Promise<TRes> {
  const res = await fetch(`${API_PREFIX}${path}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!res.ok) throw new Error(await parseError(res))
  return res.json() as Promise<TRes>
}
