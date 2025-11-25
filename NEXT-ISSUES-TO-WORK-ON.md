# Next Issues to Work On

This document lists the top 4 high-priority Sentry issues that need attention.

## 1. [WEB-7](https://tembo-io.sentry.io/issues/WEB-7) - Hydration Error

**Priority:** High
**Status:** Unresolved
**Affected Users:** 7
**Total Events:** 8
**First Seen:** 23 hours ago
**Last Seen:** 1 hour ago
**Location:** `https://app.tembo.io/sign-in?redirect_url=https%3A%2F%2Fapp.tembo.io%2F`

### Description
Hydration errors indicate a mismatch between server-rendered HTML and client-side React rendering. This is affecting multiple users at the sign-in page.

---

## 2. [WEB-6K](https://tembo-io.sentry.io/issues/WEB-6K) - Large Render Blocking Asset

**Priority:** High
**Status:** Unresolved
**Affected Users:** 2
**Total Events:** 2
**First Seen:** 21 hours ago
**Last Seen:** 1 hour ago
**Location:** `/:slug`

### Description
Large render-blocking assets are impacting page load performance. This affects user experience and Core Web Vitals.

---

## 3. [WEB-7P](https://tembo-io.sentry.io/issues/WEB-7P) - RangeError: Maximum call stack size exceeded

**Priority:** High
**Status:** Unresolved
**Affected Users:** 1
**Total Events:** 7
**First Seen:** 5 hours ago
**Last Seen:** 5 hours ago
**Location:** `/g-s-workspace-1764089716/`

### Description
A maximum call stack size error indicates infinite recursion or deeply nested function calls. This is causing crashes for users in specific workspaces.

---

## 4. [API-CC](https://tembo-io.sentry.io/issues/API-CC) - PrismaClientKnownRequestError

**Priority:** High
**Status:** Unresolved
**Affected Users:** 1
**Total Events:** 1
**First Seen:** 10 hours ago
**Last Seen:** 10 hours ago
**Details:** Required fields not found. No record found for delete operation.

### Description
Database query error where required fields are missing during a delete operation. This indicates a data integrity issue or incorrect query logic.

---

## Summary

- **Total High-Priority Issues:** 4
- **Total Affected Users:** 11
- **Total Events:** 18
- **Focus Areas:**
  - Frontend rendering (hydration, stack overflow)
  - Performance optimization (render-blocking assets)
  - Backend data integrity (Prisma queries)

## Recommended Action Plan

1. **WEB-7 (Hydration Error):** Investigate sign-in page for SSR/CSR mismatches
2. **WEB-6K (Render Blocking):** Optimize asset delivery and code splitting
3. **WEB-7P (Stack Overflow):** Debug recursive logic in workspace routes
4. **API-CC (Prisma Error):** Review delete operation logic and add validation

---

*Generated on: 2025-11-25*
*Sentry Organization: tembo-io*
