import "regenerator-runtime/runtime";
import React from "react";

import HomePage from "./HomePage";
import CreateRufflePage from "./CreateRufflePage";
import ExploreRufflesPage from "./ExploreRufflesPage";
import "./assets/global.css";

import { SignInPrompt, SignOutButton } from "./ui-components";

export default function App({ isSignedIn, contract, wallet }) {
  const [valueFromBlockchain, setValueFromBlockchain] = React.useState();

  const [uiPleaseWait, setUiPleaseWait] = React.useState(true);

  // Get blockchian state once on component load
  React.useEffect(() => {
    contract
      .getGreeting()
      .then(setValueFromBlockchain)
      .catch(alert)
      .finally(() => {
        setUiPleaseWait(false);
      });
  }, []);

  /// If user not signed-in with wallet - show prompt
  if (!isSignedIn) {
    // Sign-in flow will reload the page later
    return (
      <SignInPrompt
        greeting={valueFromBlockchain}
        onClick={() => wallet.signIn()}
      />
    );
  }

  function changeGreeting(e) {
    e.preventDefault();
    setUiPleaseWait(true);
    const { greetingInput } = e.target.elements;
    contract
      .setGreeting(greetingInput.value)
      .then(async () => {
        return contract.getGreeting();
      })
      .then(setValueFromBlockchain)
      .finally(() => {
        setUiPleaseWait(false);
      });
  }

  return (
    <>
      <SignOutButton
        accountId={wallet.accountId}
        onClick={() => wallet.signOut()}
      />
      <main className={uiPleaseWait ? "please-wait" : ""}>
        <h1>
          dRuffleApp <span className="greeting">{valueFromBlockchain}</span>
        </h1>
        {/* <form onSubmit={changeGreeting} className="change">
          <label>Change greeting:</label>
          <div>
            <input
              autoComplete="off"
              defaultValue={valueFromBlockchain}
              id="greetingInput"
            />
            <button>
              <span>Save</span>
              <div className="loader"></div>
            </button>
          </div>
        </form> */}
        <HomePage />
        <CreateRufflePage />
        <ExploreRufflesPage />
      </main>
    </>
  );
}
