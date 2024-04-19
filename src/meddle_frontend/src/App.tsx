// import { Actor, ActorMethod, HttpAgent } from "@dfinity/agent";
// import { AuthClient } from "@dfinity/auth-client";
// import type { Principal } from "@dfinity/principal";

// const webapp_id = process.env.CANISTER_ID_MEDDLE_BACKEND;
//   // The <canisterId>.localhost URL is used as opposed to setting the canister id as a parameter
//   // since the latter is brittle with regards to transitively loaded resources.
// const local_ii_url = `http://127.0.0.1:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`;

//   // @ts-ignore - The interface of the whoami canister
// const webapp_idl = ({ IDL }) => {
//     return IDL.Service({ whoami: IDL.Func([], [IDL.Principal], ["query"]) });
// };
//   // @ts-ignore
// export const init = ({ IDL }) => {
//   return [];
// };

// export interface _SERVICE {
//   whoami: ActorMethod<[], Principal>;
// }

// function App() {

//   // Autofills the <input> for the II Url to point to the correct canister.
//   document.body.onload = () => {
//     let iiUrl;

//     if (process.env.DFX_NETWORK === "local") {
//       iiUrl = local_ii_url;
//     } else if (process.env.DFX_NETWORK === "ic") {
//       iiUrl = `https://${process.env.CANISTER_ID_INTERNET_IDENTITY}.ic0.app`;
//     } else {
//       // fall back to local
//       iiUrl = local_ii_url;
//     }

//     console.log(webapp_id);

//     document.querySelector<HTMLInputElement>("#iiUrl")!.value = iiUrl;
//   };

//   document.getElementById("loginBtn")?.addEventListener("click", async () => {
//     // When the user clicks, we start the login process.
//     // First we have to create and AuthClient.
//     const authClient = await AuthClient.create();

//     // Find out which URL should be used for login.
//     const iiUrl = document.querySelector<HTMLInputElement>("#iiUrl")!.value;

//     console.log(webapp_id);

//     // Call authClient.login(...) to login with Internet Identity. This will open a new tab
//     // with the login prompt. The code has to wait for the login process to complete.
//     // We can either use the callback functions directly or wrap in a promise.
//     await new Promise<void>((resolve, reject) => {
//       authClient.login({
//         identityProvider: iiUrl,
//         onSuccess: resolve,
//         onError: reject,
//       });
//     });

//     // At this point we're authenticated, and we can get the identity from the auth client:
//     const identity = authClient.getIdentity();
//     // Using the identity obtained from the auth client, we can create an agent to interact with the IC.
//     const agent = new HttpAgent({ identity } as any);
//     // Using the interface description of our webapp, we create an actor that we use to call the service methods.
//     const webapp: _SERVICE = Actor.createActor(webapp_idl, {
//       agent,
//       canisterId: webapp_id!,
//     });
//     // Call whoami which returns the principal (user id) of the current user.
//     const principal = await webapp.whoami();
//     // show the principal on the page
//     document.getElementById("loginStatus")!.innerText = principal.toText();
//   });

//   return (
//     <div className="flex flex-col items-center w-full gap-5 p-10 font-sans text-2xl italic md:items-start md:gap-10 md:text-6xl">
//       <div className="text-center">
//         {identity ? "You are logged in." : "You are not logged in."}
//       </div>
//       <LoginButton />
//       <Principal principal={principal} />
//     </div>
//   );
// }

// export default App;
