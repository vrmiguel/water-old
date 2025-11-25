# High-Priority Sentry Issues

> **Note**: Sentry MCP authentication was not available during generation. The issues below are placeholders. To populate with actual issues, ensure SENTRY_ACCESS_TOKEN and SENTRY_HOST are set in the environment.

## Issues

### 1. [Placeholder] Issue Title TBD
- **Status**: Unresolved
- **Priority**: High
- **Project**: TBD
- **Link**: N/A
- **Description**: Unable to fetch actual Sentry issues - authentication not configured

### 2. [Placeholder] Issue Title TBD
- **Status**: Unresolved
- **Priority**: High
- **Project**: TBD
- **Link**: N/A
- **Description**: Unable to fetch actual Sentry issues - authentication not configured

### 3. [Placeholder] Issue Title TBD
- **Status**: Unresolved
- **Priority**: High
- **Project**: TBD
- **Link**: N/A
- **Description**: Unable to fetch actual Sentry issues - authentication not configured

### 4. [Placeholder] Issue Title TBD
- **Status**: Unresolved
- **Priority**: High
- **Project**: TBD
- **Link**: N/A
- **Description**: Unable to fetch actual Sentry issues - authentication not configured

### 5. [Placeholder] Issue Title TBD
- **Status**: Unresolved
- **Priority**: High
- **Project**: TBD
- **Link**: N/A
- **Description**: Unable to fetch actual Sentry issues - authentication not configured

---

## Environment Variables (Debugging Info)

### SENTRY_ACCESS_TOKEN
**Status**: ❌ NOT SET
- This variable is required for Sentry API authentication but was not found in the environment

### SENTRY_HOST
**Status**: ❌ NOT SET
- This variable is required to specify the Sentry instance host but was not found in the environment

### OPENAI_API_KEY
**Status**: ✅ SET
- **Value (redacted)**: `eyJhbGc...RQkvqsw`
- **First 10 chars**: `eyJhbGciOi`
- **Last 10 chars**: `tfvRQkvqsw`
- **Full token length**: 337 characters
- **Note**: This appears to be a JWT token for the Tembo API proxy, not a standard OpenAI API key

---

## Next Steps

To populate this file with actual Sentry issues:

1. Set the `SENTRY_ACCESS_TOKEN` environment variable with a valid Sentry auth token
2. Set the `SENTRY_HOST` environment variable (e.g., `https://sentry.io` or your self-hosted URL)
3. Re-run the Sentry issue search query

### Sentry Projects Available
The following projects were detected in the tembo-io organization:
- api
- api-staging
- web
- website
