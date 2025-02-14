module hyaena-technologies-server

go 1.24

require (
	github.com/GreptimeTeam/greptimedb-ingester-go v0.6.0
	github.com/minio/minio-go/v7 v7.0.85
	github.com/surrealdb/surrealdb.go v0.3.2
	go.opentelemetry.io/contrib/instrumentation/net/http/httptrace v0.11.0
	go.opentelemetry.io/contrib/instrumentation/net/http/otelhttp v0.59.0
	go.opentelemetry.io/otel/exporters/stdout/stdoutlog v0.10.0
	go.opentelemetry.io/otel/exporters/stdout/stdoutmetric v1.34.0
	go.opentelemetry.io/otel/exporters/stdout/stdouttrace v1.34.0
	go.opentelemetry.io/otel/log v0.10.0
	go.opentelemetry.io/otel/metric v1.34.0
	go.opentelemetry.io/otel/sdk/log v0.10.0
	go.opentelemetry.io/otel/sdk/metric v1.34.0
	go.opentelemetry.io/otel/trace v1.34.0
)
