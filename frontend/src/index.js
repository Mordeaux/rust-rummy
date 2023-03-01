import React from 'react'
import ReactDOM from 'react-dom/client'
import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom"
import './index.css'
import reportWebVitals from './reportWebVitals'
import App from './App'
import { AuthForm } from './auth'


const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
    loader: async () => {
      return fetch(
        'http://127.0.0.1:5000/auth/user',
        {
          mode: 'cors',
          credentials: 'include',
          headers: {
            'Content-Type': 'application/json',
          },
        },
    )
    },
  },
  {
    path: "/login",
    element: <AuthForm />,
  },
  {
    path: "/signup",
    element: <AuthForm />,
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
