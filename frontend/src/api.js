const headers = { 'Content-Type': 'application/json' }

const baseURL = 'http://127.0.0.1:5000'

const settings = {
  mode: 'cors',
  credentials: 'include',
  cache: 'no-cache',
  referrerPolicy: 'no-referrer',
  redirect: 'follow',
}

export const get = (path) => {
  return fetch(`${baseURL}${path}`, {
    ...settings,
    headers,
  })
}

export const getCurrentUser = get.bind(undefined, '/auth/user')

export const getGames = get.bind(undefined, '/games/')

export const postJson = (path, data) => {
  return fetch(`${baseURL}${path}`, {
    method: 'POST',
    ...settings,
    headers,
    body: JSON.stringify(data),
  })
}

export const login = (pathname, username, password) => {
  return postJson(`/auth${pathname}`, { username, password }).then((response) =>
    response.json()
  )
}

export const logout = get.bind(undefined, '/auth/logout')
