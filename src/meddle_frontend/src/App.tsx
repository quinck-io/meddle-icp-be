// import { ButtonHTMLAttributes, useState } from 'react';
// import { meddle_backend } from '../../declarations/meddle_backend';
// import { LoginButton } from './components/LoginButton';
// import { Actor, ActorMethod, HttpAgent } from "@dfinity/agent";
// import { AuthClient } from "@dfinity/auth-client";
// import type { Principal } from "@dfinity/principal";


// function App() {
//   const [greeting, setGreeting] = useState('');

//   function handleSubmit(event: React.MouseEvent<HTMLFormElement, MouseEvent>) {
//     event.preventDefault();
//     const name = document.getElementById('name') as HTMLInputElement;
//     meddle_backend.greet(name.value).then((greeting) => {
//       setGreeting(greeting);
//     });
//     return false;
//   }

//   return (
//     <main>
//       <img src="/logo2.svg" alt="DFINITY logo" />
//       <br />
//       <br />
//       <LoginButton></LoginButton>
//       <form action="#" onClick={handleSubmit}>
//         <label htmlFor="name">Enter your name: &nbsp;</label>
//         <input id="name" alt="Name" type="text" />
//         <button type="submit">Click Me!</button>
//       </form>
//       <section id="greeting">{greeting}</section>
//     </main>
//   );
// }

// export default App;

import { useEffect, useState } from "react";

import { LoginButton } from "./components/LoginButton";
import Principal from "./components/Principal";
import { useBackend } from "./ic/Actors";
import { useInternetIdentity } from "ic-use-internet-identity";
import { AnonymousIdentity } from "@dfinity/agent";

function App() {
  const { identity } = useInternetIdentity();
  const { actor: backend } = useBackend();
  const [principal, setPrincipal] = useState<string>();

  // Clear the principal when the identity is cleared
  useEffect(() => {
    if (!identity) setPrincipal(undefined);
  }, [identity]);

  // Get the principal from the backend when an identity is available
  useEffect(() => {
    if (identity && backend && !principal) {
      backend.whoami().then((p) => setPrincipal(p as any));
    }
  }, [backend, identity, principal]);

  return (
    <div className="flex flex-col items-center w-full gap-5 p-10 font-sans text-2xl italic md:items-start md:gap-10 md:text-6xl">
      <div className="text-center">
        {identity ? "You are logged in." : "You are not logged in."}
      </div>
      <LoginButton />
      <Principal principal={principal} />
    </div>
  );
}

export default App;
