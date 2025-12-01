# Fix for Sentry Issue API-8N

## Issue Summary

**Issue ID**: API-8N
**Error**: `TypeError: util.getSystemErrorMap is not a function. (In 'util.getSystemErrorMap()', 'util.getSystemErrorMap' is undefined)`
**Project**: api (Bun platform)
**Occurrences**: 1,529 events
**First Seen**: 2025-09-30
**Last Seen**: 2025-11-24
**Status**: Resolved (auto-resolved)
**Sentry URL**: https://tembo-io.sentry.io/issues/API-8N

## Root Cause

The error occurs in the Sentry Node.js SDK (v9.46.0) within the `SystemErrorIntegration`. The integration attempts to call `util.getSystemErrorMap()`, which is a Node.js-specific API that **is not available in Bun runtime**.

### Stack Trace Location
```
@sentry/node-core/build/esm/integrations/systemError.js:17:15 (isSystemError)
return util.getSystemErrorMap().has(error.errno);
```

The `systemError.js` integration is trying to check if an error is a system error by using Node.js's `util.getSystemErrorMap()` function, but this function doesn't exist in Bun's implementation of the `util` module.

## Proposed Fix

There are three possible solutions:

### Option 1: Disable SystemError Integration (Recommended)

Modify the Sentry initialization in the api project to explicitly disable the `SystemErrorIntegration`:

```typescript
import * as Sentry from '@sentry/node';

Sentry.init({
  dsn: process.env.SENTRY_DSN,
  integrations: [
    // Filter out the SystemErrorIntegration
    ...Sentry.getDefaultIntegrations().filter(
      integration => integration.name !== 'SystemError'
    ),
  ],
  // ... other configuration
});
```

### Option 2: Upgrade Sentry SDK

Check if a newer version of `@sentry/node` has been released that includes Bun compatibility fixes. The Sentry team may have addressed this in a later version:

```bash
bun update @sentry/node
```

### Option 3: Use Bun-specific Sentry Package (If Available)

Check if Sentry provides a Bun-specific package or configuration:

```bash
bun add @sentry/bun
```

If available, use the Bun-specific initialization instead of the Node.js version.

### Option 4: Polyfill the Missing Function

As a temporary workaround, you could polyfill the missing function before initializing Sentry:

```typescript
import * as util from 'util';

// Polyfill for Bun compatibility
if (typeof (util as any).getSystemErrorMap !== 'function') {
  (util as any).getSystemErrorMap = () => new Map();
}

import * as Sentry from '@sentry/node';
// ... rest of Sentry initialization
```

## Implementation Steps

1. Locate the Sentry initialization file in the api project (likely `src/sentry.ts` or similar)
2. Implement **Option 1** (recommended) by filtering out the SystemError integration
3. Deploy the change to production
4. Monitor Sentry to confirm the error no longer occurs
5. Consider opening an issue with the Sentry team about Bun compatibility

## Additional Notes

- This issue affects Bun runtime specifically because Bun doesn't implement all Node.js APIs
- The `SystemErrorIntegration` is not critical for error tracking - it only adds additional context about system-level errors
- Disabling this integration will not impact the core error tracking functionality
- The error was auto-resolved, suggesting it may be intermittent or environment-specific

## Testing

After implementing the fix:

1. Deploy to a staging environment running Bun
2. Trigger a test error to confirm Sentry still captures events
3. Verify no `util.getSystemErrorMap` errors appear in Sentry
4. Monitor for 24-48 hours before deploying to production

## References

- Sentry Issue: https://tembo-io.sentry.io/issues/API-8N
- Bun Runtime Documentation: https://bun.sh/docs
- Sentry Node.js SDK: https://docs.sentry.io/platforms/javascript/guides/node/
