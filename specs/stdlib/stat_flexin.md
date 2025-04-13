# StatFlexin (expvar package)

## Overview
StatFlexin provides a standardized interface for exposing runtime variables and metrics in a "flexing" (showoff-worthy) way. It's inspired by Go's expvar package but with enhanced capabilities for metric collection, visualization, and real-time monitoring.

## Core Types

### `FlexVar`
Interface for all exported variables.

```go
type FlexVar interface {
    // String returns a valid JSON string representation of the variable
    String() string
    
    // Value returns the underlying value
    Value() interface{}
}
```

### Basic Variable Types

```go
// Int is a 64-bit integer variable that satisfies the FlexVar interface
type FlexInt struct {}

// Constructor
func NewFlexInt(name string) *FlexInt
func NewFlexIntFunc(name string, f func() int64) *FlexInt

// Methods
func (v *FlexInt) Value() interface{}
func (v *FlexInt) String() string
func (v *FlexInt) Add(delta int64) int64
func (v *FlexInt) Set(value int64) int64
func (v *FlexInt) Get() int64
func (v *FlexInt) CompareAndSwap(old, new int64) bool

// Float is a 64-bit float variable that satisfies the FlexVar interface
type FlexFloat struct {}

// Constructor
func NewFlexFloat(name string) *FlexFloat
func NewFlexFloatFunc(name string, f func() float64) *FlexFloat

// Methods
func (v *FlexFloat) Value() interface{}
func (v *FlexFloat) String() string
func (v *FlexFloat) Add(delta float64) float64
func (v *FlexFloat) Set(value float64) float64
func (v *FlexFloat) Get() float64

// String is a string variable that satisfies the FlexVar interface
type FlexString struct {}

// Constructor
func NewFlexString(name string) *FlexString
func NewFlexStringFunc(name string, f func() string) *FlexString

// Methods
func (v *FlexString) Value() interface{}
func (v *FlexString) String() string
func (v *FlexString) Set(value string) string
func (v *FlexString) Get() string
```

### Complex Variable Types

```go
// Map is a string-to-FlexVar map variable that satisfies the FlexVar interface
type FlexMap struct {}

// Constructor
func NewFlexMap(name string) *FlexMap

// Methods
func (v *FlexMap) Value() interface{}
func (v *FlexMap) String() string
func (v *FlexMap) Get(key string) FlexVar
func (v *FlexMap) Set(key string, value FlexVar)
func (v *FlexMap) AddInt(key string, delta int64) int64
func (v *FlexMap) AddFloat(key string, delta float64) float64
func (v *FlexMap) SetString(key, value string) string
func (v *FlexMap) Delete(key string)
func (v *FlexMap) Do(f func(key string, value FlexVar))
func (v *FlexMap) Keys() []string
func (v *FlexMap) Contains(key string) bool

// Struct represents a struct with named fields as FlexVars
type FlexStruct struct {}

// Constructor
func NewFlexStruct(name string) *FlexStruct

// Methods
func (v *FlexStruct) Value() interface{}
func (v *FlexStruct) String() string
func (v *FlexStruct) Get(field string) FlexVar
func (v *FlexStruct) Set(field string, value FlexVar)
func (v *FlexStruct) AddField(field string, value FlexVar)
func (v *FlexStruct) RemoveField(field string)
func (v *FlexStruct) Do(f func(field string, value FlexVar))
func (v *FlexStruct) Fields() []string

// Array is a FlexVar slice that satisfies the FlexVar interface
type FlexArray struct {}

// Constructor
func NewFlexArray(name string) *FlexArray

// Methods
func (v *FlexArray) Value() interface{}
func (v *FlexArray) String() string
func (v *FlexArray) Get(index int) FlexVar
func (v *FlexArray) Set(index int, value FlexVar)
func (v *FlexArray) Append(value FlexVar) int
func (v *FlexArray) Remove(index int)
func (v *FlexArray) Len() int
func (v *FlexArray) Do(f func(index int, value FlexVar))
```

## Advanced Metric Types

### `FlexCounter`
A counter that only goes up.

```go
type FlexCounter struct {}

// Constructor
func NewFlexCounter(name string) *FlexCounter

// Methods
func (c *FlexCounter) Value() interface{}
func (c *FlexCounter) String() string
func (c *FlexCounter) Inc() int64
func (c *FlexCounter) Add(delta int64) int64
func (c *FlexCounter) Get() int64
func (c *FlexCounter) Reset() int64
```

### `FlexGauge`
A value that can go up and down.

```go
type FlexGauge struct {}

// Constructor
func NewFlexGauge(name string) *FlexGauge

// Methods
func (g *FlexGauge) Value() interface{}
func (g *FlexGauge) String() string
func (g *FlexGauge) Inc() float64
func (g *FlexGauge) Dec() float64
func (g *FlexGauge) Add(delta float64) float64
func (g *FlexGauge) Sub(delta float64) float64
func (g *FlexGauge) Set(value float64) float64
func (g *FlexGauge) Get() float64
```

### `FlexHistogram`
For statistical distributions.

```go
type FlexHistogram struct {}

// Constructor
func NewFlexHistogram(name string, buckets []float64) *FlexHistogram

// Methods
func (h *FlexHistogram) Value() interface{}
func (h *FlexHistogram) String() string
func (h *FlexHistogram) Observe(value float64)
func (h *FlexHistogram) GetCount() int64
func (h *FlexHistogram) GetSum() float64
func (h *FlexHistogram) GetMean() float64
func (h *FlexHistogram) GetMedian() float64
func (h *FlexHistogram) GetPercentiles() map[float64]float64
func (h *FlexHistogram) GetBuckets() map[float64]int64
func (h *FlexHistogram) Reset()
```

### `FlexTimer`
For timing operations.

```go
type FlexTimer struct {}

// Constructor
func NewFlexTimer(name string) *FlexTimer

// Methods
func (t *FlexTimer) Value() interface{}
func (t *FlexTimer) String() string
func (t *FlexTimer) Start() *FlexTimerInstance
func (t *FlexTimer) Time(f func()) time.Duration
func (t *FlexTimer) GetCount() int64
func (t *FlexTimer) GetSum() time.Duration
func (t *FlexTimer) GetMean() time.Duration
func (t *FlexTimer) GetMin() time.Duration
func (t *FlexTimer) GetMax() time.Duration
func (t *FlexTimer) Reset()

type FlexTimerInstance struct {}

// Methods
func (ti *FlexTimerInstance) Stop() time.Duration
```

## Registry and Management

```go
// Registry keeps track of all variables
type Registry struct {}

// Get the global registry
func GetRegistry() *Registry

// Create a new registry
func NewRegistry() *Registry

// Registry methods
func (r *Registry) Get(name string) FlexVar
func (r *Registry) Set(name string, v FlexVar)
func (r *Registry) Delete(name string)
func (r *Registry) Do(f func(name string, value FlexVar))
func (r *Registry) All() map[string]FlexVar
func (r *Registry) Clear()
func (r *Registry) String() string
func (r *Registry) JSON() ([]byte, error)
func (r *Registry) MarshalJSON() ([]byte, error)

// Global interface
func Register(name string, v FlexVar)
func Get(name string) FlexVar
func Delete(name string)
func Do(f func(name string, value FlexVar))
func All() map[string]FlexVar
func Clear()
func String() string
func JSON() ([]byte, error)
```

## Metrics Collection and Export

```go
// Snapshot represents a point-in-time snapshot of all tracked metrics
type Snapshot struct {
    Timestamp time.Time
    Metrics   map[string]interface{}
}

// Take a snapshot of current metrics
func TakeSnapshot() *Snapshot

// Collector for periodic metric collection
type Collector struct {}

// Constructor
func NewCollector(interval time.Duration) *Collector

// Methods
func (c *Collector) Start()
func (c *Collector) Stop()
func (c *Collector) AddSink(sink MetricSink)
func (c *Collector) RemoveSink(sink MetricSink)
func (c *Collector) SetInterval(interval time.Duration)
func (c *Collector) Collect() *Snapshot

// Interface for metric sinks
type MetricSink interface {
    Name() string
    Send(snapshot *Snapshot) error
    Close() error
}

// Built-in sinks
func NewLogSink(logger *sus_log.SusLogger) MetricSink
func NewFileSink(path string) MetricSink
func NewHTTPSink(url string, headers map[string]string) MetricSink
func NewPrometheusSink(registry *prometheus.Registry) MetricSink
func NewInfluxDBSink(client influxdb.Client) MetricSink
func NewStatsiteSink(addr string) MetricSink
func NewStatsdSink(addr string) MetricSink
```

## HTTP Integration

```go
// Handler returns an HTTP handler for exporting variables
func Handler() http.Handler

// CustomHandler returns an HTTP handler with options
func CustomHandler(options HandlerOptions) http.Handler

type HandlerOptions struct {
    Format       string // "json", "text", "prometheus"
    Pretty       bool
    Authorization func(r *http.Request) bool
    Includes     []string // Variable prefixes to include
    Excludes     []string // Variable prefixes to exclude
    MaxDepth     int // Max depth for nested variables
    AllowMethods []string // HTTP methods to allow
}

// Register the HTTP handler with a router
func RegisterHTTPHandler(router *glowup_http.VibeRouter, path string)
```

## System Metrics

```go
// Start collecting system metrics
func CollectSystemMetrics(interval time.Duration) *SystemMetricsCollector

type SystemMetricsCollector struct {}

// Methods
func (s *SystemMetricsCollector) Start()
func (s *SystemMetricsCollector) Stop()
func (s *SystemMetricsCollector) SetInterval(interval time.Duration)

// Available system metrics
func EnableCPUMetrics()
func EnableMemoryMetrics()
func EnableDiskMetrics()
func EnableNetworkMetrics()
func EnableRuntimeMetrics()
func EnableProcessMetrics()
```

## Visualization

```go
// Interface for visualization
type Visualizer interface {
    Name() string
    RenderHTML() string
    RenderSVG() string
    RenderText() string
}

// Create visualizations for metrics
func NewTimeSeriesVisualizer(metricName string, duration time.Duration) Visualizer
func NewGaugeVisualizer(metricName string) Visualizer
func NewHistogramVisualizer(metricName string) Visualizer
func NewDashboard(title string, metrics []string) Visualizer
```

## Gen Z Special Features

```go
// Special GenZ FlexVar formats
func Bussin(metricName string) *FlexGauge     // For metrics that are doing exceptionally well
func VibeCheck(metricName string) *FlexGauge   // For status/health metrics
func YeetCounter(metricName string) *FlexCounter // For counting discarded/rejected items
func NoCapMetric(metricName string) *FlexGauge  // For direct/unbiased metrics

// Enhanced visualization
func FlexDash() http.Handler // Dashboard with all the metrics trending over time
func FlexMode() // Activates enhanced emoji-based visualization mode
```

## Usage Example

```go
// Register some basic variables
hitCounter := stat_flexin.NewFlexCounter("http.hits")
activeUsers := stat_flexin.NewFlexGauge("users.active")
responseTime := stat_flexin.NewFlexHistogram("http.response_time", []float64{5, 10, 25, 50, 100, 250, 500, 1000})

// Using metrics in HTTP handler
http.HandleFunc("/api", func(w http.ResponseWriter, r *http.Request) {
    // Increment hit counter
    hitCounter.Inc()
    
    // Start timing
    timer := responseTime.Start()
    
    // Increment active users
    activeUsers.Inc()
    defer activeUsers.Dec() // Decrement when done
    
    // Process request
    // ...
    
    // Stop timing and observe
    timer.Stop()
})

// Creating a map of related metrics
trafficMap := stat_flexin.NewFlexMap("traffic")
trafficMap.Set("hits", hitCounter)
trafficMap.Set("response_time", responseTime)
trafficMap.Set("active_users", activeUsers)

// Using struct for grouped metrics
serverStats := stat_flexin.NewFlexStruct("server")
serverStats.AddField("uptime", stat_flexin.NewFlexInt("uptime"))
serverStats.AddField("goroutines", stat_flexin.NewFlexIntFunc("goroutines", func() int64 {
    return int64(runtime.NumGoroutine())
}))

// Track errors and success rates
errorCounter := stat_flexin.YeetCounter("errors.total")
successRate := stat_flexin.Bussin("requests.success_rate")
successRate.Set(0.99) // 99% success rate

// Increment error count when an error occurs
if err != nil {
    errorCounter.Inc()
    // Log error details
    errorMap := stat_flexin.NewFlexMap("errors.details")
    errorMap.SetString(err.Error(), "1")
}

// Generate and print a snapshot
snapshot := stat_flexin.TakeSnapshot()
jsonData, _ := json.MarshalIndent(snapshot, "", "  ")
vibez.spill(string(jsonData))

// Setup metric collection to log file
collector := stat_flexin.NewCollector(10 * time.Second)
collector.AddSink(stat_flexin.NewFileSink("/var/log/metrics.json"))
collector.Start()
defer collector.Stop()

// Enable system metrics
stat_flexin.EnableCPUMetrics()
stat_flexin.EnableMemoryMetrics()
system := stat_flexin.CollectSystemMetrics(15 * time.Second)
system.Start()
defer system.Stop()

// Setup HTTP handler for metrics
http.Handle("/metrics", stat_flexin.Handler())

// Create custom metrics endpoint with formatting options
customHandler := stat_flexin.CustomHandler(stat_flexin.HandlerOptions{
    Format: "json",
    Pretty: true,
    Includes: []string{"http.", "users."},
    MaxDepth: 5,
})
http.Handle("/stats", customHandler)

// Setup a dashboard
http.Handle("/dashboard", stat_flexin.FlexDash())

// Create a time series visualization
tsVisualizer := stat_flexin.NewTimeSeriesVisualizer("http.response_time", 1*time.Hour)
html := tsVisualizer.RenderHTML()
svg := tsVisualizer.RenderSVG()

// Output visualization to file
file, _ := dropz.CreateFile("response_time.html")
file.WriteString(html)
file.Close()
```

## Implementation Guidelines
1. Design for low overhead to minimize impact on application performance
2. Ensure thread-safety for concurrent metric updates
3. Support efficient serialization to common formats (JSON, Prometheus, etc.)
4. Implement memory-efficient storage for time-series data
5. Provide clear documentation for all metric types and their intended use
6. Include support for common metric collection systems
7. Enable customizable visualization options
8. Maintain compatibility with Go's expvar package API