import { useState } from 'react';
import { meddle_backend, createActor } from 'declarations/meddle_backend';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent } from '@dfinity/agent';

function App() {
  const [greeting, setGreeting] = useState('');

  function handleSubmit(event) {
    event.preventDefault();
    const name = event.target.elements.name.value;
    meddle_backend.greet(name).then((greeting) => {
      setGreeting(greeting);
    });
    return false;
  }

  function handleWhoAmI(event) {
    event.preventDefault();
    meddle_backend.whoami().then((name) => {
      console.log(name.toString());
      setGreeting(name.toString());
    })
  }

  async function login(event) {
    event.preventDefault();
    let authClient = await AuthClient.create();
    // start the login process and wait for it to finish
    await new Promise((resolve) => {
        authClient.login({
            identityProvider:
                process.env.DFX_NETWORK === "ic"
                    ? "https://identity.ic0.app"
                    : `http://127.0.0.1:4943?canisterId=b77ix-eeaaa-aaaaa-qaada-cai&id=by6od-j4aaa-aaaaa-qaadq-cai`,
            onSuccess: resolve,
        });
    });
    const identity = authClient.getIdentity();
    const agent = new HttpAgent({ identity });
    actor = createActor(process.env.CANISTER_ID_II_INTEGRATION_BACKEND, {
        agent,
    });
  }

  return (
    <main>
      <img src="/logo2.svg" alt="DFINITY logo" />
      <br />
      <br />
      <form action="#" onSubmit={handleSubmit}>
        <label htmlFor="name">Enter your name: &nbsp;</label>
        <input id="name" alt="Name" type="text" />
        <button type="submit">Click Me!</button>
      </form>
      <form action="#" onSubmit={handleWhoAmI}>
        <button type="submit">Click Me!</button>
      </form>
      <form action='#' onSubmit={login}>
        <button id="login">Login!</button>
      </form>
      <section id="greeting">{greeting}</section>
    </main>
  );
}

export default App;
