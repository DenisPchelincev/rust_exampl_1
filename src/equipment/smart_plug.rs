/// Структура (тип), представляющий умную розетку
pub struct SmartPlug {
    /// текущее состояние (включен, выключен)
    status: bool,
    /// текущая мощность: если выключено — ноль, иначе произвольное число
    power: f32,
}

impl SmartPlug {
    /// Конструктор, создает новый экземпляр умной розетки с заданными параметрами
    pub fn new(once_state: bool) -> Self {
        let pw = rand::random::<f32>();
        //if once_state { pw = random() % 100; } else { pw = 0;}
        SmartPlug { status: once_state, power: if once_state { pw } else { 0.0 } }
    }

    /// Метод для включения розетки
    pub fn turn_on(&mut self) -> bool {
        self.status = true; // включаем розетку
        self.power = rand::random::<f32>(); // при включении инициируем произвольным числом
        self.status // возвращаем текущий статус розетки
    }

    /// Метод для выключения розетки
    pub fn turn_off(&mut self) -> bool {
        self.status = false; // выключаем розетку
        self.power = 0.0;   // текущую мощность сбрасываем в 0
        self.status  // возвращаем текущий статус розетки
    }

    /// Геттер, возвращает текущее состояние розетки
    pub fn get_status(&self) -> bool {
        self.status
    }

    /// Геттер, возвращает текущую мощность
    pub fn get_power(&self) -> f32 {
        self.power
    }
}

/// Модульные тесты для "умной розетки"
/// для геттеров нет тестов, нет особого смысла, и в некоторых тестах геттеры вызываются.
#[cfg(test)]
mod tests {
    use super::SmartPlug;

    /// тест создания "уной розетки" включенной
    #[test]
    fn create_plug_on() {
        let result = SmartPlug::new(true);
        assert!(result.power > 0.0, "Умная розетка создана, статус = {} и мощность = {}", result
            .get_status(), result.get_power());
    }

    /// тест создания "уной розетки" выключенной
    #[test]
    fn create_plug_off() {
        let result = SmartPlug::new(false);
        assert_ne!(result.status, true, "Умная розетка создана, должна быть выключеной");
    }

    /// тест метода включения умной розетки
    #[test]
    fn plug_turn_on() {
        // розетка создаётся выключенной
        let mut result = SmartPlug::new(false);
        // принудительно включаем розетку - вызываем соответствующий метод
        result.turn_on();
        assert_eq!(result.status, true, "Тест включения розетки. Статус: {}, млщность {}", result
            .status, result.power);
    }

    /// тест метода выключения умной розетки
    #[test]
    fn plug_turn_off() {
        // розетка создаётся включенной
        let mut result = SmartPlug::new(true);
        // принудительно выключаем розетку - вызываем соответствующий метод
        result.turn_off();
        assert_eq!(result.status, false, "Тест выключения розетки. Статус: {}, млщность {}", result
            .status, result.power);
    }
}