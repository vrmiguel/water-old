#!/usr/bin/env python3
"""
Script to fetch and display recent Todoist tasks.
"""

import os
import sys
import json
import urllib.request
import urllib.parse
from datetime import datetime
from typing import List, Dict, Optional

def get_todoist_token() -> Optional[str]:
    """Get Todoist API token from environment variables."""
    # Check common environment variable names
    token = os.environ.get('TODOIST_API_TOKEN') or \
            os.environ.get('TODOIST_TOKEN') or \
            os.environ.get('TODOIST_API_KEY')
    return token

def fetch_tasks(api_token: str, limit: int = 30) -> List[Dict]:
    """
    Fetch recent tasks from Todoist API.
    
    Args:
        api_token: Todoist API token
        limit: Maximum number of tasks to retrieve
        
    Returns:
        List of task dictionaries
    """
    # Fetch active tasks (recently modified/created)
    url = 'https://api.todoist.com/rest/v2/tasks'
    params = {
        'limit': limit,
        'order_by': 'updated',  # Order by most recently updated
    }
    
    # Build URL with query parameters
    url_with_params = f"{url}?{urllib.parse.urlencode(params)}"
    
    try:
        req = urllib.request.Request(url_with_params)
        req.add_header('Authorization', f'Bearer {api_token}')
        
        with urllib.request.urlopen(req) as response:
            data = response.read()
            return json.loads(data.decode('utf-8'))
    except urllib.error.HTTPError as e:
        error_body = e.read().decode('utf-8') if e.fp else 'No error details'
        print(f"Error fetching tasks: HTTP {e.code} - {e.reason}", file=sys.stderr)
        print(f"Response: {error_body}", file=sys.stderr)
        return []
    except Exception as e:
        print(f"Error fetching tasks: {e}", file=sys.stderr)
        return []

def format_task(task: Dict) -> str:
    """Format a task dictionary into a readable string."""
    content = task.get('content', 'No content')
    due_date = task.get('due', {}).get('date', '') if task.get('due') else None
    priority = task.get('priority', 1)
    is_completed = task.get('is_completed', False)
    project_id = task.get('project_id', '')
    
    # Priority mapping (1-4, where 4 is highest)
    priority_map = {1: 'Low', 2: 'Normal', 3: 'High', 4: 'Urgent'}
    priority_str = priority_map.get(priority, f'Priority {priority}')
    
    status = '✓ Completed' if is_completed else '○ Active'
    
    result = f"{status} | {priority_str} | {content}"
    
    if due_date:
        try:
            # Parse ISO date
            due_dt = datetime.fromisoformat(due_date.replace('Z', '+00:00'))
            result += f" | Due: {due_dt.strftime('%Y-%m-%d %H:%M')}"
        except:
            result += f" | Due: {due_date}"
    
    return result

def main():
    """Main function to fetch and display Todoist tasks."""
    api_token = get_todoist_token()
    
    if not api_token:
        print("=" * 80)
        print("Todoist API Token Required")
        print("=" * 80)
        print("\nTo retrieve your Todoist tasks, you need to provide your Todoist API token.")
        print("\nTo get your API token:")
        print("1. Go to https://todoist.com/app/settings/integrations")
        print("2. Scroll down to 'API token' section")
        print("3. Copy your API token")
        print("\nThen set it as an environment variable:")
        print("  export TODOIST_API_TOKEN='your-token-here'")
        print("  python3 fetch_todoist_tasks.py")
        print("\nOr run the script with the token inline:")
        print("  TODOIST_API_TOKEN='your-token-here' python3 fetch_todoist_tasks.py")
        print("=" * 80)
        sys.exit(1)
    
    print("Fetching your recent Todoist tasks...\n")
    tasks = fetch_tasks(api_token, limit=30)
    
    if not tasks:
        print("No tasks found or error occurred.")
        sys.exit(1)
    
    print(f"Found {len(tasks)} recent task(s):\n")
    print("-" * 80)
    
    for i, task in enumerate(tasks, 1):
        print(f"{i}. {format_task(task)}")
    
    print("-" * 80)

if __name__ == '__main__':
    main()
