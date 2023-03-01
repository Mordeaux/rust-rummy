import Row from 'react-bootstrap/Row'
import Col from 'react-bootstrap/Col'
import Container from 'react-bootstrap/Container'
import Stack from 'react-bootstrap/Stack'
import Form from 'react-bootstrap/Form'
import Button from 'react-bootstrap/Button'

const LoginForm = () => {
  const { Label } = Form
  const onSubmit = e => {
    e.preventDefault()
    const response = fetch(
      'http://127.0.0.1:5000/login',
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
        body: JSON.stringify({ username: 'hello', password: 'dude', }),
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
            <input type="text" name="username" />
          </Label>
        </Col>
        <Col>
          <Label>
            Password:
            <input type="password" name="password" />
          </Label>
        </Col>
        <Col>
          <Button variant="primary" type="submit">
            Login
          </Button>
        </Col>
      </Stack>
    </Form>

  )
}

function App() {
  return (
    <div className="rummy-app">
      <Container>
        <Row>
          <LoginForm />
        </Row>
      </Container>
    </div>
  )
}

export default App
