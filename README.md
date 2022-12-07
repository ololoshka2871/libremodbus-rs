# libremodbus-rs
Порт [libremodbus](https://github.com/nucleron/libremodbus) на Rust.

*Отладжена только реализация RTU*

## Использование
1. Создать структуры, реализующие трейты: `SerialInterface`, `TimerInterface`, `DataInterface`
2. Создать статические объекты, из реализованых структур
    ```rust
    static mut UART: Option<
        Serial<
            libremodbus_rs::Serial<_, (_, _)>,
            _,
        >,
    > = None;

    static mut MODBUS_TIMER: Option<Timer<_>> = None;
    static mut DATA_STORAGE: Option<DataStorage> = None;

    unsafe {
        UART.replace(Serial::new(...),
            re_de_pin,
            clocks,
        );

        MODBUS_TIMER.replace(Timer::new(...));
        DATA_STORAGE.replace(DataStorage::new(...));
    }
    ```
3. Инициализировать modbus: 
    ```rust
     let rtu = unsafe {
        libremodbus_rs::Rtu::init(
            MODBUS_ADDR,
            UART.as_mut().unwrap_unchecked(),
            RS485_BOUD_RATE,
            MODBUS_TIMER.as_mut().unwrap_unchecked(),
            DATA_STORAGE.as_mut().unwrap_unchecked(),
        )
    };
    ```

4. Работа
    ```rust
    ...
    rtu.enable();
    loop {
        rtu.poll();

        ...

        cortex_m::asm::wfi();
    }
    ```

5. Прерывания
    * UART: Вызвать соответсвенно методы `rtu.on_tx()` или `rtu.on_rx()`
    * Timer: Вызвать `rtu.on_timer()`

6. Данные - Стандартные коолбеки ведут к вызоват методов трейта `DataInterface`.


## TODO
* [_] Реализовать ModbusASCII
* [_] Реализовать ModbusTCP