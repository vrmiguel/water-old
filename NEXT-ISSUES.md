# Next High-Priority Sentry Issues

This document contains 5 high-priority unresolved issues from the tembo-io Sentry organization.

## 1. [WEB-7](https://tembo-io.sentry.io/issues/WEB-7) - Hydration Error

**Status**: Unresolved
**Impact**: 8 users affected, 9 events
**Timeline**: First seen 23 hours ago, last seen 58 minutes ago
**Location**: `https://app.tembo.io/sign-in?redirect_url=https%3A%2F%2Fapp.tembo.io%2F`

Hydration errors in the sign-in flow affecting multiple users. This could be causing issues with the authentication experience.

---

## 2. [API-8R](https://tembo-io.sentry.io/issues/API-8R) - PrismaClientKnownRequestError

**Status**: Unresolved
**Impact**: 3 users affected, 11 events
**Timeline**: First seen 23 hours ago, last seen 1 hour ago
**Location**: `<anonymous>(src.server.routes.integration:index.ts)`

Database query errors in the integration routes. This is affecting multiple users and appears to be an ongoing issue.

---

## 3. [WEB-6K](https://tembo-io.sentry.io/issues/WEB-6K) - Large Render Blocking Asset

**Status**: Unresolved
**Impact**: 2 users affected, 2 events
**Timeline**: First seen 20 hours ago, last seen 43 minutes ago
**Location**: `/:slug`

Performance issue with large render-blocking assets affecting page load times in slug-based routes.

---

## 4. [API-3V](https://tembo-io.sentry.io/issues/API-3V) - PrismaClientKnownRequestError

**Status**: Unresolved
**Impact**: 2 users affected, 5 events
**Timeline**: First seen 23 hours ago, last seen 13 hours ago
**Location**: Database query error - required fields not found

Database query error where required fields were not found. This suggests missing data or incorrect query logic.

---

## 5. [WEB-7P](https://tembo-io.sentry.io/issues/WEB-7P) - RangeError: Maximum call stack size exceeded

**Status**: Unresolved
**Impact**: 1 user affected, 7 events
**Timeline**: First seen 4 hours ago, last seen 4 hours ago
**Location**: `/g-s-workspace-1764089716/`

Stack overflow error in workspace route, likely indicating an infinite loop or recursive function without proper termination.

---

## Summary

These issues represent critical problems affecting the authentication flow, database operations, performance, and application stability. Priority should be given to:

1. **WEB-7** - Most users affected (8 users)
2. **API-8R** - Most events (11 occurrences) and still active
3. **WEB-7P** - Stack overflow requiring immediate attention
4. **WEB-6K** - Performance degradation
5. **API-3V** - Data integrity issue

For more details on these issues, visit the [Sentry dashboard](https://tembo-io.sentry.io/issues/?query=is%3Aunresolved).
