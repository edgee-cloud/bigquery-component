<div align="center">
<p align="center">
  <a href="https://www.edgee.cloud">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://cdn.edgee.cloud/img/component-dark.svg">
      <img src="https://cdn.edgee.cloud/img/component.svg" height="100" alt="Edgee">
    </picture>
  </a>
</p>
</div>

<h1 align="center">BigQuery Component for Edgee</h1>

[![Edgee Component Registry](https://img.shields.io/badge/Edgee_Component_Registry-Public-green.svg)](https://www.edgee.cloud/edgee/bigquery)

This component enables seamless integration between [Edgee](https://www.edgee.cloud) and [BigQuery](https://cloud.google.com/bigquery), allowing you to collect and forward analytics events to your BigQuery tables.

## Quick Start

1. Download the latest component version from our [releases page](../../releases).
2. Place the `bigquery.wasm` file in your server (e.g., `/var/edgee/components`).
3. Add the following configuration to your `edgee.toml`:

```toml
[[destinations.data_collection]]
id = "bigquery"
file = "/var/edgee/components/bigquery.wasm"
settings.project_id = "your-project-id"
settings.dataset_id = "your-dataset-id"
settings.table_id = "your-table-id"
settings.service_json = "path/to/your/service-account-key.json"
```

## Event Handling

First of all, create a new table with the following schema:

```sql
CREATE TABLE `your-project-id.your-dataset-id.your-table-id` (
  uuid STRING,
  event_type STRING,
  timestamp INT64,
  timestamp_millis INT64,
  timestamp_micros INT64,
  consent STRING,
  context JSON,
  data JSON
);
```

### JSON Fields

New records are ingested individually using BigQuery's streaming API. If your BigQuery table supports JSON types, both `context` and `data` will contain additional JSON sub-fields, whose schema is automatically inferred at runtime.

Please note that:
- The sub-fields under `context` are always the same, so you can use queries such as ```SELECT context.client.ip AS ip FROM `your-project-id.your-dataset-id.your-table-id```.
- The sub-fields under `data` depend on the value of `event_type`, so you can use queries such as:
  - ```SELECT data.Track.name FROM `your-project-id.your-dataset-id.your-table-id` WHERE event_type = 'Track'```
  - ```SELECT data.Page.path FROM `your-project-id.your-dataset-id.your-table-id` WHERE event_type = 'Page'```

### Event Mapping

The component maps Edgee events to BigQuery records as follows.

|Edgee Event|BigQuery Record|Description|
|---|---|---|
|Page|A new record in the configured table|Full JSON dump of the Page event|
|Track|A new record in the configured table|Full JSON dump of the Track event|
|User|A new record in the configured table|Full JSON dump of the User event|

## Configuration Options

### Basic Configuration

```toml
[[destinations.data_collection]]
id = "bigquery"
file = "/var/edgee/components/bigquery.wasm"
settings.project_id = "your-project-id"
settings.dataset_id = "your-dataset-id"
settings.table_id = "your-table-id"
settings.service_json = "path/to/your/service-account-key.json"
```

### Event Controls

Control which events are forwarded to BigQuery:

```toml
settings.edgee_page_event_enabled = true   # Enable/disable page view tracking
settings.edgee_track_event_enabled = true  # Enable/disable custom event tracking
settings.edgee_user_event_enabled = true   # Enable/disable user identification
```

## Development

### Building from Source

Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)

Build command:
```bash
edgee component build
```

Test command:
```bash
make test
```

### Contributing

Interested in contributing? Read our [contribution guidelines](./CONTRIBUTING.md).

### Security

Report security vulnerabilities to [security@edgee.cloud](mailto:security@edgee.cloud).
