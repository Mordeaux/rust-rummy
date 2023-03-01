
const headers = { 'Content-Type': 'application/json', }

const baseURL = 'http://127.0.0.1:5000'

const settings = {
  mode: 'cors',
  credentials: 'include',
  cache: 'no-cache',
  referrerPolicy: 'no-referrer',
  redirect: 'follow',

}

const get = path => {
  return fetch(
    `${baseURL}${path}`,
    {
      ...settings,
      headers,
    },
  )
}

export const getCurrentUser = get.bind(undefined, '/auth/user')

export const postJson = (path, data) => {
  return fetch(
    `${baseURL}${path}`,
    {
      method: 'POST',
      ...settings,
      headers,
      body: JSON.stringify(data),
    },
  )
}

