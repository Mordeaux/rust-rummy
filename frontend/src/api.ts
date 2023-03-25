import {GameState, User} from './apiTypes'

const headers = { 'Content-Type': 'application/json' }

const baseURL = 'http://127.0.0.1:5000'

const settings = {
  mode: 'cors',
  credentials: 'include',
  cache: 'no-cache',
  referrerPolicy: 'no-referrer',
  redirect: 'follow',
}

const getJson = <T>(response: Response): Promise<T> => {
           if (!response.ok) {
        throw new Error(response.statusText)
      }
      return response.json() as Promise<T>
    }

export const get = <T>(path: string): Promise<T> => {
  return fetch(`${baseURL}${path}`, {
    ...settings,
    headers,
  } as RequestInit)
  .then(getJson<T>)
}

export const getCurrentUser = get.bind(undefined, '/auth/user')

export const getGames = () => get<Array<GameState>>('/games/')
export const getGame = (gameId: string = '') => get<GameState>(`/games/${gameId}`)

export const postJson = <T>(path: string, data: Object) => {
  return fetch(`${baseURL}${path}`, {
    method: 'POST',
    ...settings,
    headers,
    body: JSON.stringify(data),
  } as RequestInit)
  .then(getJson<T>)
}

export const login = (pathname: string, username: string, password: string) => {
  return postJson<User>(`/auth${pathname}`, { username, password })
}

export const logout = get.bind(undefined, '/auth/logout')

export const draw = (gameId: string = '') => get<GameState>(`/games/${gameId}/draw`)
