import { createTheme, ThemeProvider } from '@mui/material';
import React from 'react';
import { createRoot } from 'react-dom/client';
import { createBrowserRouter } from 'react-router-dom';
import { App } from './components/app';
import { Home } from './components/pages/Home';
import { Suggestions } from './components/pages/Suggestions';

const theme = createTheme({
  palette: {
    primary: {
      main: '#DD042B',
    },
    secondary: {
      main: '#F5F5F5',
    }
  }
});

const container = document.getElementById('root');
const root = createRoot(container);

const router = createBrowserRouter([
  {
    path: '/',
    element: <Home />,
  },
  {
    path: '/suggestions',
    element: <Suggestions />,
  }
]);

root.render(
  <React.StrictMode>
    <ThemeProvider theme={theme}>
      <App router={router} />
    </ThemeProvider>
  </React.StrictMode>
);