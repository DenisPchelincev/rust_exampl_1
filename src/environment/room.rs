use crate::equipment::SmartDevice;

/// Тип, представляющий комнату с множеством устройств.
pub struct Room {
    /// Список умных устройств в комнате (динамический массив).
    equipments: Vec<SmartDevice>
}

impl Room {
    /// Конструктор, создает новый экземпляр комнаты с заданными устройствами
    pub fn new(equipments: Vec<SmartDevice>) -> Self {
        Room { equipments }
    }

    /// Метод для получения списка устройств в комнате
    pub fn get_equipments(&self) -> &Vec<SmartDevice> {
        &self.equipments
    }

    /// Метод возвращает ссылку на конкретное (по индексу) устройство в массиве всх устройств
    /// "комнаты".
    /// Возвращает неизменяемую ссылку на устройство, если индекс валиден, или None в противном
    /// случае.
    pub fn get_device(&self, idx: usize) -> Option<&SmartDevice> {
        self.equipments.get(idx)
    }

    /// Метод возвращает изменяемую ссылку на устройство, если индекс валиден, или None в
    /// противном случае. Это позволяет изменять устройство в "комнате" по ссылке.
    pub fn set_device(&mut self, idx: usize) -> Option<&mut SmartDevice> {
        self.equipments.get_mut(idx)
    }

    /// Метод добавляет к списку устройств комнаты новое устройство.
    pub fn add_device(&mut self, device: SmartDevice) {
        self.equipments.push(device);
    }

    /// Метод удаляет устройство из списка в комнате.
    pub fn remove(&mut self, idx: usize) {
        if idx > self.equipments.len() {
            eprintln!("Ошибка! в списке нет устройства с таким индексом.");
        } else {
            self.equipments.remove(idx);  // remove удаляет и возвращает (удалённый ?) элемент
        }
    }

    /// Метод итеративно выводит состояние по всем устройствам в комнате.
    pub fn report(&self) {
        for (idx, element) in self.equipments.iter().enumerate() {
            print!("- устройство № {}: ", idx);
            element.print_state();
        }
    }

}
