//! Модуль предоставляет реализацию "умного устройства", с подчинёнными модулями "умного
//! термометра" и "умной розетки".

// подключил подчинённые модули
pub mod smart_plug;
pub mod smart_thermometr;

/// Тип "умного устройства", может быть термометром или розеткой
pub enum SmartDevice {
    SmartPlug(smart_plug::SmartPlug),
    SmartThermometer(smart_thermometr::SmartThermometer),
}

impl SmartDevice {
    /// Конструктор, создает новый экземпляр умного устройства
    pub fn new(device_type: &str) -> Self {
        match device_type {
            "plug" =>
                // по умолчанию устройство типа розетка создаётся включенной и с произвольной мощностью
                SmartDevice::SmartPlug(smart_plug::SmartPlug::new(true)),
            "thermometer" => {
                // по умолчанию устройство типа термометр создаётся с произвольной температурой
                SmartDevice::SmartThermometer(smart_thermometr::SmartThermometer::new(-1.0))
            }
            _ => panic!("Unknown device type"),
        }
    }

    /// Вывод в консоль состояния устройства.
    /// Используется неизменяемая ссылка на созданный объект (умное устройство), потому что нужно
    /// только прочитать значение свойств объекта.
    pub fn print_state_ext(&self) -> &str {
        // поскольку SmartDevice — это enum с вариантами SmartPlug и SmartThermometer, то
        // match проверяет, какой вариант у текущего экземпляра,
        // и извлекает вложенные данные (например, plug или thermometer)
        match self {
            // тут `self` — это &SmartDevice, т.е. неизменяемая ссылка на перечисление
            SmartDevice::SmartPlug(plug) => {
                let status = if plug.get_status() {
                    "включена"
                } else {
                    "выключена"
                };
                println!(
                    "Умная розетка: {}, мощность: {:.2} Вт",
                    status,
                    plug.get_power());
                "Создана розетка"
            }
            SmartDevice::SmartThermometer(thermometer) => {
                println!(
                    "Умный термометр: температура: {:.2} °C",
                    thermometer.get_temperature());
                "Создан термометр"
            }
            // _ => компилятор не видит смысла в этом варианте, так как задействовано перечисление
        }
    }

    pub fn print_state(&self) {
        // поскольку SmartDevice — это enum с вариантами SmartPlug и SmartThermometer, то
        // match проверяет, какой вариант у текущего экземпляра,
        // и извлекает вложенные данные (например, plug или thermometer)
        match self {
            // тут `self` — это &SmartDevice, т.е. неизменяемая ссылка на перечисление
            SmartDevice::SmartPlug(plug) => {
                let status = if plug.get_status() {
                    "включена"
                } else {
                    "выключена"
                };
                println!(
                    "Умная розетка: {}, мощность: {:.2} Вт",
                    status,
                    plug.get_power());
            }
            SmartDevice::SmartThermometer(thermometer) => {
                println!(
                    "Умный термометр: температура: {:.2} °C",
                    thermometer.get_temperature());
            }
            // _ => компилятор не видит смысла в этом варианте, так как задействовано перечисление
        }
    }
}

/// Модульные тесты
#[cfg(test)]
mod tests {
    use crate::equipment::SmartDevice;

    /// тест создания "умного устройства" с типом -> "умная розетка"
    #[test]
    fn create_device_plug() {
        // по умолчанию розетка создаётся выключенной
        let result = SmartDevice::new("plug");
        assert_eq!(result.print_state_ext(), "выключена", "Умная розетка создана включенной")
    }

    /// тест создания устройства с заведомо неверным типом
    #[test]
    fn print_status_wrong() {
        // вызовем конструктор с неправильным типом устройства
        let result = SmartDevice::new("thermometer");
        assert_ne!(result.print_state_ext(), "Устройство не создано. Проверьте правильность типа.",
                   "Устройство создано с неправильным типом")
    }
}