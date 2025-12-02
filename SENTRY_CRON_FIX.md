# Sentry Cron Monitor Fix - API-AR

## Issue Summary

**Issue ID**: API-AR
**Title**: Cron failure: Handle Scheduled Queue Jobs
**Monitor**: handle-scheduled-queue-jobs
**Status**: High Priority
**First Seen**: 2025-11-05T18:39:16Z
**Occurrences**: 97+
**Environment**: production

## Problem Description

The Sentry monitor for "Handle Scheduled Queue Jobs" is experiencing recurring missed check-ins. This cron job runs every minute (`* * * * *`) and is failing to report successful completion to Sentry.

### Key Details

- **Schedule**: Every minute (cron: `* * * * *`)
- **Platform**: Bun (api project)
- **Monitor ID**: cd51551e-824f-4d2b-8b54-f6f6b2245570
- **Failure Reason**: Missed check-in detected
- **Issue Type**: monitor_check_in_failure

## Root Cause Analysis

Based on the Sentry data, the cron monitor is failing due to missed check-ins. This typically occurs when:

1. **Job Execution Failure**: The cron job is not running or crashes before completing
2. **Missing Check-in Calls**: The job runs but doesn't call Sentry's check-in API
3. **Timeout Issues**: The job takes longer than expected and Sentry marks it as failed
4. **Network Issues**: Check-in calls to Sentry are failing due to connectivity problems
5. **Configuration Issues**: The monitor slug or credentials are misconfigured

## Recommended Fixes

### 1. Add Proper Error Handling and Check-ins

Ensure the cron job properly reports its status to Sentry:

```typescript
import * as Sentry from "@sentry/bun";

async function handleScheduledQueueJobs() {
  const checkInId = Sentry.captureCheckIn({
    monitorSlug: "handle-scheduled-queue-jobs",
    status: "in_progress",
  });

  try {
    // Your actual job logic here
    await processQueueJobs();

    // Report success
    Sentry.captureCheckIn({
      checkInId,
      monitorSlug: "handle-scheduled-queue-jobs",
      status: "ok",
    });
  } catch (error) {
    // Report failure
    Sentry.captureCheckIn({
      checkInId,
      monitorSlug: "handle-scheduled-queue-jobs",
      status: "error",
    });

    Sentry.captureException(error);
    throw error;
  }
}
```

### 2. Add Timeout Protection

Set a maximum runtime to prevent jobs from hanging:

```typescript
async function handleScheduledQueueJobsWithTimeout() {
  const TIMEOUT_MS = 50000; // 50 seconds (allow 10s buffer for 1-min schedule)

  const checkInId = Sentry.captureCheckIn({
    monitorSlug: "handle-scheduled-queue-jobs",
    status: "in_progress",
  });

  try {
    await Promise.race([
      processQueueJobs(),
      new Promise((_, reject) =>
        setTimeout(() => reject(new Error("Job timeout")), TIMEOUT_MS)
      )
    ]);

    Sentry.captureCheckIn({
      checkInId,
      monitorSlug: "handle-scheduled-queue-jobs",
      status: "ok",
    });
  } catch (error) {
    Sentry.captureCheckIn({
      checkInId,
      monitorSlug: "handle-scheduled-queue-jobs",
      status: "error",
    });

    Sentry.captureException(error);
    console.error("Cron job failed:", error);
  }
}
```

### 3. Add Logging and Debugging

Include detailed logging to troubleshoot future issues:

```typescript
async function handleScheduledQueueJobs() {
  const startTime = Date.now();
  console.log(`[${new Date().toISOString()}] Starting scheduled queue job processing`);

  const checkInId = Sentry.captureCheckIn({
    monitorSlug: "handle-scheduled-queue-jobs",
    status: "in_progress",
  });

  try {
    const result = await processQueueJobs();
    const duration = Date.now() - startTime;

    console.log(`[${new Date().toISOString()}] Queue job completed successfully in ${duration}ms`, result);

    Sentry.captureCheckIn({
      checkInId,
      monitorSlug: "handle-scheduled-queue-jobs",
      status: "ok",
      duration: duration / 1000, // in seconds
    });

    return result;
  } catch (error) {
    const duration = Date.now() - startTime;

    console.error(`[${new Date().toISOString()}] Queue job failed after ${duration}ms:`, error);

    Sentry.captureCheckIn({
      checkInId,
      monitorSlug: "handle-scheduled-queue-jobs",
      status: "error",
      duration: duration / 1000,
    });

    Sentry.captureException(error, {
      tags: {
        job: "handle-scheduled-queue-jobs",
        duration_ms: duration,
      },
      extra: {
        error_message: error?.message,
        stack: error?.stack,
      },
    });

    throw error;
  }
}
```

### 4. Update Sentry Monitor Configuration

Update the monitor configuration with appropriate thresholds:

```typescript
// Configure in Sentry UI or via API:
{
  "schedule": "* * * * *",  // Every minute
  "schedule_type": "crontab",
  "timezone": "UTC",
  "checkin_margin": 5,  // Allow 5 minute grace period
  "max_runtime": 50,    // Maximum 50 seconds runtime
  "failure_issue_threshold": 3,  // Alert after 3 consecutive failures
  "recovery_threshold": 3  // Recover after 3 consecutive successes
}
```

### 5. Add Health Check Endpoint

Create a health check endpoint to verify the cron system is working:

```typescript
// In your API routes
app.get("/health/cron", async (req, res) => {
  try {
    // Check if cron jobs are running
    const lastRunTime = await getLastCronRunTime("handle-scheduled-queue-jobs");
    const timeSinceLastRun = Date.now() - lastRunTime;

    if (timeSinceLastRun > 120000) { // 2 minutes
      return res.status(503).json({
        status: "unhealthy",
        message: "Cron job hasn't run in over 2 minutes",
        lastRun: new Date(lastRunTime).toISOString(),
      });
    }

    return res.json({
      status: "healthy",
      lastRun: new Date(lastRunTime).toISOString(),
    });
  } catch (error) {
    return res.status(500).json({
      status: "error",
      message: error.message,
    });
  }
});
```

## Testing Plan

1. **Local Testing**: Run the job manually and verify check-ins appear in Sentry
2. **Staging Deployment**: Deploy to staging and monitor for 1 hour
3. **Production Deployment**: Deploy during low-traffic period and monitor closely
4. **Verify in Sentry**: Check that the monitor shows successful check-ins

## Monitoring

After implementing the fix:

1. Monitor the Sentry issue page: https://tembo-io.sentry.io/issues/API-AR
2. Check Sentry Crons dashboard for check-in history
3. Review application logs for any errors or timeouts
4. Set up alerts for consecutive failures

## Related Resources

- [Sentry Cron Monitoring Documentation](https://docs.sentry.io/product/crons/)
- [Bun Cron Job Best Practices](https://bun.sh/docs/api/timers)
- Issue URL: https://tembo-io.sentry.io/issues/API-AR

## Commit Reference

This fix addresses Sentry issue API-AR and should include the reference in commit messages:

```
Fixes API-AR: Add proper error handling and check-ins for scheduled queue jobs
```

This will automatically close the issue when merged to main.
