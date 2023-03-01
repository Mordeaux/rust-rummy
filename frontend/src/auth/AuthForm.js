import { useState } from 'react'
import Col from 'react-bootstrap/Col'
import Stack from 'react-bootstrap/Stack'
import Form from 'react-bootstrap/Form'
import Button from 'react-bootstrap/Button'
import { useLocation } from 'react-router-dom'

const AuthForm = () => {
  const { Label } = Form
  const [ username, setUsername ] = useState('')
  const [ password, setPassword ] = useState('')
  const { pathname } = useLocation()

  const onSubmit = e => {
    e.preventDefault()
    const response = fetch(
      `http://127.0.0.1:5000/auth${pathname}`,
      {
        method: 'POST',
        mode: 'cors',
        cache: 'no-cache',
        credentials: 'same-origin',
        headers: {
          'Content-Type': 'application/json',
        },
        redirect: 'follow',
        referrerPolicy: 'no-referrer',
        body: JSON.stringify({ username, password }),
      },
    )
      .then(response => response.json())
      .then(console.log)

  }
  return (
    <Form onSubmit={onSubmit}>
      <Stack direction="vertical" gap={3}>
        <Col>
          <Label>
            Username:
            <input
              type="text"
              name="username"
              value={username}
              onChange={e => setUsername(e.target.value)}
            />
          </Label>
        </Col>
        <Col>
          <Label>
            Password:
            <input
              type="password"
              name="password"
              value={password}
              onChange={e => setPassword(e.target.value)}
            />
          </Label>
        </Col>
        <Col>
          <Button variant="primary" type="submit">
            { pathname === '/signup' ?
                'Sign Up' :
                'Login'
            }
          </Button>
        </Col>
      </Stack>
    </Form>

  )
}

export default AuthForm
