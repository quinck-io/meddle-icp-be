import { Actor, ActorMethod, HttpAgent } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import type { Principal } from "@dfinity/principal";
import { isSafari } from 'react-device-detect';
import { _SERVICE } from '../../declarations/meddle_backend/meddle_backend.did';

// @ts-ignore
export const init = ({ IDL }) => {
  return [];
};

function App() {
  const webapp_id = process.env.CANISTER_ID_MEDDLE_BACKEND;

  // @ts-ignore - The interface of the whoami canister
  const webapp_idl =  ({ IDL }) => {
    const OutUnitId = IDL.Record({
      'len' : IDL.Nat32,
      'unit_ids' : IDL.Vec(IDL.Text),
    });
    const Data = IDL.Record({
      'value' : IDL.Float32,
      'sensor_id' : IDL.Text,
      'timestamp' : IDL.Nat64,
      'unit_id' : IDL.Text,
    });
    const OutDataRecords = IDL.Record({
      'len' : IDL.Nat32,
      'data' : IDL.Vec(Data),
    });
    const OperationResult = IDL.Record({
      'code' : IDL.Nat16,
      'message' : IDL.Text,
      'unit_id' : IDL.Vec(IDL.Text),
    });
    const OutputData = IDL.Variant({
      'Ok' : OutDataRecords,
      'Err' : OperationResult,
    });
    const SingleInput = IDL.Record({
      'value' : IDL.Float32,
      'sensorId' : IDL.Text,
      'timestamp' : IDL.Nat64,
      'timestampString' : IDL.Text,
    });
    const JsonInput = IDL.Record({
      'endpoint' : IDL.Text,
      'variables' : IDL.Vec(SingleInput),
    });
    return IDL.Service({
      'get_all_unit_ids' : IDL.Func(
          [IDL.Nat32, IDL.Nat32, IDL.Bool],
          [OutUnitId],
          ['query'],
        ),
      'get_data' : IDL.Func(
          [IDL.Nat32, IDL.Nat32, IDL.Bool],
          [OutDataRecords],
          ['query'],
        ),
      'get_data_by_multiple_ids' : IDL.Func(
          [IDL.Vec(IDL.Text), IDL.Nat32, IDL.Nat32, IDL.Bool],
          [OutputData],
          ['query'],
        ),
      'get_data_by_range' : IDL.Func(
          [IDL.Nat64, IDL.Opt(IDL.Nat64), IDL.Nat32, IDL.Nat32, IDL.Bool],
          [OutDataRecords],
          ['query'],
        ),
      'get_data_by_sensor' : IDL.Func(
          [IDL.Text, IDL.Nat32, IDL.Nat32, IDL.Bool],
          [OutDataRecords],
          ['query'],
        ),
      'get_data_by_sensor_filter' : IDL.Func(
          [IDL.Text, IDL.Float32, IDL.Text, IDL.Nat32, IDL.Nat32, IDL.Bool],
          [OutputData],
          ['query'],
        ),
      'get_record' : IDL.Func(
          [IDL.Text],
          [IDL.Variant({ 'Ok' : IDL.Vec(Data), 'Err' : OperationResult })],
          ['query'],
        ),
      'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
      'post_data' : IDL.Func([IDL.Vec(JsonInput)], [OperationResult], []),
      'whoami' : IDL.Func([], [IDL.Principal], ['query']),
    });
  };

  // The <canisterId>.localhost URL is used as opposed to setting the canister id as a parameter
  // since the latter is brittle with regards to transitively loaded resources.
  const local_ii_url = isSafari ? 
    `http://127.0.0.1:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}` : 
    `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943/`;

  let iiUrl: string;

  if (process.env.DFX_NETWORK === "local") {
    iiUrl = local_ii_url;
  } else if (process.env.DFX_NETWORK === "ic") {
    iiUrl = `https://${process.env.CANISTER_ID_INTERNET_IDENTITY}.ic0.app`;
  } else {
    iiUrl = local_ii_url;
  }

  async function handleLogin() {
      // When the user clicks, we start the login process.
      // First we have to create and AuthClient.
      const authClient = await AuthClient.create();

      // Find out which URL should be used for login.
      const iiUrl = document.querySelector<HTMLInputElement>("#iiUrl")!.value;

      // Call authClient.login(...) to login with Internet Identity. This will open a new tab
      // with the login prompt. The code has to wait for the login process to complete.
      // We can either use the callback functions directly or wrap in a promise.
      await new Promise<void>((resolve, reject) => {
        authClient.login({
          identityProvider: iiUrl,
          onSuccess: resolve,
          onError: reject,
        });
      });

      // At this point we're authenticated, and we can get the identity from the auth client:
      const identity = authClient.getIdentity();
      // Using the identity obtained from the auth client, we can create an agent to interact with the IC.
      const agent = new HttpAgent({ identity } as any);
      // Using the interface description of our webapp, we create an actor that we use to call the service methods.
      const webapp: _SERVICE = Actor.createActor(webapp_idl, {
        agent,
        canisterId: webapp_id!,
      });
      // Call whoami which returns the principal (user id) of the current user.
      const principal = await webapp.whoami();

      // show the principal on the page
      document.getElementById("loginStatus")!.innerText = principal.toText();
  }

  async function handleGreet() {
    const authClient = await AuthClient.create();
    const identity = authClient.getIdentity();
      // Using the identity obtained from the auth client, we can create an agent to interact with the IC.
      const agent = new HttpAgent({ identity } as any);
      // Using the interface description of our webapp, we create an actor that we use to call the service methods.
      const webapp: _SERVICE = Actor.createActor(webapp_idl, {
        agent,
        canisterId: webapp_id!,
      });
    const temp = await webapp.greet(identity.getPrincipal().toString());
      console.log(temp);
  }

  return (
    <main>
      <h1>Internet Identity Demo Webapp</h1>
      <section>
        <label htmlFor="iiUrl">Internet Identity URL:</label>
        <input size={50} id="iiUrl" type="text" value={iiUrl} readOnly/>
      </section>
      <section>
        <button id="loginBtn" onClick={handleLogin}>Login with Internet Identity</button>
      </section>
      <section>
        <button id="loginBtn" onClick={handleGreet}>Greet with Internet Identity</button>
      </section>
      <section id="loginStatus">
        <p>Not logged in</p>
      </section>
    </main>
  );
}

export default App;
