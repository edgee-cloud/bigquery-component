# BigQuery Component

This component will allow you to save events to a BigQuery table.

# Inputs
- `project_id`: The ID of the Google Cloud project.
- `dataset_id`: The ID of the BigQuery dataset.
- `table_id`: The ID of the BigQuery table.
- `service-json`: The JSON key file for the service account with permissions to write to BigQuery.

# Table requirements
- The table must have a schema that matches the events you want to save.
```yaml
uuid: STRING
event_type: STRING
timestamp: INTEGER
timestamp_millis: INTEGER
timestamp_micros: INTEGER
consent: STRING
context: JSON
data: JSON
```
