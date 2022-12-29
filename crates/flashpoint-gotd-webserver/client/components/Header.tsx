import { UserContext } from './app';
import React from 'react';
import Button from '@mui/material/Button';

export function Header() {
  const user = React.useContext(UserContext);

  return (
    <div className='header'>
      <div className='headerLeft'>
        <div>Flashpoint GOTD Server</div>
        <Button variant="contained" color="secondary" href="/">Submit</Button>
        <Button variant="contained" color="secondary" href="/suggestions">Suggestions</Button>
        {user.admin && (
          <Button variant="contained" color="secondary" href="/admin">Admin</Button>
        )}
      </div>
      <div className='headerRight'>
        {user.authenticated ? (
          <Button variant="contained" color="secondary" href="/api/auth/logout">Logout</Button>
        ) : (
          <Button variant="contained" color="success" href="/api/auth/login">Login</Button>
        )}
      </div>
    </div>
  );
}