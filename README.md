# Smart Building KPI Dashboard Example Project

A web application for visualizing Smart Building KPI metrics example project developed in [Rust](https://www.rust-lang.org/). This project show how to use rust for developing wep application.

<p align="center"><img src="https://github.com/mac999/smartbuilding_kpi_sample_server/blob/main/img/img.png" height="300"></img></br><img src="https://github.com/mac999/smartbuilding_kpi_sample_server/blob/main/img/rust_logo.webp" height="100"></img></p>

## Key Features

- **Real-time KPI Monitoring**: Monitor 8 core smart building metrics in real-time
- **Multiple Chart Types**: Data visualization with Line, Bar, and Doughnut charts
- **Responsive Design**: Optimized user experience on all devices
- **Real-time Mode**: Automatic data refresh every 3 seconds
- **Dark Mode UI**: Sophisticated dark theme with glassmorphism design

## Monitored KPI Metrics

1. **Energy Efficiency** (%) - Building energy usage efficiency
2. **Temperature** (°C) - Indoor temperature
3. **Humidity** (%) - Indoor humidity
4. **Air Quality Index** (AQI) - Indoor air quality level
5. **Lighting Usage** (%) - Lighting system usage rate
6. **Occupancy Rate** (%) - Building space occupancy rate
7. **Water Consumption** (L) - Water usage
8. **CO2 Level** (ppm) - Carbon dioxide concentration

## How to Run

### Automatic Execution (Windows)
```batch
run_dashboard.bat
```

### Manual Execution
```bash
# Install dependencies and build
cargo build

# Run application
cargo run
```

Once the server starts, visit `http://localhost:8080` in your browser.

##  Technology Stack

### Backend (Rust)
- **actix-web**: High-performance web framework
- **serde**: JSON serialization/deserialization
- **rand**: Random data generation
- **chrono**: Date/time handling

### Frontend
- **Chart.js**: Data visualization
- **HTML5/CSS3**: Responsive UI
- **JavaScript**: Dynamic interactions

##  Project Structure

```
rust_hello_world/
├── src/
│   └── main.rs              # Main Rust application
├── static/
│   └── index.html           # Web dashboard UI
├── Cargo.toml               # Rust dependency configuration
├── run_dashboard.bat        # Windows execution script
└── README.md                # Project documentation
```

##  UI Features

- **Dark Mode Theme**: Sophisticated and professional dark UI design
- **Gradient Background**: Deep dark gradient background with depth
- **Glassmorphism Effects**: Semi-transparent cards with blur effects
- **Real-time Statistics Cards**: Key KPI values at a glance
- **Interactive Charts**: Dark mode optimized charts with animations
- **Responsive Grid**: Layout optimized for various screen sizes

## Customization

### Adding New KPIs
1. Add fields to the `SmartBuildingKPI` struct in `src/main.rs`
2. Add random value generation logic in the `generate_random_kpi_data()` function
3. Update chart configuration in `static/index.html`

### Changing Data Collection Interval
You can adjust the data refresh interval by modifying the `realtimeInterval` setting in `static/index.html`.

## API Endpoints

- `GET /`: Main dashboard page
- `GET /api/kpi-data`: KPI data in JSON format

## Real-time Features

- Automatic loading of new data every 3 seconds when real-time mode is activated
- Smooth chart updates with animations
- Real-time display of latest values in statistics cards

## Future Improvements

- Integration with real IoT sensor data
- Database connection (PostgreSQL, MongoDB, etc.)
- User authentication and authorization
- Alarm and threshold setting features
- Data export functionality (CSV, Excel)
- Mobile app development

---

**Developer**: Smart Building Monitoring Solution using Rust + Web Technologies

## Author
Taewook Kang (laputa99999@gmail.com)









