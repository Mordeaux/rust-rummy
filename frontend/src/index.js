import React from 'react'
import ReactDOM from 'react-dom/client'
import { createBrowserRouter, RouterProvider } from 'react-router-dom'
import './index.css'
import reportWebVitals from './reportWebVitals'
import App from './App'
import { AuthForm } from './auth'
import { getCurrentUser, getGames, getGame } from './api'
import { GameView } from './game'

const router = createBrowserRouter([
  {
    path: '/',
    element: <App />,
    loader: async () =>
      Promise.all([getCurrentUser(), getGames()])
        .then((responses) =>
          Promise.all(responses.map((response) => response.json()))
        )
        .then(([user, games]) => {
          return { user, games }
        }).catch(() => {return {user:{},games:[]}}),
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
    loader: async ({ params: { gameId } }) => getGame(gameId),
  },
])

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
