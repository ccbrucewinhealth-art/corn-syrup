import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import './i18n';
import App from './App';
import './styles.css';
import { debugAction, logger } from './lib/logger';

const rootElement = document.getElementById('root');
if (!rootElement) throw new Error('React root element #root not found');

ReactDOM.createRoot(rootElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>,
);

if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => {
    navigator.serviceWorker.register('/serviceWorker.js')
      .then((registration) => debugAction('serviceWorker', 'registered', { scope: registration.scope }))
      .catch((error) => logger.warn('[serviceWorker] registration.failed', error));
  });
}
