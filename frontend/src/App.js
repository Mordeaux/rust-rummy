import Row from 'react-bootstrap/Row'
import Container from 'react-bootstrap/Container'
import { Link, useLoaderData } from 'react-router-dom'
import { get } from './api'


const Unauthenticated = () => {
  return (
    <>
      <Link to='/login'>Login</Link>
      - or -
      <Link to='/signup'>Sign Up</Link>
    </>
  )
}

const App = () => {
  const { username } = useLoaderData()
  const logout = () => {}

  return (
    <div className="rummy-app">
      <Container>
        <Row>
          {
            username ?
              <>
                <h1>hello {username}</h1>
                <a onClick={get.bind(undefined, '/auth/logout')}>logout</a>
              </>
              : <Unauthenticated />
          }
        </Row>
      </Container>
    </div>
  )
}


export default App
