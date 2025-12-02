# Sentry Issue API-CZ: util.getSystemErrorMap Compatibility Issue

**Issue ID:** API-CZ
**Sentry URL:** https://tembo-io.sentry.io/issues/7079789515/
**Status:** Unresolved
**First Seen:** 2025-12-02T00:47:57.917Z
**Project:** api
**Runtime:** Bun on Node.js v22.6.0

## Error Summary

```
TypeError: util.getSystemErrorMap is not a function.
(In 'util.getSystemErrorMap()', 'util.getSystemErrorMap' is undefined)
```

## Root Cause

The error occurs in the Sentry Node.js SDK's `SystemErrorIntegration` when it attempts to call `util.getSystemErrorMap()`. This function is a Node.js built-in that may not be fully compatible or available in the Bun runtime environment.

**Stack Trace Location:**
- Error originates in: `@sentry/node-core/build/esm/integrations/systemError.js:17:15`
- Triggered from: `/apps/api/src/worker/loop.ts:121:11` when calling `Sentry.captureException(error)`

**Relevant Code Context:**
```typescript
// In worker/loop.ts:121
try {
    await this.start(abortController, queueWorker);
} catch (error) {
    Sentry.captureException(error); // <-- Error triggers here
    logger.error(serializeError(error) as any, 'Worker loop failed');
    await new Promise((resolve) => setTimeout(resolve, ms('5s')));
    return this.startAndRetryOnFailure(abortController, queueWorker);
}
```

## Proposed Solutions

### Solution 1: Disable SystemErrorIntegration (Recommended)

Modify your Sentry initialization to exclude the problematic integration when running on Bun:

```typescript
import * as Sentry from '@sentry/node';

Sentry.init({
  dsn: process.env.SENTRY_DSN,
  // ... other options
  integrations: (defaultIntegrations) => {
    return defaultIntegrations.filter(
      (integration) => integration.name !== 'SystemError'
    );
  },
});
```

### Solution 2: Conditional Integration Based on Runtime

```typescript
import * as Sentry from '@sentry/node';

const isBun = typeof Bun !== 'undefined';

Sentry.init({
  dsn: process.env.SENTRY_DSN,
  integrations: (defaultIntegrations) => {
    if (isBun) {
      // Filter out SystemError integration for Bun
      return defaultIntegrations.filter(
        (integration) => integration.name !== 'SystemError'
      );
    }
    return defaultIntegrations;
  },
});
```

### Solution 3: Upgrade Sentry SDK

Check if a newer version of `@sentry/node` has better Bun compatibility:

```bash
pnpm update @sentry/node @sentry/core @sentry/node-core
```

Current version experiencing the issue: `@sentry/core@9.47.1`

### Solution 4: Add Polyfill (Advanced)

If you need the `SystemError` integration functionality, you could add a polyfill:

```typescript
// Add before Sentry initialization
import { util } from 'node:util';

if (!util.getSystemErrorMap) {
  // Polyfill for Bun compatibility
  util.getSystemErrorMap = () => new Map();
}
```

## Testing

After applying the fix, test by:

1. Triggering an error in the worker loop
2. Verifying the error is captured by Sentry without the `getSystemErrorMap` error
3. Checking that error reporting still works as expected

## Additional Notes

- The error occurs during error reporting, creating a meta-error situation
- This doesn't affect the primary application logic but prevents proper error tracking
- The `SystemError` integration is used for Node.js system-level error handling
- Bun's Node.js API compatibility may not include all Node.js internals

## References

- Node.js `util.getSystemErrorMap()` documentation: https://nodejs.org/api/util.html#utilgetsystemerrormap
- Sentry Bun compatibility: https://docs.sentry.io/platforms/javascript/guides/bun/
- Related Sentry issue tracker (if applicable)

## Implementation Checklist

- [ ] Apply one of the proposed solutions
- [ ] Test error capture in development
- [ ] Test error capture in production
- [ ] Verify Sentry dashboard shows errors correctly
- [ ] Update Sentry configuration documentation
- [ ] Consider reporting this to Sentry as a Bun compatibility issue
