# High-Priority Sentry Issues

## 1. [WEB-7](https://tembo-io.sentry.io/issues/WEB-7) - Hydration Error

- **Status**: Unresolved
- **Users Affected**: 7
- **Event Count**: 8
- **First Seen**: 23 hours ago
- **Last Seen**: 1 hour ago
- **Location**: `https://app.tembo.io/sign-in?redirect_url=https%3A%2F%2Fapp.tembo.io%2F`

**Priority**: High - Affecting multiple users with hydration issues on the sign-in page.

---

## 2. [WEB-6K](https://tembo-io.sentry.io/issues/WEB-6K) - Large Render Blocking Asset

- **Status**: Unresolved
- **Users Affected**: 2
- **Event Count**: 2
- **First Seen**: 21 hours ago
- **Last Seen**: 1 hour ago
- **Location**: `/:slug`

**Priority**: High - Performance issue causing render blocking, impacting user experience.

---

## 3. [API-8R](https://tembo-io.sentry.io/issues/API-8R) - PrismaClientKnownRequestError

- **Status**: Unresolved
- **Users Affected**: 2
- **Event Count**: 8
- **First Seen**: 23 hours ago
- **Last Seen**: 2 hours ago
- **Location**: `<anonymous>(src.server.routes.integration:index.ts)`

**Priority**: High - Database query error affecting multiple users in the integration routes.

---

## 4. [API-3V](https://tembo-io.sentry.io/issues/API-3V) - PrismaClientKnownRequestError

- **Status**: Unresolved
- **Users Affected**: 2
- **Event Count**: 5
- **First Seen**: 23 hours ago
- **Last Seen**: 13 hours ago
- **Issue**: Record not found for query

**Priority**: High - Multiple database queries failing due to missing records.

---

## 5. [WEB-7P](https://tembo-io.sentry.io/issues/WEB-7P) - RangeError: Maximum call stack size exceeded

- **Status**: Unresolved
- **Users Affected**: 1
- **Event Count**: 7
- **First Seen**: 4 hours ago
- **Last Seen**: 4 hours ago
- **Location**: `/g-s-workspace-1764089716/`

**Priority**: High - Stack overflow error indicating potential infinite recursion or memory leak.

---

*Last Updated: 2025-11-25*
*View all issues: https://tembo-io.sentry.io/issues/?query=is%3Aunresolved*
