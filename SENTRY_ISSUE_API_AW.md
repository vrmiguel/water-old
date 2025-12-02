# Sentry Issue API-AW: Process Integrations Cron Failure

## Issue Summary

**Issue ID:** API-AW
**Issue URL:** https://tembo-io.sentry.io/issues/API-AW
**Type:** Cron Monitor Check-In Failure
**Status:** Unresolved (Regressed)
**Priority:** High
**Project:** api (Bun)
**First Seen:** 2025-11-07
**Last Seen:** 2025-12-02
**Occurrences:** 26

## Problem Description

The "Process Integrations" cron job is failing to send check-ins to Sentry. The monitor is configured to run daily at 1:00 AM UTC (schedule: `0 1 * * *`), but missed check-ins are being detected, indicating that either:

1. The cron job is not executing at all
2. The cron job is executing but failing before it can send check-ins to Sentry
3. The Sentry check-in instrumentation is misconfigured or broken

## Monitor Configuration

- **Monitor ID:** bc8d71af-f21b-4e36-9db9-7475352a4b67
- **Monitor Slug:** process-integrations
- **Schedule:** Daily at 1:00 AM UTC (`0 1 * * *`)
- **Schedule Type:** Cron (type 1)
- **Timezone:** UTC
- **Status:** Error

## Root Cause Analysis

Based on the Sentry data, the most likely causes are:

### 1. Missing or Broken Cron Job Scheduler
The cron job may not be properly registered in the scheduler. Check:
- Cron job registration in the application startup
- Environment-specific cron configurations
- Scheduler health and status

### 2. Failure Before Check-In
The job may be failing early in execution before it can send the initial check-in signal to Sentry. Check:
- Error logs around 1:00 AM UTC daily
- Database connection issues at job startup
- Authentication/authorization failures
- Missing environment variables or configuration

### 3. Sentry Instrumentation Issues
The Sentry monitor check-in code may be misconfigured. Check:
- Sentry SDK version compatibility
- Monitor slug matches configuration
- Check-in API calls are being made correctly

## Recommended Fixes

### Immediate Actions

1. **Add Comprehensive Error Handling and Logging**
   ```typescript
   import * as Sentry from '@sentry/bun';

   async function processIntegrations() {
     const checkInId = Sentry.captureCheckIn({
       monitorSlug: 'process-integrations',
       status: 'in_progress',
     });

     try {
       console.log('[Process Integrations] Starting job...');

       // Add detailed logging at each step
       console.log('[Process Integrations] Connecting to database...');
       // Database operations

       console.log('[Process Integrations] Fetching integrations...');
       // Fetch integrations

       console.log('[Process Integrations] Processing integrations...');
       // Process each integration

       console.log('[Process Integrations] Job completed successfully');

       Sentry.captureCheckIn({
         checkInId,
         monitorSlug: 'process-integrations',
         status: 'ok',
       });
     } catch (error) {
       console.error('[Process Integrations] Job failed:', error);

       Sentry.captureCheckIn({
         checkInId,
         monitorSlug: 'process-integrations',
         status: 'error',
       });

       // Also capture the exception
       Sentry.captureException(error, {
         tags: {
           job: 'process-integrations',
         },
       });

       throw error;
     }
   }
   ```

2. **Verify Cron Job Registration**
   Ensure the cron job is properly registered in the scheduler:
   ```typescript
   // In your scheduler configuration file
   import { CronJob } from 'cron';

   const processIntegrationsJob = new CronJob(
     '0 1 * * *', // Daily at 1:00 AM UTC
     async () => {
       try {
         await processIntegrations();
       } catch (error) {
         console.error('Process Integrations cron job failed:', error);
       }
     },
     null,
     true, // Start immediately
     'UTC'
   );

   // Make sure the job is started
   processIntegrationsJob.start();
   console.log('Process Integrations cron job registered');
   ```

3. **Add Health Check Endpoint**
   Create an endpoint to verify the cron job status:
   ```typescript
   app.get('/health/cron/process-integrations', async (req, res) => {
     const lastRun = await getLastJobRun('process-integrations');
     const nextRun = processIntegrationsJob.nextDate();

     return res.json({
       status: 'ok',
       lastRun,
       nextRun,
       isRunning: processIntegrationsJob.running,
     });
   });
   ```

4. **Add Timeout Protection**
   ```typescript
   import * as Sentry from '@sentry/bun';

   const TIMEOUT_MS = 5 * 60 * 1000; // 5 minutes

   async function processIntegrationsWithTimeout() {
     const checkInId = Sentry.captureCheckIn({
       monitorSlug: 'process-integrations',
       status: 'in_progress',
     });

     try {
       const result = await Promise.race([
         processIntegrations(),
         new Promise((_, reject) =>
           setTimeout(() => reject(new Error('Job timeout')), TIMEOUT_MS)
         ),
       ]);

       Sentry.captureCheckIn({
         checkInId,
         monitorSlug: 'process-integrations',
         status: 'ok',
       });

       return result;
     } catch (error) {
       Sentry.captureCheckIn({
         checkInId,
         monitorSlug: 'process-integrations',
         status: 'error',
       });

       throw error;
     }
   }
   ```

### Long-term Improvements

1. **Add Retry Logic**
   Implement exponential backoff for transient failures

2. **Add Alerting**
   Set up additional alerts for job failures beyond Sentry

3. **Add Metrics**
   Track job duration, success rate, and processed items

4. **Add Dead Letter Queue**
   Store failed integration processing attempts for retry

5. **Consider Job Idempotency**
   Ensure the job can be safely retried without side effects

## Testing the Fix

After implementing the fixes:

1. Monitor Sentry for successful check-ins at the next scheduled run (1:00 AM UTC)
2. Check application logs for the detailed logging output
3. Verify the health check endpoint returns expected data
4. Test the job manually to ensure check-ins are sent correctly
5. Monitor for 7 days to ensure the issue is resolved

## Related Resources

- [Sentry Cron Monitoring Documentation](https://docs.sentry.io/product/crons/)
- [Bun Cron Job Best Practices](https://bun.sh/)
- Issue URL: https://tembo-io.sentry.io/issues/API-AW

---

**Note:** This issue affects the `api` project, not this repository. This document serves as documentation for the fix that should be implemented in the API codebase.
