import { useState } from "react";

function requestAccessToken(setAccessToken: (accessToken: string) => void) {
  const client = google.accounts.oauth2.initTokenClient({
    client_id:
      "1081824270532-a9mpf5t6nc3dd4taaof7lljus8ekh1gj.apps.googleusercontent.com",
    scope: "https://www.googleapis.com/auth/userinfo.email",
    callback: (response) => {
      console.log(response);

      setAccessToken(response.access_token);
    }
  });

  client.requestAccessToken();
}

export function Component() {
  const [accessToken, setAccessToken] = useState<string | null>(null);

  return (
    <main>
      <h1>Login</h1>
      <p>{accessToken}</p>
      <button onClick={() => requestAccessToken(setAccessToken)}>
        Sign in with Google
      </button>
    </main>
  );
}
