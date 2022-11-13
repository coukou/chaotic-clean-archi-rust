import { useCallback, useState } from "react"
import { register } from "auth-module-wasm"

function Register() {
    const [email, setEmail] = useState('')
    const [password, setPassword] = useState('')

    const callRegister = useCallback(() => {
        register(email, password)
    }, [email, password])

    return (
        <section>
            <h1>Register</h1>
            <div><input type="text" placeholder='email' value={email} onChange={evt => setEmail(evt.target.value)}></input></div>
            <div><input type="password" placeholder='password' value={password} onChange={evt => setPassword(evt.target.value)}></input></div>
            <div><button onClick={callRegister}>register</button></div>
        </section>
    )
}

export default Register;