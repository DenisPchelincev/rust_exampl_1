use smart_home::environment::{home::Home, room::Room};
use smart_home::equipment::{smart_plug, smart_thermometr, SmartDevice};

fn main() {
    println!("Запуск демонстрации приложения 'Умный дом'.");

    // ----------- Демонстрация одиночного создания "умной розетки", и отработки методов -----------
    let mut plug = smart_plug::SmartPlug::new(true);
    println!("Демонстрация №1. Создана умная розетка, статус {}, мощность {}",
             plug.get_status(), plug.get_power());
    // выключим созданную розетку и запросим её состояние
    plug.turn_off();
    println!("Демонстрация №2. Созданную умную розетку выключаем, теперь: статус {}, мощность {}",
             plug.get_status(), plug.get_power());

    // ----------- Демонстрация одиночного создания "умного термометра", и отработки методов -------
    let mut thermometer = smart_thermometr::SmartThermometer::new(23.5);
    println!("Демонстрация №3. Создан умный термометр, текущая температура {}",
             thermometer.get_temperature());
    // изменим показания температуры
    thermometer.set_temperature(20.7);
    println!("Демонстрация №4. Температура изменена, теперь: текущая температура {}",
             thermometer.get_temperature());

    // ----------- Демонстрация создания "умного устройства" (с разными типами), и методы ----------
    // сначала создадим устройство-розетку, и выведем состояние устройства методом
    println!("Демонстрация №5. Создано умное устройство с типом 'розетка': ");
    let device_1 = SmartDevice::new("plug");
    device_1.print_state();
    // теперь демонстрируем создание умного устройства-термометра с выводом его состояния
    println!("Демонстрация №6. Создано умное устройство с типом 'термометр': ");
    let device_2 = SmartDevice::new("thermometer");
    device_2.print_state();
    // ToDo: можно было заморочиться, и сделать методу управления состоянием
    // (включение/выключение и пр. для "умного устройства", которые вызывали внутренние методы
    // типов розетка и термометр, соответственно)

    // ----------- Демонстрация создания "комнаты" (с 10 умными устройствами разных типов) ---------
    println!("Демонстрация №7. Создана комната с 10-ю умными устройствами: ");
    // так как "комната" не создаёт устройства, а принимает массив устройств - создадим массив
    let devices: Vec<SmartDevice> = Vec::new();
    let mut room = Room::new(devices);
    room.add_device(SmartDevice::new("plug")); // добавим только что созданную розетку
    room.add_device( SmartDevice::new("thermometer"));
    room.add_device(SmartDevice::new("plug"));
    room.report();  // вывод состояния всех умных устройств в комнате

    // ----------- Демонстрация создания дома с полным отчётом -------------------------------------
    println!(" ============================================================ ");
    println!("Демонстрация №8. Создан дом с 3-мя комнатами: ");
    // Первая комната
    let devices_1: Vec<SmartDevice> = Vec::new();    // список устройств для первой комнаты
    let mut room_1 = Room::new(devices_1);  // первая комната
    room_1.add_device(SmartDevice::new("plug")); // добавим только что созданную розетку
    room_1.add_device( SmartDevice::new("thermometer"));
    room_1.add_device(SmartDevice::new("plug"));

    // Вторая комната
    let devices_2: Vec<SmartDevice> = Vec::new();    // список устройств для второй комнаты
    let mut room_2 = Room::new(devices_2);
    room_2.add_device(SmartDevice::new("plug")); // добавим только что созданную розетку
    room_2.add_device( SmartDevice::new("thermometer"));
    room_2.add_device(SmartDevice::new("plug"));
    room_2.add_device( SmartDevice::new("thermometer"));
    room_2.add_device(SmartDevice::new("plug"));

    // Третья комната
    let devices_3: Vec<SmartDevice> = Vec::new();    // список устройств для третьей комнаты
    let mut room_3 = Room::new(devices_3);
    room_3.add_device(SmartDevice::new("plug")); // добавим только что созданную розетку
    room_3.add_device( SmartDevice::new("thermometer"));
    room_3.add_device(SmartDevice::new("plug"));
    room_3.add_device( SmartDevice::new("thermometer"));
    room_3.add_device(SmartDevice::new("plug"));
    room_3.add_device( SmartDevice::new("thermometer"));
    room_3.add_device(SmartDevice::new("plug"));

    // Дом, и подключаем комнаты с индивидуальными списками устройств
    let rooms: Vec<Room> = Vec::new(); // создадим динамический массив комнат для дома
    let mut home = Home::new(rooms);  // создадим объект дом
    home.add_room(room_1);  // добавим в список первую комнату
    home.add_room(room_2);  // добавим в список вторую комнату
    home.add_room(room_3);  // добавим в список третью комнату
    println!("__ отчёт по всем комнатам __ ");
    home.report();  // вывод состояния всех умных устройств в комнатах дома

    println!(" ============================================================ ");
    println!("Демонстрация №9. Выключаем 2-ю розетку во 2-й комнате: ");
    // получаем изменяемую ссылку на 2-ю розетку (третье в списке устройство) во 2-й комнате
    let current_device = home.set_room(1).unwrap().set_device(2).unwrap();
    // выключаем нужную нам розетку (если это не розетка - выдаём сообщение об ошибке)
    match current_device {
        SmartDevice::SmartPlug(smart_plug) => {
            smart_plug.turn_off();
        },
        SmartDevice::SmartThermometer(_smart_thermometr) =>
            eprintln!("Ошибка! Попытка выключить розетку, а там оказался термометр.")
    }
    // снова выводим полный отчёт об устройствах в комнатах дома
    home.report();
}
