import React from 'react'
import ReactDOM from 'react-dom/client'
import { createBrowserRouter, RouterProvider } from 'react-router-dom'
import './index.css'
import reportWebVitals from './reportWebVitals'
import App from './App'
import { AuthForm } from './auth'
import { getCurrentUser, getGames, getGame } from './api'
import { GameView } from './game'
import wasmPromise from './rummy-wasm'

const router = createBrowserRouter([
  {
    path: '/',
    element: <App />,
    loader: async () =>
      Promise.all([getCurrentUser(), getGames()])
        .then(([user, games]) =>
          wasmPromise.then((wasm) => {
            const newGames = games.map((game) =>
              game.available_moves
                ? game
                : JSON.parse(wasm.getAvailableMoves(JSON.stringify(game)))
            )
            return { user, games: newGames }
          })
        )
        .catch(() => {
          return { user: {}, games: [] }
        }),
  },
  {
    path: '/login',
    element: <AuthForm />,
  },
  {
    path: '/signup',
    element: <AuthForm />,
  },
  {
    path: '/games/:gameId',
    element: <GameView />,
    loader: async ({ params: { gameId } }) =>
      getGame(gameId).then((game) =>
        wasmPromise.then((wasm) =>
          game.available_moves
            ? game
            : JSON.parse(wasm.getAvailableMoves(JSON.stringify(game)))
        )
      ),
  },
])

// @ts-ignore
const root = ReactDOM.createRoot(document.getElementById('root'))
root.render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
)

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals()
