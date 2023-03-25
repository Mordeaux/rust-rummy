import Row from 'react-bootstrap/Row'
import Container from 'react-bootstrap/Container'
import { Link, useLoaderData } from 'react-router-dom'
import { logout } from './api'
import { GameList } from './game'
import { User, GameState } from './apiTypes'

const Unauthenticated = () => {
  return (
    <>
      <Link to="/login">Login</Link>- or -<Link to="/signup">Sign Up</Link>
    </>
  )
}

const LandingPage = () => {
  const {
    user,
    games,
  } = useLoaderData() as { user: User, games: GameState[]}
  return (
    <>
      <h1>Hello {user.username}</h1>
      <span onClick={logout}>logout</span>
      <GameList games={games} user={user} />
    </>
  )
}

const App = () => {
  const {
    user: { username },
  } = useLoaderData() as { user: User, games: GameState[]}
  return (
    <div className="rummy-app">
      <Container>
        <Row>{username ? <LandingPage /> : <Unauthenticated />}</Row>
      </Container>
    </div>
  )
}

export default App
