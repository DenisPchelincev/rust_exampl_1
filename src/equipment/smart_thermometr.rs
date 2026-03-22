use rand::random;

/// Структура (тип), представляющий умный термометр
pub struct SmartThermometer {
    /// значение текущей температуры
    temperature: f32,
}

impl SmartThermometer {
    /// Конструктор, создает новый экземпляр умного термометра
    pub fn new(temp: f32) -> Self {
        SmartThermometer {
            temperature: if temp < 0.0 { random() } else { temp }
            // temperature: random() // при создании инициируем произвольным числом
        }
    }

    /// Сеттер, устанавливает значение температуры
    pub fn set_temperature(&mut self, temp: f32) {
        self.temperature = temp;
    }

    /// Геттер, возвращает текущее значение температуры
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

/// Модульные тесты для "умного термометра"
/// для геттеров нет тестов, нет особого смысла, и в некоторых тестах геттеры вызываются.
#[cfg(test)]
mod tests {
    use super::SmartThermometer;

    /// тест конструктора "уного термометра"
    #[test]
    fn create_thermometer() {
        let result = SmartThermometer::new(0.0);
        assert_eq!(result.temperature, 0.0, "Умный термометр создан, текущая температура = {}",
                   result.get_temperature());
    }

}