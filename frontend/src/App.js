import Row from 'react-bootstrap/Row'
import Container from 'react-bootstrap/Container'
import { Link, useLoaderData } from 'react-router-dom'
import { logout } from './api'
import { GameList } from './game'


const Unauthenticated = () => {
  return (
    <>
      <Link to='/login'>Login</Link>
      - or -
      <Link to='/signup'>Sign Up</Link>
    </>
  )
}

const LandingPage = () => {
  const { user: { username }, games } = useLoaderData()
  return (
    <>
      <h1>Hello {username}</h1>
      <span onClick={logout}>logout</span>
      <GameList games={games}/>
    </>
  )
}

const App = () => {
  const { user: { username } } = useLoaderData()

  return (
    <div className="rummy-app">
      <Container>
        <Row>
          {
            username ?
              <LandingPage />
              : <Unauthenticated />
          }
        </Row>
      </Container>
    </div>
  )
}


export default App
