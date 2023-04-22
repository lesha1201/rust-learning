fn main() {
    let celsius = 30.0;

    let fahrenheits = convert_temperature(
        celsius,
        TemperatureUnit::Celsius,
        TemperatureUnit::Fahrenheit,
    );

    println!("{celsius}C -> {fahrenheits}F");

    let fahrenheits = 212.0;

    let celsius = convert_temperature(
        fahrenheits,
        TemperatureUnit::Fahrenheit,
        TemperatureUnit::Celsius,
    );

    println!("{fahrenheits}F -> {celsius}C");
}

//==============================================================================
// convert_temperature
//==============================================================================

enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

/// Receives a temperature and converts from one unit to another.
fn convert_temperature(
    temperature: f64,
    from_unit: TemperatureUnit,
    to_unit: TemperatureUnit,
) -> f64 {
    match (from_unit, to_unit) {
        (TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit) => {
            return (temperature * 9.0 / 5.0) + 32.0;
        }
        (TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius) => {
            return (temperature - 32.0) * 5.0 / 9.0;
        }
        (TemperatureUnit::Celsius, TemperatureUnit::Celsius)
        | (TemperatureUnit::Fahrenheit, TemperatureUnit::Fahrenheit) => {
            return temperature;
        }
    }
}
