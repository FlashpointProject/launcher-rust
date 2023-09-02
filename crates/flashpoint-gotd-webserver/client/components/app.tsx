import { Router } from '@remix-run/router';
import React from 'react';
import { RouterProvider } from 'react-router-dom';
import { Header } from './Header';

export const UserContext = React.createContext<User | undefined>(undefined);

export type AppProps = {
  router: Router;
}

export type User = {
  username: string;
  authenticated: boolean;
  admin: boolean;
}

export function App(props: AppProps) {
  const [user, setUser] = React.useState<User | undefined>(undefined);

  React.useEffect(() => {
    fetch('/api/auth/info')
      .then(async (res) => {
        if (res.ok) {
          const identity = await res.json();
          setUser({
            username: identity.username,
            authenticated: true,
            admin: identity.admin
          });
        } else {
          setUser({
            username: '',
            authenticated: false,
            admin: false,
          });
        }
      });
  }, []);

  return (
    <div className='app'>
      <UserContext.Provider value={user}>
        {user ? (
          <>
            <Header />
            <div className='main'>
              <RouterProvider router={props.router} />
            </div>
          </>
        ) : (
          <div></div>
        )}
      </UserContext.Provider>
    </div>
  );
}