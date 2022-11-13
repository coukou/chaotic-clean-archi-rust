import { useCallback, useState } from "react"
import { authenticate } from "auth-module-wasm"

function Auth() {
    const [email, setEmail] = useState('')
    const [password, setPassword] = useState('')

    const callAuth = useCallback(async () => {
        let token = await authenticate(email, password)
        console.log({ token })
    }, [email, password])

    return (
        <section>
            <h1>Auth</h1>
            <div><input type="text" placeholder='email' value={email} onChange={evt => setEmail(evt.target.value)}></input></div>
            <div><input type="password" placeholder='password' value={password} onChange={evt => setPassword(evt.target.value)}></input></div>
            <div><button onClick={callAuth}>authenticate</button></div>
        </section>
    )
}

export default Auth;