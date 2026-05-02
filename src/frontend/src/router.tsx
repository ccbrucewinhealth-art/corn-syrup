import type { ReactElement } from 'react';
import { Navigate, RouteObject } from 'react-router-dom';
import Layout from './layouts/Layout';
import EmptyLayout from './layouts/EmptyLayout';
import Dashboard from './pages/Dashboard';
import DashboardHome from './pages/DashboardHome';
import Details from './pages/Details';
import EditMonitor from './pages/EditMonitor';
import List from './pages/List';
import Settings from './pages/Settings';
import Setup from './pages/Setup';
import SetupDatabase from './pages/SetupDatabase';
import AddStatusPage from './pages/AddStatusPage';
import ManageStatusPage from './pages/ManageStatusPage';
import StatusPage from './pages/StatusPage';
import ManageMaintenance from './pages/ManageMaintenance';
import EditMaintenance from './pages/EditMaintenance';
import Entry from './pages/Entry';
import NotFound from './pages/NotFound';
import General from './components/settings/General';
import Appearance from './components/settings/Appearance';
import Notifications from './components/settings/Notifications';
import ReverseProxy from './components/settings/ReverseProxy';
import Tags from './components/settings/Tags';
import MonitorHistory from './components/settings/MonitorHistory';
import Docker from './components/settings/Docker';
import RemoteBrowsers from './components/settings/RemoteBrowsers';
import Security from './components/settings/Security';
import APIKeys from './components/settings/APIKeys';
import Proxies from './components/settings/Proxies';
import About from './components/settings/About';
import { debugAction } from './lib/logger';

export interface RouteMeta { title: string; public?: boolean; empty?: boolean; }
export interface AppRoute { path?: string; index?: boolean; element: ReactElement; meta: RouteMeta; children?: AppRoute[]; }

export const appRoutes: AppRoute[] = [
  { path: '/', element: <Layout />, meta: { title: 'Dashboard' }, children: [
    { index: true, element: <Navigate to="/dashboard" replace />, meta: { title: 'Dashboard' } },
    { path: '/dashboard', element: <Dashboard />, meta: { title: 'Dashboard' }, children: [
      { index: true, element: <DashboardHome />, meta: { title: 'Quick Stats' } },
      { path: ':id', element: <Details />, meta: { title: 'Details' } },
    ] },
    { path: '/manage-status-page', element: <Dashboard />, meta: { title: 'Status Pages' }, children: [
      { index: true, element: <ManageStatusPage />, meta: { title: 'Status Pages' } },
    ] },
    { path: '/add-status-page', element: <Dashboard />, meta: { title: 'Add Status Page' }, children: [
      { index: true, element: <AddStatusPage />, meta: { title: 'Add Status Page' } },
    ] },
    { path: '/list', element: <List />, meta: { title: 'List' } },
    { path: '/add', element: <Dashboard />, meta: { title: 'Add New Monitor' }, children: [
      { index: true, element: <EditMonitor />, meta: { title: 'Add New Monitor' } },
    ] },
    { path: '/clone/:id', element: <Dashboard />, meta: { title: 'Clone Monitor' }, children: [
      { index: true, element: <EditMonitor />, meta: { title: 'Clone Monitor' } },
    ] },
    { path: '/edit/:id', element: <Dashboard />, meta: { title: 'Edit Monitor' }, children: [
      { index: true, element: <EditMonitor />, meta: { title: 'Edit Monitor' } },
    ] },
    { path: '/settings', element: <Settings />, meta: { title: 'Settings' }, children: [
      { index: true, element: <Navigate to="/settings/general" replace />, meta: { title: 'General' } },
      { path: 'general', element: <General />, meta: { title: 'General' } },
      { path: 'appearance', element: <Appearance />, meta: { title: 'Appearance' } },
      { path: 'notifications', element: <Notifications />, meta: { title: 'Notifications' } },
      { path: 'reverse-proxy', element: <ReverseProxy />, meta: { title: 'Reverse Proxy' } },
      { path: 'tags', element: <Tags />, meta: { title: 'Tags' } },
      { path: 'monitor-history', element: <MonitorHistory />, meta: { title: 'Monitor History' } },
      { path: 'docker-hosts', element: <Docker />, meta: { title: 'Docker Hosts' } },
      { path: 'remote-browsers', element: <RemoteBrowsers />, meta: { title: 'Remote Browsers' } },
      { path: 'security', element: <Security />, meta: { title: 'Security' } },
      { path: 'api-keys', element: <APIKeys />, meta: { title: 'API Keys' } },
      { path: 'proxies', element: <Proxies />, meta: { title: 'Proxies' } },
      { path: 'about', element: <About />, meta: { title: 'About' } },
    ] },
    { path: '/maintenance', element: <ManageMaintenance />, meta: { title: 'Maintenance' } },
    { path: '/add-maintenance', element: <EditMaintenance />, meta: { title: 'Add Maintenance' } },
    { path: '/maintenance/add', element: <EditMaintenance />, meta: { title: 'Add Maintenance' } },
    { path: '/maintenance/edit/:id', element: <EditMaintenance />, meta: { title: 'Edit Maintenance' } },
    { path: '/maintenance/clone/:id', element: <EditMaintenance />, meta: { title: 'Clone Maintenance' } },
    { path: '/status-page', element: <Navigate to="/manage-status-page" replace />, meta: { title: 'Status Pages' } },
  ]},
  { path: '/', element: <EmptyLayout />, meta: { title: 'Entry', public: true, empty: true }, children: [
    { path: '/setup', element: <Setup />, meta: { title: 'Setup', public: true, empty: true } },
    { path: '/setup-database', element: <SetupDatabase />, meta: { title: 'Setup Database', public: true, empty: true } },
    { path: '/entry', element: <Entry />, meta: { title: 'Entry', public: true, empty: true } },
    { path: '/status-page', element: <StatusPage />, meta: { title: 'Status Page', public: true, empty: true } },
    { path: '/status', element: <StatusPage />, meta: { title: 'Status Page', public: true, empty: true } },
    { path: '/status/:slug', element: <StatusPage />, meta: { title: 'Status Page', public: true, empty: true } },
  ]},
  { path: '*', element: <NotFound />, meta: { title: 'Not Found', public: true } },
];

function toRouteObject(route: AppRoute): RouteObject {
  debugAction('router', 'register.route', { path: route.path, meta: route.meta });
  const index = route.index;
  return { path: route.path, index, element: route.element, children: route.children?.map(toRouteObject) } as RouteObject;
}

export const routeObjects: RouteObject[] = appRoutes.map(toRouteObject);

export function fallbackRoute() {
  debugAction('router', 'fallback.to.notFound');
  return <Navigate to="/404" replace />;
}
