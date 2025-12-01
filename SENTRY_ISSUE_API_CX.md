# Sentry Issue API-CX: ReferenceError: _ is not defined

## Issue Details

- **Issue ID**: API-CX
- **Project**: api
- **Environment**: development
- **Severity**: High Priority
- **First Seen**: 2025-12-01T19:13:23.768Z
- **Occurrences**: 3
- **Status**: Unresolved

## Error Description

```
ReferenceError: _ is not defined
```

## Root Cause

The error occurs in `/Users/vrmiguel/monorepo/apps/api/src/server.ts` at line 180:

```typescript
177 │ 	logger.info('Starting server');
178 │ 	for (const i of [1, 2, 3]) {
179 │ 		try {
180 │ 			_ = i;  // ❌ ERROR: _ is not defined
181 │ 			throw new Error('Test error 293203 startServer');
182 │ 		} catch (error) {
183 │ 			Sentry.captureException(error);
```

The code attempts to assign a value to `_` (underscore) without declaring it. In strict mode (which is the default in ES modules and TypeScript), this results in a ReferenceError.

## Analysis

The underscore `_` is commonly used as a convention to indicate an intentionally unused variable. However, in this code:

1. The variable `_` is not declared with `let`, `const`, or `var`
2. The code is running in strict mode, which prevents implicit global variable creation
3. This appears to be test code (note the comment "Test error 293203")

## Proposed Fix

There are two possible approaches:

### Option 1: Remove the unused assignment (Recommended)

Since `_` is not used anywhere after assignment, simply remove the line:

```typescript
177 │ 	logger.info('Starting server');
178 │ 	for (const i of [1, 2, 3]) {
179 │ 		try {
180 │ 			// Removed: _ = i;
181 │ 			throw new Error('Test error 293203 startServer');
182 │ 		catch (error) {
183 │ 			Sentry.captureException(error);
```

### Option 2: Properly declare the variable

If the assignment is needed for some reason, declare the variable:

```typescript
177 │ 	logger.info('Starting server');
178 │ 	for (const i of [1, 2, 3]) {
179 │ 		try {
180 │ 			const _ = i;
181 │ 			throw new Error('Test error 293203 startServer');
182 │ 		} catch (error) {
183 │ 			Sentry.captureException(error);
```

Or use the void operator to explicitly ignore the value:

```typescript
180 │ 			void i;
```

## Recommendation

**Option 1 is recommended** because:
- The variable `_` is never used after assignment
- This appears to be leftover test/debug code
- Removing it simplifies the code and eliminates the error

## Additional Notes

- This error is occurring in the `startServer` function within a try-catch block that's specifically designed to test Sentry error capturing
- The error is being caught and reported to Sentry (as intended), but the ReferenceError itself is not intentional
- Consider removing or properly isolating test code from production code paths

## References

- Sentry Issue: https://tembo-io.sentry.io/issues/API-CX
- File: `/Users/vrmiguel/monorepo/apps/api/src/server.ts:180`
- Function: `startServer`
