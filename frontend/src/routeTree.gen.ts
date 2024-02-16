/* prettier-ignore-start */

/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file is auto-generated by TanStack Router

// Import Routes

import { Route as rootRoute } from './routes/__root'
import { Route as AccountImport } from './routes/_account'
import { Route as AccountIndexImport } from './routes/_account.index'
import { Route as DevicesIdImport } from './routes/devices.$id'
import { Route as ClientsIdImport } from './routes/clients.$id'
import { Route as AccountSessionsIndexImport } from './routes/_account.sessions.index'
import { Route as EmailsIdVerifyImport } from './routes/emails.$id.verify'
import { Route as AccountSessionsBrowsersImport } from './routes/_account.sessions.browsers'
import { Route as AccountSessionsIdImport } from './routes/_account.sessions.$id'

// Create/Update Routes

const AccountRoute = AccountImport.update({
  id: '/_account',
  getParentRoute: () => rootRoute,
} as any)

const AccountIndexRoute = AccountIndexImport.update({
  path: '/',
  getParentRoute: () => AccountRoute,
} as any)

const DevicesIdRoute = DevicesIdImport.update({
  path: '/devices/$id',
  getParentRoute: () => rootRoute,
} as any)

const ClientsIdRoute = ClientsIdImport.update({
  path: '/clients/$id',
  getParentRoute: () => rootRoute,
} as any)

const AccountSessionsIndexRoute = AccountSessionsIndexImport.update({
  path: '/sessions/',
  getParentRoute: () => AccountRoute,
} as any)

const EmailsIdVerifyRoute = EmailsIdVerifyImport.update({
  path: '/emails/$id/verify',
  getParentRoute: () => rootRoute,
} as any)

const AccountSessionsBrowsersRoute = AccountSessionsBrowsersImport.update({
  path: '/sessions/browsers',
  getParentRoute: () => AccountRoute,
} as any)

const AccountSessionsIdRoute = AccountSessionsIdImport.update({
  path: '/sessions/$id',
  getParentRoute: () => AccountRoute,
} as any)

// Populate the FileRoutesByPath interface

declare module '@tanstack/react-router' {
  interface FileRoutesByPath {
    '/_account': {
      preLoaderRoute: typeof AccountImport
      parentRoute: typeof rootRoute
    }
    '/clients/$id': {
      preLoaderRoute: typeof ClientsIdImport
      parentRoute: typeof rootRoute
    }
    '/devices/$id': {
      preLoaderRoute: typeof DevicesIdImport
      parentRoute: typeof rootRoute
    }
    '/_account/': {
      preLoaderRoute: typeof AccountIndexImport
      parentRoute: typeof AccountImport
    }
    '/_account/sessions/$id': {
      preLoaderRoute: typeof AccountSessionsIdImport
      parentRoute: typeof AccountImport
    }
    '/_account/sessions/browsers': {
      preLoaderRoute: typeof AccountSessionsBrowsersImport
      parentRoute: typeof AccountImport
    }
    '/emails/$id/verify': {
      preLoaderRoute: typeof EmailsIdVerifyImport
      parentRoute: typeof rootRoute
    }
    '/_account/sessions/': {
      preLoaderRoute: typeof AccountSessionsIndexImport
      parentRoute: typeof AccountImport
    }
  }
}

// Create and export the route tree

export const routeTree = rootRoute.addChildren([
  AccountRoute.addChildren([
    AccountIndexRoute,
    AccountSessionsIdRoute,
    AccountSessionsBrowsersRoute,
    AccountSessionsIndexRoute,
  ]),
  ClientsIdRoute,
  DevicesIdRoute,
  EmailsIdVerifyRoute,
])

/* prettier-ignore-end */