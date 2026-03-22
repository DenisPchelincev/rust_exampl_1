use crate::environment::room::Room;

/// Тип, представляющий дом с множеством (5 штук) комнат.
pub struct Home {
    /// Список комнат, динамический массив (вектор)
    rooms: Vec<Room>,
}

impl Home {

    /// Конструктор, принимает на вход динамический массив комнат
    pub fn new(rooms: Vec<Room>) -> Self {
        Home { rooms }
    }

    /// Метод возвращает ссылку на конкретную (по индексу) комнату в массиве всех комнат дома.
    /// Возвращает неизменяемую ссылку на комнату, если индекс валиден, или None в противном
    /// случае.
    pub fn get_room(&self, idx: usize) -> Option<&Room> {
        self.rooms.get(idx)
    }

    /// Метод возвращает изменяемую ссылку на комнату, если индекс валиден, или None в
    /// противном случае. Это позволяет изменять комнату (по ссылке) в доме.
    pub fn set_room(&mut self, idx: usize) -> Option<&mut Room> {
        self.rooms.get_mut(idx)
    }

    /// Метод добавляет комнату к списку комнат дома.
    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    /// Метод удаляет комнату из списка.
    pub fn remove(&mut self, idx: usize) {
        if idx > self.rooms.len() {
            eprintln!("Ошибка! в списке нет комнаты с таким индексом.");
        } else {
            self.rooms.remove(idx);  // remove удаляет и возвращает (удалённый ?) элемент
        }
    }

    /// Метод рапортует по всем комнатам дома, для каждой комнаты полный отчёт по списку устройств
    pub fn report(&self) {
        for (idx, room) in self.rooms.iter().enumerate() {
            println!(" * комната № {} ---------------------- ", idx);
            for (i, device) in room.get_equipments().iter().enumerate() {
                print!("   - устройство № {} ==> ", i);
                device.print_state();
            }
        }
    }
}