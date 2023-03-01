import authFormFactory from './authFormFactory'

const LoginForm = authFormFactory('login')
const SignupForm = authFormFactory('signup')

export {
  LoginForm,
  SignupForm,
}
