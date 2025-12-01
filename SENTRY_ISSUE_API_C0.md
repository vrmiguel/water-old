# Sentry Issue Analysis: API-C0

## Issue Summary

**Issue ID**: API-C0
**Sentry URL**: https://tembo-io.sentry.io/issues/API-C0
**Status**: Unresolved (Regressed)
**Priority**: High
**Project**: api (tembo-ts-agent)
**First Seen**: 2025-11-19
**Last Seen**: 2025-12-01
**Occurrences**: 3

## Error Description

```
Error: OpenCode execution failed: Bad Request
```

The error occurs in the OpenCode sandbox execution system when attempting to solve an issue. The execution reads multiple files successfully but ultimately fails with a "Bad Request" error.

## Stack Trace Analysis

**Primary Error Location**:
- File: `/apps/api/src/worker/sandbox/agents/opencode.ts:166`
- Error Type: `SandboxError`
- Parent Error: `TemboError`

**Error Flow**:
1. OpenCode agent attempts to execute a task (`solve_issue`)
2. Successfully reads 11 files from the target repository
3. Creates 3 todos
4. Performs a grep search for "401"
5. Fails with "Bad Request" error

## Root Cause Analysis

Based on the error details and context, the issue appears to be:

1. **HTTP 400 Bad Request**: The OpenCode execution is making an API request that returns a 400 status code
2. **Possible Causes**:
   - Invalid request payload when communicating with an external service
   - Missing or malformed authentication credentials
   - Request exceeds size limits or contains invalid data
   - The grep search for "401" suggests authentication-related debugging

## Files Involved in Failed Execution

The OpenCode agent successfully read these files before failing:
- `app/api/products/route.ts` (primary endpoint)
- `middleware.ts`
- `app/products/page.tsx`
- `lib/env.ts`
- `lib/security.ts`
- `app/page.tsx`
- `app/components/ProductsSection.tsx`
- `vercel.json`
- `app/api/products/[id]/route.ts`
- `lib/rateLimit.ts`
- `lib/supabase-server.ts`

## Recommended Fixes

### 1. Add Request Validation in OpenCode Agent

**File**: `apps/api/src/worker/sandbox/agents/opencode.ts:166`

The error occurs when the OpenCode agent attempts to send a request. Add validation:

```typescript
// Before making the request, validate the payload
if (!isValidRequest(requestPayload)) {
  throw new SandboxError(
    'Invalid request payload',
    { payload: JSON.stringify(requestPayload) },
    'INVALID_REQUEST'
  );
}

// Add size limits
if (requestPayload.length > MAX_PAYLOAD_SIZE) {
  throw new SandboxError(
    'Request payload exceeds maximum size',
    { size: requestPayload.length, max: MAX_PAYLOAD_SIZE },
    'PAYLOAD_TOO_LARGE'
  );
}
```

### 2. Implement Better Error Handling

Catch and log the specific Bad Request error:

```typescript
try {
  const response = await executeOpenCodeTask(payload);
  return response;
} catch (error) {
  if (error.statusCode === 400) {
    // Log detailed information about what caused the bad request
    logger.error('Bad Request in OpenCode execution', {
      requestBody: sanitizeForLogging(payload),
      responseBody: error.response?.body,
      headers: sanitizeHeaders(error.response?.headers)
    });

    throw new SandboxError(
      `OpenCode execution failed: ${error.message}`,
      {
        statusCode: '400',
        details: error.response?.body || 'No details available'
      },
      'BAD_REQUEST'
    );
  }
  throw error;
}
```

### 3. Add Request Retry Logic with Exponential Backoff

```typescript
async function executeWithRetry(fn, maxRetries = 3) {
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      return await fn();
    } catch (error) {
      if (error.statusCode === 400 && attempt < maxRetries) {
        // Bad Request errors typically won't succeed on retry
        // unless the payload is modified
        break;
      }

      if (attempt === maxRetries) throw error;

      const delay = Math.pow(2, attempt) * 1000;
      await sleep(delay);
    }
  }
}
```

### 4. Validate Environment Configuration

Since the error involves reading `lib/env.ts`, ensure all required environment variables are properly set:

```typescript
// Add to initialization
function validateOpenCodeEnvironment() {
  const required = [
    'OPENCODE_API_URL',
    'OPENCODE_API_KEY',
    'MAX_REQUEST_SIZE'
  ];

  const missing = required.filter(key => !process.env[key]);

  if (missing.length > 0) {
    throw new Error(`Missing required environment variables: ${missing.join(', ')}`);
  }
}
```

### 5. Add Request/Response Logging

Implement comprehensive logging to debug future occurrences:

```typescript
logger.info('OpenCode execution starting', {
  taskName: 'solve_issue',
  filesRead: filesRead.length,
  todosCreated: todos.length,
  jobId: jobData.id
});

// Log the actual request being made
logger.debug('OpenCode API request', {
  url: sanitizeUrl(requestUrl),
  method: requestMethod,
  payloadSize: requestPayload.length,
  headers: sanitizeHeaders(requestHeaders)
});
```

## Testing Recommendations

1. **Unit Tests**: Add tests for request validation logic
2. **Integration Tests**: Test OpenCode execution with various payload sizes
3. **Error Scenarios**: Test handling of 400, 401, 403, 500 status codes
4. **Load Tests**: Verify behavior under concurrent requests

## Monitoring Recommendations

1. Add custom metrics for OpenCode execution failures
2. Set up alerts for Bad Request errors exceeding threshold
3. Track the correlation between file read counts and failures
4. Monitor request payload sizes approaching limits

## Related Issues

- The grep search for "401" suggests there may be authentication issues in the products API
- The regressed status indicates this was previously resolved but has reoccurred
- Consider investigating the `/api/products` endpoint for authentication handling

## Action Items

- [ ] Implement request validation in opencode.ts
- [ ] Add comprehensive error logging
- [ ] Review environment variable configuration
- [ ] Add monitoring for Bad Request errors
- [ ] Investigate the products API authentication flow
- [ ] Add integration tests for OpenCode execution failures

---

**References**: Fixes API-C0
