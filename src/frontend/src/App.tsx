import { useRoutes } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { routeObjects } from './router';
import { debugAction } from './lib/logger';

export default function App() {
  const { i18n } = useTranslation();
  const element = useRoutes(routeObjects);
  debugAction('App', 'render.route.table', { language: i18n.language, routes: routeObjects.length });
  return element;
}
