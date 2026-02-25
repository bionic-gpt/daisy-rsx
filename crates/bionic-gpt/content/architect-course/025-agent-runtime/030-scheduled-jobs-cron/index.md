# Scheduled Jobs (Cron)

```js
# Create a scheduled job
cron(schedule: string, tool_name: string, arguments: object): string

# List scheduled jobs
cron_list(): object[]

# Update an existing scheduled job
cron_update(job_id: string, schedule: string, arguments: object): string

# Delete a scheduled job
cron_delete(job_id: string): string

# Health check for scheduler loop
heartbeat(): string
```
