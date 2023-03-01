import Row from 'react-bootstrap/Row'
import Container from 'react-bootstrap/Container'
import { Link } from 'react-router-dom'


const App = () => {
  return (
    <div className="rummy-app">
      <Container>
        <Row>
          <Link to='/login'>Login</Link>
          - or -
          <Link to='/signup'>Sign Up</Link>
        </Row>
      </Container>
    </div>
  )
}


export default App
