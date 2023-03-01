import Row from 'react-bootstrap/Row'
import Container from 'react-bootstrap/Container'
import { Link, useLoaderData } from 'react-router-dom'


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

  return (
    <div className="rummy-app">
      <Container>
        <Row>
          {
            username ? 
              <h1>hello {username}</h1>
              : <Unauthenticated />
          }
        </Row>
      </Container>
    </div>
  )
}


export default App
