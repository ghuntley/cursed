# StatFlexin (expvar package)

## Overview
StatFlexin provides a standardized collab for exposing runtime variables and metrics in a "flexing" (showoff-worthy) way. It's inspired by Go's expvar package but with enhanced capabilities for metric collection, visualization, and real-time monitoring.

## Core Types

### `FlexVar`
Interface for all exported variables.

```
be_like FlexVar collab {
    fr fr String yolos a valid JSON tea representation of the variable
    String() tea
    
    fr fr Value yolos the underlying value
    Value() interface{}
}
```

### Basic Variable Types

```
fr fr Int is a 64-bit integer variable that satisfies the FlexVar interface
be_like FlexInt squad {}

fr fr Consquador
slay NewFlexInt(name tea) *FlexInt
slay NewFlexIntFunc(name tea, f func() int64) *FlexInt

fr fr Methods
slay (v *FlexInt) Value() interface{}
slay (v *FlexInt) String() tea
slay (v *FlexInt) Add(delta int64) int64
slay (v *FlexInt) Set(value int64) int64
slay (v *FlexInt) Get() int64
slay (v *FlexInt) CompareAndSwap(old, new int64) lit

fr fr Float is a 64-bit float variable that satisfies the FlexVar interface
be_like FlexFloat squad {}

fr fr Consquador
slay NewFlexFloat(name tea) *FlexFloat
slay NewFlexFloatFunc(name tea, f func() float64) *FlexFloat

fr fr Methods
slay (v *FlexFloat) Value() interface{}
slay (v *FlexFloat) String() tea
slay (v *FlexFloat) Add(delta float64) float64
slay (v *FlexFloat) Set(value float64) float64
slay (v *FlexFloat) Get() float64

fr fr String is a tea variable that satisfies the FlexVar interface
be_like FlexString squad {}

fr fr Consquador
slay NewFlexString(name tea) *FlexString
slay NewFlexStringFunc(name tea, f func() tea) *FlexString

fr fr Methods
slay (v *FlexString) Value() interface{}
slay (v *FlexString) String() tea
slay (v *FlexString) Set(value tea) tea
slay (v *FlexString) Get() tea
```

### Complex Variable Types

```
fr fr Map is a tea-to-FlexVar map variable that satisfies the FlexVar interface
be_like FlexMap squad {}

fr fr Consquador
slay NewFlexMap(name tea) *FlexMap

fr fr Methods
slay (v *FlexMap) Value() interface{}
slay (v *FlexMap) String() tea
slay (v *FlexMap) Get(key tea) FlexVar
slay (v *FlexMap) Set(key tea, value FlexVar)
slay (v *FlexMap) AddInt(key tea, delta int64) int64
slay (v *FlexMap) AddFloat(key tea, delta float64) float64
slay (v *FlexMap) SetString(key, value tea) tea
slay (v *FlexMap) Delete(key tea)
slay (v *FlexMap) Do(f func(key tea, value FlexVar))
slay (v *FlexMap) Keys() []tea
slay (v *FlexMap) Contains(key tea) lit

fr fr Struct represents a squad with named fields as FlexVars
be_like FlexStruct squad {}

fr fr Consquador
slay NewFlexStruct(name tea) *FlexStruct

fr fr Methods
slay (v *FlexStruct) Value() interface{}
slay (v *FlexStruct) String() tea
slay (v *FlexStruct) Get(field tea) FlexVar
slay (v *FlexStruct) Set(field tea, value FlexVar)
slay (v *FlexStruct) AddField(field tea, value FlexVar)
slay (v *FlexStruct) RemoveField(field tea)
slay (v *FlexStruct) Do(f func(field tea, value FlexVar))
slay (v *FlexStruct) Fields() []tea

fr fr Array is a FlexVar slice that satisfies the FlexVar interface
be_like FlexArray squad {}

fr fr Consquador
slay NewFlexArray(name tea) *FlexArray

fr fr Methods
slay (v *FlexArray) Value() interface{}
slay (v *FlexArray) String() tea
slay (v *FlexArray) Get(index normie) FlexVar
slay (v *FlexArray) Set(index int, value FlexVar)
slay (v *FlexArray) Append(value FlexVar) int
slay (v *FlexArray) Remove(index normie)
slay (v *FlexArray) Len() int
slay (v *FlexArray) Do(f func(index int, value FlexVar))
```

## Advanced Metric Types

### `FlexCounter`
A counter that only goes up.

```
be_like FlexCounter squad {}

fr fr Consquador
slay NewFlexCounter(name tea) *FlexCounter

fr fr Methods
slay (c *FlexCounter) Value() interface{}
slay (c *FlexCounter) String() tea
slay (c *FlexCounter) Inc() int64
slay (c *FlexCounter) Add(delta int64) int64
slay (c *FlexCounter) Get() int64
slay (c *FlexCounter) Reset() int64
```

### `FlexGauge`
A value that can go up and down.

```
be_like FlexGauge squad {}

fr fr Consquador
slay NewFlexGauge(name tea) *FlexGauge

fr fr Methods
slay (g *FlexGauge) Value() interface{}
slay (g *FlexGauge) String() tea
slay (g *FlexGauge) Inc() float64
slay (g *FlexGauge) Dec() float64
slay (g *FlexGauge) Add(delta float64) float64
slay (g *FlexGauge) Sub(delta float64) float64
slay (g *FlexGauge) Set(value float64) float64
slay (g *FlexGauge) Get() float64
```

### `FlexHistogram`
For statistical distributions.

```
be_like FlexHistogram squad {}

fr fr Consquador
slay NewFlexHistogram(name tea, buckets []float64) *FlexHistogram

fr fr Methods
slay (h *FlexHistogram) Value() interface{}
slay (h *FlexHistogram) String() tea
slay (h *FlexHistogram) Observe(value float64)
slay (h *FlexHistogram) GetCount() int64
slay (h *FlexHistogram) GetSum() float64
slay (h *FlexHistogram) GetMean() float64
slay (h *FlexHistogram) GetMedian() float64
slay (h *FlexHistogram) GetPercentiles() map[float64]float64
slay (h *FlexHistogram) GetBuckets() map[float64]int64
slay (h *FlexHistogram) Reset()
```

### `FlexTimer`
For timing operations.

```
be_like FlexTimer squad {}

fr fr Consquador
slay NewFlexTimer(name tea) *FlexTimer

fr fr Methods
slay (t *FlexTimer) Value() interface{}
slay (t *FlexTimer) String() tea
slay (t *FlexTimer) Start() *FlexTimerInstance
slay (t *FlexTimer) Time(f func()) time.Duration
slay (t *FlexTimer) GetCount() int64
slay (t *FlexTimer) GetSum() time.Duration
slay (t *FlexTimer) GetMean() time.Duration
slay (t *FlexTimer) GetMin() time.Duration
slay (t *FlexTimer) GetMax() time.Duration
slay (t *FlexTimer) Reset()

be_like FlexTimerInstance squad {}

fr fr Methods
slay (ti *FlexTimerInstance) Stop() time.Duration
```

## Registry and Management

```
fr fr Registry keeps track of all variables
be_like Registry squad {}

fr fr Get the global registry
slay GetRegistry() *Registry

fr fr Create a new registry
slay NewRegistry() *Registry

fr fr Registry methods
slay (r *Registry) Get(name tea) FlexVar
slay (r *Registry) Set(name tea, v FlexVar)
slay (r *Registry) Delete(name tea)
slay (r *Registry) Do(f func(name tea, value FlexVar))
slay (r *Registry) All() map[tea]FlexVar
slay (r *Registry) Clear()
slay (r *Registry) String() tea
slay (r *Registry) JSON() ([]byte, tea)
slay (r *Registry) MarshalJSON() ([]byte, tea)

fr fr Global interface
slay Register(name tea, v FlexVar)
slay Get(name tea) FlexVar
slay Delete(name tea)
slay Do(f func(name tea, value FlexVar))
slay All() map[tea]FlexVar
slay Clear()
slay String() tea
slay JSON() ([]byte, tea)
```

## Metrics Collection and Export

```
fr fr Snapshot represents a point-in-time snapshot of all tracked metrics
be_like Snapshot squad {
    Timestamp time.Time
    Metrics   map[tea]interface{}
}

fr fr Take a snapshot of current metrics
slay TakeSnapshot() *Snapshot

fr fr Collector for periodic metric collection
be_like Collector squad {}

fr fr Consquador
slay NewCollector(interval time.Duration) *Collector

fr fr Methods
slay (c *Collector) Start()
slay (c *Collector) Stop()
slay (c *Collector) AddSink(sink MetricSink)
slay (c *Collector) RemoveSink(sink MetricSink)
slay (c *Collector) SetInterval(interval time.Duration)
slay (c *Collector) Collect() *Snapshot

fr fr Interface for metric sinks
be_like MetricSink collab {
    Name() tea
    Send(snapshot *Snapshot) tea
    Close() tea
}

fr fr Built-in sinks
slay NewLogSink(logger *sus_log.SusLogger) MetricSink
slay NewFileSink(path tea) MetricSink
slay NewHTTPSink(url tea, headers map[tea]tea) MetricSink
slay NewPrometheusSink(registry *prometheus.Registry) MetricSink
slay NewInfluxDBSink(client influxdb.Client) MetricSink
slay NewStatsiteSink(addr tea) MetricSink
slay NewStatsdSink(addr tea) MetricSink
```

## HTTP Integration

```
fr fr Handler yolos an HTTP handler for exporting variables
slay Handler() http.Handler

fr fr CustomHandler yolos an HTTP handler with options
slay CustomHandler(options HandlerOptions) http.Handler

be_like HandlerOptions squad {
    Format       tea fr fr "json", "text", "prometheus"
    Pretty       lit
    Authorization func(r *http.Request) lit
    Includes     []tea fr fr Variable prefixes to include
    Excludes     []tea fr fr Variable prefixes to exclude
    MaxDepth     normie fr fr Max depth for nested variables
    AllowMethods []tea fr fr HTTP methods to allow
}

fr fr Register the HTTP handler with a router
slay RegisterHTTPHandler(router *glowup_http.VibeRouter, path tea)
```

## System Metrics

```
fr fr Start collecting system metrics
slay CollectSystemMetrics(interval time.Duration) *SystemMetricsCollector

be_like SystemMetricsCollector squad {}

fr fr Methods
slay (s *SystemMetricsCollector) Start()
slay (s *SystemMetricsCollector) Stop()
slay (s *SystemMetricsCollector) SetInterval(interval time.Duration)

fr fr Available system metrics
slay EnableCPUMetrics()
slay EnableMemoryMetrics()
slay EnableDiskMetrics()
slay EnableNetworkMetrics()
slay EnableRuntimeMetrics()
slay EnableProcessMetrics()
```

## Visualization

```
fr fr Interface for visualization
be_like Visualizer collab {
    Name() tea
    RenderHTML() tea
    RenderSVG() tea
    RenderText() tea
}

fr fr Create visualizations for metrics
slay NewTimeSeriesVisualizer(metricName tea, duration time.Duration) Visualizer
slay NewGaugeVisualizer(metricName tea) Visualizer
slay NewHistogramVisualizer(metricName tea) Visualizer
slay NewDashboard(title tea, metrics []tea) Visualizer
```

## Gen Z Special Features

```
fr fr Special GenZ FlexVar formats
slay Bussin(metricName tea) *FlexGauge     fr fr For metrics that are doing exceptionally well
slay VibeCheck(metricName tea) *FlexGauge   fr fr For status/health metrics
slay YeetCounter(metricName tea) *FlexCounter fr fr For counting discarded/rejected items
slay NoCapMetric(metricName tea) *FlexGauge  fr fr For direct/unbiased metrics

fr fr Enhanced visualization
slay FlexDash() http.Handler fr fr Dashboard with all the metrics trending over time
slay FlexMode() fr fr Activates enhanced emoji-based visualization mode
```

## Usage Example

```
fr fr Register some basic variables
hitCounter := stat_flexin.NewFlexCounter("http.hits")
activeUsers := stat_flexin.NewFlexGauge("users.active")
responseTime := stat_flexin.NewFlexHistogram("http.response_time", []float64{5, 10, 25, 50, 100, 250, 500, 1000})

fr fr Using metrics in HTTP handler
http.HandleFunc("/api", func(w http.ResponseWriter, r *http.Request) {
    fr fr Increment hit counter
    hitCounter.Inc()
    
    fr fr Start timing
    timer := responseTime.Start()
    
    fr fr Increment active users
    activeUsers.Inc()
    defer activeUsers.Dec() fr fr Decrement when done
    
    fr fr Process request
    fr fr ...
    
    fr fr Stop timing and observe
    timer.Stop()
})

fr fr Creating a map of related metrics
trafficMap := stat_flexin.NewFlexMap("traffic")
trafficMap.Set("hits", hitCounter)
trafficMap.Set("response_time", responseTime)
trafficMap.Set("active_users", activeUsers)

fr fr Using squad for grouped metrics
serverStats := stat_flexin.NewFlexStruct("server")
serverStats.AddField("uptime", stat_flexin.NewFlexInt("uptime"))
serverStats.AddField("goroutines", stat_flexin.NewFlexIntFunc("goroutines", func() int64 {
    yolo int64(runtime.NumGoroutine())
}))

fr fr Track teas and success rates
teaCounter := stat_flexin.YeetCounter("teas.total")
successRate := stat_flexin.Bussin("requests.success_rate")
successRate.Set(0.99) fr fr 99% success rate

fr fr Increment tea count when an tea occurs
if err != nah {
    teaCounter.Inc()
    fr fr Log tea details
    teaMap := stat_flexin.NewFlexMap("teas.details")
    teaMap.SetString(err.Error(), "1")
}

fr fr Generate and prnormie a snapshot
snapshot := stat_flexin.TakeSnapshot()
jsonData, _ := json.MarshalIndent(snapshot, "", "  ")
vibez.spill(tea(jsonData))

fr fr Setup metric collection to log file
collector := stat_flexin.NewCollector(10 * time.Second)
collector.AddSink(stat_flexin.NewFileSink("/var/log/metrics.json"))
collector.Start()
defer collector.Stop()

fr fr Enable system metrics
stat_flexin.EnableCPUMetrics()
stat_flexin.EnableMemoryMetrics()
system := stat_flexin.CollectSystemMetrics(15 * time.Second)
system.Start()
defer system.Stop()

fr fr Setup HTTP handler for metrics
http.Handle("/metrics", stat_flexin.Handler())

fr fr Create custom metrics endponormie with formatting options
customHandler := stat_flexin.CustomHandler(stat_flexin.HandlerOptions{
    Format: "json",
    Pretty: based,
    Includes: []tea{"http.", "users."},
    MaxDepth: 5,
})
http.Handle("/stats", customHandler)

fr fr Setup a dashboard
http.Handle("/dashboard", stat_flexin.FlexDash())

fr fr Create a time series visualization
tsVisualizer := stat_flexin.NewTimeSeriesVisualizer("http.response_time", 1*time.Hour)
html := tsVisualizer.RenderHTML()
svg := tsVisualizer.RenderSVG()

fr fr Output visualization to file
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